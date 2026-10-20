//! Settlements as built-up footprints (from the snapshot), ruins in grey, and a
//! marker with a size label at each anchor of a town or larger.

use super::{DrawCtx, Layer};
use egui::{Color32, ColorImage, Stroke, TextureHandle, TextureOptions};
use hinterlands_core::state::{Rank, NONE};

#[derive(Default)]
pub struct SettlementsLayer {
    tex: Option<TextureHandle>,
    key: (u64, u32),
}

impl Layer for SettlementsLayer {
    fn id(&self) -> &'static str {
        "settlements"
    }
    fn name(&self) -> &'static str {
        "Settlements"
    }
    fn draw(&mut self, d: &DrawCtx) {
        let v = d.view;
        let t = &v.terrain;
        let snap = &v.snapshot;
        let key = (v.terrain_version, snap.year);
        if self.tex.is_none() || self.key != key {
            let px = (0..t.n())
                .map(|i| {
                    if snap.built[i] != NONE {
                        Color32::from_rgb(178, 74, 58)
                    } else if snap.ruin[i] > 0 {
                        Color32::from_rgba_unmultiplied(120, 110, 100, 60 + snap.ruin[i] * 2)
                    } else {
                        Color32::TRANSPARENT
                    }
                })
                .collect();
            let img = ColorImage::new([t.grid.w as usize, t.grid.h as usize], px);
            match &mut self.tex {
                Some(tex) => tex.set(img, TextureOptions::NEAREST),
                None => self.tex = Some(d.ctx.load_texture("built", img, TextureOptions::NEAREST)),
            }
            self.key = key;
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
        let w = t.grid.w;
        let z = d.camera.zoom;
        for s in &snap.settlements {
            let p = d.camera.cell_center(d.rect, w, s.anchor as usize);
            let r = match s.rank {
                Rank::Hamlet => 0.18,
                Rank::Village => 0.24,
                Rank::Town => 0.32,
                Rank::City => 0.42,
            } * z;
            d.painter.circle(
                p,
                r.max(1.5),
                Color32::from_rgb(120, 30, 22),
                Stroke::new(1.0, Color32::from_rgb(250, 235, 220)),
            );
            if s.rank >= Rank::Town && z > 4.0 {
                d.painter.text(
                    p + egui::vec2(r + 2.0, -r),
                    egui::Align2::LEFT_BOTTOM,
                    format!("{:.0}", s.pop),
                    egui::FontId::proportional(11.0),
                    Color32::from_rgb(30, 20, 15),
                );
            }
        }
    }
}
