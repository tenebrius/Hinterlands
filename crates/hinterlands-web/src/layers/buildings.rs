//! Site-earned buildings and river crossings, derived from each year's snapshot
//! (1.x layer; drawing only, nothing feeds back). A settlement earns a church as
//! a village, a market square as a town, walls as a city, and a cathedral as the
//! largest city or one over 8,000; a water mill beside a stream; a quay where a
//! town meets a navigable river or the sea, with warehouses once its trade is
//! heavy. Road crossings of rivers become bridges where traffic is heavy; quieter
//! crossings at fords are drawn as fords.

use super::routes::segments;
use super::{DrawCtx, Layer, Swatch};
use egui::{Color32, Pos2, Shape, Stroke, Vec2};
use hinterlands_core::state::{Rank, NONE};
use hinterlands_core::terrain::Water;
use std::collections::BTreeMap;

const STONE: Color32 = Color32::from_rgb(232, 224, 206);
const OUTLINE: Color32 = Color32::from_rgb(70, 55, 45);
const WALL: Color32 = Color32::from_rgb(150, 140, 125);
const QUAY: Color32 = Color32::from_rgb(92, 64, 40);
/// A crossing becomes a bridge when its traffic is this share of the busiest segment.
const BRIDGE_SHARE: f32 = 0.04;
const CATHEDRAL_POP: f32 = 8000.0;
const WAREHOUSE_TRADE: f32 = 1500.0;

/// Positions are in map space: cell units, (x + 0.5, y + 0.5) is a cell centre.
enum Glyph {
    Church(Vec2),
    Cathedral(Vec2),
    Square(Vec2),
    Mill(Vec2),
    Quay(Vec2, Vec2),
    Warehouse(Vec2),
    Wall(Vec2, Vec2),
    Tower(Vec2),
    Bridge(Vec2, f32),
    Ford(Vec2, f32),
}

#[derive(Default)]
pub struct BuildingsLayer {
    glyphs: Vec<Glyph>,
    key: Option<(u64, u32)>,
}

fn centre(w: u32, cell: usize) -> Vec2 {
    Vec2::new(
        (cell as u32 % w) as f32 + 0.5,
        (cell as u32 / w) as f32 + 0.5,
    )
}

impl BuildingsLayer {
    fn rebuild(&mut self, d: &DrawCtx) {
        let v = d.view;
        let t = &v.terrain;
        let g = t.grid;
        let snap = &v.snapshot;
        let mut out = Vec::new();
        let largest = snap
            .settlements
            .iter()
            .map(|s| s.pop)
            .fold(0.0f32, f32::max);
        // Built cells per settlement.
        let mut built: BTreeMap<u32, Vec<usize>> = BTreeMap::new();
        for (i, &b) in snap.built.iter().enumerate() {
            if b != NONE {
                built.entry(b).or_default().push(i);
            }
        }
        for s in &snap.settlements {
            let a = s.anchor as usize;
            let c = centre(g.w, a);
            let cells = built.get(&s.id).cloned().unwrap_or_else(|| vec![a]);
            if s.rank >= Rank::Town {
                out.push(Glyph::Square(c));
            }
            if s.rank == Rank::City && (s.pop >= CATHEDRAL_POP || s.pop >= largest) {
                out.push(Glyph::Cathedral(c + Vec2::new(0.05, 0.3)));
            } else if s.rank >= Rank::Village {
                out.push(Glyph::Church(c + Vec2::new(0.28, 0.26)));
            }
            // A water mill on a stream beside the settlement.
            if s.rank >= Rank::Village {
                let stream = cells
                    .iter()
                    .flat_map(|&b| (0..8).filter_map(move |k| g.neighbor(b, k)))
                    .find(|&j| t.water[j] == Water::Stream);
                if let Some(j) = stream {
                    let p = centre(g.w, j);
                    out.push(Glyph::Mill(p + (c - p) * 0.3));
                }
            }
            // Quays along the edges of built cells that face navigable water.
            if s.rank >= Rank::Town {
                let mut quays = 0;
                for &b in &cells {
                    for k in [0usize, 2, 4, 6] {
                        let Some(j) = g.neighbor(b, k) else { continue };
                        if !(t.is_sea(j) || (t.is_nav(j) && snap.built[j] == NONE)) {
                            continue;
                        }
                        let (cb, cj) = (centre(g.w, b), centre(g.w, j));
                        let mid = cb + (cj - cb) * 0.45;
                        let along = Vec2::new(-(cj - cb).y, (cj - cb).x) * 0.45;
                        out.push(Glyph::Quay(mid - along, mid + along));
                        if s.food_in + s.trade_in >= WAREHOUSE_TRADE && quays < 3 {
                            out.push(Glyph::Warehouse(cb + (cj - cb) * 0.2));
                        }
                        quays += 1;
                    }
                }
            }
            // Walls round a city's built-up area, with towers at the corners.
            if s.rank == Rank::City {
                let inside = |i: usize| snap.built[i] == s.id;
                let mut corners: BTreeMap<(i32, i32), u8> = BTreeMap::new();
                for &b in &cells {
                    let (x, y) = g.xy(b);
                    // (direction, edge start, edge end) in cell-corner coordinates.
                    for (k, p0, p1) in [
                        (0usize, (x + 1, y), (x + 1, y + 1)),
                        (2, (x, y + 1), (x + 1, y + 1)),
                        (4, (x, y), (x, y + 1)),
                        (6, (x, y), (x + 1, y)),
                    ] {
                        let outside = g.neighbor(b, k).is_none_or(|j| !inside(j));
                        if outside {
                            let v0 = Vec2::new(p0.0 as f32, p0.1 as f32);
                            let v1 = Vec2::new(p1.0 as f32, p1.1 as f32);
                            out.push(Glyph::Wall(v0, v1));
                            *corners.entry(p0).or_insert(0) |= if k % 4 == 0 { 1 } else { 2 };
                            *corners.entry(p1).or_insert(0) |= if k % 4 == 0 { 1 } else { 2 };
                        }
                    }
                }
                for ((x, y), dirs) in corners {
                    if dirs == 3 {
                        out.push(Glyph::Tower(Vec2::new(x as f32, y as f32)));
                    }
                }
            }
        }
        // River crossings on roads: a river cell the road enters from one bank and
        // leaves by the other (road neighbours off the river, on opposite sides).
        // Roads running along a river through river cells are not crossings.
        let seg = segments(d);
        let max = seg.values().copied().fold(1.0f32, f32::max);
        let mut banks: BTreeMap<u32, Vec<(Vec2, f32)>> = BTreeMap::new();
        for &(a, b) in &snap.road_edges {
            let flow = seg.get(&(a, b, false)).copied().unwrap_or(0.0);
            for (r, o) in [(a, b), (b, a)] {
                if t.is_river(r as usize) && !t.is_river(o as usize) {
                    let dir = centre(g.w, o as usize) - centre(g.w, r as usize);
                    banks.entry(r).or_default().push((dir.normalized(), flow));
                }
            }
        }
        for (r, sides) in banks {
            let mut best: Option<(f32, f32)> = None;
            for (i, &(da, fa)) in sides.iter().enumerate() {
                for &(db, fb) in &sides[i + 1..] {
                    if da.dot(db) < -0.3 {
                        let flow = fa.min(fb);
                        if best.is_none_or(|bst| flow > bst.0) {
                            let along = db - da;
                            best = Some((flow, along.y.atan2(along.x)));
                        }
                    }
                }
            }
            let Some((flow, angle)) = best else { continue };
            let p = centre(g.w, r as usize);
            if flow >= BRIDGE_SHARE * max {
                out.push(Glyph::Bridge(p, angle));
            } else if t.ford[r as usize] {
                out.push(Glyph::Ford(p, angle));
            }
        }
        self.glyphs = out;
    }
}

fn rot(v: Vec2, a: f32) -> Vec2 {
    let (s, c) = a.sin_cos();
    Vec2::new(v.x * c - v.y * s, v.x * s + v.y * c)
}

impl Layer for BuildingsLayer {
    fn id(&self) -> &'static str {
        "buildings"
    }
    fn name(&self) -> &'static str {
        "Buildings and bridges"
    }
    fn shown_at(&self, zoom: f32) -> bool {
        zoom >= 4.0
    }
    fn legend(&self) -> Vec<(Swatch, &'static str)> {
        vec![
            (Swatch::Fill(STONE), "Church, square, cathedral"),
            (Swatch::Line(WALL, 3.0), "Town walls"),
            (Swatch::Dot(Color32::from_rgb(90, 120, 150)), "Water mill"),
            (Swatch::Line(QUAY, 3.0), "Quay and warehouses"),
            (Swatch::Line(STONE, 4.0), "Bridge"),
        ]
    }
    fn draw(&mut self, d: &DrawCtx) {
        let v = d.view;
        let key = (v.terrain_version, v.snapshot.year);
        if self.key != Some(key) {
            self.rebuild(d);
            self.key = Some(key);
        }
        let z = d.camera.zoom;
        if z < 4.0 {
            return;
        }
        let to = |m: Vec2| d.camera.to_screen(d.rect, m);
        let s = (0.3 * z).clamp(4.0, 22.0);
        let p = d.painter;
        let thin = Stroke::new(1.0, OUTLINE);
        // The relief shader draws the walls along the smoothed footprint.
        let walls = !super::relief::active(v);
        for gl in &self.glyphs {
            match *gl {
                Glyph::Wall(..) | Glyph::Tower(_) if !walls => {}
                Glyph::Wall(a, b) => {
                    p.line_segment(
                        [to(a), to(b)],
                        Stroke::new((0.07 * z).clamp(1.5, 4.0), WALL),
                    );
                }
                Glyph::Tower(c) => {
                    p.circle(to(c), (0.09 * z).clamp(1.5, 5.0), WALL, thin);
                }
                _ if z < 8.0 => {}
                Glyph::Square(c) => {
                    let r = egui::Rect::from_center_size(to(c), egui::vec2(0.36 * z, 0.36 * z));
                    p.rect(
                        r,
                        1.0,
                        Color32::from_rgb(239, 230, 207),
                        thin,
                        egui::StrokeKind::Inside,
                    );
                }
                Glyph::Church(c) => {
                    let o = to(c);
                    let body = egui::Rect::from_center_size(
                        o + egui::vec2(0.0, s * 0.2),
                        egui::vec2(s * 0.6, s * 0.5),
                    );
                    p.rect(body, 0.0, STONE, thin, egui::StrokeKind::Inside);
                    let roof = vec![
                        body.left_top(),
                        body.right_top(),
                        Pos2::new(o.x, o.y - s * 0.35),
                    ];
                    p.add(Shape::convex_polygon(
                        roof,
                        Color32::from_rgb(150, 75, 55),
                        thin,
                    ));
                    p.line_segment(
                        [
                            Pos2::new(o.x, o.y - s * 0.35),
                            Pos2::new(o.x, o.y - s * 0.65),
                        ],
                        thin,
                    );
                    p.line_segment(
                        [
                            Pos2::new(o.x - s * 0.12, o.y - s * 0.53),
                            Pos2::new(o.x + s * 0.12, o.y - s * 0.53),
                        ],
                        thin,
                    );
                }
                Glyph::Cathedral(c) => {
                    let o = to(c);
                    let s = s * 1.4;
                    let nave = egui::Rect::from_center_size(
                        o + egui::vec2(0.0, s * 0.2),
                        egui::vec2(s * 0.9, s * 0.45),
                    );
                    p.rect(nave, 0.0, STONE, thin, egui::StrokeKind::Inside);
                    for dx in [-0.38f32, 0.38] {
                        let tower = egui::Rect::from_center_size(
                            o + egui::vec2(dx * s, -s * 0.05),
                            egui::vec2(s * 0.2, s * 0.65),
                        );
                        p.rect(tower, 0.0, STONE, thin, egui::StrokeKind::Inside);
                        let tip = vec![
                            tower.left_top(),
                            tower.right_top(),
                            Pos2::new(tower.center().x, tower.top() - s * 0.25),
                        ];
                        p.add(Shape::convex_polygon(
                            tip,
                            Color32::from_rgb(120, 110, 100),
                            thin,
                        ));
                    }
                }
                Glyph::Mill(c) => {
                    let o = to(c);
                    let r = s * 0.35;
                    p.circle(o, r, Color32::from_rgb(90, 120, 150), thin);
                    for k in 0..4 {
                        let a = k as f32 * std::f32::consts::FRAC_PI_4 * 2.0 + 0.4;
                        let dv = rot(Vec2::new(r, 0.0), a);
                        p.line_segment(
                            [o - dv, o + dv],
                            Stroke::new(1.0, Color32::from_rgb(230, 235, 240)),
                        );
                    }
                }
                Glyph::Quay(a, b) => {
                    p.line_segment(
                        [to(a), to(b)],
                        Stroke::new((0.08 * z).clamp(2.0, 5.0), QUAY),
                    );
                }
                Glyph::Warehouse(c) => {
                    let r = egui::Rect::from_center_size(to(c), egui::vec2(s * 0.7, s * 0.4));
                    p.rect(
                        r,
                        0.0,
                        Color32::from_rgb(120, 90, 60),
                        thin,
                        egui::StrokeKind::Inside,
                    );
                }
                Glyph::Bridge(c, a) | Glyph::Ford(c, a) => {
                    let o = to(c);
                    let along = rot(Vec2::new(0.45 * z, 0.0), a);
                    let across = rot(Vec2::new(0.0, 0.1 * z), a);
                    if matches!(gl, Glyph::Bridge(..)) {
                        let quad = vec![
                            o - along - across,
                            o + along - across,
                            o + along + across,
                            o - along + across,
                        ];
                        p.add(Shape::convex_polygon(
                            quad,
                            STONE,
                            Stroke::new(1.2, OUTLINE),
                        ));
                    } else {
                        for k in -1..=1 {
                            let m = o + along * (k as f32 * 0.5);
                            p.line_segment(
                                [m - across, m + across],
                                Stroke::new(1.5, Color32::from_rgb(230, 220, 190)),
                            );
                        }
                    }
                }
            }
        }
    }
}
