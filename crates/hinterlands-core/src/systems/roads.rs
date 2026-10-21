//! Roads (Dalgaard et al. 2022; the Azgaar generator loop, simplified). Every
//! `road_interval` years the largest trade flows whose path is not yet all road
//! have their land cells marked as road, and the cost grid is rebuilt.
//! Roads never decay in v1. With `traffic_roads` (D54) each road cell has a
//! condition `q`: the top pairs' paths are raised to 1, every other road cell
//! whose trade load this step is below `road_keep_load` loses `road_decay`,
//! and a cell below 0.1 stops being road (Dalgaard et al. 2022: Roman roads
//! persisted where traffic kept them in use). Initial roads start at 1.
//! With `traffic_bridges` (D55) a road cell on a river or stream, not a ford,
//! that carries `bridge_load` for `bridge_steps` road steps in a row gets a
//! bridge, which is never removed. Writes: road, road_q, road_edges, bridge,
//! bridge_streak, cost.

use crate::params::Params;
use crate::state::State;
use crate::terrain::Terrain;
use crate::Rng;

pub fn run(state: &mut State, t: &Terrain, p: &Params, _rng: &mut Rng) {
    if !p.roads_enabled || state.year == 0 || state.year % p.road_interval.max(1) != 0 {
        return;
    }
    let mut order: Vec<usize> = (0..state.flows.len()).collect();
    order.sort_by(|&a, &b| {
        state.flows[b]
            .amount
            .total_cmp(&state.flows[a].amount)
            .then(a.cmp(&b))
    });
    let n = t.n() as u32;
    let mut built = 0;
    let mut changed = false;
    let mut raised = vec![false; if p.traffic_roads { t.n() } else { 0 }];
    for fi in order {
        if built >= p.road_pairs {
            break;
        }
        let f = &state.flows[fi];
        let Some(si) = state.index_of(f.from) else {
            continue;
        };
        let path = &state.settlements[si].reach.partners[f.partner_idx as usize].path;
        let land: Vec<usize> = path
            .iter()
            .filter(|&&node| node < n)
            .map(|&node| node as usize)
            .collect();
        if land.len() < 2 || land.iter().all(|&c| state.road[c]) {
            continue;
        }
        // The segments the road follows, for drawing: consecutive land nodes.
        for w in path.windows(2) {
            if w[0] < n && w[1] < n && w[0] != w[1] {
                let e = (w[0].min(w[1]), w[0].max(w[1]));
                if let Err(at) = state.road_edges.binary_search(&e) {
                    state.road_edges.insert(at, e);
                }
            }
        }
        for c in land {
            state.road[c] = true;
            if p.traffic_roads {
                state.road_q[c] = 1.0;
                raised[c] = true;
            }
        }
        built += 1;
        changed = true;
    }
    if p.traffic_roads || p.traffic_bridges {
        let load = cell_load(state, t);
        if p.traffic_roads {
            changed |= decay(state, p, &raised, &load);
        }
        if p.traffic_bridges {
            changed |= bridges(state, t, p, &load);
        }
    }
    if changed {
        state.rebuild_costs(t);
    }
}

/// The trade value crossing each land cell this year.
fn cell_load(state: &State, t: &Terrain) -> Vec<f32> {
    let n = t.n() as u32;
    let mut load = vec![0f32; t.n()];
    for f in &state.flows {
        if f.amount <= 0.0 {
            continue;
        }
        let Some(si) = state.index_of(f.from) else {
            continue;
        };
        for &node in &state.settlements[si].reach.partners[f.partner_idx as usize].path {
            if node < n {
                load[node as usize] += f.amount;
            }
        }
    }
    load
}

/// Fading of little-used roads (D54). Returns whether any condition changed.
fn decay(state: &mut State, p: &Params, raised: &[bool], load: &[f32]) -> bool {
    let mut changed = false;
    let mut gone = false;
    for c in 0..load.len() {
        if !state.road[c] || raised[c] || load[c] >= p.road_keep_load {
            continue;
        }
        state.road_q[c] -= p.road_decay;
        changed = true;
        if state.road_q[c] < 0.1 {
            state.road_q[c] = 0.0;
            state.road[c] = false;
            gone = true;
        }
    }
    if gone {
        let road = &state.road;
        state
            .road_edges
            .retain(|&(a, b)| road[a as usize] && road[b as usize]);
    }
    changed
}


/// Bridges at busy crossings (D55). Returns whether one was built.
fn bridges(state: &mut State, t: &Terrain, p: &Params, load: &[f32]) -> bool {
    let mut built = false;
    for c in 0..t.n() {
        if state.bridge[c] || !t.is_river(c) || t.free_crossing(c) {
            continue;
        }
        if state.road[c] && load[c] >= p.bridge_load {
            state.bridge_streak[c] = state.bridge_streak[c].saturating_add(1);
        } else {
            state.bridge_streak[c] = 0;
        }
        if u32::from(state.bridge_streak[c]) >= p.bridge_steps {
            state.bridge[c] = true;
            built = true;
        }
    }
    built
}
