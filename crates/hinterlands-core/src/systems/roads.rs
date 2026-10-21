//! Roads (Dalgaard et al. 2022; the Azgaar generator loop, simplified). Every
//! `road_interval` years the largest trade flows whose path is not yet all road
//! have their land cells marked as road, and the cost grid is rebuilt.
//! Roads never decay in v1. With `traffic_roads` (D54) roads follow traffic
//! instead: each land edge keeps an average of the trade value crossing it;
//! an edge whose load reaches `road_build_load` becomes road at full
//! condition, and a road whose load stays below `road_keep_load` loses
//! condition over `road_decay_years` and goes at zero (Dalgaard et al. 2022:
//! used Roman roads persisted). Initial roads start as road edges. A cell is
//! road while a road edge touches it. Writes: road, road_edges, road_load,
//! road_cond, cost.

use crate::params::Params;
use crate::state::State;
use crate::terrain::Terrain;
use crate::Rng;

pub fn run(state: &mut State, t: &Terrain, p: &Params, _rng: &mut Rng) {
    if !p.roads_enabled || state.year == 0 || state.year % p.road_interval.max(1) != 0 {
        return;
    }
    if p.traffic_roads {
        return by_traffic(state, t, p);
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
        }
        built += 1;
        changed = true;
    }
    if changed {
        state.rebuild_costs(t);
    }
}

/// Roads that follow traffic (D54).
fn by_traffic(state: &mut State, t: &Terrain, p: &Params) {
    let n = t.n() as u32;
    let g = t.grid;
    // The first step turns the initial road cells into road edges: each cell
    // joins its road neighbours along the axes, and diagonally where no axis
    // neighbour links the two.
    if state.road_cond.is_empty() && state.road_load.is_empty() {
        for i in 0..t.n() {
            if !state.road[i] || !t.is_land(i) {
                continue;
            }
            for d in 0..8 {
                let Some(j) = g.neighbor(i, d) else { continue };
                if j <= i || !state.road[j] || !t.is_land(j) {
                    continue;
                }
                let (dx, dy) = crate::grid::DIRS[d];
                if dx != 0 && dy != 0 {
                    let (x, y) = g.xy(i);
                    let (a, b) = (g.idx(x + dx, y), g.idx(x, y + dy));
                    if state.road[a] || state.road[b] {
                        continue;
                    }
                }
                state.road_cond.insert((i as u32, j as u32), 1.0);
            }
        }
    }
    // This step's load on each edge.
    let mut now: std::collections::BTreeMap<(u32, u32), f32> = Default::default();
    for f in &state.flows {
        if f.amount <= 0.0 {
            continue;
        }
        let Some(si) = state.index_of(f.from) else {
            continue;
        };
        let path = &state.settlements[si].reach.partners[f.partner_idx as usize].path;
        for w in path.windows(2) {
            if w[0] < n && w[1] < n && w[0] != w[1] {
                *now.entry((w[0].min(w[1]), w[0].max(w[1]))).or_insert(0.0) += f.amount;
            }
        }
    }
    // Average over `road_load_years`, a step of `road_interval` years at a time.
    let k = (p.road_interval as f32 / p.road_load_years.max(1.0)).min(1.0);
    for (e, v) in state.road_load.iter_mut() {
        *v += (now.get(e).copied().unwrap_or(0.0) - *v) * k;
    }
    for (e, v) in now {
        state.road_load.entry(e).or_insert(v * k);
    }
    state.road_load.retain(|_, v| *v > 1e-3);
    // Build, keep or decay.
    let fall = p.road_interval as f32 / p.road_decay_years.max(1.0);
    for (&e, &load) in &state.road_load {
        if load >= p.road_build_load {
            state.road_cond.insert(e, 1.0);
        }
    }
    let load = &state.road_load;
    state.road_cond.retain(|e, c| {
        if load.get(e).copied().unwrap_or(0.0) < p.road_keep_load {
            *c -= fall;
        }
        *c > 0.0
    });
    let mut road = vec![false; t.n()];
    for &(a, b) in state.road_cond.keys() {
        road[a as usize] = true;
        road[b as usize] = true;
    }
    state.road_edges = state.road_cond.keys().copied().collect();
    if road != state.road {
        state.road = road;
        state.rebuild_costs(t);
    }
}
