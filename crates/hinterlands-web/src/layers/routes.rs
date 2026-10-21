//! Roads and trade routes. Every segment is drawn once: roads in brown along the
//! segments they were built on, water routes in blue; each segment's width
//! follows the yearly trade flow over it. Land trade off the roads, and minor
//! water routes, are drawn faintly or not at all, so the main corridors read.

use super::{DrawCtx, Layer, Swatch};
use egui::{Color32, Stroke};
use std::collections::BTreeMap;

/// Segments below this share of the largest flow are not drawn unless they are roads.
const MIN_SHARE: f32 = 0.02;

#[derive(Default)]
pub struct RoutesLayer {
    /// (cell a, cell b, on water) -> flow, rebuilt once per year.
    segments: BTreeMap<(u32, u32, bool), f32>,
    key: Option<(u64, u32)>,
}

pub fn segments(d: &DrawCtx) -> BTreeMap<(u32, u32, bool), f32> {
    let n = d.view.terrain.n() as u32;
    let mut acc: BTreeMap<(u32, u32, bool), f32> = BTreeMap::new();
    for f in &d.view.snapshot.flows {
        for w in f.path.windows(2) {
            let (a, b) = (w[0] % n, w[1] % n);
            if a == b {
                continue;
            }
            let water = w[0] >= n && w[1] >= n;
            *acc.entry((a.min(b), a.max(b), water)).or_insert(0.0) += f.amount;
        }
    }
    acc
}

impl Layer for RoutesLayer {
    fn id(&self) -> &'static str {
        "routes"
    }
    fn name(&self) -> &'static str {
        "Roads and trade routes"
    }
    fn legend(&self) -> Vec<(Swatch, &'static str)> {
        vec![
            (
                Swatch::Line(Color32::from_rgb(122, 80, 42), 3.0),
                "Road (width: trade)",
            ),
            (
                Swatch::Line(Color32::from_rgba_unmultiplied(110, 70, 35, 90), 2.0),
                "Land trade off-road",
            ),
            (
                Swatch::Line(Color32::from_rgb(175, 228, 255), 3.0),
                "River and sea trade",
            ),
        ]
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
        let max = self.segments.values().copied().fold(1.0f32, f32::max);
        let width = |flow: f32| (flow / max).sqrt() * 0.5 * z;
        let at = |c: u32| d.camera.cell_center(d.rect, g.w, c as usize);

        // Water routes first, under the roads.
        for (&(a, b, water), &flow) in &self.segments {
            if !water || flow < MIN_SHARE * max {
                continue;
            }
            let w = width(flow).max(1.0);
            d.painter.line_segment(
                [at(a), at(b)],
                Stroke::new(w, Color32::from_rgba_unmultiplied(175, 228, 255, 210)),
            );
        }
        // Land trade that does not follow a road: faint.
        let edges = &v.snapshot.road_edges;
        for (&(a, b, water), &flow) in &self.segments {
            if water || flow < MIN_SHARE * max || edges.binary_search(&(a, b)).is_ok() {
                continue;
            }
            let w = width(flow).max(0.8);
            d.painter.line_segment(
                [at(a), at(b)],
                Stroke::new(w, Color32::from_rgba_unmultiplied(110, 70, 35, 90)),
            );
        }
        // Roads, thickened by the trade they carry.
        let base = (0.12 * z).clamp(1.0, 3.0);
        for &(a, b) in edges {
            let flow = self.segments.get(&(a, b, false)).copied().unwrap_or(0.0);
            // Roads that now carry little trade fade back.
            let busy = (flow / (MIN_SHARE * 5.0 * max)).clamp(0.0, 1.0);
            let alpha = (90.0 + 165.0 * busy) as u8;
            let w = (base * (0.6 + 0.4 * busy)).max(width(flow));
            d.painter.line_segment(
                [at(a), at(b)],
                Stroke::new(w, Color32::from_rgba_unmultiplied(122, 80, 42, alpha)),
            );
        }
    }
}
