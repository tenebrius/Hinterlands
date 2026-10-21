//! Granaries (S3, D81; drawing only): beside each place of more than 500
//! people, a small upright bar filled to its grain store's share of the most it
//! can hold (`store_cap` × a year's need). Empty bars outline the places whose
//! stores are spent.

use super::{DrawCtx, Layer, Swatch};
use egui::{vec2, Color32, Rect, Stroke};

pub const GRAIN: Color32 = Color32::from_rgb(214, 170, 64);
const FRAME: Color32 = Color32::from_rgb(70, 52, 30);
/// Smallest place given a granary mark.
const MIN_POP: f32 = 500.0;

#[derive(Default)]
pub struct GranaryLayer;

impl Layer for GranaryLayer {
    fn id(&self) -> &'static str {
        "granary"
    }
    fn name(&self) -> &'static str {
        "Granaries (grain stores)"
    }
    fn legend(&self) -> Vec<(Swatch, &'static str)> {
        vec![(Swatch::Fill(GRAIN), "Granary, filled to its store")]
    }
    fn shown_at(&self, zoom: f32) -> bool {
        zoom >= 2.0
    }
    fn draw(&mut self, d: &DrawCtx) {
        let v = d.view;
        if !v.params.grain_stores || d.camera.zoom < 2.0 {
            return;
        }
        let w = v.terrain.grid.w;
        let h = (d.camera.zoom * 0.9).clamp(6.0, 14.0);
        let bw = h * 0.4;
        for s in v.snapshot.settlements.iter().filter(|s| s.pop > MIN_POP) {
            let full = v.params.store_cap * s.need;
            let fill = if full > 0.0 {
                (s.store / full).clamp(0.0, 1.0)
            } else {
                0.0
            };
            let p = d.camera.cell_center(d.rect, w, s.anchor as usize);
            let frame = Rect::from_min_size(p + vec2(h * 0.6, -h * 0.5), vec2(bw, h));
            if !d.rect.intersects(frame) {
                continue;
            }
            d.painter.rect_filled(
                frame,
                1.0,
                Color32::from_rgba_unmultiplied(250, 245, 230, 200),
            );
            if fill > 0.0 {
                let top = frame.bottom() - frame.height() * fill;
                let grain = Rect::from_min_max(egui::pos2(frame.left(), top), frame.right_bottom());
                d.painter.rect_filled(grain, 1.0, GRAIN);
            }
            d.painter.rect_stroke(
                frame,
                1.0,
                Stroke::new(1.0, FRAME),
                egui::StrokeKind::Inside,
            );
        }
    }
}
