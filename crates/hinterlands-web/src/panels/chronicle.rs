//! The chronicle: foundings and failures in plain prose, newest first.
//! Clicking an entry selects the place on the map.

use super::Panel;
use crate::view::{Msg, ViewState};
use hinterlands_core::state::{FailCause, LogKind};

pub struct Chronicle;

impl Panel for Chronicle {
    fn title(&self) -> &'static str {
        "Chronicle"
    }
    fn default_open(&self) -> bool {
        false
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
        egui::ScrollArea::vertical()
            .max_height(260.0)
            .id_salt("chronicle")
            .show(ui, |ui| {
                for e in v.chronicle.iter().rev().take(300) {
                    let text = match &e.kind {
                        LogKind::Initial => format!("{}. {} is settled.", e.year, e.name),
                        LogKind::Founded { donor, .. } => {
                            let from = name_of(*donor).unwrap_or_else(|| "elsewhere".into());
                            format!("{}. {} is founded by settlers from {from}.", e.year, e.name)
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
                    let label = egui::Label::new(egui::RichText::new(text).small())
                        .sense(egui::Sense::click());
                    if ui.add(label).clicked() {
                        out.push(Msg::Select(Some(e.anchor as usize)));
                    }
                }
            });
        out
    }
}
