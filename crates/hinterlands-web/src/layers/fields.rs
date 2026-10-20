//! Cultivated cells, tinted.

use super::{DrawCtx, Layer};
use egui::{Color32, ColorImage, TextureHandle, TextureOptions};
use hinterlands_core::state::NONE;

#[derive(Default)]
pub struct FieldsLayer {
    tex: Option<TextureHandle>,
    key: (u64, u32),
}

impl Layer for FieldsLayer {
    fn id(&self) -> &'static str {
        "fields"
    }
    fn name(&self) -> &'static str {
        "Fields"
    }
    fn draw(&mut self, d: &DrawCtx) {
        let v = d.view;
        let t = &v.terrain;
        let key = (v.terrain_version, v.snapshot.year);
        if self.tex.is_none() || self.key != key {
            let px = (0..t.n())
                .map(|i| {
                    if v.snapshot.owner[i] != NONE && v.snapshot.built[i] == NONE {
                        Color32::from_rgba_unmultiplied(236, 206, 96, 150)
                    } else {
                        Color32::TRANSPARENT
                    }
                })
                .collect();
            let img = ColorImage::new([t.grid.w as usize, t.grid.h as usize], px);
            match &mut self.tex {
                Some(tex) => tex.set(img, TextureOptions::NEAREST),
                None => self.tex = Some(d.ctx.load_texture("fields", img, TextureOptions::NEAREST)),
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
    }
}
