//! Cultivation and fuel (von Thünen; VEP calorie-cost placement). Each settlement
//! claims free cells in order of walking cost until produce, less the marketed
//! share, covers need times the margin, and releases its farthest cells when produce exceeds the target by the
//! hysteresis share. Writes: owner, fields, forest, food_prod, resource_out.

use crate::params::Params;
use crate::state::{State, NONE};
use crate::terrain::{Resource, Terrain};
use crate::Rng;

pub fn run(state: &mut State, t: &Terrain, p: &Params, _rng: &mut Rng) {
    if !p.cultivation_enabled {
        return;
    }
    let ppc = p.people_per_prime_cell;
    let State {
        settlements,
        owner,
        forest,
        ..
    } = state;

    for s in settlements.iter_mut() {
        // Enough to eat after the marketed share leaves, with the margin on top.
        let target = s.need(p) * p.cult_margin / (1.0 - p.market_share).max(0.05);
        let mut produce: f32 = s
            .fields
            .iter()
            .map(|&c| t.food_yield[c as usize] * ppc)
            .sum();
        if produce < target {
            for &c in &s.reach.field_order {
                if produce >= target {
                    break;
                }
                let ci = c as usize;
                if owner[ci] == NONE && t.is_land(ci) && t.food_yield[ci] >= p.min_field_yield {
                    owner[ci] = s.id;
                    s.fields.push(c);
                    produce += t.food_yield[ci] * ppc;
                }
            }
        } else {
            while let Some(&last) = s.fields.last() {
                let y = t.food_yield[last as usize] * ppc;
                if produce - y >= target * (1.0 + p.release_hysteresis) {
                    s.fields.pop();
                    owner[last as usize] = NONE;
                    produce -= y;
                } else {
                    break;
                }
            }
        }
        s.food_prod = produce;
        s.resource_out = s
            .fields
            .iter()
            .filter(|&&c| t.resource[c as usize] != Resource::None)
            .count() as f32
            * p.resource_output;
    }

    // Fields are cleared; everything else regrows toward its natural cover.
    for i in 0..t.n() {
        if !t.is_land(i) {
            continue;
        }
        if owner[i] != NONE {
            forest[i] = (forest[i] - p.clear_rate).max(0.0);
        } else {
            forest[i] = (forest[i] + p.regrow_rate).min(t.forest0[i]);
        }
    }

    // Fuel is cut nearest first.
    for s in settlements.iter() {
        let mut demand = s.pop * p.fuel_per_capita;
        for &c in &s.reach.fuel_order {
            if demand <= 0.0 {
                break;
            }
            let ci = c as usize;
            let take = forest[ci].min(p.max_fuel_take).min(demand);
            forest[ci] -= take;
            demand -= take;
        }
    }
}
