//! Gravity trade (Wilson 1970; Huff 1964; Barjamovic et al. 2019). Each
//! settlement's surplus is split over its partners in proportion to
//! `P_j^alpha * exp(-beta * c_ij)`; food surplus moves the same way, but only to
//! settlements short of food. Flows are assigned to the stored least-cost paths,
//! giving the load on each cell and the route junctions (hubs).
//! Writes: flows, cell_load, hub, trade_in, food_in, food_out, through.

use crate::cost::{back_dir, step_dir};
use crate::params::Params;
use crate::state::{Flow, State};
use crate::terrain::Terrain;
use crate::Rng;

pub fn run(state: &mut State, t: &Terrain, p: &Params, _rng: &mut Rng) {
    for s in state.settlements.iter_mut() {
        s.trade_in = 0.0;
        s.food_in = 0.0;
        s.food_out = 0.0;
        s.through = 0.0;
    }
    state.flows.clear();
    state.cell_load.iter_mut().for_each(|x| *x = 0.0);
    state.hub.iter_mut().for_each(|x| *x = false);
    if !p.trade_enabled {
        return;
    }

    let m = state.settlements.len();
    let mut idx_of = vec![u32::MAX; state.next_id as usize];
    for (i, s) in state.settlements.iter().enumerate() {
        idx_of[s.id as usize] = i as u32;
    }
    let attract: Vec<f32> = state
        .settlements
        .iter()
        .map(|s| libm::powf(s.pop.max(0.0), p.alpha))
        .collect();
    let mut deficit: Vec<f32> = state
        .settlements
        .iter()
        .map(|s| (s.need(p) - s.food_prod).max(0.0))
        .collect();
    let mut trade_in = vec![0f32; m];
    let mut food_in = vec![0f32; m];
    let mut food_out = vec![0f32; m];
    let mut flows = Vec::new();
    let mut weights: Vec<(usize, u32, f32, f32)> = Vec::new();

    for i in 0..m {
        let s = &state.settlements[i];
        let food_surplus = (s.food_prod - s.need(p)).max(0.0);
        let out = food_surplus + s.resource_out;
        if out <= 0.0 {
            continue;
        }
        weights.clear();
        for (k, partner) in s.reach.partners.iter().enumerate() {
            let Some(&j) = idx_of.get(partner.id as usize) else {
                continue;
            };
            if j == u32::MAX {
                continue;
            }
            let w = attract[j as usize] * libm::expf(-p.beta * partner.cost);
            weights.push((j as usize, k as u32, w, partner.cost));
        }
        let total: f32 = weights.iter().map(|w| w.2).sum();
        if total <= 0.0 {
            continue;
        }
        let first_flow = flows.len();
        for &(j, k, w, _) in &weights {
            let amount = out * w / total;
            trade_in[j] += amount;
            flows.push(Flow {
                from: s.id,
                to: state.settlements[j].id,
                amount,
                food: 0.0,
                partner_idx: k,
            });
        }

        // Food goes only where it is short, by the same gravity weights.
        if food_surplus > 0.0 {
            let food_total: f32 = weights
                .iter()
                .filter(|w| deficit[w.0] > 0.0)
                .map(|w| w.2)
                .sum();
            if food_total > 0.0 {
                for (n, &(j, _, w, c)) in weights.iter().enumerate() {
                    if deficit[j] <= 0.0 {
                        continue;
                    }
                    let keep = (1.0 - p.transport_loss * c).max(0.0);
                    if keep <= 0.0 {
                        continue;
                    }
                    let send = (food_surplus * w / food_total).min(deficit[j] / keep);
                    let delivered = send * keep;
                    deficit[j] -= delivered;
                    food_in[j] += delivered;
                    food_out[i] += send;
                    flows[first_flow + n].food = send;
                }
            }
        }
    }

    // Assign flows to paths: load per cell, and the directions routes use there.
    let grid = t.grid;
    let mut dirs = vec![0u16; t.n()];
    for f in &flows {
        let s = &state.settlements[idx_of[f.from as usize] as usize];
        let path = &s.reach.partners[f.partner_idx as usize].path;
        let mark = f.amount >= p.hub_min_flow;
        for q in 1..path.len().saturating_sub(1) {
            let cell = state.cost.node_cell(path[q]);
            state.cell_load[cell] += f.amount;
            if mark {
                let din = back_dir(step_dir(&grid, &state.cost, path[q - 1], path[q]));
                let dout = step_dir(&grid, &state.cost, path[q], path[q + 1]);
                dirs[cell] |= (1 << din) | (1 << dout);
            }
        }
    }
    for (i, &mask) in dirs.iter().enumerate() {
        state.hub[i] = t.is_land(i) && mask.count_ones() >= 3;
    }

    for (i, s) in state.settlements.iter_mut().enumerate() {
        s.trade_in = trade_in[i];
        s.food_in = food_in[i];
        s.food_out = food_out[i];
        s.through = state.cell_load[s.anchor as usize];
    }
    state.flows = flows;
}
