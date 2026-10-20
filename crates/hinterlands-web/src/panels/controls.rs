//! Time controls: play, pause, step, speed, year and seeds.

use super::Panel;
use crate::view::{Msg, ViewState};

#[derive(Default)]
pub struct Controls {
    map_seed: Option<u64>,
    sim_seed: Option<u64>,
}

impl Panel for Controls {
    fn title(&self) -> &'static str {
        "Time"
    }
    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        v: &ViewState,
        _layers: &[(&'static str, &'static str)],
    ) -> Vec<Msg> {
        let mut out = Vec::new();
        ui.heading(format!("Year {}", v.snapshot.year));
        ui.horizontal(|ui| {
            if ui
                .button(if v.playing { "⏸ Pause" } else { "▶ Play" })
                .clicked()
            {
                out.push(Msg::TogglePlay);
            }
            if ui.button("+1").clicked() {
                out.push(Msg::Step(1));
            }
            if ui.button("+10").clicked() {
                out.push(Msg::Step(10));
            }
        });
        let mut speed = v.speed;
        if ui
            .add(
                egui::Slider::new(&mut speed, 1.0..=200.0)
                    .logarithmic(true)
                    .text("years/s"),
            )
            .changed()
        {
            out.push(Msg::SetSpeed(speed));
        }
        let map_seed = self.map_seed.get_or_insert(v.map_seed);
        let sim_seed = self.sim_seed.get_or_insert(v.sim_seed);
        ui.horizontal(|ui| {
            ui.label("Map seed");
            ui.add(egui::DragValue::new(map_seed));
            if ui.button("New map").clicked() {
                out.push(Msg::NewMap(*map_seed));
            }
        });
        ui.horizontal(|ui| {
            ui.label("Run seed");
            ui.add(egui::DragValue::new(sim_seed));
            if ui.button("Restart").clicked() {
                out.push(Msg::Reset(*sim_seed));
            }
        });
        out
    }
}
