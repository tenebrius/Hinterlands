//! Parameters, generated from `Params::INFO`: a new parameter in `core` appears
//! here with no change to the viewer.

use super::Panel;
use crate::view::{Msg, ViewState};
use hinterlands_core::Params;

pub struct ParamsPanel;

impl Panel for ParamsPanel {
    fn title(&self) -> &'static str {
        "Parameters"
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
        ui.horizontal(|ui| {
            if ui.button("Defaults").clicked() {
                out.push(Msg::ResetParams);
            }
            ui.weak("Changes apply from the next year; Restart to rerun.");
        });
        let mut groups: Vec<&str> = Params::INFO.iter().map(|i| i.group).collect();
        groups.dedup();
        for g in groups {
            egui::CollapsingHeader::new(g)
                .id_salt(("pgroup", g))
                .show(ui, |ui| {
                    for info in Params::INFO.iter().filter(|i| i.group == g) {
                        let cur = v.params.get(info.name).unwrap_or(0.0);
                        if info.boolean {
                            let mut b = cur > 0.5;
                            if ui
                                .checkbox(&mut b, info.name)
                                .on_hover_text(info.doc)
                                .changed()
                            {
                                out.push(Msg::SetParam(info.name, if b { 1.0 } else { 0.0 }));
                            }
                        } else {
                            let mut x = cur;
                            let mut slider =
                                egui::Slider::new(&mut x, info.min..=info.max).text(info.name);
                            if info.integer {
                                slider = slider.integer();
                            }
                            if ui.add(slider).on_hover_text(info.doc).changed() {
                                out.push(Msg::SetParam(info.name, x));
                            }
                        }
                    }
                });
        }
        out
    }
}
