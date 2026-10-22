//! Footpaths and parish-like boundaries (1.x layers; drawing only), both from a
//! short walk out of each settlement over the existing cost grid:
//! - footpaths are the cheapest walking paths from a settlement to each of its
//!   fields and to its two nearest neighbours, where no road already runs;
//! - parishes give every land cell within reach to the settlement that reaches
//!   it most cheaply; faint lines mark where neighbouring parishes meet.
//!
//! Both are drawn as smoothed lines: footpaths through cell centres, kept sharp
//! at settlements; boundaries along cell corners, kept sharp where three
//! parishes meet.

use super::{lines, DrawCtx, Layer, Swatch};
use crate::view::ViewState;
use egui::{Color32, Pos2, Shape, Stroke, Vec2};
use hinterlands_core::cost::{CostGrid, Dijkstra};
use hinterlands_core::state::NONE;
use std::collections::BTreeSet;

/// Walking cost within which a settlement's parish and paths extend.
const REACH: f32 = 8.0;
const PATH: Color32 = Color32::from_rgb(238, 224, 186);
const BOUNDARY: Color32 = Color32::from_rgba_premultiplied(70, 60, 80, 110);

#[derive(Default)]
pub struct Local {
    /// Footpath segments (cell pairs, sorted), excluding road segments.
    paths: Vec<(u32, u32)>,
    /// Parish of each cell (settlement id), or NONE.
    parish: Vec<u32>,
    key: Option<(u64, u32)>,
}

/// The footpath segments (cell pairs, sorted, roads left out), walked again
/// every five years; the true-width paths in the relief shader use them too.
pub fn footpaths<'a>(v: &ViewState, local: &'a mut Local) -> &'a [(u32, u32)] {
    compute(v, local);
    &local.paths
}

fn compute(v: &ViewState, local: &mut Local) {
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

/// Lines in map space (cells), rebuilt when the walk is.
#[derive(Default)]
struct Lines {
    lines: Vec<Vec<Vec2>>,
    key: Option<(u64, u32)>,
}

pub struct FootpathsLayer(Local, Lines);
pub struct ParishLayer(Local, Lines);

impl FootpathsLayer {
    pub fn new() -> Self {
        Self(Local::default(), Lines::default())
    }
}

impl ParishLayer {
    pub fn new() -> Self {
        Self(Local::default(), Lines::default())
    }
}

fn smoothed(chains: Vec<Vec<u32>>, at: impl Fn(u32) -> Vec2, rounds: usize) -> Vec<Vec<Vec2>> {
    chains
        .into_iter()
        .map(|c| {
            let pts: Vec<Vec2> = c.iter().map(|&n| at(n)).collect();
            lines::chaikin(&pts, &vec![(); pts.len()], rounds, |_, _, _| ()).0
        })
        .collect()
}

/// Draws each line dashed, skipping lines entirely off screen.
fn draw_dashed(d: &DrawCtx, lines: &[Vec<Vec2>], stroke: Stroke, dash: f32, gap: f32) {
    let view = d.rect.expand(20.0);
    for l in lines {
        let pts: Vec<Pos2> = l.iter().map(|&p| d.camera.to_screen(d.rect, p)).collect();
        if !pts.iter().any(|p| view.contains(*p)) {
            continue;
        }
        for s in Shape::dashed_line(&pts, stroke, dash, gap) {
            d.painter.add(s);
        }
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
        compute(d.view, &mut self.0);
        let v = d.view;
        if self.1.key != self.0.key {
            let g = v.terrain.grid;
            let edges: BTreeSet<(u32, u32)> = self.0.paths.iter().copied().collect();
            let pins: BTreeSet<u32> = v.snapshot.settlements.iter().map(|s| s.anchor).collect();
            let at = |c: u32| {
                let (x, y) = g.xy(c as usize);
                Vec2::new(x as f32 + 0.5, y as f32 + 0.5)
            };
            self.1 = Lines {
                lines: smoothed(lines::chains(&edges, &pins), at, 2),
                key: self.0.key,
            };
        }
        let z = d.camera.zoom;
        // A map symbol: it fades out close in with the roads' symbols. (Paths at
        // true width belong with the fields, in the next stage of the overhaul.)
        let fade = crate::scale::symbol_alpha(z, v.layer_on("symbols"));
        if fade <= 0.0 {
            return;
        }
        let stroke = Stroke::new((0.05 * z).clamp(1.0, 2.2), PATH.gamma_multiply(fade));
        let dash = (0.14 * z).clamp(3.0, 9.0);
        draw_dashed(d, &self.1.lines, stroke, dash, dash * 0.8);
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
        compute(d.view, &mut self.0);
        if self.1.key != self.0.key {
            let g = d.view.terrain.grid;
            let p = &self.0.parish;
            // Boundary edges between cell corners; corner (x, y) has id y * (w + 1) + x.
            let corner = |x: i32, y: i32| (y as u32) * (g.w + 1) + x as u32;
            let mut edges: BTreeSet<(u32, u32)> = BTreeSet::new();
            for i in 0..g.n() {
                if p[i] == NONE {
                    continue;
                }
                let (x, y) = g.xy(i);
                // East and south neighbours, so each shared edge is found once.
                for (k, a, b) in [
                    (0usize, (x + 1, y), (x + 1, y + 1)),
                    (2, (x, y + 1), (x + 1, y + 1)),
                ] {
                    if let Some(j) = g.neighbor(i, k) {
                        if p[j] != NONE && p[j] != p[i] {
                            let (ca, cb) = (corner(a.0, a.1), corner(b.0, b.1));
                            edges.insert((ca.min(cb), ca.max(cb)));
                        }
                    }
                }
            }
            let at = |c: u32| Vec2::new((c % (g.w + 1)) as f32, (c / (g.w + 1)) as f32);
            self.1 = Lines {
                lines: smoothed(lines::chains(&edges, &BTreeSet::new()), at, 3),
                key: self.0.key,
            };
        }
        let z = d.camera.zoom;
        let dash = (0.2 * z).clamp(4.0, 12.0);
        draw_dashed(
            d,
            &self.1.lines,
            Stroke::new(1.2, BOUNDARY),
            dash,
            dash * 0.5,
        );
    }
}
