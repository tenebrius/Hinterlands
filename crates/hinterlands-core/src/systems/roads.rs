//! Roads (Dalgaard et al. 2022; the Azgaar generator loop, simplified). Every
//! `road_interval` years the largest trade flows whose path is not yet all road
//! have their land cells marked as road, and the cost grid is rebuilt.
//! Roads never decay in v1. Writes: road, road_edges, cost.

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
