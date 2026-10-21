//! Shocks on the map (v4; drawing only): a brief dark tint over the land in the
//! years a plague is raging, a mark on each silted harbour, and a pennant on
//! each planted town.

use super::{DrawCtx, Layer, Swatch};
use crate::view::EventMark;
use egui::{pos2, vec2, Color32, Shape, Stroke};

const PLAGUE: Color32 = Color32::from_rgb(70, 20, 30);
const SILT: Color32 = Color32::from_rgb(150, 120, 70);
const PENNANT: Color32 = Color32::from_rgb(170, 40, 40);
const INK: Color32 = Color32::from_rgb(40, 34, 30);
/// Years the plague tint lasts after a plague strikes.
const TINT_YEARS: u32 = 3;

#[derive(Default)]
pub struct EventsLayer;

impl Layer for EventsLayer {
    fn id(&self) -> &'static str {
        "events"
    }
    fn name(&self) -> &'static str {
        "Plague, silted harbours, planted towns"
    }
    fn legend(&self) -> Vec<(Swatch, &'static str)> {
        vec![
            (Swatch::Fill(PLAGUE), "Plague years (tint)"),
            (Swatch::Dot(SILT), "Silted harbour"),
            (Swatch::Fill(PENNANT), "Lord's planted town (pennant)"),
        ]
    }
    fn draw(&mut self, d: &DrawCtx) {
        let v = d.view;
        let snap = &v.snapshot;
        let g = v.terrain.grid;
        // The tint fades over the plague's years.
        let since = v
            .events
            .iter()
            .filter(|(_, m)| *m == EventMark::Plague)
            .map(|&(y, _)| snap.year.saturating_sub(y))
            .filter(|&a| a < TINT_YEARS)
            .min();
        if let Some(age) = since {
            let a = 0.32 * (1.0 - age as f32 / TINT_YEARS as f32);
            d.painter
                .rect_filled(d.rect, 0.0, PLAGUE.gamma_multiply(a));
        }
        let z = d.camera.zoom;
        // A silted harbour: a ring in mud colour, struck through.
        for &cell in &snap.silted {
            let c = d.camera.cell_center(d.rect, g.w, cell as usize);
            if !d.rect.expand(20.0).contains(c) {
                continue;
            }
            let r = (z * 0.55).clamp(5.0, 12.0);
            d.painter.circle_stroke(c, r, Stroke::new(3.0, INK.gamma_multiply(0.6)));
            d.painter.circle_stroke(c, r, Stroke::new(1.8, SILT));
            d.painter.line_segment(
                [c + vec2(-r * 0.7, r * 0.7), c + vec2(r * 0.7, -r * 0.7)],
                Stroke::new(1.8, SILT),
            );
        }
        // A planted town: a pennant on a staff above the anchor.
        for s in snap.settlements.iter().filter(|s| s.planted) {
            let c = d.camera.cell_center(d.rect, g.w, s.anchor as usize);
            if !d.rect.expand(20.0).contains(c) {
                continue;
            }
            let h = (z * 0.9).clamp(8.0, 18.0);
            let foot = pos2(c.x + h * 0.35, c.y - h * 0.2);
            let top = foot + vec2(0.0, -h);
            d.painter.line_segment([foot, top], Stroke::new(1.5, INK));
            d.painter.add(Shape::convex_polygon(
                vec![top, top + vec2(h * 0.7, h * 0.18), top + vec2(0.0, h * 0.36)],
                PENNANT,
                Stroke::new(1.0, INK),
            ));
        }
    }
}
