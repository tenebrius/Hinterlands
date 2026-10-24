//! Smoke (1.x layer; drawing only): hearth smoke rising from built-up cells,
//! more plumes in larger places, drifting with a steady westerly wind; and
//! charcoal burners' smoke in woodland thinned by fuel cutting.

use super::{DrawCtx, Layer, Swatch};
use egui::{Color32, Vec2};
use hinterlands_core::state::NONE;

const WIND: Vec2 = Vec2::new(0.35, -1.0);

fn hash(i: usize, k: u32) -> u32 {
    let mut h = (i as u32).wrapping_mul(2654435761) ^ k.wrapping_mul(97);
    h ^= h >> 13;
    h = h.wrapping_mul(0x5bd1e995);
    h ^ (h >> 15)
}

#[derive(Default)]
pub struct SmokeLayer;

impl SmokeLayer {
    /// One plume of puffs rising from `base`; `phase` staggers plumes.
    fn plume(d: &DrawCtx, base: egui::Pos2, phase: f32, dark: bool) {
        // Plumes are symbols: they stop growing at the old deepest zoom.
        let z = crate::scale::symbol_zoom(d.camera.zoom);
        let rise = 0.9 * z;
        for k in 0..4 {
            // Each puff takes 4 s to rise and fade; four are in the air at once.
            let u = ((d.time as f32 * 0.25 + phase + k as f32 * 0.25) % 1.0).abs();
            let p = base + WIND * rise * u;
            let r = (0.07 + 0.16 * u) * z;
            let a = ((1.0 - u) * if dark { 150.0 } else { 140.0 }) as u8;
            let col = if dark {
                Color32::from_rgba_unmultiplied(60, 55, 50, a)
            } else {
                Color32::from_rgba_unmultiplied(225, 225, 225, a)
            };
            if d.rect.contains(p) {
                d.painter.circle_filled(p, r, col);
            }
        }
    }
}

impl Layer for SmokeLayer {
    fn id(&self) -> &'static str {
        "smoke"
    }
    fn name(&self) -> &'static str {
        "Smoke"
    }
    fn shown_at(&self, zoom: f32) -> bool {
        zoom >= 6.0
    }
    fn legend(&self) -> Vec<(Swatch, &'static str)> {
        vec![
            (
                Swatch::Dot(Color32::from_rgb(210, 210, 210)),
                "Hearth smoke",
            ),
            (
                Swatch::Dot(Color32::from_rgb(70, 65, 60)),
                "Charcoal burning",
            ),
        ]
    }
    fn draw(&mut self, d: &DrawCtx) {
        let z = d.camera.zoom;
        if z < 6.0 {
            return;
        }
        let v = d.view;
        let t = &v.terrain;
        let g = t.grid;
        let snap = &v.snapshot;
        let (lo, hi) = (
            d.camera.to_map(d.rect, d.rect.min),
            d.camera.to_map(d.rect, d.rect.max),
        );
        let (x0, y0) = (
            (lo.x.floor() as i32 - 1).max(0),
            (lo.y.floor() as i32 - 1).max(0),
        );
        let (x1, y1) = (
            (hi.x.ceil() as i32 + 1).min(g.w as i32 - 1),
            (hi.y.ceil() as i32 + 1).min(g.h as i32 - 1),
        );
        let mut cells: std::collections::BTreeMap<u32, f32> = std::collections::BTreeMap::new();
        for &b in snap.built.iter().filter(|&&b| b != NONE) {
            *cells.entry(b).or_insert(0.0) += 1.0;
        }
        for y in y0..=y1 {
            for x in x0..=x1 {
                let i = g.idx(x, y);
                let c = d.camera.cell_center(d.rect, g.w, i);
                if snap.built[i] != NONE {
                    // Roughly one plume per few hundred people in the cell.
                    let pop = snap.settlement(snap.built[i]).map_or(0.0, |s| s.pop);
                    let n = cells.get(&snap.built[i]).copied().unwrap_or(1.0);
                    let plumes = ((pop / n / 400.0).ceil() as u32).clamp(1, 4);
                    for k in 0..plumes {
                        let h = hash(i, k);
                        let off = Vec2::new(
                            (h & 0xff) as f32 / 255.0 - 0.5,
                            (h >> 8 & 0xff) as f32 / 255.0 - 0.5,
                        ) * 0.6
                            * z;
                        Self::plume(d, c + off, (h >> 16 & 0xff) as f32 / 255.0, false);
                    }
                } else if t.is_land(i) && snap.owner[i] == NONE {
                    let f = snap.forest[i] as f32 / 255.0;
                    // A few coppiced cells host a charcoal clamp.
                    if f > 0.15 && f < t.forest0[i] - 0.08 && hash(i, 77) % 9 == 0 {
                        Self::plume(d, c, (hash(i, 5) & 0xff) as f32 / 255.0, true);
                    }
                }
            }
        }
        d.ctx.request_repaint();
    }
}
