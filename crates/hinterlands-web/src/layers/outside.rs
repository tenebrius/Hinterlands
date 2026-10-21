//! The outside world (v5; drawing only): imports from the outside ports as a
//! sixth colour of route, wine red, and each outside port as a ship at the map
//! edge.

use super::lines::chaikin;
use super::{DrawCtx, Layer, Swatch};
use egui::{vec2, Color32, Pos2, Shape, Stroke, Vec2};

pub const IMPORTS: Color32 = Color32::from_rgb(128, 30, 72);
const HULL: Color32 = Color32::from_rgb(92, 60, 36);
const SAIL: Color32 = Color32::from_rgb(240, 232, 214);
const INK: Color32 = Color32::from_rgb(40, 34, 30);
/// Most import flows drawn.
const MAX_DRAWN: usize = 25;

#[derive(Default)]
pub struct OutsideLayer;

impl Layer for OutsideLayer {
    fn id(&self) -> &'static str {
        "outside"
    }
    fn name(&self) -> &'static str {
        "Outside ports and imports"
    }
    fn legend(&self) -> Vec<(Swatch, &'static str)> {
        vec![
            (Swatch::Line(IMPORTS, 3.0), "Imports from beyond the sea"),
            (Swatch::Fill(HULL), "Outside port (ship at the map edge)"),
        ]
    }
    fn draw(&mut self, d: &DrawCtx) {
        let v = d.view;
        let snap = &v.snapshot;
        if snap.ports.is_empty() {
            return;
        }
        let g = v.terrain.grid;
        let n = v.terrain.n() as u32;
        let z = d.camera.zoom;
        let on_map = |node: u32| -> Vec2 {
            let (x, y) = g.xy((node % n) as usize);
            vec2(x as f32 + 0.5, y as f32 + 0.5)
        };
        // Only the larger import flows, so the routes read rather than flood the
        // sea; the rest are small and follow the same lanes.
        let mut flows: Vec<_> = snap.flows.iter().filter(|f| f.import).collect();
        flows.sort_by(|a, b| b.amount.total_cmp(&a.amount));
        flows.truncate(MAX_DRAWN);
        flows.reverse();
        for f in flows {
            if f.path.len() < 2 {
                continue;
            }
            let pts: Vec<Vec2> = f.path.iter().map(|&c| on_map(c)).collect();
            let (smooth, _) = chaikin(&pts, &vec![(); pts.len()], 2, |a, _, _| a);
            let line: Vec<Pos2> = smooth
                .iter()
                .map(|&p| d.camera.to_screen(d.rect, p))
                .collect();
            let w = (0.4 + f.amount.sqrt() * 0.12).min(3.0) * (z / 6.0).clamp(0.6, 2.0);
            d.painter
                .add(Shape::line(line, Stroke::new(w, IMPORTS.gamma_multiply(0.75))));
        }
        // A ship: a hull and a square sail.
        for &cell in &snap.ports {
            let c = d.camera.cell_center(d.rect, g.w, cell as usize);
            let s = (z * 1.2).clamp(16.0, 30.0);
            let hull = vec![
                c + vec2(-s * 0.6, 0.0),
                c + vec2(s * 0.6, 0.0),
                c + vec2(s * 0.4, s * 0.3),
                c + vec2(-s * 0.4, s * 0.3),
            ];
            d.painter
                .add(Shape::convex_polygon(hull, HULL, Stroke::new(1.0, INK)));
            d.painter
                .line_segment([c, c + vec2(0.0, -s * 0.8)], Stroke::new(1.2, INK));
            let sail = vec![
                c + vec2(-s * 0.3, -s * 0.75),
                c + vec2(s * 0.3, -s * 0.75),
                c + vec2(s * 0.3, -s * 0.15),
                c + vec2(-s * 0.3, -s * 0.15),
            ];
            d.painter
                .add(Shape::convex_polygon(sail, SAIL, Stroke::new(1.0, INK)));
        }
    }
}
