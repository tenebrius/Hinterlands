//! The chronicle: what happened, in plain prose, newest first. Foundings and
//! failures come from the core's log; growth into a village, town or city, a
//! new largest place and new roads are noticed by the viewer. Filters pick
//! what to show; by default the many hamlets that are founded and fail without
//! ever growing are left out, so the story reads. Clicking an entry selects the place and brings it into view.

use super::Panel;
use crate::camera::Camera;
use crate::view::{Msg, NoteKind, ViewState};
use hinterlands_core::state::{FailCause, LogKind, Rank};

/// Which kinds of entry are shown.
pub struct Chronicle {
    growth: bool,
    roads: bool,
    foundings: bool,
    failures: bool,
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
            ui.checkbox(&mut self.hamlets, "Hamlets").on_hover_text(
                "Also list the hamlets that were founded and failed without growing",
            );
        });
        let mut lines: Vec<Line> = Vec::new();
        for n in &v.notes {
            let shown = match n.kind {
                NoteKind::Growth | NoteKind::Lead => self.growth,
                NoteKind::Roads => self.roads,
            };
            if shown {
                lines.push(Line {
                    year: n.year,
                    text: format!("{}. {}", n.year, n.text),
                    anchor: n.anchor,
                });
            }
        }
        for e in &v.chronicle {
            let shown = match e.kind {
                LogKind::Failed { .. } => self.failures,
                _ => self.foundings,
            };
            let grew = v
                .best_rank
                .get(&e.settlement)
                .is_some_and(|&r| r > Rank::Hamlet);
            if !shown || !(self.hamlets || grew || e.kind == LogKind::Initial) {
                continue;
            }
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
                year: e.year,
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
