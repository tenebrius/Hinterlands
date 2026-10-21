//! Events (v4, D57): which shocks fall due this year. In `Historical` mode
//! (`events_mode` 1, year 0 = 1100) the calendar is the Great Famine
//! (`famine_year`, for `famine_years`), the Black Death (`plague_year`) and
//! two recurrences (`plague_recur_1`, `plague_recur_2`) at `plague_recur_share`
//! of its mortality. In `Random` mode (2) each kind is drawn yearly from the
//! run's `Rng` at a mean interval. Off (0) does nothing and draws nothing.
//! Writes: the event queue only.

use crate::event::Event;
use crate::params::Params;
use crate::state::State;
use crate::terrain::Terrain;
use crate::Rng;
use rand::Rng as _;

pub fn run(state: &mut State, _t: &Terrain, p: &Params, rng: &mut Rng) {
    let y = state.year;
    let (famine, plague) = match p.events_mode {
        1 => {
            let plague = if y == p.plague_year {
                Some(p.plague_mortality)
            } else if y == p.plague_recur_1 || y == p.plague_recur_2 {
                Some(p.plague_mortality * p.plague_recur_share)
            } else {
                None
            };
            (y == p.famine_year, plague)
        }
        2 if y > 0 => {
            let famine = rng.gen::<f32>() * p.famine_interval.max(1.0) < 1.0;
            let plague = rng.gen::<f32>() * p.plague_interval.max(1.0) < 1.0;
            (famine, plague.then_some(p.plague_mortality))
        }
        _ => (false, None),
    };
    if famine {
        state.events.push(Event::Famine {
            years: p.famine_years,
            loss: p.famine_loss,
        });
    }
    if let Some(mean) = plague {
        // The share is drawn per settlement, uniform within the spread around
        // the mean (scaled down with it for a recurrence), and taken in equal
        // yearly parts: `1 − (1 − m)^(1/years)` a year.
        let spread = p.plague_spread * mean / p.plague_mortality.max(1e-6);
        let years = p.plague_years.max(1) as f32;
        let shares = state
            .settlements
            .iter()
            .map(|s| {
                let m = (mean + spread * (2.0 * rng.gen::<f32>() - 1.0)).clamp(0.0, 0.95);
                (s.id, 1.0 - libm::powf(1.0 - m, 1.0 / years))
            })
            .collect();
        state.events.push(Event::Plague {
            shares,
            years: p.plague_years,
            first: true,
        });
    }
}
