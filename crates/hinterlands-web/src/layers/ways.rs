//! Roads and rivers at true width (R9): the data the relief shader needs to
//! draw them per pixel from the distance to their centrelines.
//!
//! The centrelines are the same runs the map symbols use (the river paths and
//! the road network), smoothed further so they stay curves at a metre a pixel.
//! Each smoothed piece becomes a segment with a half-width in metres at both
//! ends; each cell lists the segments that can reach into it, so the shader
//! only looks at a few. Widths come from the model: rivers from their drainage,
//! roads from the trade they carry. Nothing here feeds back.

use super::lines;
use super::routes::{self, Class};
use crate::view::ViewState;
use egui::Vec2;
use hinterlands_core::terrain::{Terrain, Water};

/// Texels per row of the segment and index textures (2048 is the least any
/// WebGL2 device allows).
pub const ROW: usize = 2048;
/// Rounds of corner cutting: segments of about 1/16 of a cell, so bends turn
/// a few degrees at a time.
const ROUNDS: usize = 4;
/// Segment kinds, as the shader reads them.
const STREAM: f32 = 0.0;
const RIVER: f32 = 1.0;
const TRACK: f32 = 2.0;
const ROAD: f32 = 3.0;
const PATH: f32 = 4.0;
/// A street a town's plan made (R11).
const STREET: f32 = 5.0;
/// Metres beyond a road's edge that its verges, ditches, hedges and their
/// shadows reach (roads spread by up to half again on mud), and beyond a
/// river's edge for its banks, reeds and quays; plus half the widest pixel at
/// which the shader draws them (15 m).
const ROAD_REACH: f32 = 14.0 + 8.0;
const RIVER_REACH: f32 = 8.0 + 8.0;
/// The segment index is kept on a finer grid than the cells, about this many
/// metres square, so each pixel reads only the segments near it.
const INDEX_M: f32 = 250.0;
/// Width of a land track, and of a road from quiet to busiest, in metres.
const TRACK_W: f32 = 2.2;
/// Width of a footpath, metres.
const PATH_W: f32 = 1.2;
const ROAD_W: (f32, f32) = (3.0, 8.0);
/// Metres kept between a road and the water where it runs beside a river.
const RIVER_GAP: f32 = 8.0;

/// River width in metres from the drainage area: about 0.6 √A (A in km²),
/// so 4–5 m where a brook starts and some 50 m on the largest rivers.
pub fn river_width_m(flux: f32, cell_km: f32) -> f32 {
    0.6 * (flux.max(1.0) * cell_km * cell_km).sqrt()
}

/// How far a river of this half-width swings either side of its line, metres
/// (the shader uses the same rule).
fn meander_m(hw: f32) -> f32 {
    2.0 + 0.8 * hw
}

/// Road width in metres from how busy it is, 0..1.
pub fn road_width_m(busy: f32) -> f32 {
    ROAD_W.0 + (ROAD_W.1 - ROAD_W.0) * busy
}

/// Kilometres to the end of a river line are carried where a road carries how
/// busy it is, below 1.
const TO_END_KM: f32 = 0.999;

/// One smoothed line: points in cells, half-width in metres at each.
struct Line {
    kind: f32,
    /// How busy a road is at each point, 0..1 (0 for rivers and tracks).
    busy: Vec<f32>,
    pts: Vec<Vec2>,
    hw: Vec<f32>,
}

/// The textures' contents, laid out as RGBA floats.
pub struct WayData {
    /// Two texels per segment: (x0, y0, x1, y1) in cells, then (half-width at
    /// each end in metres, 4 x kind + busy, metres along the line at its start).
    /// On rivers, in place of busy: kilometres from the segment's start to the
    /// end of the line, up to 0.999.
    pub seg: Vec<f32>,
    pub seg_size: (i32, i32),
    /// Per square of the index grid: first entry in `idx`, number of entries.
    pub cell: Vec<f32>,
    /// Index squares per cell along each side, and the index grid's size.
    pub sub: f32,
    pub cell_size: (i32, i32),
    /// Segment numbers, four to a texel.
    pub idx: Vec<f32>,
    pub idx_size: (i32, i32),
    /// The most segments any cell lists (the shader looks at up to 96).
    pub most: usize,
}

/// River lines, which change only with the terrain.
fn rivers(t: &Terrain) -> Vec<Line> {
    let g = t.grid;
    super::rivers::paths(t)
        .into_iter()
        .map(|(pts, cells)| {
            let nav = cells.iter().any(|&i| t.water[i] == Water::NavigableRiver);
            // A run that ends where it joins a larger river keeps its own width to
            // the end, rather than taking the larger river's.
            let k = cells.len();
            let mut hw: Vec<f32> = (0..k)
                .map(|j| {
                    let i = if j + 1 == k && k > 1 {
                        cells[j - 1]
                    } else {
                        cells[j]
                    };
                    0.5 * river_width_m(t.flux[i], g.cell_km)
                })
                .collect();
            // Estuaries: over the last 1.5 km to the sea the river opens out.
            let last = *cells.last().unwrap_or(&0);
            if t.downstream(last).is_some_and(|j| t.is_sea(j)) {
                let reach = 1.5 / g.cell_km;
                let mut from_mouth = 0.0;
                for k in (0..pts.len()).rev() {
                    if k + 1 < pts.len() {
                        from_mouth += (pts[k + 1] - pts[k]).length();
                    }
                    let u = (1.0 - from_mouth / reach).max(0.0);
                    hw[k] *= 1.0 + 3.0 * u * u;
                }
            }
            let (pts, hw) = lines::chaikin(&pts, &hw, ROUNDS, |a, b, f| a + (b - a) * f);
            Line {
                kind: if nav { RIVER } else { STREAM },
                busy: vec![0.0; pts.len()],
                pts,
                hw,
            }
        })
        .collect()
}

/// River segments by cell, so roads can be kept off the water: each with the
/// metres a road's centre must keep from its line (water, meander, half the
/// widest road, and a gap).
struct Banks {
    cells: Vec<Vec<(Vec2, Vec2, f32)>>,
}

impl Banks {
    fn new(t: &Terrain, rivers: &[Line]) -> Self {
        let g = t.grid;
        let cell_m = g.cell_km * 1000.0;
        let mut cells = vec![Vec::new(); t.n()];
        for l in rivers {
            for k in 1..l.pts.len() {
                let (a, b) = (l.pts[k - 1], l.pts[k]);
                let hw = l.hw[k - 1].max(l.hw[k]);
                let clear = hw + meander_m(hw) + 0.5 * ROAD_W.1 + RIVER_GAP;
                let m = clear / cell_m;
                let lo = (a.min(b) - Vec2::splat(m)).floor();
                let hi = (a.max(b) + Vec2::splat(m)).floor();
                for y in (lo.y as i32).max(0)..=(hi.y as i32).min(g.h as i32 - 1) {
                    for x in (lo.x as i32).max(0)..=(hi.x as i32).min(g.w as i32 - 1) {
                        cells[g.idx(x, y)].push((a, b, clear));
                    }
                }
            }
        }
        Self { cells }
    }

    /// Moves a road point that lies within a river's clearance out to it, on
    /// the side it is already on (or the higher side if it lies on the line).
    /// Points where the road crosses the river (its direction `dir` is not
    /// along the river's) stay, so crossings remain. A line's end points move
    /// by position alone, so every road meeting there moves alike.
    fn push(&self, t: &Terrain, p: Vec2, dir: Option<Vec2>) -> Vec2 {
        let g = t.grid;
        let cell_m = g.cell_km * 1000.0;
        let (x, y) = (p.x.floor() as i32, p.y.floor() as i32);
        if !g.in_bounds(x, y) {
            return p;
        }
        let mut best: Option<(f32, Vec2, Vec2, f32)> = None;
        for &(a, b, clear) in &self.cells[g.idx(x, y)] {
            let ab = b - a;
            let u = ((p - a).dot(ab) / ab.dot(ab).max(1e-12)).clamp(0.0, 1.0);
            let q = a + ab * u;
            let d = (p - q).length() * cell_m;
            if d < clear && best.is_none_or(|bst| d - clear < bst.0) {
                best = Some((d - clear, q, ab.normalized(), clear));
            }
        }
        let Some((_, q, along, clear)) = best else {
            return p;
        };
        if dir.is_some_and(|d| d.dot(along).abs() < 0.75) {
            return p;
        }
        let n = Vec2::new(-along.y, along.x);
        let off = (p - q).dot(n);
        let side = if off.abs() > 1e-4 {
            off.signum()
        } else {
            let e = |m: Vec2| elevation_at(t, m);
            if e(q + n * 0.6) >= e(q - n * 0.6) {
                1.0
            } else {
                -1.0
            }
        };
        q + n * side * clear / cell_m
    }
}

/// What the town plan asks of the water: rivers from their lines, the sea from
/// the coast's distance field (cells, land positive).
struct WaterAt<'a> {
    t: &'a Terrain,
    banks: &'a Banks,
    coast: &'a [f32],
}

impl super::town::Water for WaterAt<'_> {
    fn river_gap(&self, p_m: Vec2) -> f32 {
        let g = self.t.grid;
        let cell_m = g.cell_km * 1000.0;
        let p = p_m / cell_m;
        let (x, y) = (p.x.floor() as i32, p.y.floor() as i32);
        if !g.in_bounds(x, y) {
            return 1e9;
        }
        let mut best = 1e9f32;
        for &(a, b, clear) in &self.banks.cells[g.idx(x, y)] {
            let ab = b - a;
            let u = ((p - a).dot(ab) / ab.dot(ab).max(1e-12)).clamp(0.0, 1.0);
            let d = (p - (a + ab * u)).length() * cell_m;
            // The water's edge: in a town the river is embanked and does not
            // meander (the shader stills it where the ground is built over).
            let meander_hw = clear - 0.5 * ROAD_W.1 - RIVER_GAP;
            let hw = (meander_hw - 2.0) / 1.8;
            best = best.min(d - hw);
        }
        best
    }
    fn land(&self, p_m: Vec2) -> bool {
        let g = self.t.grid;
        let q = p_m / (g.cell_km * 1000.0) - Vec2::splat(0.5);
        let (x0, y0) = (q.x.floor() as i32, q.y.floor() as i32);
        let (fx, fy) = (q.x - x0 as f32, q.y - y0 as f32);
        let c = |x: i32, y: i32| {
            self.coast[g.idx(x.clamp(0, g.w as i32 - 1), y.clamp(0, g.h as i32 - 1))]
        };
        let top = c(x0, y0) + (c(x0 + 1, y0) - c(x0, y0)) * fx;
        let bottom = c(x0, y0 + 1) + (c(x0 + 1, y0 + 1) - c(x0, y0 + 1)) * fx;
        top + (bottom - top) * fy > 0.08
    }
}

/// Elevation at a map point (cells), interpolated between cell centres.
fn elevation_at(t: &Terrain, m: Vec2) -> f32 {
    let g = t.grid;
    let q = m - Vec2::splat(0.5);
    let (x0, y0) = (q.x.floor() as i32, q.y.floor() as i32);
    let (fx, fy) = (q.x - x0 as f32, q.y - y0 as f32);
    let e =
        |x: i32, y: i32| t.elevation[g.idx(x.clamp(0, g.w as i32 - 1), y.clamp(0, g.h as i32 - 1))];
    let top = e(x0, y0) + (e(x0 + 1, y0) - e(x0, y0)) * fx;
    let bottom = e(x0, y0 + 1) + (e(x0 + 1, y0 + 1) - e(x0, y0 + 1)) * fx;
    top + (bottom - top) * fy
}

/// Road and track lines for this year.
fn roads(v: &ViewState, banks: &Banks) -> Vec<Line> {
    let t = &v.terrain;
    let net = routes::network(v);
    let max = net.max;
    let mut out = Vec::new();
    for (class, pts, flow) in routes::runs_of(v, &net) {
        let kind = match class {
            Class::Road => ROAD,
            Class::Track => TRACK,
            Class::Water => continue,
        };
        let busy: Vec<f32> = flow
            .iter()
            .map(|&f| {
                if kind == ROAD {
                    routes::busy(f, max)
                } else {
                    0.0
                }
            })
            .collect();
        let (pts, busy) = lines::chaikin(&pts, &busy, ROUNDS, |a, b, f| a + (b - a) * f);
        // Keep off the rivers, then one more round to ease the bends that makes.
        let last = pts.len() - 1;
        let pts: Vec<Vec2> = (0..=last)
            .map(|k| {
                let dir = (k > 0 && k < last).then(|| (pts[k + 1] - pts[k - 1]).normalized());
                banks.push(t, pts[k], dir)
            })
            .collect();
        let (pts, busy) = lines::chaikin(&pts, &busy, 1, |a, b, f| a + (b - a) * f);
        let hw = busy
            .iter()
            .map(|&b| {
                0.5 * if kind == ROAD {
                    road_width_m(b)
                } else {
                    TRACK_W
                }
            })
            .collect();
        out.push(Line {
            kind,
            busy,
            pts,
            hw,
        });
    }
    out
}

/// Footpaths from each place to its fields and its nearest neighbours (the
/// footpaths layer's walk), as lines like the roads, kept off the rivers.
fn footpaths(v: &ViewState, banks: &Banks, edges: &[(u32, u32)]) -> Vec<Line> {
    let t = &v.terrain;
    let g = t.grid;
    let edges: std::collections::BTreeSet<(u32, u32)> = edges.iter().copied().collect();
    let pins: std::collections::BTreeSet<u32> =
        v.snapshot.settlements.iter().map(|s| s.anchor).collect();
    lines::chains(&edges, &pins)
        .into_iter()
        .map(|cells| {
            let pts: Vec<Vec2> = cells
                .iter()
                .map(|&c| {
                    let (x, y) = g.xy(c as usize);
                    Vec2::new(x as f32 + 0.5, y as f32 + 0.5)
                })
                .collect();
            let zero = vec![0.0; pts.len()];
            let (pts, _) = lines::chaikin(&pts, &zero, ROUNDS, |a, _, _| a);
            let last = pts.len() - 1;
            let pts: Vec<Vec2> = (0..=last)
                .map(|k| {
                    let dir = (k > 0 && k < last).then(|| (pts[k + 1] - pts[k - 1]).normalized());
                    banks.push(t, pts[k], dir)
                })
                .collect();
            let n = pts.len();
            Line {
                kind: PATH,
                busy: vec![0.0; n],
                hw: vec![0.5 * PATH_W; n],
                pts,
            }
        })
        .collect()
}

/// Builds the textures from the river lines (cached per terrain by the caller)
/// and this year's roads.
fn layout(t: &Terrain, lines: &[&Line]) -> WayData {
    let g = t.grid;
    let cell_m = g.cell_km * 1000.0;
    let sub = (cell_m / INDEX_M).round().max(1.0);
    let (w, h) = ((g.w as f32 * sub) as i32, (g.h as f32 * sub) as i32);
    let mut seg = Vec::new();
    let mut lists: Vec<Vec<u32>> = vec![Vec::new(); (w * h) as usize];
    for l in lines {
        let reach = if l.kind >= TRACK {
            ROAD_REACH
        } else {
            RIVER_REACH
        };
        let mut along = 0.0;
        let total: f32 = l
            .pts
            .windows(2)
            .map(|w| (w[1] - w[0]).length())
            .sum::<f32>()
            * cell_m;
        for k in 1..l.pts.len() {
            let (a, b) = (l.pts[k - 1], l.pts[k]);
            let id = (seg.len() / 8) as u32;
            let hw = l.hw[k - 1].max(l.hw[k]);
            // Rivers swing in meanders and widen by half in spring.
            let margin = if l.kind >= TRACK {
                hw + reach
            } else {
                1.5 * hw + meander_m(hw) + reach
            } / cell_m;
            let (x0, x1) = (
                (((a.x.min(b.x) - margin) * sub).floor() as i32).max(0),
                (((a.x.max(b.x) + margin) * sub).floor() as i32).min(w - 1),
            );
            let (y0, y1) = (
                (((a.y.min(b.y) - margin) * sub).floor() as i32).max(0),
                (((a.y.max(b.y) + margin) * sub).floor() as i32).min(h - 1),
            );
            for y in y0..=y1 {
                for x in x0..=x1 {
                    lists[(y * w + x) as usize].push(id);
                }
            }
            seg.extend_from_slice(&[a.x, a.y, b.x, b.y]);
            let extra = if l.kind >= TRACK {
                0.5 * (l.busy[k - 1] + l.busy[k])
            } else {
                ((total - along) / 1000.0).min(TO_END_KM)
            };
            seg.extend_from_slice(&[l.hw[k - 1], l.hw[k], 4.0 * l.kind + extra, along]);
            along += (b - a).length() * cell_m;
        }
    }
    let mut cell = Vec::with_capacity(lists.len() * 4);
    let mut idx: Vec<f32> = Vec::new();
    let mut most = 0;
    for list in &lists {
        cell.extend_from_slice(&[idx.len() as f32, list.len() as f32, 0.0, 0.0]);
        idx.extend(list.iter().map(|&i| i as f32));
        most = most.max(list.len());
    }
    let pad = |v: &mut Vec<f32>| -> (i32, i32) {
        let rows = v.len().div_ceil(4).div_ceil(ROW).max(1);
        v.resize(rows * ROW * 4, 0.0);
        (ROW as i32, rows as i32)
    };
    let seg_size = pad(&mut seg);
    let idx_size = pad(&mut idx);
    WayData {
        seg,
        seg_size,
        cell,
        sub,
        cell_size: (w, h),
        idx,
        idx_size,
        most,
    }
}

/// The river lines for the current terrain, and the data built last.
#[derive(Default)]
pub struct Ways {
    rivers: Vec<Line>,
    banks: Option<Banks>,
    /// Signed distance to the coast per cell (land positive), for the towns.
    coast: Vec<f32>,
    terrain_key: Option<u64>,
    /// The footpaths' walk, shared with the footpaths layer's rule.
    walk: super::parish::Local,
}

impl Ways {
    /// This year's roads, rivers, footpaths and town plans, as textures.
    pub fn build(&mut self, v: &ViewState) -> (WayData, super::town::ShapeData, super::town::Plan) {
        if self.terrain_key != Some(v.terrain_version) {
            self.rivers = rivers(&v.terrain);
            self.banks = Some(Banks::new(&v.terrain, &self.rivers));
            self.coast = super::relief::land_data(&v.terrain)
                .chunks(4)
                .map(|c| c[1])
                .collect();
            self.terrain_key = Some(v.terrain_version);
        }
        let mut roads = match &self.banks {
            Some(banks) => roads(v, banks),
            None => Vec::new(),
        };
        if let Some(banks) = &self.banks {
            let edges = super::parish::footpaths(v, &mut self.walk).to_vec();
            roads.extend(footpaths(v, banks, &edges));
        }
        // The towns, laid out along the roads and tracks (in metres).
        let cell_m = v.terrain.grid.cell_km * 1000.0;
        let main: Vec<(Vec<Vec2>, f32, bool)> = roads
            .iter()
            .filter(|l| l.kind == ROAD || l.kind == TRACK)
            .map(|l| {
                let hw = l.hw.iter().sum::<f32>() / l.hw.len().max(1) as f32;
                (
                    l.pts.iter().map(|&p| p * cell_m).collect(),
                    hw,
                    l.kind == ROAD,
                )
            })
            .collect();
        let plan = match &self.banks {
            Some(banks) => {
                let water = WaterAt {
                    t: &v.terrain,
                    banks,
                    coast: &self.coast,
                };
                super::town::plan(v, &main, &water)
            }
            None => super::town::Plan::default(),
        };
        for st in &plan.streets {
            let n = st.pts.len();
            roads.push(Line {
                kind: STREET,
                busy: vec![0.0; n],
                hw: vec![st.hw; n],
                pts: st.pts.iter().map(|&p| p / cell_m).collect(),
            });
        }
        let all: Vec<&Line> = self.rivers.iter().chain(roads.iter()).collect();
        let shapes = super::town::layout(&v.terrain, &plan.shapes, &plan.places);
        (layout(&v.terrain, &all), shapes, plan)
    }
}
