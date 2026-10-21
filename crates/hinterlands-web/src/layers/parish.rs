//! Footpaths and parish-like boundaries (1.x layers; drawing only), both from a
//! short walk out of each settlement over the existing cost grid:
//! - footpaths are the cheapest walking paths from a settlement to each of its
//!   fields and to its two nearest neighbours, where no road already runs;
//! - parishes give every land cell within reach to the settlement that reaches
//!   it most cheaply; faint lines mark where neighbouring parishes meet.

use super::{DrawCtx, Layer, Swatch};
use egui::{Color32, Stroke};
use hinterlands_core::cost::{CostGrid, Dijkstra};
use hinterlands_core::state::NONE;
use std::collections::BTreeSet;

/// Walking cost within which a settlement's parish and paths extend.
const REACH: f32 = 8.0;
const PATH: Color32 = Color32::from_rgb(238, 224, 186);
const BOUNDARY: Color32 = Color32::from_rgba_premultiplied(70, 60, 80, 110);

#[derive(Default)]
struct Local {
    /// Footpath segments (cell pairs, sorted), excluding road segments.
    paths: Vec<(u32, u32)>,
    /// Parish of each cell (settlement id), or NONE.
    parish: Vec<u32>,
    key: Option<(u64, u32)>,
}

fn compute(d: &DrawCtx, local: &mut Local) {
    let v = d.view;
    let key = (v.terrain_version, v.snapshot.year / 5);
    if local.key == Some(key) {
        return;
    }
    let t = &v.terrain;
    let n = t.n();
    let snap = &v.snapshot;
    let g = CostGrid::build(t, &snap.road, &snap.transport);
    let mut dj = Dijkstra::new(n);
    let mut best = vec![f32::INFINITY; n];
    let mut parish = vec![NONE; n];
    let mut segs: BTreeSet<(u32, u32)> = BTreeSet::new();
    let add_path = |path: &[u32], segs: &mut BTreeSet<(u32, u32)>| {
        for w in path.windows(2) {
            if (w[0] as usize) < n && (w[1] as usize) < n {
                let e = (w[0].min(w[1]), w[0].max(w[1]));
                if snap.road_edges.binary_search(&e).is_err() {
                    segs.insert(e);
                }
            }
        }
    };
    for s in &snap.settlements {
        dj.run(&g, s.anchor, REACH, true);
        for &node in dj.touched() {
            let c = node as usize;
            if c < n && t.is_land(c) && dj.dist[c] < best[c] {
                best[c] = dj.dist[c];
                parish[c] = s.id;
            }
        }
        for (c, &o) in snap.owner.iter().enumerate() {
            if o == s.id && dj.dist[c].is_finite() {
                add_path(&dj.path_to(c as u32), &mut segs);
            }
        }
        let mut near: Vec<(f32, u32)> = snap
            .settlements
            .iter()
            .filter(|o| o.id != s.id && dj.dist[o.anchor as usize].is_finite())
            .map(|o| (dj.dist[o.anchor as usize], o.anchor))
            .collect();
        near.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
        for &(_, a) in near.iter().take(2) {
            add_path(&dj.path_to(a), &mut segs);
        }
    }
    *local = Local {
        paths: segs.into_iter().collect(),
        parish,
        key: Some(key),
    };
}

pub struct FootpathsLayer(Local);
pub struct ParishLayer(Local);

impl FootpathsLayer {
    pub fn new() -> Self {
        Self(Local::default())
    }
}

impl ParishLayer {
    pub fn new() -> Self {
        Self(Local::default())
    }
}

impl Layer for FootpathsLayer {
    fn id(&self) -> &'static str {
        "footpaths"
    }
    fn name(&self) -> &'static str {
        "Footpaths"
    }
    fn shown_at(&self, zoom: f32) -> bool {
        zoom >= 8.0
    }
    fn legend(&self) -> Vec<(Swatch, &'static str)> {
        vec![(Swatch::Line(PATH, 1.0), "Footpath")]
    }
    fn draw(&mut self, d: &DrawCtx) {
        if d.camera.zoom < 8.0 {
            return;
        }
        compute(d, &mut self.0);
        let w = d.view.terrain.grid.w;
        let stroke = Stroke::new((0.05 * d.camera.zoom).clamp(1.0, 2.2), PATH);
        for &(a, b) in &self.0.paths {
            let (pa, pb) = (
                d.camera.cell_center(d.rect, w, a as usize),
                d.camera.cell_center(d.rect, w, b as usize),
            );
            if !d.rect.contains(pa) && !d.rect.contains(pb) {
                continue;
            }
            // Dashed: two dashes per segment.
            for k in 0..2 {
                let (u0, u1) = (k as f32 * 0.5 + 0.08, k as f32 * 0.5 + 0.36);
                d.painter
                    .line_segment([pa + (pb - pa) * u0, pa + (pb - pa) * u1], stroke);
            }
        }
    }
}

impl Layer for ParishLayer {
    fn id(&self) -> &'static str {
        "parishes"
    }
    fn name(&self) -> &'static str {
        "Parish boundaries"
    }
    fn default_on(&self) -> bool {
        false
    }
    fn shown_at(&self, zoom: f32) -> bool {
        zoom >= 4.0
    }
    fn legend(&self) -> Vec<(Swatch, &'static str)> {
        vec![(Swatch::Line(BOUNDARY, 1.0), "Parish boundary")]
    }
    fn draw(&mut self, d: &DrawCtx) {
        if d.camera.zoom < 4.0 {
            return;
        }
        compute(d, &mut self.0);
        let g = d.view.terrain.grid;
        let z = d.camera.zoom;
        let stroke = Stroke::new(1.0, BOUNDARY);
        let p = &self.0.parish;
        for i in 0..g.n() {
            if p[i] == NONE {
                continue;
            }
            let c = d.camera.cell_center(d.rect, g.w, i);
            if !d.rect.expand(z).contains(c) {
                continue;
            }
            // East and south edges only, so each shared edge is drawn once.
            for (k, a, b) in [
                (0usize, egui::vec2(0.5, -0.5), egui::vec2(0.5, 0.5)),
                (2, egui::vec2(-0.5, 0.5), egui::vec2(0.5, 0.5)),
            ] {
                if let Some(j) = g.neighbor(i, k) {
                    if p[j] != NONE && p[j] != p[i] {
                        d.painter.line_segment([c + a * z, c + b * z], stroke);
                    }
                }
            }
        }
    }
}
