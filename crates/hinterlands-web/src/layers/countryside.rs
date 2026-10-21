//! The farmed countryside, close up (1.x layers; drawing only). Each is computed
//! from one year's snapshot:
//! - field strips: cultivated cells drawn as strips running away from the
//!   nearest road or river, one strip in three lying fallow in a rotation that
//!   advances each year;
//! - clearing frontier: fields still carrying forest are being cleared, drawn
//!   with stumps;
//! - pasture and managed woodland: open uncultivated land just beyond a
//!   settlement's fields is grazed (sheep), and woodland thinned below its natural
//!   cover by fuel cutting is coppice.

use super::{DrawCtx, Layer, Swatch};
use egui::{Color32, Rect, Stroke};
use hinterlands_core::state::NONE;

const CROP: Color32 = Color32::from_rgb(226, 196, 98);
const FALLOW: Color32 = Color32::from_rgb(160, 178, 110);

/// Crop, alternate crop and fallow colours by season (spring, summer, autumn, winter).
const SEASON_FIELDS: [[Color32; 3]; 4] = [
    [
        Color32::from_rgb(132, 176, 86),
        Color32::from_rgb(118, 160, 76),
        Color32::from_rgb(150, 186, 106),
    ],
    [CROP, Color32::from_rgb(206, 172, 80), FALLOW],
    [
        Color32::from_rgb(186, 152, 98),
        Color32::from_rgb(170, 136, 86),
        Color32::from_rgb(150, 160, 100),
    ],
    [
        Color32::from_rgb(140, 118, 92),
        Color32::from_rgb(126, 104, 80),
        Color32::from_rgb(136, 142, 108),
    ],
];
const STUMP: Color32 = Color32::from_rgb(110, 78, 48);
const PASTURE: Color32 = Color32::from_rgba_premultiplied(52, 68, 34, 70);
const COPPICE: Color32 = Color32::from_rgb(40, 92, 44);

fn hash(i: usize, k: u32) -> u32 {
    let mut h = (i as u32).wrapping_mul(2654435761) ^ k.wrapping_mul(40503);
    h ^= h >> 15;
    h = h.wrapping_mul(0x2c1b3c6d);
    h ^= h >> 12;
    h
}

/// Deterministic points in a cell, as fractions of its side.
fn scatter(i: usize, n: u32) -> impl Iterator<Item = (f32, f32)> {
    (0..n).map(move |k| {
        let h = hash(i, k + 1);
        (
            0.15 + 0.7 * (h & 0xffff) as f32 / 65535.0,
            0.15 + 0.7 * (h >> 16) as f32 / 65535.0,
        )
    })
}

fn cell_rect(d: &DrawCtx, i: usize) -> Rect {
    let g = d.view.terrain.grid;
    let c = d.camera.cell_center(d.rect, g.w, i);
    Rect::from_center_size(c, egui::vec2(d.camera.zoom, d.camera.zoom))
}

#[derive(Clone, Copy, PartialEq)]
pub enum Kind {
    Strips,
    Frontier,
    Grazing,
}

#[derive(Default)]
struct Orientation {
    /// Per cell: strips run east-west (true) or north-south (false).
    horizontal: Vec<bool>,
    key: Option<(u64, u32)>,
}

pub struct CountrysideLayer {
    pub kind: Kind,
    orient: Orientation,
}

impl CountrysideLayer {
    pub fn new(kind: Kind) -> Self {
        Self {
            kind,
            orient: Orientation::default(),
        }
    }

    /// Strips run away from the nearest road or river: if the nearest such cell
    /// (within 3 cells) lies east or west, strips run east-west.
    fn orient(&mut self, d: &DrawCtx) {
        let v = d.view;
        let key = (v.terrain_version, v.snapshot.year / 10);
        if self.orient.key == Some(key) {
            return;
        }
        let t = &v.terrain;
        let g = t.grid;
        let mut out = vec![false; g.n()];
        for (i, o) in out.iter_mut().enumerate() {
            if v.snapshot.owner[i] == NONE {
                continue;
            }
            let (x, y) = g.xy(i);
            let mut best: Option<(i32, i32, i32)> = None;
            for dy in -3..=3 {
                for dx in -3..=3 {
                    let (nx, ny) = (x + dx, y + dy);
                    if (dx, dy) == (0, 0) || !g.in_bounds(nx, ny) {
                        continue;
                    }
                    let j = g.idx(nx, ny);
                    if v.snapshot.road[j] || t.is_river(j) {
                        let dist = dx * dx + dy * dy;
                        if best.is_none_or(|b| dist < b.0) {
                            best = Some((dist, dx, dy));
                        }
                    }
                }
            }
            *o = match best {
                Some((_, dx, dy)) => dx.abs() >= dy.abs(),
                None => hash(i, 99) % 2 == 0,
            };
        }
        self.orient = Orientation {
            horizontal: out,
            key: Some(key),
        };
    }
}

impl Layer for CountrysideLayer {
    fn id(&self) -> &'static str {
        match self.kind {
            Kind::Strips => "strips",
            Kind::Frontier => "frontier",
            Kind::Grazing => "grazing",
        }
    }
    fn name(&self) -> &'static str {
        match self.kind {
            Kind::Strips => "Field strips and fallow",
            Kind::Frontier => "Clearing frontier",
            Kind::Grazing => "Pasture and woodland",
        }
    }
    fn shown_at(&self, zoom: f32) -> bool {
        zoom >= if self.kind == Kind::Frontier {
            8.0
        } else {
            6.0
        }
    }
    fn legend(&self) -> Vec<(Swatch, &'static str)> {
        match self.kind {
            Kind::Strips => vec![
                (Swatch::Fill(CROP), "Crop strips"),
                (Swatch::Fill(FALLOW), "Fallow (1 in 3)"),
            ],
            Kind::Frontier => vec![(Swatch::Dot(STUMP), "Land being cleared")],
            Kind::Grazing => vec![
                (Swatch::Fill(Color32::from_rgb(150, 190, 100)), "Pasture"),
                (Swatch::Dot(COPPICE), "Managed woodland"),
            ],
        }
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
        let p = d.painter;
        // Only cells on screen.
        let (lo, hi) = (
            d.camera.to_map(d.rect, d.rect.min),
            d.camera.to_map(d.rect, d.rect.max),
        );
        let (x0, y0) = ((lo.x.floor() as i32).max(0), (lo.y.floor() as i32).max(0));
        let (x1, y1) = (
            (hi.x.ceil() as i32).min(g.w as i32 - 1),
            (hi.y.ceil() as i32).min(g.h as i32 - 1),
        );
        let visible = (y0..=y1).flat_map(move |y| (x0..=x1).map(move |x| g.idx(x, y)));
        match self.kind {
            Kind::Strips => {
                self.orient(d);
                let season = v.season_index(d.time);
                let strips = 6;
                for i in visible {
                    if snap.owner[i] == NONE || snap.built[i] != NONE {
                        continue;
                    }
                    let r = cell_rect(d, i);
                    let horizontal = self.orient.horizontal[i];
                    let rot = hash(i, 7);
                    for k in 0..strips {
                        let (a, b) = (k as f32 / strips as f32, (k + 1) as f32 / strips as f32);
                        let sr = if horizontal {
                            Rect::from_min_max(
                                egui::pos2(r.left(), r.top() + a * r.height()),
                                egui::pos2(r.right(), r.top() + b * r.height()),
                            )
                        } else {
                            Rect::from_min_max(
                                egui::pos2(r.left() + a * r.width(), r.top()),
                                egui::pos2(r.left() + b * r.width(), r.bottom()),
                            )
                        };
                        // Three-field rotation: the fallow third moves on each year.
                        let fallow = (k as u32 + snap.year + rot) % 3 == 0;
                        let pal = SEASON_FIELDS[season];
                        let col = if fallow {
                            pal[2]
                        } else if k % 2 == 0 {
                            pal[0]
                        } else {
                            pal[1]
                        };
                        p.rect_filled(sr, 0.0, col);
                    }
                    if z >= 14.0 {
                        p.rect_stroke(
                            r,
                            0.0,
                            Stroke::new(1.0, Color32::from_rgba_premultiplied(90, 80, 40, 90)),
                            egui::StrokeKind::Inside,
                        );
                    }
                }
            }
            Kind::Frontier => {
                if z < 8.0 {
                    return;
                }
                for i in visible {
                    let f = snap.forest[i] as f32 / 255.0;
                    if snap.owner[i] == NONE || snap.built[i] != NONE || f < 0.02 {
                        continue;
                    }
                    let r = cell_rect(d, i);
                    let n = (2.0 + f * 10.0) as u32;
                    for (fx, fy) in scatter(i, n) {
                        let c = egui::pos2(r.left() + fx * r.width(), r.top() + fy * r.height());
                        p.circle_filled(c, (0.035 * z).clamp(1.0, 3.0), STUMP);
                    }
                }
            }
            Kind::Grazing => {
                for i in visible {
                    if !t.is_land(i)
                        || snap.owner[i] != NONE
                        || snap.built[i] != NONE
                        || t.is_river(i)
                    {
                        continue;
                    }
                    let f = snap.forest[i] as f32 / 255.0;
                    let r = cell_rect(d, i);
                    let near_fields =
                        (0..8).any(|k| g.neighbor(i, k).is_some_and(|j| snap.owner[j] != NONE));
                    if near_fields && f < 0.35 {
                        let winter = v.season_index(d.time) == 3;
                        let tint = if winter {
                            Color32::from_rgba_premultiplied(60, 58, 44, 70)
                        } else {
                            PASTURE
                        };
                        p.rect_filled(r, 0.0, tint);
                        if z >= 10.0 {
                            for (fx, fy) in scatter(i, 3) {
                                let c = egui::pos2(
                                    r.left() + fx * r.width(),
                                    r.top() + fy * r.height(),
                                );
                                p.circle(
                                    c,
                                    (0.04 * z).clamp(1.2, 3.5),
                                    Color32::from_rgb(245, 245, 240),
                                    Stroke::new(0.8, Color32::from_gray(60)),
                                );
                            }
                        }
                    } else if f > 0.15 && f < t.forest0[i] - 0.08 && z >= 8.0 {
                        // Thinned by fuel cutting: coppice stools in rows.
                        for (fx, fy) in scatter(i, 6) {
                            let c =
                                egui::pos2(r.left() + fx * r.width(), r.top() + fy * r.height());
                            p.circle(
                                c,
                                (0.05 * z).clamp(1.2, 4.0),
                                COPPICE,
                                Stroke::new(0.6, Color32::from_rgb(20, 50, 20)),
                            );
                        }
                    }
                }
            }
        }
    }
}
