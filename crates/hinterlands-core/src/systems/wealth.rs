//! Wealth raises stocks (D3, D158). Each place's real income per head, in
//! grain units, is the food its own fields give its people (uncapped, D162)
//! plus the coin its grain, wool and cloth earned over the grain price; it is
//! averaged over `wealth_years`. Against the region's 1300-1339 mean y0, a
//! place's stocks of home cloth, household iron and building stone are scaled
//! by (y / y0)^e the next year. Writes `wealth_y`, `wealth_mult` and the
//! reference.

use crate::params::Params;
use crate::state::State;
use crate::terrain::Terrain;
use crate::Rng;

/// First and last years of the reference (1300-1339).
const REF_FROM: u32 = 200;
const REF_TO: u32 = 239;

pub fn run(state: &mut State, _t: &Terrain, p: &Params, _rng: &mut Rng) {
    if !p.wealth_stocks {
        return;
    }
    let year = state.year;
    let (mut sum_y, mut sum_pop) = (0f32, 0f32);
    for s in state.settlements.iter_mut() {
        let pop = s.pop.max(0.0);
        if pop <= 0.0 {
            continue;
        }
        // Own food counts uncapped (D162): food above need is income in kind.
        let own = s.food_prod.max(0.0);
        // Coin over the local grain price is grain units (D97).
        let price = if s.price[0] > 0.0 { s.price[0] } else { p.port_world_price };
        let earned = (s.grain_earned + s.wool_got + s.cloth_margin).max(0.0);
        let y = (own + earned / price.max(1e-6)) / pop;
        s.wealth_y = if s.wealth_y > 0.0 {
            s.wealth_y + (y - s.wealth_y) / p.wealth_years.max(1.0)
        } else {
            y
        };
        sum_y += y * pop;
        sum_pop += pop;
    }
    if (REF_FROM..=REF_TO).contains(&year) && sum_pop > 0.0 {
        state.wealth_ref.0 += sum_y / sum_pop;
        state.wealth_ref.1 += 1;
    }
    let y0 = if state.wealth_ref.1 > 0 && year > REF_TO {
        state.wealth_ref.0 / state.wealth_ref.1 as f32
    } else {
        0.0
    };
    for s in state.settlements.iter_mut() {
        s.wealth_mult = if y0 > 0.0 && s.wealth_y > 0.0 {
            libm::powf(s.wealth_y / y0, p.wealth_elast)
        } else {
            1.0
        };
    }
}
