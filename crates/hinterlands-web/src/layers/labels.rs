//! Name labels, one layer per class so each can be toggled: settlements (sized
//! and revealed by rank as you zoom), rivers (along the main stem), ranges, seas
//! and bays, and forests (hidden once mostly cleared). Labels that would overlap
//! an earlier one in the same class are skipped. Settlement names take
//! priority: a feature name that would cover one moves a little, or hides. Lettering is the bundled serif,
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
    /// False for a dry run that only reserves space.
    draw: bool,
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
    if !placer.draw {
        return true;
    }
    // The halo is the text traced at points round it, thick and nearly
    // opaque so a name reads over roofs and water alike (R25).
    let halo = Color32::from_rgba_unmultiplied(250, 246, 235, 215);
    for k in 0..12 {
        let a = k as f32 * std::f32::consts::TAU / 12.0;
        let off = egui::vec2(a.cos(), a.sin()) * 2.0;
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

/// Settlement names (and town populations), largest first, revealed by rank as
/// you zoom. With a dry placer this only reserves their space.
fn place_settlements(d: &DrawCtx, placer: &mut Placer) {
    let v = d.view;
    let g = v.terrain.grid;
    let z = d.camera.zoom;
    let at = |c: u32| d.camera.cell_center(d.rect, g.w, c as usize);
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
        let mut p = at(s.anchor) + egui::vec2(0.0, -(size * 0.5 + 0.35 * z.min(12.0)));
        let col = Color32::from_rgb(35, 22, 16);
        // Close in, where the centre is a bridge or a market place, the name
        // stands in a small cartouche above it on a leader line (R25).
        let cartouche = z > 64.0 && placer.draw;
        let size = if cartouche { size + 3.0 } else { size };
        if cartouche {
            let c = at(s.anchor);
            p = c + egui::vec2(0.0, -90.0);
            let w = size * 0.62 * s.name.chars().count() as f32 + 18.0;
            let r = Rect::from_center_size(p, egui::vec2(w, size * 1.9));
            if d.rect.intersects(r) && placer.free(r.expand(2.0)) {
                let ink = Color32::from_rgb(60, 44, 32);
                d.painter
                    .line_segment([c, r.center_bottom()], egui::Stroke::new(1.2, ink));
                d.painter.circle_filled(c, 3.0, ink);
                d.painter
                    .rect_filled(r, 3.0, Color32::from_rgba_unmultiplied(246, 238, 218, 235));
                d.painter.rect_stroke(
                    r,
                    3.0,
                    egui::Stroke::new(1.2, ink),
                    egui::StrokeKind::Inside,
                );
                // The name itself is then drawn into the freed spot.
                placer.taken.pop();
            } else {
                continue;
            }
        }
        if label(d, placer, &s.name, p, size, col, false, 0.0) && s.rank >= Rank::Town && !cartouche
        {
            let q = p + egui::vec2(0.0, size * 0.8);
            label(
                d,
                placer,
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

/// A feature name at `pos`, or, if that spot is taken, just above, below, left
/// or right of it; hidden if none is free.
#[allow(clippy::too_many_arguments)]
fn label_moving(
    d: &DrawCtx,
    placer: &mut Placer,
    text: &str,
    pos: Pos2,
    size: f32,
    color: Color32,
    italic: bool,
    angle: f32,
) -> bool {
    let h = size * 1.5;
    let w = size * 0.3 * text.chars().count() as f32;
    [
        egui::vec2(0.0, 0.0),
        egui::vec2(0.0, -h),
        egui::vec2(0.0, h),
        egui::vec2(-w, 0.0),
        egui::vec2(w, 0.0),
    ]
    .iter()
    .any(|&off| label(d, placer, text, pos + off, size, color, italic, angle))
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
        let mut placer = Placer {
            taken: Vec::new(),
            draw: true,
        };
        // Feature names give way to settlement names when those are shown.
        if self.class != Class::Settlements && v.layers_on.contains("names_settlements") {
            let mut dry = Placer {
                taken: Vec::new(),
                draw: false,
            };
            place_settlements(d, &mut dry);
            placer.taken = dry.taken;
        }
        match self.class {
            Class::Settlements => place_settlements(d, &mut placer),
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
                    label_moving(
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
                    label_moving(
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
                    label_moving(
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
                    label_moving(
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
