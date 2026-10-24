//! A ring round the selected place, so a chronicle entry or a pinned cell can be
//! found on the map. It marks the settlement there if there is one.

use super::{DrawCtx, Layer};
use egui::{Color32, Stroke};
use hinterlands_core::query;

pub struct SelectionLayer;

impl Layer for SelectionLayer {
    fn id(&self) -> &'static str {
        "selection"
    }
    fn name(&self) -> &'static str {
        "Selected place"
    }
    fn draw(&mut self, d: &DrawCtx) {
        let v = d.view;
        let Some(cell) = v.selected else { return };
        let g = v.terrain.grid;
        let at = query::settlement_near(&v.snapshot, &v.terrain, cell, 1.0)
            .map_or(cell, |s| s.anchor as usize);
        let c = d.camera.cell_center(d.rect, g.w, at);
        // A slow pulse, so the eye finds it.
        let pulse = 0.5 + 0.5 * (d.time * 3.0).sin() as f32;
        let r = (0.9 * crate::scale::symbol_zoom(d.camera.zoom)).max(12.0) + 3.0 * pulse;
        d.painter.circle_stroke(
            c,
            r + 1.5,
            Stroke::new(4.0, Color32::from_rgba_unmultiplied(250, 246, 235, 200)),
        );
        d.painter
            .circle_stroke(c, r, Stroke::new(2.0, Color32::from_rgb(170, 40, 30)));
        d.ctx.request_repaint();
    }
}
