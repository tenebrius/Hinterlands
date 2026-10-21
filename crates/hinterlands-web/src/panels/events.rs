//! Event timeline (v4): the run's shocks along a strip of years, under the
//! population chart, with the list below it in calendar years.

use super::Panel;
use crate::view::{EventMark, Msg, ViewState};
use egui::{Color32, Pos2, Sense, Stroke, Vec2};

const FAMINE: Color32 = Color32::from_rgb(200, 150, 40);
const PLAGUE: Color32 = Color32::from_rgb(150, 30, 40);
const SILT: Color32 = Color32::from_rgb(150, 120, 70);
const PLANT: Color32 = Color32::from_rgb(60, 130, 70);

pub struct EventsPanel;

fn colour(m: EventMark) -> Color32 {
    match m {
        EventMark::Famine => FAMINE,
        EventMark::Plague => PLAGUE,
        EventMark::Silted => SILT,
        EventMark::Planted => PLANT,
    }
}

fn label(m: EventMark) -> &'static str {
    match m {
        EventMark::Famine => "famine",
        EventMark::Plague => "plague",
        EventMark::Silted => "harbour silted",
        EventMark::Planted => "town planted",
    }
}

impl Panel for EventsPanel {
    fn title(&self) -> &'static str {
        "Events"
    }
    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        v: &ViewState,
        _layers: &[(&'static str, &'static str)],
    ) -> Vec<Msg> {
        if v.params.events_mode == 0 {
            ui.weak("Events are off (events_mode 0).");
            return Vec::new();
        }
        let end = v.snapshot.year.max(1) as f32;
        let (rect, _) =
            ui.allocate_exact_size(Vec2::new(ui.available_width(), 28.0), Sense::hover());
        let painter = ui.painter_at(rect);
        painter.rect_stroke(
            rect,
            2.0,
            Stroke::new(1.0, ui.visuals().weak_text_color()),
            egui::StrokeKind::Inside,
        );
        let inner = rect.shrink2(Vec2::new(6.0, 4.0));
        let x = |y: u32| inner.left() + y as f32 / end * inner.width();
        for &(y, m) in &v.events {
            let col = colour(m);
            let px = x(y);
            // Famine spans its years; the others are ticks.
            let w = if m == EventMark::Famine {
                (x(y + v.params.famine_years) - px).max(2.0)
            } else {
                2.0
            };
            painter.rect_filled(
                egui::Rect::from_min_max(
                    Pos2::new(px, inner.top()),
                    Pos2::new(px + w, inner.bottom()),
                ),
                0.0,
                col,
            );
        }
        ui.horizontal_wrapped(|ui| {
            for m in [
                EventMark::Famine,
                EventMark::Plague,
                EventMark::Silted,
                EventMark::Planted,
            ] {
                ui.colored_label(colour(m), "■");
                ui.small(label(m));
            }
        });
        egui::ScrollArea::vertical()
            .max_height(120.0)
            .id_salt("events")
            .show(ui, |ui| {
                for &(y, m) in v.events.iter().rev() {
                    let shown = hinterlands_core::event::shown_year(&v.params, y);
                    ui.small(format!("{shown}: {}", label(m)));
                }
            });
        Vec::new()
    }
}
