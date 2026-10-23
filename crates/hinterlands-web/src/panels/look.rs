//! The map's style close in (R12): sizes of roads, rivers, trees and buildings
//! against true size, and people to a house.

use super::Panel;
use crate::look::Look;
use crate::view::{Msg, ViewState};

/// The sliders work on a copy, sent when a drag ends, so the towns are laid
/// out again once rather than on every step of the drag.
#[derive(Default)]
pub struct LookPanel {
    draft: Option<Look>,
}

impl Panel for LookPanel {
    fn title(&self) -> &'static str {
        "Map style"
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
            for (name, look) in [
                ("True scale", Look::TRUE_SCALE),
                ("Storybook", Look::STORYBOOK),
            ] {
                if ui.selectable_label(v.look == look, name).clicked() {
                    self.draft = None;
                    out.push(Msg::SetLook(look));
                }
            }
        });
        let mut look = self.draft.unwrap_or(v.look);
        let mut send = false;
        for (name, lo, hi, field) in Look::FIELDS {
            let x = field(&mut look);
            let r = ui.add(egui::Slider::new(x, lo..=hi).text(name));
            if r.drag_stopped() || (r.changed() && !r.dragged()) {
                send = true;
            }
        }
        if send {
            self.draft = None;
            out.push(Msg::SetLook(look));
        } else if look != v.look {
            self.draft = Some(look);
        }
        ui.weak("Seen close in. Larger roads, trees and houses, with more people to a house, give fewer and bolder buildings.");
        out
    }
}
