//! Settlement markers sized by population.

use super::{DrawCtx, Layer};
use egui::{Color32, Stroke};

pub struct SettlementsLayer;

impl Layer for SettlementsLayer {
    fn id(&self) -> &'static str {
        "settlements"
    }
    fn name(&self) -> &'static str {
        "Settlements"
    }
    fn draw(&mut self, d: &DrawCtx) {
        let w = d.view.terrain.grid.w;
        for s in &d.view.snapshot.settlements {
            let p = d.camera.cell_center(d.rect, w, s.anchor as usize);
            let r = (0.35 + s.pop.max(10.0).log10() * 0.35) * d.camera.zoom;
            d.painter.circle(
                p,
                r,
                Color32::from_rgb(200, 40, 30),
                Stroke::new(1.0, Color32::from_rgb(40, 10, 10)),
            );
        }
    }
}
