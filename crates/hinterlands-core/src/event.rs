//! The event queue (v4, D57). The `events` system decides each year which
//! events fall due and queues them; they are applied at the start of the year,
//! and an event that lasts several years queues its own next part.
//! Writes, through `apply` only: famine state, settlements' pop and market,
//! silted harbour costs, the sea cost ratio and the cost grid, planted
//! settlements, the queue and the log.

use crate::params::Params;
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
    /// The harbour of the settlement silts up: every land–water step within a
    /// cell of its anchor costs `cost` more, for good (D60).
    Silt { settlement: u32, cost: f32 },
    /// A lord plants a town at `cell` with `pop` people and market size
    /// `market` (D63).
    Plant { cell: u32, pop: f32, market: f32 },
    /// Better ships (v5, D65): sea cost per km becomes `sea`; the cost grid is
    /// rebuilt. `announce` puts the change in the chronicle.
    Ships { sea: f32, announce: bool },
}

impl Event {
    pub fn apply(&self, state: &mut State, t: &Terrain, p: &Params) {
        match self {
            Event::Ships { sea, announce } => {
                state.transport.sea = *sea;
                state.rebuild_costs(t);
                if *announce {
                    state.log.push(LogEntry {
                        year: state.year,
                        settlement: crate::state::NONE,
                        name: String::new(),
                        anchor: 0,
                        pop: 0.0,
                        kind: LogKind::Ships,
                    });
                }
            }
            Event::Plant { cell, pop, market } => {
                let id = state.add_settlement(t, p, *cell, *pop, false);
                state.tally.founded += 1;
                if let Some(i) = state.index_of(id) {
                    state.settlements[i].market = market.min(*pop);
                    state.settlements[i].planted = true;
                }
                state.log.push(LogEntry {
                    year: state.year,
                    settlement: id,
                    name: state.name_of(id),
                    anchor: *cell,
                    pop: *pop,
                    kind: LogKind::Planted,
                });
            }
            Event::Silt { settlement, cost } => {
                let Some(i) = state.index_of(*settlement) else {
                    return;
                };
                if state.silt.is_empty() {
                    state.silt = vec![0.0; t.n()];
                }
                let a = state.settlements[i].anchor as usize;
                state.silted_ports.push(a as u32);
                for c in std::iter::once(a).chain((0..8).filter_map(|d| t.grid.neighbor(a, d))) {
                    state.silt[c] += cost;
                }
                let s = &state.settlements[i];
                let entry = LogEntry {
                    year: state.year,
                    settlement: s.id,
                    name: s.name.clone(),
                    anchor: s.anchor,
                    pop: s.pop,
                    kind: LogKind::Silted,
                };
                state.log.push(entry);
                // Paths pick up the new costs at their next scheduled refresh,
                // as after a road step.
                state.rebuild_costs(t);
            }
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
                if *first {
                    state.last_plague = state.year;
                }
                for &(id, y) in shares {
                    let Some(i) = state.index_of(id) else {
                        continue;
                    };
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

/// Year 0 of the historical calendar (D57).
pub const HISTORICAL_BASE: u32 = 1100;

/// The year as shown: the calendar year in historical mode, else the run year.
pub fn shown_year(p: &Params, year: u32) -> u32 {
    if p.events_mode == 1 {
        year + HISTORICAL_BASE
    } else {
        year
    }
}
