//! Initial conditions: starting hamlets on the cells with the most free food
//! within the cultivation radius, with water nearby, spaced apart.

use crate::params::Params;
use crate::state::{LogEntry, LogKind, State};
use crate::systems::founding::{free_food_near, has_water_supply};
use crate::terrain::Terrain;

pub fn populate(state: &mut State, t: &Terrain, p: &Params) {
    let mut cands: Vec<(f32, usize)> = Vec::new();
    for cell in 0..t.n() {
        if t.is_land(cell) && has_water_supply(t, p, cell) {
            cands.push((free_food_near(state, t, p, cell), cell));
        }
    }
    cands.sort_by(|a, b| b.0.total_cmp(&a.0).then(a.1.cmp(&b.1)));
    let mut chosen: Vec<usize> = Vec::new();
    for &(_, cell) in &cands {
        if chosen.len() as u32 >= p.n_initial {
            break;
        }
        if chosen
            .iter()
            .all(|&c| t.grid.dist_km(c, cell) >= p.initial_spacing_km)
        {
            chosen.push(cell);
        }
    }
    for cell in chosen {
        let id = state.add_settlement(t, p, cell as u32, p.initial_pop, true);
        state.log.push(LogEntry {
            year: 0,
            settlement: id,
            name: state.name_of(id),
            anchor: cell as u32,
            pop: p.initial_pop,
            kind: LogKind::Initial,
        });
    }
}
