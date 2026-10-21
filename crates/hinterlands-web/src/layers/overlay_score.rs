//! Overlay: site score at the last founding step (where the next settlement
//! would appear). Cells that are not candidates are dimmed.

use super::overlay_cost::heat;
use super::terrain::to_color;
use super::{DrawCtx, Layer, Swatch};
use egui::{Color32, ColorImage, TextureHandle, TextureOptions};

#[derive(Default)]
pub struct ScoreOverlay {
    tex: Option<TextureHandle>,
    key: Option<(u64, u32)>,
}

impl Layer for ScoreOverlay {
    fn id(&self) -> &'static str {
        "score"
    }
    fn name(&self) -> &'static str {
        "Site score (next founding)"
    }
    fn default_on(&self) -> bool {
        false
    }
    fn legend(&self) -> Vec<(Swatch, &'static str)> {
        vec![(
            Swatch::Ramp(
                Color32::from_rgb(26, 26, 89),
                Color32::from_rgb(255, 242, 102),
            ),
            "Site score (dim: below threshold)",
        )]
    }
    fn draw(&mut self, d: &DrawCtx) {
        let v = d.view;
        let t = &v.terrain;
        let s = &v.snapshot.site_score;
        let key = (v.terrain_version, v.snapshot.year);
        if self.key != Some(key) {
            let (lo, hi) = s
                .iter()
                .filter(|x| x.is_finite())
                .fold((f32::MAX, f32::MIN), |a, &x| (a.0.min(x), a.1.max(x)));
            let thr = v.params.found_threshold;
            let px = s
                .iter()
                .map(|&x| {
                    if x.is_finite() {
                        let c = to_color(heat((x - lo) / (hi - lo).max(1e-6)));
                        if x >= thr {
                            c
                        } else {
                            c.gamma_multiply(0.55)
                        }
                    } else {
                        Color32::from_black_alpha(140)
                    }
                })
                .collect();
            let img = ColorImage::new([t.grid.w as usize, t.grid.h as usize], px);
            match &mut self.tex {
                Some(tex) => tex.set(img, TextureOptions::NEAREST),
                None => self.tex = Some(d.ctx.load_texture("score", img, TextureOptions::NEAREST)),
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
