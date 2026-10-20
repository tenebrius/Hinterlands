//! The chronicle: foundings and failures, newest first, with year and cause.

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
            "Founded {} · failed {} of those",
            t.founded, t.failed
        ));
        egui::ScrollArea::vertical()
            .max_height(260.0)
            .id_salt("chronicle")
            .show(ui, |ui| {
                for e in v.chronicle.iter().rev().take(300) {
                    let (x, y) = v.terrain.grid.xy(e.anchor as usize);
                    let text = match &e.kind {
                        LogKind::Initial => {
                            format!("{:>4}  #{} settled at ({x},{y})", e.year, e.settlement)
                        }
                        LogKind::Founded { score, donor } => {
                            format!(
                                "{:>4}  #{} founded at ({x},{y}) from #{donor}, score {score:.2}",
                                e.year, e.settlement
                            )
                        }
                        LogKind::Failed { cause, moved_to } => {
                            let why = match cause {
                                FailCause::Depopulated => "dwindled",
                                FailCause::Starved => "starved",
                            };
                            let to =
                                moved_to.map_or(String::new(), |m| format!("; people to #{m}"));
                            format!(
                                "{:>4}  #{} at ({x},{y}) {why} ({:.0} left{to})",
                                e.year, e.settlement, e.pop
                            )
                        }
                    };
                    if ui
                        .add(
                            egui::Label::new(egui::RichText::new(text).monospace().small())
                                .sense(egui::Sense::click()),
                        )
                        .clicked()
                    {
                        out.push(Msg::Select(Some(e.anchor as usize)));
                    }
                }
            });
        out
    }
}
