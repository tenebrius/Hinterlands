//! The chronicle: what happened, in plain prose, newest first. Foundings and
//! failures come from the core's log; growth into a village, town or city, a
//! new largest place and new roads are noticed by the viewer. Filters pick
//! what to show; by default the many hamlets that are founded and fail without
//! ever growing are left out, so the story reads. Shocks and the outside world
//! (v4, v5) have their own filter. Years are calendar years in historical mode.
//! Clicking an entry selects the place and brings it into view.

use super::Panel;
use crate::camera::Camera;
use crate::view::{Msg, NoteKind, ViewState};
use hinterlands_core::state::{FailCause, LogEntry, LogKind, Rank};

/// Which kinds of entry are shown.
pub struct Chronicle {
    growth: bool,
    roads: bool,
    foundings: bool,
    failures: bool,
    /// Famine, plague, silting, planted towns and ships (v4, v5).
    events: bool,
    /// Also show foundings and failures of places that never grew past a hamlet.
    hamlets: bool,
}

impl Default for Chronicle {
    fn default() -> Self {
        Self {
            growth: true,
            roads: true,
            foundings: true,
            failures: true,
            events: true,
            hamlets: false,
        }
    }
}

/// One line of the chronicle, from either source.
struct Line {
    year: u32,
    text: String,
    anchor: u32,
}

impl Panel for Chronicle {
    fn title(&self) -> &'static str {
        "Chronicle"
    }
    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        v: &ViewState,
        _layers: &[(&'static str, &'static str)],
    ) -> Vec<Msg> {
        let mut out = Vec::new();
        let t = &v.snapshot.tally;
        ui.label(format!(
            "Founded {} · {} of those have failed",
            t.founded, t.failed
        ));
        // Names of places that no longer exist come from the chronicle itself.
        let name_of = |id: u32| {
            v.snapshot
                .settlement(id)
                .map(|s| s.name.clone())
                .or_else(|| {
                    v.chronicle
                        .iter()
                        .find(|c| c.settlement == id)
                        .map(|c| c.name.clone())
                })
        };
        ui.horizontal_wrapped(|ui| {
            ui.checkbox(&mut self.growth, "Growth");
            ui.checkbox(&mut self.roads, "Roads");
            ui.checkbox(&mut self.foundings, "Foundings");
            ui.checkbox(&mut self.failures, "Failures");
            ui.checkbox(&mut self.events, "Events");
            ui.checkbox(&mut self.hamlets, "Hamlets").on_hover_text(
                "Also list the hamlets that were founded and failed without growing",
            );
        });
        let shown_year = |y: u32| hinterlands_core::event::shown_year(&v.params, y);
        let mut lines: Vec<Line> = Vec::new();
        for n in &v.notes {
            let shown = match n.kind {
                NoteKind::Growth | NoteKind::Lead => self.growth,
                NoteKind::Roads => self.roads,
            };
            if shown {
                lines.push(Line {
                    year: n.year,
                    text: format!("{}. {}", shown_year(n.year), n.text),
                    anchor: n.anchor,
                });
            }
        }
        for e in &v.chronicle {
            let event = matches!(
                e.kind,
                LogKind::Famine { .. }
                    | LogKind::Plague { .. }
                    | LogKind::Silted
                    | LogKind::Planted
                    | LogKind::Ships
            );
            let shown = match e.kind {
                _ if event => self.events,
                LogKind::Failed { .. } => self.failures,
                _ => self.foundings,
            };
            let grew = v
                .best_rank
                .get(&e.settlement)
                .is_some_and(|&r| r > Rank::Hamlet);
            if !shown || !(event || self.hamlets || grew || e.kind == LogKind::Initial) {
                continue;
            }
            let raw = e.year;
            let year = shown_year(e.year);
            let e = &LogEntry { year, ..e.clone() };
            let text = match &e.kind {
                LogKind::Initial => format!("{}. {} is settled.", e.year, e.name),
                LogKind::Founded { donor, .. } => {
                    let from = name_of(*donor).unwrap_or_else(|| "elsewhere".into());
                    format!("{}. {} is founded by settlers from {from}.", e.year, e.name)
                }
                LogKind::Works { good, .. } => {
                    let what = match good {
                        hinterlands_core::state::Good::Salt => "Salt works",
                        hinterlands_core::state::Good::Iron => "An iron mine",
                        hinterlands_core::state::Good::Stone => "A quarry",
                    };
                    format!("{}. {what} founded at {}.", e.year, e.name)
                }
                LogKind::Outgrew { good } => {
                    let what = match good {
                        hinterlands_core::state::Good::Salt => "salt works",
                        hinterlands_core::state::Good::Iron => "iron mine",
                        hinterlands_core::state::Good::Stone => "quarry",
                    };
                    format!("{}. {} has outgrown its {what}.", e.year, e.name)
                }
                LogKind::Famine { years } => {
                    let span = match years {
                        1 => "a year".to_string(),
                        2 => "two years".to_string(),
                        3 => "three years".to_string(),
                        n => format!("{n} years"),
                    };
                    format!("{}. The harvests failed; famine for {span}.", e.year)
                }
                LogKind::Plague { lost } => format!(
                    "{}. The pestilence came; {} lost {} of its people.",
                    e.year,
                    e.name,
                    thousands(lost)
                ),
                LogKind::Silted => format!("{}. The harbour at {} silted.", e.year, e.name),
                LogKind::Planted => format!("{}. Lord's town planted at {}.", e.year, e.name),
                LogKind::Ships => format!("{}. Larger ships now sail the coast.", e.year),
                LogKind::Failed { cause, moved_to } => {
                    let why = match cause {
                        FailCause::Depopulated => "dwindled away",
                        FailCause::Starved => "could not feed itself",
                    };
                    let to = moved_to.and_then(name_of).map_or_else(
                        || "; its people scattered".to_string(),
                        |m| format!("; its people went to {m}"),
                    );
                    format!("{}. {} {why}{to}.", e.year, e.name)
                }
            };
            lines.push(Line {
                year: raw,
                text,
                anchor: e.anchor,
            });
        }
        // Newest first; within a year, the order things were recorded.
        lines.sort_by_key(|l| std::cmp::Reverse(l.year));
        egui::ScrollArea::vertical()
            .max_height(260.0)
            .id_salt("chronicle")
            .show(ui, |ui| {
                for l in lines.iter().take(300) {
                    let label = egui::Label::new(egui::RichText::new(&l.text).small())
                        .sense(egui::Sense::click());
                    if ui
                        .add(label)
                        .on_hover_text("Show this place on the map")
                        .clicked()
                    {
                        out.push(Msg::Select(Some(l.anchor as usize)));
                        let (x, y) = v.terrain.grid.xy(l.anchor as usize);
                        let zoom = v.camera.map_or(10.0, |c| c.zoom.max(10.0));
                        out.push(Msg::SetCamera(Camera {
                            center: egui::vec2(x as f32 + 0.5, y as f32 + 0.5),
                            zoom,
                        }));
                    }
                }
            });
        out
    }
}

/// People lost, rounded to the hundred over a thousand, with a thousands comma.
fn thousands(x: &f32) -> String {
    let n = if *x >= 1000.0 {
        (x / 100.0).round() as u64 * 100
    } else {
        (x / 10.0).round() as u64 * 10
    };
    let s = n.to_string();
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if i > 0 && (s.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    out
}
