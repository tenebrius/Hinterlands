//! Overlay: travel cost from the selected cell, in 10-unit bands.

use super::terrain::{mix, to_color};
use super::{DrawCtx, Layer, Swatch};
use egui::{Color32, ColorImage, TextureHandle, TextureOptions};
use hinterlands_core::cost::{CostGrid, Dijkstra};

#[derive(Default)]
pub struct CostOverlay {
    tex: Option<TextureHandle>,
    key: Option<(usize, u32, u64)>,
}

pub fn heat(v: f32) -> [f32; 3] {
    let v = v.clamp(0.0, 1.0);
    if v < 0.5 {
        mix([0.1, 0.1, 0.35], [0.8, 0.2, 0.3], v * 2.0)
    } else {
        mix([0.8, 0.2, 0.3], [1.0, 0.95, 0.4], (v - 0.5) * 2.0)
    }
}

impl Layer for CostOverlay {
    fn id(&self) -> &'static str {
        "cost"
    }
    fn name(&self) -> &'static str {
        "Travel cost from selected cell"
    }
    fn default_on(&self) -> bool {
        false
    }
    fn legend(&self) -> Vec<(Swatch, &'static str)> {
        vec![(
            Swatch::Ramp(
                Color32::from_rgb(255, 242, 102),
                Color32::from_rgb(26, 26, 89),
            ),
            "Travel cost: low to high",
        )]
    }
    fn draw(&mut self, d: &DrawCtx) {
        let v = d.view;
        let Some(src) = v.selected else { return };
        let t = &v.terrain;
        let key = (src, v.snapshot.year / 10, v.terrain_version);
        if self.key != Some(key) {
            let n = t.n();
            let g = CostGrid::build(t, &v.snapshot.road, &v.snapshot.transport);
            let mut dj = Dijkstra::new(n);
            dj.run(&g, src as u32, 150.0, false);
            let px = (0..n)
                .map(|i| {
                    let c = dj.cell_cost(n, i);
                    if c.is_finite() {
                        let band = if ((c / 10.0) as i32) % 2 == 0 {
                            1.0
                        } else {
                            0.82
                        };
                        let h = heat(1.0 - c / 100.0);
                        to_color([h[0] * band, h[1] * band, h[2] * band]).gamma_multiply(0.85)
                    } else {
                        Color32::from_black_alpha(160)
                    }
                })
                .collect();
            let img = ColorImage::new([t.grid.w as usize, t.grid.h as usize], px);
            match &mut self.tex {
                Some(tex) => tex.set(img, TextureOptions::NEAREST),
                None => self.tex = Some(d.ctx.load_texture("cost", img, TextureOptions::NEAREST)),
            }
            self.key = Some(key);
        }
        if let Some(tex) = &self.tex {
            let r = d.camera.map_rect(d.rect, t.grid.w, t.grid.h);
            d.painter.image(
                tex.id(),
                r,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                Color32::WHITE,
            );
        }
    }
}
