//! Growth: natural increase from the food ratio, plus the Rihll–Wilson inflow term
//! `epsilon * (I_j - K * P_j)` (Rihll & Wilson 1987; Harris & Wilson 1978).
//! Inflow is the food the settlement has (own produce kept plus food received),
//! plus `through_weight` times non-food trade received and trade passing through.
//! Writes: pop, food_ratio, last_fed, low_years, smoothed_loss, history.

use crate::params::Params;
use crate::state::State;
use crate::terrain::Terrain;
use crate::Rng;

pub fn run(state: &mut State, _t: &Terrain, p: &Params, _rng: &mut Rng) {
    let year = state.year;
    for s in state.settlements.iter_mut() {
        let need = s.need(p).max(1e-6);
        let food = s.food_prod + s.food_in - s.food_out;
        let ratio = food / need;
        s.food_ratio = ratio;
        if ratio >= p.fed_ratio {
            s.last_fed = year;
        }
        if p.growth_enabled {
            // Natural term: r_grow at a full surplus, zero when exactly fed, and a
            // shortfall loss spread over `loss_years`.
            let capped = ratio.min(p.food_cap);
            let gain = if capped > 1.0 {
                p.r_grow * (capped - 1.0) / (p.food_cap - 1.0).max(1e-6)
            } else {
                0.0
            };
            let loss = if ratio < 1.0 {
                p.r_shrink * (1.0 - ratio) * s.pop
            } else {
                0.0
            };
            s.smoothed_loss += (loss - s.smoothed_loss) / p.loss_years.max(1.0);
            let natural = gain * s.pop - s.smoothed_loss;

            let inflow = food + p.through_weight * (s.trade_in + s.through);
            let structural = p.epsilon * (inflow - p.k_maint * p.food_per_person * s.pop);

            let delta = (natural + structural).min(p.max_growth * s.pop);
            s.pop = (s.pop + delta).max(0.0);
        }
        if s.pop < p.fail_pop {
            s.low_years += 1;
        } else {
            s.low_years = 0;
        }
        s.history.push(s.pop);
        if s.history.len() > 51 {
            s.history.remove(0);
        }
    }
}
