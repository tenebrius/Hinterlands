//! Name labels, one layer per class so each can be toggled: settlements (sized
//! and revealed by rank as you zoom), rivers (along the main stem), ranges, seas
//! and bays, and forests (hidden once mostly cleared). Labels that would overlap
//! an earlier one in the same class are skipped. Lettering is the bundled serif,
//! italic for water and woodland, on a pale halo so it reads over any ground.

use super::{DrawCtx, Layer};
use crate::style::{serif, serif_italic};
use egui::epaint::TextShape;
use egui::text::{LayoutJob, TextFormat};
use egui::{Color32, Pos2, Rect};
use hinterlands_core::names::FeatureKind;
use hinterlands_core::state::Rank;

#[derive(Clone, Copy, PartialEq)]
pub enum Class {
    Settlements,
    Rivers,
    Ranges,
    Waters,
    Forests,
}

pub struct LabelsLayer {
    pub class: Class,
}

struct Placer {
    taken: Vec<Rect>,
}

impl Placer {
    fn free(&mut self, r: Rect) -> bool {
        if self.taken.iter().any(|t| t.intersects(r)) {
            return false;
        }
        self.taken.push(r);
        true
    }
}

/// Draw `text` centred at `pos` with a pale halo, rotated by `angle` radians;
/// `size` is in the old sans-serif points, scaled to match the serif's x-height.
/// Returns false (and draws nothing) if the spot is taken.
fn label(
    d: &DrawCtx,
    placer: &mut Placer,
    text: &str,
    pos: Pos2,
    size: f32,
    color: Color32,
    italic: bool,
    angle: f32,
) -> bool {
    let mut job = LayoutJob::default();
    job.append(
        text,
        0.0,
        TextFormat {
            font_id: if italic {
                serif_italic(size * 1.15)
            } else {
                serif(size * 1.15)
            },
            color,
            ..Default::default()
        },
    );
    let galley = d.ctx.fonts_mut(|f| f.layout_job(job));
    let sz = galley.size();
    let (s, c) = angle.sin_cos();
    // Rotate about the top-left corner so the text's centre lands on `pos`.
    let half = egui::vec2(sz.x / 2.0, sz.y / 2.0);
    let rotated = egui::vec2(half.x * c - half.y * s, half.x * s + half.y * c);
    let origin = pos - rotated;
    let extent = egui::vec2(
        (sz.x * c).abs() + (sz.y * s).abs(),
        (sz.x * s).abs() + (sz.y * c).abs(),
    );
    let bounds = Rect::from_center_size(pos, extent);
    if !d.rect.intersects(bounds) || !placer.free(bounds.expand(2.0)) {
        return false;
    }
    // The halo is the text traced at eight points round it.
    let halo = Color32::from_rgba_unmultiplied(250, 246, 235, 150);
    for k in 0..8 {
        let a = k as f32 * std::f32::consts::FRAC_PI_4;
        let off = egui::vec2(a.cos(), a.sin()) * 1.4;
        d.painter.add(
            TextShape::new(origin + off, galley.clone(), halo)
                .with_angle(angle)
                .with_override_text_color(halo),
        );
    }
    d.painter
        .add(TextShape::new(origin, galley, color).with_angle(angle));
    true
}

impl Layer for LabelsLayer {
    fn id(&self) -> &'static str {
        match self.class {
            Class::Settlements => "names_settlements",
            Class::Rivers => "names_rivers",
            Class::Ranges => "names_ranges",
            Class::Waters => "names_waters",
            Class::Forests => "names_forests",
        }
    }
    fn name(&self) -> &'static str {
        match self.class {
            Class::Settlements => "Names: settlements",
            Class::Rivers => "Names: rivers",
            Class::Ranges => "Names: hills and mountains",
            Class::Waters => "Names: seas and bays",
            Class::Forests => "Names: forests",
        }
    }
    fn draw(&mut self, d: &DrawCtx) {
        let v = d.view;
        let t = &v.terrain;
        let g = t.grid;
        let z = d.camera.zoom;
        let at = |c: u32| d.camera.cell_center(d.rect, g.w, c as usize);
        let mut placer = Placer { taken: Vec::new() };
        match self.class {
            Class::Settlements => {
                let mut by_size: Vec<_> = v.snapshot.settlements.iter().collect();
                by_size.sort_by(|a, b| b.pop.total_cmp(&a.pop).then(a.id.cmp(&b.id)));
                for s in by_size {
                    let (min_zoom, size) = match s.rank {
                        Rank::City => (0.0, 16.0),
                        Rank::Town => (6.0, 13.0),
                        Rank::Village => (12.0, 11.0),
                        Rank::Hamlet => (24.0, 10.0),
                    };
                    if z < min_zoom {
                        continue;
                    }
                    let p = at(s.anchor) + egui::vec2(0.0, -(size * 0.5 + 0.35 * z.min(12.0)));
                    let col = Color32::from_rgb(35, 22, 16);
                    if label(d, &mut placer, &s.name, p, size, col, false, 0.0)
                        && s.rank >= Rank::Town
                    {
                        let q = p + egui::vec2(0.0, size * 0.8);
                        label(
                            d,
                            &mut placer,
                            &format!("{:.0}", s.pop),
                            q,
                            size * 0.62,
                            Color32::from_rgb(90, 70, 60),
                            false,
                            0.0,
                        );
                    }
                }
            }
            Class::Rivers => {
                for f in t.features.iter().filter(|f| f.kind == FeatureKind::River) {
                    if f.size < 800.0 && z < 10.0 {
                        continue;
                    }
                    let k = f.cells.iter().position(|&c| c == f.label_cell).unwrap_or(0);
                    let a = at(f.cells[k.saturating_sub(3)]);
                    let b = at(f.cells[(k + 3).min(f.cells.len() - 1)]);
                    let mut angle = (b.y - a.y).atan2(b.x - a.x);
                    if angle > std::f32::consts::FRAC_PI_2 {
                        angle -= std::f32::consts::PI;
                    } else if angle < -std::f32::consts::FRAC_PI_2 {
                        angle += std::f32::consts::PI;
                    }
                    let size = (10.0 + 2.5 * (f.size / 200.0).max(1.0).log2()).clamp(10.0, 17.0);
                    let p = at(f.label_cell) + egui::vec2(-angle.sin(), angle.cos()) * (size * 0.8);
                    label(
                        d,
                        &mut placer,
                        &f.name,
                        p,
                        size,
                        Color32::from_rgb(25, 70, 150),
                        true,
                        angle,
                    );
                }
            }
            Class::Ranges => {
                for f in t.features.iter().filter(|f| f.kind == FeatureKind::Range) {
                    let size = (11.0 + (f.size / 25.0).log2() * 1.5).clamp(11.0, 18.0);
                    let spaced: String = f
                        .name
                        .to_uppercase()
                        .chars()
                        .flat_map(|c| [c, '\u{2009}'])
                        .collect();
                    label(
                        d,
                        &mut placer,
                        spaced.trim_end(),
                        at(f.label_cell),
                        size,
                        Color32::from_rgb(105, 72, 45),
                        false,
                        0.0,
                    );
                }
            }
            Class::Waters => {
                for f in t
                    .features
                    .iter()
                    .filter(|f| matches!(f.kind, FeatureKind::Sea | FeatureKind::Bay))
                {
                    let size = if f.kind == FeatureKind::Sea {
                        18.0
                    } else {
                        12.0
                    };
                    label(
                        d,
                        &mut placer,
                        &f.name,
                        at(f.label_cell),
                        size,
                        Color32::from_rgb(20, 45, 95),
                        true,
                        0.0,
                    );
                }
            }
            Class::Forests => {
                for f in t.features.iter().filter(|f| f.kind == FeatureKind::Forest) {
                    let cover = f
                        .cells
                        .iter()
                        .map(|&c| v.snapshot.forest[c as usize] as f32 / 255.0)
                        .sum::<f32>()
                        / f.cells.len().max(1) as f32;
                    if cover < 0.35 {
                        continue;
                    }
                    let size = (10.0 + (f.size / 60.0).log2() * 1.5).clamp(10.0, 15.0);
                    label(
                        d,
                        &mut placer,
                        &f.name,
                        at(f.label_cell),
                        size,
                        Color32::from_rgb(20, 70, 30),
                        true,
                        0.0,
                    );
                }
            }
        }
    }
}
