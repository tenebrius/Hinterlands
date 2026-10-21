//! The chronicle: foundings and failures in plain prose, newest first.
//! Clicking an entry selects the place on the map.

use super::Panel;
use crate::view::{Msg, ViewState};
use hinterlands_core::state::LogEntry;
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
                    // Calendar years in historical mode (v4, D57).
                    let year = hinterlands_core::event::shown_year(&v.params, e.year);
                    let e = &LogEntry {
                        year,
                        ..e.clone()
                    };
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
                        LogKind::Ships => format!("{}. Larger ships now sail the coast.", e.year),
                        LogKind::Planted => format!("{}. Lord's town planted at {}.", e.year, e.name),
                        LogKind::Silted => format!("{}. The harbour at {} silted.", e.year, e.name),
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
