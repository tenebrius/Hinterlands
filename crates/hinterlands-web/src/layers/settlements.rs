//! Settlements as built-up footprints (from the snapshot), ruins in grey, and a
//! marker with a size label at each anchor of a town or larger.

use super::{DrawCtx, Layer, Swatch};
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
    fn legend(&self) -> Vec<(Swatch, &'static str)> {
        vec![
            (
                Swatch::Fill(Color32::from_rgb(196, 160, 120)),
                "Built-up area",
            ),
            (Swatch::Fill(Color32::from_rgb(125, 115, 104)), "Ruins"),
            (
                Swatch::Dot(Color32::from_rgb(120, 30, 22)),
                "Settlement centre",
            ),
        ]
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
        let z = d.camera.zoom;
        if z < 4.0 {
            if let Some(tex) = &self.tex {
                let r = d.camera.map_rect(d.rect, t.grid.w, t.grid.h);
                d.painter.image(
                    tex.id(),
                    r,
                    egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                    Color32::WHITE,
                );
            }
        } else {
            draw_blocks(d);
        }
        let w = t.grid.w;
        for s in &snap.settlements {
            let p = d.camera.cell_center(d.rect, w, s.anchor as usize);
            let r = match s.rank {
                Rank::Hamlet => 0.18,
                Rank::Village => 0.24,
                Rank::Town => 0.32,
                Rank::City => 0.42,
            } * z;
            // Close up the town blocks show the extent; the marker only marks the centre.
            let r = if z >= 4.0 { r.min(5.0) } else { r };
            d.painter.circle(
                p,
                r.max(1.5),
                Color32::from_rgb(120, 30, 22),
                Stroke::new(1.0, Color32::from_rgb(250, 235, 220)),
            );
        }
    }
}

/// Close up, built-up cells are drawn as rounded town blocks; further in, each
/// block shows a few houses, placed by a hash of the cell so they stay put.
fn draw_blocks(d: &DrawCtx) {
    let v = d.view;
    let g = v.terrain.grid;
    let snap = &v.snapshot;
    let z = d.camera.zoom;
    let half = egui::vec2(0.5, 0.5) * z;
    for i in 0..g.n() {
        let built = snap.built[i] != NONE;
        if !built && snap.ruin[i] == 0 {
            continue;
        }
        let c = d.camera.cell_center(d.rect, g.w, i);
        let cell = egui::Rect::from_center_size(c, half * 2.0);
        if !d.rect.intersects(cell) {
            continue;
        }
        let block = cell.shrink(z * 0.07);
        let radius = (z * 0.18) as u8;
        if !built {
            let a = 40 + snap.ruin[i] * 2;
            d.painter.rect_filled(
                block,
                radius,
                Color32::from_rgba_unmultiplied(125, 115, 104, a),
            );
            continue;
        }
        d.painter
            .rect_filled(block, radius, Color32::from_rgb(196, 160, 120));
        if z >= 12.0 {
            let mut h = (i as u32).wrapping_mul(2654435761);
            for _ in 0..6 {
                h ^= h << 13;
                h ^= h >> 17;
                h ^= h << 5;
                let fx = (h & 0xff) as f32 / 255.0;
                let fy = ((h >> 8) & 0xff) as f32 / 255.0;
                let sz = 0.14 + ((h >> 16) & 0x3f) as f32 / 63.0 * 0.1;
                let p = block.min
                    + egui::vec2(
                        fx * (block.width() - sz * z),
                        fy * (block.height() - sz * z),
                    );
                let house = egui::Rect::from_min_size(p, egui::vec2(sz * z, sz * z * 0.8));
                d.painter
                    .rect_filled(house, 1, Color32::from_rgb(158, 64, 46));
            }
        }
    }
}
