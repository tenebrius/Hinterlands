//! Gravity trade (Wilson 1970; Huff 1964; Barjamovic et al. 2019), in the
//! Rihll–Wilson form: each settlement sends a fixed marketed share of its harvest,
//! plus its resource output, to markets in proportion to
//! `W_j^alpha * exp(-beta * c_ij)`, where `W_j` is the non-farming population, its own market included with weight `home_weight` (D13).
//! With `outside_enabled` (D25) the outside world is one more partner, of fixed
//! size `outside_size` and its own decay `outside_beta`: it buys marketed produce,
//! which leaves the map, and sends `outside_out` of goods to the settlements in
//! proportion to their pull, along the same paths.
//! Food arrives less a loss per unit cost. Flows are assigned to the stored
//! least-cost paths, giving the load on each cell and the route junctions (hubs).
//! Writes: flows, cell_load, hub, trade_in, food_in, food_out, through, market_in.

use crate::cost::{back_dir, step_dir};
use crate::params::Params;
use crate::state::{Flow, State, OUTSIDE};
use crate::terrain::Terrain;
use crate::Rng;

/// People beyond those the settlement's own harvest feeds: Rihll–Wilson's W.
pub fn urban_pop(s: &crate::state::Settlement, p: &Params) -> f32 {
    (s.pop - s.food_prod / p.food_per_person).max(0.0)
}

pub fn run(state: &mut State, t: &Terrain, p: &Params, _rng: &mut Rng) {
    for s in state.settlements.iter_mut() {
        s.trade_in = 0.0;
        s.food_in = 0.0;
        s.food_out = 0.0;
        s.through = 0.0;
        s.market_in = 0.0;
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
    // Market pull is the non-farming population (Rihll–Wilson's W): everyone
    // beyond those its own harvest feeds, plus a small seed.
    let attract: Vec<f32> = state
        .settlements
        .iter()
        .map(|s| libm::powf(urban_pop(s, p) + p.urban_seed, p.alpha))
        .collect();
    let mut trade_in = vec![0f32; m];
    let mut food_in = vec![0f32; m];
    let mut food_out = vec![0f32; m];
    let mut market_in = vec![0f32; m];
    let mut flows = Vec::new();
    // (destination index, partner index, weight, cost); usize::MAX is the outside.
    let mut weights: Vec<(usize, u32, f32, f32)> = Vec::new();
    let outside_pull = libm::powf(p.outside_size, p.alpha);
    // Imports from the outside: (settlement index, partner index, weight).
    let mut imports: Vec<(usize, u32, f32)> = Vec::new();

    for i in 0..m {
        let s = &state.settlements[i];
        if p.outside_enabled {
            if let Some(k) = s.reach.partners.iter().position(|q| q.id == OUTSIDE) {
                let c = s.reach.partners[k].cost;
                imports.push((i, k as u32, attract[i] * libm::expf(-p.outside_beta * c)));
            }
        }
        let food = s.food_prod * p.market_share;
        let out = food + s.resource_out;
        if out <= 0.0 {
            continue;
        }
        weights.clear();
        // A home-market weight keeps grain at home when markets are far or small.
        let mut total = attract[i] * p.home_weight;
        for (k, partner) in s.reach.partners.iter().enumerate() {
            if partner.id == OUTSIDE {
                if p.outside_enabled {
                    let w = outside_pull * libm::expf(-p.outside_beta * partner.cost);
                    total += w;
                    weights.push((usize::MAX, k as u32, w, partner.cost));
                }
                continue;
            }
            let Some(&j) = idx_of.get(partner.id as usize) else {
                continue;
            };
            if j == u32::MAX {
                continue;
            }
            let w = attract[j as usize] * libm::expf(-p.beta * partner.cost);
            total += w;
            weights.push((j as usize, k as u32, w, partner.cost));
        }
        if total <= 0.0 {
            continue;
        }
        // Produce sold at the home market counts towards that market's turnover.
        market_in[i] += food * attract[i] * p.home_weight / total;
        for &(j, k, w, c) in &weights {
            let share = w / total;
            let amount = out * share;
            let sent = food * share;
            if j == usize::MAX {
                // Sold off the map.
                food_out[i] += sent;
                flows.push(Flow {
                    from: s.id,
                    to: OUTSIDE,
                    amount,
                    food: sent,
                    partner_idx: k,
                });
                continue;
            }
            let delivered = sent * (1.0 - p.transport_loss * c).max(0.0);
            trade_in[j] += amount - sent;
            food_out[i] += sent;
            food_in[j] += delivered;
            market_in[j] += delivered;
            flows.push(Flow {
                from: s.id,
                to: state.settlements[j].id,
                amount,
                food: sent,
                partner_idx: k,
            });
        }
    }

    // Goods from the outside, shared by pull and closeness; recorded on the
    // importer's own path to the sea edge.
    let total_import: f32 = imports.iter().map(|x| x.2).sum();
    if total_import > 0.0 {
        for &(i, k, w) in &imports {
            let amount = p.outside_out * w / total_import;
            trade_in[i] += amount;
            flows.push(Flow {
                from: state.settlements[i].id,
                to: OUTSIDE,
                amount,
                food: 0.0,
                partner_idx: k,
            });
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
        s.market_in = market_in[i];
        s.through = state.cell_load[s.anchor as usize];
    }
    state.flows = flows;
}
