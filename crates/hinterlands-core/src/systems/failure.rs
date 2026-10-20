//! Failure. Most new markets fade (English market charters: 39% of 1300-era
//! markets survived to c. 1600). A settlement below `fail_pop` for
//! `fail_low_years`, or unfed for `fail_unfed_years`, is removed; its fields are
//! released and its people move to the nearest fed settlement.
//! Writes: settlements (removes), owner, tally, log.

use crate::params::Params;
use crate::state::{FailCause, LogEntry, LogKind, State, NONE};
use crate::terrain::Terrain;
use crate::Rng;

pub fn run(state: &mut State, _t: &Terrain, p: &Params, _rng: &mut Rng) {
    if !p.failure_enabled {
        return;
    }
    let year = state.year;
    let mut i = 0;
    while i < state.settlements.len() {
        let s = &state.settlements[i];
        let cause = if s.low_years >= p.fail_low_years {
            Some(FailCause::Depopulated)
        } else if year.saturating_sub(s.last_fed) >= p.fail_unfed_years {
            Some(FailCause::Starved)
        } else {
            None
        };
        let Some(cause) = cause else {
            i += 1;
            continue;
        };
        let dead = state.settlements.remove(i);
        for &c in &dead.fields {
            state.owner[c as usize] = NONE;
        }
        // Nearest partner that is fed this year takes the survivors.
        let mut partners: Vec<(f32, u32)> =
            dead.reach.partners.iter().map(|q| (q.cost, q.id)).collect();
        partners.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
        let moved_to = partners.iter().find_map(|&(_, id)| {
            let j = state.index_of(id)?;
            (state.settlements[j].food_ratio >= 1.0).then_some(j)
        });
        if let Some(j) = moved_to {
            state.settlements[j].pop += dead.pop;
        }
        if !dead.initial {
            state.tally.failed += 1;
        }
        state.log.push(LogEntry {
            year,
            settlement: dead.id,
            anchor: dead.anchor,
            pop: dead.pop,
            kind: LogKind::Failed {
                cause,
                moved_to: moved_to.map(|j| state.settlements[j].id),
            },
        });
    }
}
