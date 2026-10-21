//! Built-up footprints, for rendering only: nothing in the systems reads them.
//! A settlement covers `pop / density_per_cell` cells, filled outward from its
//! anchor by a fixed attractiveness (near the anchor, flat, by water, on a road),
//! never over a navigable river (a town centred on one keeps that cell).
//! Built cells stay built, so towns grow by accretion; a shrinking town loses its
//! least attractive edge, which stays a ruin for `ruin_years`.

use crate::params::Params;
use crate::state::{State, NONE};
use crate::terrain::Terrain;

fn attractiveness(state: &State, t: &Terrain, anchor: usize, cell: usize) -> f32 {
    let flat = if t.slope[cell] < 0.03 { 0.6 } else { 0.0 };
    let water = if t.touches(cell, |j| t.water[j] != crate::terrain::Water::None) {
        0.6
    } else {
        0.0
    };
    let road = if state.road[cell] { 0.8 } else { 0.0 };
    -t.grid.dist_km(anchor, cell) / t.grid.cell_km + flat + water + road
}

pub fn update(state: &mut State, t: &Terrain, p: &Params) {
    let g = t.grid;
    for r in state.ruin.iter_mut() {
        *r = r.saturating_sub(1);
    }
    // Cells of settlements that no longer exist become ruins.
    for i in 0..g.n() {
        let b = state.built[i];
        if b != NONE && state.index_of(b).is_none() {
            state.built[i] = NONE;
            state.ruin[i] = p.ruin_years.min(255) as u8;
        }
    }
    for k in 0..state.settlements.len() {
        let (id, anchor, pop) = {
            let s = &state.settlements[k];
            (s.id, s.anchor as usize, s.pop)
        };
        if state.built[anchor] == NONE {
            state.built[anchor] = id;
        }
        let target = ((pop / p.density_per_cell.max(1.0)).ceil() as usize).max(1);
        let mine: Vec<usize> = g
            .disk(anchor, 12.0)
            .filter(|&c| state.built[c] == id)
            .collect();
        let mut count = mine.len();
        if count < target {
            let mut frontier: Vec<(f32, usize)> = Vec::new();
            for &c in &mine {
                for d in 0..8 {
                    if let Some(j) = g.neighbor(c, d) {
                        // Towns grow along the banks, not over the navigable river.
                        if t.is_land(j)
                            && !t.is_nav(j)
                            && state.built[j] == NONE
                            && !frontier.iter().any(|f| f.1 == j)
                        {
                            frontier.push((attractiveness(state, t, anchor, j), j));
                        }
                    }
                }
            }
            frontier.sort_by(|a, b| b.0.total_cmp(&a.0).then(a.1.cmp(&b.1)));
            for (_, j) in frontier.into_iter().take(target - count) {
                state.built[j] = id;
                state.ruin[j] = 0;
                count += 1;
            }
        } else if count > target + 1 {
            let mut edge: Vec<(f32, usize)> = mine
                .iter()
                .filter(|&&c| c != anchor)
                .map(|&c| (attractiveness(state, t, anchor, c), c))
                .collect();
            edge.sort_by(|a, b| a.0.total_cmp(&b.0).then(b.1.cmp(&a.1)));
            for (_, c) in edge.into_iter().take(count - target) {
                state.built[c] = NONE;
                state.ruin[c] = p.ruin_years.min(255) as u8;
            }
        }
    }
}
