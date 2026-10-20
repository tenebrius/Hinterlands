//! Layer toggles.

use super::Panel;
use crate::view::{Msg, ViewState};

pub struct Layers;

impl Panel for Layers {
    fn title(&self) -> &'static str {
        "Layers"
    }
    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        v: &ViewState,
        layers: &[(&'static str, &'static str)],
    ) -> Vec<Msg> {
        let mut out = Vec::new();
        for &(id, name) in layers {
            let mut on = v.layer_on(id);
            if ui.checkbox(&mut on, name).changed() {
                out.push(Msg::ToggleLayer(id));
            }
        }
        if v.layer_on("cost") && v.selected.is_none() {
            ui.weak("Click a cell to measure travel cost from it.");
        }
        out
    }
}
