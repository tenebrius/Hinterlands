//! Goods (v2; drawing only): goods trade as smoothed lines coloured by good (salt
//! white, iron rust, stone grey), resource sites as symbols (square salt works,
//! triangle mine, diamond quarry; filled while selling), and a ring in the
//! good's colour round each specialist settlement.

use super::lines::chaikin;
use super::{DrawCtx, Layer, Swatch};
use egui::{pos2, vec2, Color32, Pos2, Shape, Stroke, Vec2};
use hinterlands_core::state::{Good, Kind};

pub const SALT: Color32 = Color32::from_rgb(246, 244, 236);
pub const IRON: Color32 = Color32::from_rgb(168, 78, 38);
pub const STONE: Color32 = Color32::from_rgb(132, 132, 128);
const INK: Color32 = Color32::from_rgb(40, 34, 30);

pub fn colour(g: Good) -> Color32 {
    match g {
        Good::Salt => SALT,
        Good::Iron => IRON,
        Good::Stone => STONE,
    }
}

#[derive(Default)]
pub struct GoodsLayer;

impl GoodsLayer {
    fn site_symbol(d: &DrawCtx, at: Pos2, good: Good, working: bool) {
        let r = (d.camera.zoom * 0.35).clamp(3.5, 9.0);
        let pts: Vec<Pos2> = match good {
            Good::Salt => vec![
                at + vec2(-r, -r),
                at + vec2(r, -r),
                at + vec2(r, r),
                at + vec2(-r, r),
            ],
            Good::Iron => vec![
                at + vec2(0.0, -r * 1.2),
                at + vec2(r, r * 0.8),
                at + vec2(-r, r * 0.8),
            ],
            Good::Stone => vec![
                at + vec2(0.0, -r * 1.2),
                at + vec2(r, 0.0),
                at + vec2(0.0, r * 1.2),
                at + vec2(-r, 0.0),
            ],
        };
        let fill = if working {
            colour(good)
        } else {
            Color32::TRANSPARENT
        };
        d.painter
            .add(Shape::convex_polygon(pts, fill, Stroke::new(1.2, INK)));
    }
}

impl Layer for GoodsLayer {
    fn id(&self) -> &'static str {
        "goods"
    }
    fn name(&self) -> &'static str {
        "Goods trade and works"
    }
    fn legend(&self) -> Vec<(Swatch, &'static str)> {
        vec![
            (Swatch::Line(SALT, 3.0), "Salt trade"),
            (Swatch::Line(IRON, 3.0), "Iron trade"),
            (Swatch::Line(STONE, 3.0), "Stone trade"),
            (Swatch::Fill(SALT), "Salt works (square)"),
            (Swatch::Fill(IRON), "Iron mine (triangle)"),
            (Swatch::Fill(STONE), "Quarry (diamond)"),
        ]
    }
    fn shown_at(&self, _zoom: f32) -> bool {
        true
    }
    fn draw(&mut self, d: &DrawCtx) {
        let v = d.view;
        let snap = &v.snapshot;
        if snap.sites.is_empty() {
            return;
        }
        let t = &v.terrain;
        let g = t.grid;
        let n = t.n() as u32;
        let z = d.camera.zoom;
        // Flow lines are map symbols: they fade out close in, with the routes.
        let fade = crate::scale::symbol_alpha(z, v.layer_on("symbols"));
        let on_screen = |cell: u32| -> Vec2 {
            let c = cell % n;
            let c = v.sea_lane.get(c as usize).copied().unwrap_or(c);
            let (x, y) = g.xy(c as usize);
            vec2(x as f32 + 0.5, y as f32 + 0.5)
        };
        // Goods flows, thickest last so they read on top.
        let mut flows: Vec<_> = snap.flows.iter().filter(|f| f.good.is_some()).collect();
        flows.sort_by(|a, b| a.amount.total_cmp(&b.amount));
        if fade <= 0.0 {
            flows.clear();
        }
        for f in flows {
            let Some(good) = f.good else { continue };
            if f.path.len() < 2 {
                continue;
            }
            let pts: Vec<Vec2> = f.path.iter().map(|&c| on_screen(c)).collect();
            let (smooth, _) = chaikin(&pts, &vec![(); pts.len()], 2, |a, _, _| a);
            let line: Vec<Pos2> = smooth
                .iter()
                .map(|&p| d.camera.to_screen(d.rect, p))
                .collect();
            let w = (0.6 + f.amount.sqrt() * 0.18).min(4.0) * (z / 6.0).clamp(0.6, 2.0);
            d.painter.add(Shape::line(
                line.clone(),
                Stroke::new(w + 1.4, INK.gamma_multiply(0.6 * fade)),
            ));
            d.painter.add(Shape::line(
                line,
                Stroke::new(w, colour(good).gamma_multiply(fade)),
            ));
        }
        // Specialists: a ring in the good's colour.
        for s in &snap.settlements {
            let good = match s.kind {
                Kind::Agrarian => continue,
                Kind::Mine => Good::Iron,
                Kind::Saltern => Good::Salt,
                Kind::Quarry => Good::Stone,
            };
            let c = d.camera.cell_center(d.rect, g.w, s.anchor as usize);
            let r = (z * 0.6).clamp(6.0, 14.0);
            d.painter
                .circle_stroke(c, r, Stroke::new(2.5, INK.gamma_multiply(0.7)));
            d.painter
                .circle_stroke(c, r, Stroke::new(1.6, colour(good)));
        }
        // Sites last, so they sit on top of everything.
        for site in &snap.sites {
            let c = d.camera.cell_center(d.rect, g.w, site.cell as usize);
            if !d.rect.expand(20.0).contains(c) {
                continue;
            }
            let at = pos2(c.x, c.y);
            Self::site_symbol(d, at, site.good, site.sold > 0.0);
        }
    }
}
