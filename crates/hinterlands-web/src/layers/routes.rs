//! Roads and trade routes. Segments are joined into chains between junctions,
//! ends and settlement centres, and each chain is smoothed so routes do not
//! zig-zag on the grid. Roads are drawn cased (a dark edge round a lighter
//! fill) once they are wide enough, land trade off the roads as a dashed track,
//! water routes in pale blue; widths follow the yearly trade flow. Land trade
//! off the roads, and minor water routes, are drawn faintly or not at all, so
//! the main corridors read.

use super::{lines, DrawCtx, Layer, Swatch};
use crate::view::ViewState;
use egui::{Color32, Pos2, Rect, Shape, Stroke, Vec2};
use std::collections::{BTreeMap, BTreeSet};

/// Segments below this share of the largest flow are not drawn unless they are roads.
const MIN_SHARE: f32 = 0.02;
const ROAD: Color32 = Color32::from_rgb(122, 80, 42);
const ROAD_EDGE: Color32 = Color32::from_rgb(74, 46, 24);
const ROAD_FILL: Color32 = Color32::from_rgb(214, 176, 112);
const TRACK: Color32 = Color32::from_rgba_premultiplied(48, 31, 15, 110);
const WATER: Color32 = Color32::from_rgba_premultiplied(144, 188, 210, 210);
/// Roads at least this wide on screen (pixels) are drawn cased.
const CASED: f32 = 2.5;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Class {
    Water,
    Track,
    Road,
}

/// A run of route between junctions, smoothed, with the flow at each point.
struct Chain {
    class: Class,
    pts: Vec<Vec2>,
    flow: Vec<f32>,
    bounds: Rect,
}

/// The routes drawn this year, and the cells where lines keep their corner:
/// junctions, ends, changes of kind, and settlement centres.
pub struct Network {
    edges: BTreeMap<(u32, u32, Class), f32>,
    pub pins: BTreeSet<u32>,
    max: f32,
}

#[derive(Default)]
pub struct RoutesLayer {
    chains: Vec<Chain>,
    max: f32,
    key: Option<(u64, u32)>,
}

/// (cell a, cell b, on water) -> yearly flow over the segment, with a < b.
pub fn segments(d: &DrawCtx) -> BTreeMap<(u32, u32, bool), f32> {
    segments_of(d.view)
}

fn segments_of(v: &ViewState) -> BTreeMap<(u32, u32, bool), f32> {
    let n = v.terrain.n() as u32;
    let mut acc: BTreeMap<(u32, u32, bool), f32> = BTreeMap::new();
    let lane = |c: u32| v.sea_lane.get(c as usize).copied().unwrap_or(c);
    for f in &v.snapshot.flows {
        for w in f.path.windows(2) {
            // Sea paths along a coast are drawn on one lane (see `sea_lane`).
            let (a, b) = (lane(w[0] % n), lane(w[1] % n));
            if a == b {
                continue;
            }
            let water = w[0] >= n && w[1] >= n;
            *acc.entry((a.min(b), a.max(b), water)).or_insert(0.0) += f.amount;
        }
    }
    acc
}

pub fn network(v: &ViewState) -> Network {
    let seg = segments_of(v);
    let max = seg.values().copied().fold(1.0f32, f32::max);
    let roads: BTreeSet<(u32, u32)> = v
        .snapshot
        .road_edges
        .iter()
        .map(|&(a, b)| (a.min(b), a.max(b)))
        .collect();
    let mut edges = BTreeMap::new();
    for (&(a, b, water), &flow) in &seg {
        if flow < MIN_SHARE * max {
            continue;
        }
        if water {
            edges.insert((a, b, Class::Water), flow);
        } else if !roads.contains(&(a, b)) {
            edges.insert((a, b, Class::Track), flow);
        }
    }
    for &(a, b) in &roads {
        let flow = seg.get(&(a, b, false)).copied().unwrap_or(0.0);
        edges.insert((a, b, Class::Road), flow);
    }
    let mut neighbours: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();
    let mut degree: BTreeMap<(u32, Class), u32> = BTreeMap::new();
    for &(a, b, c) in edges.keys() {
        neighbours.entry(a).or_default().insert(b);
        neighbours.entry(b).or_default().insert(a);
        *degree.entry((a, c)).or_insert(0) += 1;
        *degree.entry((b, c)).or_insert(0) += 1;
    }
    let mut pins: BTreeSet<u32> = v.snapshot.settlements.iter().map(|s| s.anchor).collect();
    pins.extend(
        neighbours
            .iter()
            .filter(|(_, s)| s.len() != 2)
            .map(|(&n, _)| n),
    );
    pins.extend(degree.iter().filter(|(_, &k)| k != 2).map(|(&(n, _), _)| n));
    Network { edges, pins, max }
}

/// Map position of a cell's centre, in cells.
fn centre(v: &ViewState, cell: u32) -> Vec2 {
    let (x, y) = v.terrain.grid.xy(cell as usize);
    Vec2::new(x as f32 + 0.5, y as f32 + 0.5)
}

/// Splits the network into chains that run from pin to pin, then smooths each.
fn chains(v: &ViewState, net: &Network) -> Vec<Chain> {
    let key = |a: u32, b: u32, c: Class| (a.min(b), a.max(b), c);
    let mut out = Vec::new();
    for class in [Class::Water, Class::Track, Class::Road] {
        let edges: BTreeSet<(u32, u32)> = net
            .edges
            .keys()
            .filter(|e| e.2 == class)
            .map(|&(a, b, _)| (a, b))
            .collect();
        for cells in lines::chains(&edges, &net.pins) {
            let flows: Vec<f32> = cells
                .windows(2)
                .map(|w| net.edges[&key(w[0], w[1], class)])
                .collect();
            let m = flows.len();
            let at: Vec<f32> = (0..=m)
                .map(|j| match j {
                    0 => flows[0],
                    j if j == m => flows[m - 1],
                    j => 0.5 * (flows[j - 1] + flows[j]),
                })
                .collect();
            let pts: Vec<Vec2> = cells.iter().map(|&c| centre(v, c)).collect();
            let (pts, flow) = lines::chaikin(&pts, &at, 2, |a, b, f| a + (b - a) * f);
            let mut bounds = Rect::NOTHING;
            for p in &pts {
                bounds.extend_with(p.to_pos2());
            }
            out.push(Chain {
                class,
                pts,
                flow,
                bounds: bounds.expand(0.5),
            });
        }
    }
    out
}

/// Draws a polyline whose width and colour vary along it, as runs of one style.
/// Transparent stretches are skipped; `round` caps the joints of wide runs.
fn runs(d: &DrawCtx, pts: &[Pos2], style: impl Fn(usize) -> (f32, Color32), round: bool) {
    let mut run: Vec<Pos2> = Vec::new();
    let mut cur: Option<(u32, Color32)> = None;
    let flush = |run: &mut Vec<Pos2>, cur: Option<(u32, Color32)>| {
        if let Some((half_px, colour)) = cur {
            let w = half_px as f32 / 2.0;
            if run.len() >= 2 && colour.a() > 0 {
                if round && w > CASED && colour.a() == 255 {
                    for p in [run[0], run[run.len() - 1]] {
                        d.painter.circle_filled(p, w / 2.0, colour);
                    }
                }
                d.painter
                    .add(Shape::line(std::mem::take(run), Stroke::new(w, colour)));
            }
        }
        run.clear();
    };
    for (k, &p) in pts.iter().enumerate() {
        let (w, colour) = style(k);
        let s = ((w * 2.0).round() as u32, colour);
        if cur.is_some_and(|old| old != s) {
            run.push(p);
            flush(&mut run, cur);
        }
        cur = Some(s);
        run.push(p);
    }
    flush(&mut run, cur);
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
            (Swatch::Line(ROAD, 3.0), "Road (width: trade)"),
            (Swatch::Line(TRACK, 2.0), "Track: land trade off-road"),
            (Swatch::Line(WATER, 3.0), "River and sea trade"),
        ]
    }
    fn draw(&mut self, d: &DrawCtx) {
        let v = d.view;
        let key = (v.terrain_version, v.snapshot.year);
        if self.key != Some(key) {
            let net = network(v);
            self.chains = chains(v, &net);
            self.max = net.max;
            self.key = Some(key);
        }
        let z = d.camera.zoom;
        let max = self.max;
        let width = |flow: f32| (flow / max).sqrt() * 0.5 * z;
        let view = Rect::from_min_max(
            d.camera.to_map(d.rect, d.rect.min).to_pos2(),
            d.camera.to_map(d.rect, d.rect.max).to_pos2(),
        );
        let on_screen = |c: &&Chain| c.bounds.intersects(view);
        let screen = |c: &Chain| -> Vec<Pos2> {
            c.pts
                .iter()
                .map(|&p| d.camera.to_screen(d.rect, p))
                .collect()
        };
        let of = |class: Class| self.chains.iter().filter(move |c| c.class == class);

        // Water routes first, under the roads.
        for c in of(Class::Water).filter(on_screen) {
            runs(d, &screen(c), |k| (width(c.flow[k]).max(1.0), WATER), false);
        }
        // Land trade that does not follow a road: a dashed track.
        let dash = (0.25 * z).clamp(3.0, 10.0);
        for c in of(Class::Track).filter(on_screen) {
            let mean = c.flow.iter().sum::<f32>() / c.flow.len() as f32;
            let stroke = Stroke::new(width(mean).max(0.8), TRACK);
            for s in Shape::dashed_line(&screen(c), stroke, dash, dash * 0.6) {
                d.painter.add(s);
            }
        }
        // Roads, thickened by the trade they carry; roads that now carry little fade back.
        let base = (0.12 * z).clamp(1.0, 3.0);
        let road = |flow: f32| {
            let busy = (flow / (MIN_SHARE * 5.0 * max)).clamp(0.0, 1.0);
            let w = (base * (0.6 + 0.4 * busy)).max(width(flow));
            (w, (90.0 + 165.0 * busy) as u8)
        };
        let roads: Vec<(&Chain, Vec<Pos2>)> = of(Class::Road)
            .filter(on_screen)
            .map(|c| (c, screen(c)))
            .collect();
        // All edges first, then all fills, so junctions join cleanly.
        for (c, pts) in &roads {
            runs(
                d,
                pts,
                |k| {
                    let (w, a) = road(c.flow[k]);
                    if w >= CASED {
                        (w + 2.0, ROAD_EDGE.gamma_multiply(a as f32 / 255.0))
                    } else {
                        (w, ROAD.gamma_multiply(a as f32 / 255.0))
                    }
                },
                true,
            );
        }
        for (c, pts) in &roads {
            runs(
                d,
                pts,
                |k| {
                    let (w, a) = road(c.flow[k]);
                    if w >= CASED {
                        (w, ROAD_FILL.gamma_multiply(a as f32 / 255.0))
                    } else {
                        (w, Color32::TRANSPARENT)
                    }
                },
                true,
            );
        }
    }
}
