//! Cultivation and fuel (von Thünen; VEP calorie-cost placement). Each settlement
//! claims free cells in order of walking cost until produce, less the marketed
//! share, covers need times the margin, and releases its farthest cells when produce exceeds the target by the
//! hysteresis share. Fuel is cut nearest first, for households and, under v2, for
//! the iron sites a settlement holds (D38). A settlement with a stream or river
//! among its fields has a mill, which adds `mill_bonus` to its produce (D56).
//! In a famine year (D57) the harvest is cut by the famine's loss after the
//! fields are chosen, and under local harvests (v6 W1) each field's yield is
//! multiplied by its cell's multiplier for the year. Writes: owner, fields, forest, food_prod, mill,
//! resource_out, sites' fuel_got.

use crate::params::Params;
use crate::state::{State, NONE};
use crate::terrain::{Resource, Terrain};
use crate::Rng;

pub fn run(state: &mut State, t: &Terrain, p: &Params, _rng: &mut Rng) {
    if !p.cultivation_enabled {
        return;
    }
    let ppc = p.people_per_prime_cell;
    let famine = state.year < state.famine_until;
    let famine_loss = state.famine_loss;
    let State {
        settlements,
        owner,
        forest,
        sites,
        harvest,
        ..
    } = state;

    for s in settlements.iter_mut() {
        // Enough to eat after the marketed share leaves, with the margin on top.
        // Under S2 the rent is the share that leaves (D78).
        let share = if p.market_grain {
            p.rent_share
        } else {
            p.market_share
        };
        // Market people buy their grain at market; the fields feed the
        // farmers (D79, D92).
        let fed = (s.pop - s.market.max(0.0)).max(0.0) * p.food_per_person;
        let target = fed * p.cult_margin / (1.0 - share).max(0.05);
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
        // A mill on a stream or river among the fields (D56).
        let cell_km2 = t.grid.cell_km * t.grid.cell_km;
        s.mill = p.mills
            && s.fields.iter().any(|&c| {
                let c = c as usize;
                t.is_river(c) && t.flux[c] * cell_km2 >= p.mill_area
            });
        // Local harvests (v6 W1): the year's weather, after the fields are chosen.
        if !harvest.is_empty() {
            produce = s
                .fields
                .iter()
                .map(|&c| t.food_yield[c as usize] * ppc * harvest[c as usize])
                .sum();
        }
        s.food_prod = if s.mill {
            produce * (1.0 + p.mill_bonus)
        } else {
            produce
        };
        // A famine year (D57): the harvest fails after the fields are chosen.
        if famine {
            s.food_prod *= 1.0 - famine_loss;
        }
        // Under v2 resource sites produce through `production`; the flat
        // resource output is retired.
        s.resource_out = if p.goods_enabled { 0.0 } else { 1.0 }
            * s.fields
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

    // Iron sites burn charcoal from their holder's woods (Weber: smelting sits
    // at the fuel).
    for site in sites.iter_mut() {
        site.fuel_got = 0.0;
        if !p.goods_enabled || site.fuel_want <= 0.0 {
            continue;
        }
        let Ok(k) = settlements.binary_search_by_key(&site.holder, |s| s.id) else {
            continue;
        };
        let mut demand = site.fuel_want;
        for &c in &settlements[k].reach.fuel_order {
            if demand <= 0.0 {
                break;
            }
            let ci = c as usize;
            // Coppice (D51): the woods are cut only down to the floor.
            let floor = if p.coppice { p.coppice_floor } else { 0.0 };
            let take = (forest[ci] - floor)
                .max(0.0)
                .min(p.max_fuel_take)
                .min(demand);
            forest[ci] -= take;
            demand -= take;
        }
        site.fuel_got = site.fuel_want - demand;
    }
}
