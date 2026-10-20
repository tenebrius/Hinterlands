//! Roads and trade routes. Roads are drawn brown under the routes; each route
//! segment's width follows the yearly flow summed over all paths that use it,
//! brown on land and blue on water.

use super::{DrawCtx, Layer};
use egui::{Color32, Stroke};
use std::collections::BTreeMap;

#[derive(Default)]
pub struct RoutesLayer {
    /// (cell a, cell b, on water) -> flow, rebuilt once per year.
    segments: Vec<(usize, usize, bool, f32)>,
    key: Option<(u64, u32)>,
}

pub fn segments(d: &DrawCtx) -> Vec<(usize, usize, bool, f32)> {
    let n = d.view.terrain.n() as u32;
    let mut acc: BTreeMap<(u32, u32, bool), f32> = BTreeMap::new();
    for f in &d.view.snapshot.flows {
        for w in f.path.windows(2) {
            let (a, b) = (w[0] % n, w[1] % n);
            if a == b {
                continue;
            }
            let water = w[0] >= n && w[1] >= n;
            let key = if a < b { (a, b, water) } else { (b, a, water) };
            *acc.entry(key).or_insert(0.0) += f.amount;
        }
    }
    acc.into_iter()
        .map(|((a, b, wtr), v)| (a as usize, b as usize, wtr, v))
        .collect()
}

impl Layer for RoutesLayer {
    fn id(&self) -> &'static str {
        "routes"
    }
    fn name(&self) -> &'static str {
        "Roads and trade routes"
    }
    fn draw(&mut self, d: &DrawCtx) {
        let v = d.view;
        let g = v.terrain.grid;
        let key = (v.terrain_version, v.snapshot.year);
        if self.key != Some(key) {
            self.segments = segments(d);
            self.key = Some(key);
        }
        let z = d.camera.zoom;
        // Roads.
        let road = &v.snapshot.road;
        let road_stroke = Stroke::new((0.25 * z).max(1.0), Color32::from_rgb(120, 78, 40));
        for i in 0..g.n() {
            if !road[i] || !v.terrain.is_land(i) {
                continue;
            }
            for dir in [0usize, 1, 2, 3] {
                if let Some(j) = g.neighbor(i, dir) {
                    if road[j] && v.terrain.is_land(j) {
                        d.painter.line_segment(
                            [
                                d.camera.cell_center(d.rect, g.w, i),
                                d.camera.cell_center(d.rect, g.w, j),
                            ],
                            road_stroke,
                        );
                    }
                }
            }
        }
        // Trade.
        let max = self.segments.iter().map(|s| s.3).fold(1.0f32, f32::max);
        for &(a, b, water, flow) in &self.segments {
            let w = (flow / max).sqrt() * 0.6 * z;
            if w < 0.4 {
                continue;
            }
            let col = if water {
                Color32::from_rgba_unmultiplied(170, 225, 255, 200)
            } else {
                Color32::from_rgba_unmultiplied(90, 50, 20, 200)
            };
            d.painter.line_segment(
                [
                    d.camera.cell_center(d.rect, g.w, a),
                    d.camera.cell_center(d.rect, g.w, b),
                ],
                Stroke::new(w, col),
            );
        }
    }
}
