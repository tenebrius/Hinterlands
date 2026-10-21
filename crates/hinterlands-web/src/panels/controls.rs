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
        let time = ui.input(|i| i.time);
        // In historical mode the calendar year follows the run year (v4, D57).
        let year = if v.params.events_mode == 1 {
            format!(
                "{} ({})",
                hinterlands_core::event::shown_year(&v.params, v.snapshot.year),
                v.snapshot.year
            )
        } else {
            v.snapshot.year.to_string()
        };
        ui.heading(format!(
            "Year {year} · {}",
            crate::view::SEASONS[v.season_index(time)]
        ));
        ui.horizontal(|ui| {
            ui.label("Season");
            let mut sel = v.season_fixed;
            egui::ComboBox::from_id_salt("season")
                .selected_text(sel.map_or("Follow the clock", |k| crate::view::SEASONS[k as usize]))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut sel, None, "Follow the clock");
                    for (k, name) in crate::view::SEASONS.iter().enumerate() {
                        ui.selectable_value(&mut sel, Some(k as u8), *name);
                    }
                });
            if sel != v.season_fixed {
                out.push(Msg::SetSeason(sel));
            }
        });
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
        ui.horizontal(|ui| {
            if ui
                .button("Save")
                .on_hover_text("Download the run (seeds, parameters, year)")
                .clicked()
            {
                out.push(Msg::Save);
            }
            if ui
                .button("Load")
                .on_hover_text("Replay a saved run to its year")
                .clicked()
            {
                crate::app::pick_save_file();
            }
        });
        out
    }
}
