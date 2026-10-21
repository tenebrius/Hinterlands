//! The event queue (v4, D57). The `events` system decides each year which
//! events fall due and queues them; they are applied at the start of the year,
//! and an event that lasts several years queues its own next part.
//! Writes, through `apply` only: famine state, settlements' pop and market,
//! the queue and the log.

use crate::state::{LogEntry, LogKind, State};
use crate::terrain::Terrain;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Event {
    /// Yields fall by `loss` for `years` years, starting this year.
    Famine { years: u32, loss: f32 },
    /// Plague: each settlement listed loses its yearly share of people and of
    /// market size for `years` more years, this one included; `first` marks
    /// the year it strikes, for the chronicle.
    Plague {
        shares: Vec<(u32, f32)>,
        years: u32,
        first: bool,
    },
}

impl Event {
    pub fn apply(&self, state: &mut State, _t: &Terrain) {
        match self {
            Event::Famine { years, loss } => {
                state.famine_until = state.year + years;
                state.famine_loss = *loss;
                state.log.push(LogEntry {
                    year: state.year,
                    settlement: crate::state::NONE,
                    name: String::new(),
                    anchor: 0,
                    pop: 0.0,
                    kind: LogKind::Famine { years: *years },
                });
            }
            Event::Plague {
                shares,
                years,
                first,
            } => {
                let years = (*years).max(1);
                for &(id, y) in shares {
                    let Some(i) = state.index_of(id) else { continue };
                    let s = &mut state.settlements[i];
                    let before = s.pop;
                    s.pop -= before * y;
                    s.market *= 1.0 - y;
                    if *first {
                        // The people it will take over all its years.
                        let lost_all = before * (1.0 - libm::powf(1.0 - y, years as f32));
                        let entry = LogEntry {
                            year: state.year,
                            settlement: id,
                            name: s.name.clone(),
                            anchor: s.anchor,
                            pop: s.pop,
                            kind: LogKind::Plague { lost: lost_all },
                        };
                        state.log.push(entry);
                    }
                }
                if years > 1 {
                    state.events.push(Event::Plague {
                        shares: shares.clone(),
                        years: years - 1,
                        first: false,
                    });
                }
            }
        }
    }
}
