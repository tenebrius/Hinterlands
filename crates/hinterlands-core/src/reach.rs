//! Per-settlement travel costs: the bounded Dijkstra from each anchor, the
//! cultivation and fuel orders, and the stored paths to trading partners.

use crate::params::Params;
use crate::state::{Partner, Reach, State, COST_SCALE, COST_UNREACHED};
use crate::terrain::Terrain;

/// Recompute every settlement's reach.
pub fn refresh_all(state: &mut State, t: &Terrain, p: &Params) {
    for idx in 0..state.settlements.len() {
        refresh_one(state, t, p, idx);
    }
}

/// Refresh the share of settlements due this year. Each settlement is refreshed
/// every `cost_refresh_years`, staggered by id so the work is spread evenly.
pub fn refresh_due(state: &mut State, t: &Terrain, p: &Params) {
    let k = p.cost_refresh_years.max(1);
    if state.year == 0 {
        return;
    }
    for idx in 0..state.settlements.len() {
        if state.settlements[idx].id % k == state.year % k {
            refresh_one(state, t, p, idx);
        }
    }
}

pub fn refresh_one(state: &mut State, t: &Terrain, p: &Params, idx: usize) {
    let n = t.n();
    let mut dj = std::mem::take(&mut state.scratch);
    let anchor = state.settlements[idx].anchor;
    let self_id = state.settlements[idx].id;
    let mut reach = Reach {
        cost_field: vec![COST_UNREACHED; n],
        refreshed: state.year,
        ..Reach::default()
    };

    // Everything within trading reach, on both layers.
    dj.run(&state.cost, anchor, p.c_max, false);
    for &node in dj.touched() {
        let cell = state.cost.node_cell(node);
        let q = (dj.dist[node as usize] * COST_SCALE)
            .round()
            .min((COST_UNREACHED - 1) as f32) as u16;
        if q < reach.cost_field[cell] {
            reach.cost_field[cell] = q;
        }
    }
    for other in &state.settlements {
        if other.id == self_id {
            continue;
        }
        let c = dj.dist[other.anchor as usize];
        if c <= p.c_max {
            reach.partners.push(Partner {
                id: other.id,
                cost: c,
                path: dj.path_to(other.anchor),
            });
        }
    }

    // Fields and fuel are reached on foot.
    dj.run(&state.cost, anchor, p.r_fuel.max(p.r_cult), true);
    let mut near: Vec<(u32, u32)> = dj
        .touched()
        .iter()
        .filter(|&&node| (node as usize) < n)
        .map(|&node| (dj.dist[node as usize].to_bits(), node))
        .collect();
    near.sort_unstable();
    for &(dbits, cell) in &near {
        let d = f32::from_bits(dbits);
        if d <= p.r_cult {
            reach.field_order.push(cell);
        }
        if d <= p.r_fuel {
            reach.fuel_order.push(cell);
        }
    }

    state.settlements[idx].reach = reach;
    state.scratch = dj;
}
