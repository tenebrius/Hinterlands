//! Towns and villages at real scale (R11): each place laid out the way it grew,
//! from its roads. The roads through a place are its main streets; a town adds
//! side streets and back lanes; a market place opens where the main street
//! passes the centre, a green where a village's roads meet. Plots run back from
//! the streets, one to a household, laid out from the centre outward, narrower
//! near the middle; each has its house at the street front and, often, a barn
//! or back range behind. Churches stand one to about a thousand people; a city
//! has walls round its core with gates where the roads pass, and suburbs along
//! the roads beyond. Sizes are in metres. Drawing only: nothing feeds back.

use super::countryside::hash;
use crate::look::Look;
use crate::view::ViewState;
use egui::Vec2;
use hinterlands_core::state::Rank;
use hinterlands_core::terrain::Terrain;
use std::collections::BTreeMap;

/// Kinds of shape, as the shader reads them. Below `BUILDING` they are ground
/// (drawn under the streets); from it they are buildings (drawn over).
pub const PLOT: f32 = 0.0;
pub const MARKET: f32 = 1.0;
pub const GREEN: f32 = 2.0;
pub const CHURCHYARD: f32 = 3.0;
pub const PRECINCT: f32 = 4.0;
pub const DITCH: f32 = 5.0;
/// A plot built over as a court: tenements at the front, a row of cottages
/// down one side, an alley beside them.
pub const COURT: f32 = 6.0;
pub const BUILDING: f32 = 10.0;
pub const HOUSE: f32 = 10.0;
pub const BARN: f32 = 11.0;
pub const CHURCH: f32 = 12.0;
pub const TOWER: f32 = 13.0;
pub const WALL: f32 = 14.0;
pub const WALL_TOWER: f32 = 15.0;
pub const GATE: f32 = 16.0;
pub const HALL: f32 = 17.0;
pub const CLOISTER: f32 = 18.0;
/// A house of four free corners fitted to the gap it fills (R16).
pub const QHOUSE: f32 = 19.0;

/// A rectangle (or, for greens and wall towers, an ellipse) in metres.
#[derive(Clone, Copy, Debug)]
pub struct Shape {
    pub c: Vec2,
    /// Half its length (along `angle`) and half its width.
    pub h: Vec2,
    pub angle: f32,
    pub kind: f32,
    /// 0..1, for small differences of colour and texture.
    pub tone: f32,
    /// Plots: the place's rank (0 hamlet .. 3 city). Houses and barns: roof,
    /// 0 thatch, 1 tile, 2 stone slate. Others: unused.
    pub extra: f32,
    /// Metres high, for shadows.
    pub height: f32,
    /// A building of four free corners (R16, kind `QHOUSE`), in metres; `c`,
    /// `h` and `angle` then give its middle and a box round it.
    pub q: Option<[Vec2; 4]>,
}

/// A street the planner made (side streets, back lanes, lanes of hamlets off
/// the roads): points in metres, half-width in metres.
pub struct Street {
    pub pts: Vec<Vec2>,
    pub hw: f32,
}

/// One place's plan, for the record and for other layers.
pub struct Place {
    pub id: u32,
    pub households: u32,
    pub houses: u32,
    /// Metres from the centre to the farthest plot.
    pub reach: f32,
    pub walled: bool,
    /// The place's boundary: metres from the centre in each of `BOUND`
    /// directions (from east, turning as the map's y runs). Fields stop here;
    /// within it the land is the place's own closes, gardens and orchards.
    pub bound: [f32; BOUND],
}

impl Plan {
    /// One line per place of a thousand or more: households, houses placed,
    /// and the settled area inside the boundary against the expected area
    /// for its size (`expected_ha`), for the record of R11.
    pub fn summary(&self, pops: &BTreeMap<u32, f32>) -> Vec<String> {
        let mut out = Vec::new();
        for p in &self.places {
            let pop = pops.get(&p.id).copied().unwrap_or(0.0);
            if pop < 1000.0 {
                continue;
            }
            let step = std::f32::consts::TAU / BOUND as f32;
            let area: f32 = p.bound.iter().map(|r| 0.5 * r * r * step).sum::<f32>() / 1e4;
            out.push(format!(
                "place {} pop {:.0} households {} houses {} reach {:.0} m walled {} area {:.0} ha expected {:.0} ha",
                p.id,
                pop,
                p.households,
                p.houses,
                p.reach,
                p.walled,
                area,
                expected_ha(pop)
            ));
        }
        out
    }
}

/// Directions of a place's boundary.
pub const BOUND: usize = 64;

/// The boundary round a place's ground (plots, yards, greens, market): the
/// farthest corner in each direction plus a margin for its back hedges and
/// lanes, with gaps between neighbouring directions bridged so it does not
/// reach out in single spikes.
fn boundary(centre: Vec2, shapes: &[Shape], z: f32) -> [f32; BOUND] {
    let mut r = [0.0f32; BOUND];
    for s in shapes
        .iter()
        .filter(|s| s.kind < BUILDING && s.kind != DITCH)
    {
        for c in s.corners().into_iter().chain(std::iter::once(s.c)) {
            let d = c - centre;
            let a = d.y.atan2(d.x) / std::f32::consts::TAU;
            let k = ((a + 1.0) * BOUND as f32) as usize % BOUND;
            r[k] = r[k].max(d.length() + 12.0 * z);
        }
    }
    let base = 30.0 * z;
    let mut out = [0.0f32; BOUND];
    for k in 0..BOUND {
        let (a, b) = (r[(k + BOUND - 1) % BOUND], r[(k + 1) % BOUND]);
        out[k] = r[k].max(0.7 * a.min(b)).max(base);
    }
    out
}

#[derive(Default)]
pub struct Plan {
    pub shapes: Vec<Shape>,
    pub streets: Vec<Street>,
    pub places: Vec<Place>,
}

/// What the planner needs to know about water: metres a point keeps from any
/// river's water (negative inside), and whether a point is land.
pub trait Water {
    fn river_gap(&self, p_m: Vec2) -> f32;
    fn land(&self, p_m: Vec2) -> bool;
}

fn h01(i: usize, k: u32) -> f32 {
    (hash(i, k) & 0xffffff) as f32 / 16777216.0
}

fn perp(v: Vec2) -> Vec2 {
    Vec2::new(-v.y, v.x)
}

fn dir(a: f32) -> Vec2 {
    Vec2::new(a.cos(), a.sin())
}

fn smoothstep(a: f32, b: f32, x: f32) -> f32 {
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Distance from p to the segment ab.
fn seg_dist(p: Vec2, a: Vec2, b: Vec2) -> f32 {
    let ab = b - a;
    let t = ((p - a).dot(ab) / ab.dot(ab).max(1e-9)).clamp(0.0, 1.0);
    (p - (a + ab * t)).length()
}

impl Shape {
    fn axes(&self) -> (Vec2, Vec2) {
        let u = dir(self.angle);
        (u, perp(u))
    }
    fn corners(&self) -> [Vec2; 4] {
        if let Some(q) = self.q {
            return q;
        }
        let (u, w) = self.axes();
        let (a, b) = (u * self.h.x, w * self.h.y);
        [
            self.c + a + b,
            self.c + a - b,
            self.c - a - b,
            self.c - a + b,
        ]
    }
    fn overlaps(&self, o: &Shape) -> bool {
        let (ca, cb) = (self.corners(), o.corners());
        // Separating axes: the edge normals of both (convex) outlines.
        let normals = |c: &[Vec2; 4]| {
            let mut n = [Vec2::ZERO; 4];
            for k in 0..4 {
                n[k] = perp(c[(k + 1) % 4] - c[k]);
            }
            n
        };
        for ax in normals(&ca).into_iter().chain(normals(&cb)) {
            let pa = ca.iter().map(|p| p.dot(ax));
            let pb = cb.iter().map(|p| p.dot(ax));
            let (amin, amax) = pa.fold((f32::MAX, f32::MIN), |m, x| (m.0.min(x), m.1.max(x)));
            let (bmin, bmax) = pb.fold((f32::MAX, f32::MIN), |m, x| (m.0.min(x), m.1.max(x)));
            if amax <= bmin || bmax <= amin {
                return false;
            }
        }
        true
    }
    /// Signed distance from p to the outline (negative inside), as the
    /// largest distance outside any edge's line: exact inside, near enough
    /// outside for a few metres' clearance.
    fn sd(&self, p: Vec2) -> f32 {
        let c = self.corners();
        let mid = (c[0] + c[1] + c[2] + c[3]) * 0.25;
        let mut d = f32::MIN;
        for k in 0..4 {
            let (a, b) = (c[k], c[(k + 1) % 4]);
            let mut n = perp(b - a).normalized();
            if n.dot(a - mid) < 0.0 {
                n = -n;
            }
            d = d.max((p - a).dot(n));
        }
        d
    }
    fn bounds(&self) -> (Vec2, Vec2) {
        let c = self.corners();
        let mut lo = c[0];
        let mut hi = c[0];
        for p in &c[1..] {
            lo = lo.min(*p);
            hi = hi.max(*p);
        }
        (lo, hi)
    }
    /// Points to test against streets and water: corners, mid-edges, centre.
    fn probes(&self) -> [Vec2; 9] {
        let c = self.corners();
        [
            c[0],
            c[1],
            c[2],
            c[3],
            (c[0] + c[1]) * 0.5,
            (c[1] + c[2]) * 0.5,
            (c[2] + c[3]) * 0.5,
            (c[3] + c[0]) * 0.5,
            self.c,
        ]
    }
}

/// A polyline in metres with its arc lengths, for walking along it.
struct Poly {
    pts: Vec<Vec2>,
    acc: Vec<f32>,
    hw: f32,
    /// A road (main street) rather than one the planner made.
    main: bool,
}

impl Poly {
    fn new(pts: Vec<Vec2>, hw: f32, main: bool) -> Self {
        let mut acc = vec![0.0];
        for k in 1..pts.len() {
            acc.push(acc[k - 1] + (pts[k] - pts[k - 1]).length());
        }
        Self { pts, acc, hw, main }
    }
    fn len(&self) -> f32 {
        *self.acc.last().unwrap_or(&0.0)
    }
    /// Point and unit tangent at arc length s.
    fn at(&self, s: f32) -> (Vec2, Vec2) {
        let s = s.clamp(0.0, self.len());
        let k = self
            .acc
            .partition_point(|&a| a < s)
            .clamp(1, self.pts.len() - 1);
        let (a, b) = (self.pts[k - 1], self.pts[k]);
        let seg = (self.acc[k] - self.acc[k - 1]).max(1e-6);
        let f = ((s - self.acc[k - 1]) / seg).clamp(0.0, 1.0);
        (a + (b - a) * f, (b - a).normalized())
    }
    /// Arc length of the point nearest p.
    fn nearest(&self, p: Vec2) -> (f32, f32) {
        let mut best = (f32::MAX, 0.0);
        for k in 1..self.pts.len() {
            let (a, b) = (self.pts[k - 1], self.pts[k]);
            let ab = b - a;
            let t = ((p - a).dot(ab) / ab.dot(ab).max(1e-9)).clamp(0.0, 1.0);
            let d = (p - (a + ab * t)).length();
            if d < best.0 {
                best = (d, self.acc[k - 1] + t * ab.length());
            }
        }
        best
    }
}

/// Buckets of 50 m, for finding shapes and street segments near a point.
const BUCKET: f32 = 50.0;

fn buckets(lo: Vec2, hi: Vec2) -> impl Iterator<Item = (i32, i32)> {
    let (x0, y0) = (
        (lo.x / BUCKET).floor() as i32,
        (lo.y / BUCKET).floor() as i32,
    );
    let (x1, y1) = (
        (hi.x / BUCKET).floor() as i32,
        (hi.y / BUCKET).floor() as i32,
    );
    (y0..=y1).flat_map(move |y| (x0..=x1).map(move |x| (x, y)))
}

/// Everything placed so far, for collision tests.
#[derive(Default)]
struct Space {
    shapes: Vec<Shape>,
    shape_at: BTreeMap<(i32, i32), Vec<u32>>,
    /// Street segments: ends and half-width.
    segs: Vec<(Vec2, Vec2, f32)>,
    seg_at: BTreeMap<(i32, i32), Vec<u32>>,
}

impl Space {
    fn add_shape(&mut self, s: Shape) {
        let id = self.shapes.len() as u32;
        let (lo, hi) = s.bounds();
        for b in buckets(lo, hi) {
            self.shape_at.entry(b).or_default().push(id);
        }
        self.shapes.push(s);
    }
    fn add_street(&mut self, pts: &[Vec2], hw: f32) {
        for w in pts.windows(2) {
            let id = self.segs.len() as u32;
            let m = Vec2::splat(hw + 1.0);
            for b in buckets(w[0].min(w[1]) - m, w[0].max(w[1]) + m) {
                self.seg_at.entry(b).or_default().push(id);
            }
            self.segs.push((w[0], w[1], hw));
        }
    }
    /// Whether a shape is clear of every shape placed and of every street by
    /// `gap` metres.
    fn clear(&self, s: &Shape, gap: f32) -> bool {
        self.clear_of_shapes(s) && self.off_streets(s, gap)
    }
    fn clear_of_shapes(&self, s: &Shape) -> bool {
        let (lo, hi) = s.bounds();
        let mut seen: Vec<u32> = Vec::new();
        for b in buckets(lo, hi) {
            if let Some(ids) = self.shape_at.get(&b) {
                for &i in ids {
                    if !seen.contains(&i) {
                        seen.push(i);
                        if self.shapes[i as usize].overlaps(s) {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
    fn off_streets(&self, s: &Shape, gap: f32) -> bool {
        let (lo, hi) = s.bounds();
        let probes = s.probes();
        for b in buckets(lo, hi) {
            if let Some(ids) = self.seg_at.get(&b) {
                for &i in ids {
                    let (a, c, hw) = self.segs[i as usize];
                    for p in probes {
                        if seg_dist(p, a, c) < hw + gap {
                            return false;
                        }
                    }
                    // A street crossing the shape between probes.
                    if crosses(s, a, c) {
                        return false;
                    }
                }
            }
        }
        true
    }
}

/// Whether segment ac passes through the rectangle s.
fn crosses(s: &Shape, a: Vec2, c: Vec2) -> bool {
    if s.q.is_some() {
        let q = s.corners();
        return s.sd(a) < 0.0
            || s.sd(c) < 0.0
            || (0..4).any(|k| intersect(a, c, q[k], q[(k + 1) % 4]).is_some());
    }
    let (u, w) = s.axes();
    let la = Vec2::new((a - s.c).dot(u), (a - s.c).dot(w));
    let lc = Vec2::new((c - s.c).dot(u), (c - s.c).dot(w));
    // Liang-Barsky clip against the box.
    let d = lc - la;
    let (mut t0, mut t1) = (0.0f32, 1.0f32);
    for (p, q) in [
        (-d.x, la.x + s.h.x),
        (d.x, s.h.x - la.x),
        (-d.y, la.y + s.h.y),
        (d.y, s.h.y - la.y),
    ] {
        if p.abs() < 1e-9 {
            if q < 0.0 {
                return false;
            }
        } else {
            let r = q / p;
            if p < 0.0 {
                t0 = t0.max(r);
            } else {
                t1 = t1.min(r);
            }
        }
    }
    t0 <= t1
}

/// The plan's sizes for a place of `pop` people: frontage and depth of its
/// plots, metres. Burgage plots in towns are narrow and long; a village's tofts
/// and crofts broad; a hamlet's broader still.
fn plot_size(pop: f32) -> (f32, f32) {
    let l = pop.max(10.0).log10();
    // log10: 50 -> 1.7, 300 -> 2.5, 800 -> 2.9, 3000 -> 3.5, 10000 -> 4.
    let front = if l < 2.5 {
        32.0 + (20.0 - 32.0) * smoothstep(1.7, 2.5, l)
    } else if l < 2.9 {
        20.0 + (9.0 - 20.0) * smoothstep(2.5, 2.9, l)
    } else {
        9.0 + (5.5 - 9.0) * smoothstep(2.9, 4.0, l)
    };
    // Crofts run back some 70 m; burgage plots shorten as towns crowd, to
    // about 45 m in the largest cities.
    let depth = 70.0 - 25.0 * smoothstep(2.9, 4.2, l);
    (front, depth)
}

/// Settled area in hectares by Cesaretti et al. (2016) for English towns,
/// with the crofts of villages: for comparison with what the plan builds.
pub fn expected_ha(pop: f32) -> f32 {
    0.12 * pop.max(1.0).powf(0.73) + (pop / 4.5) * 0.3 * (1.0 - smoothstep(300.0, 1500.0, pop))
}

/// Clips a polyline to a disc, giving the runs inside.
fn clip_disc(pts: &[Vec2], c: Vec2, r: f32) -> Vec<Vec<Vec2>> {
    let mut out = Vec::new();
    let mut run: Vec<Vec2> = Vec::new();
    for &p in pts {
        if (p - c).length() <= r {
            run.push(p);
        } else if run.len() >= 2 {
            out.push(std::mem::take(&mut run));
        } else {
            run.clear();
        }
    }
    if run.len() >= 2 {
        out.push(run);
    }
    out
}

/// A plot ready to be placed: where it fronts and how big.
struct Slot {
    pri: f32,
    street: usize,
    s: f32,
    side: f32,
    front: f32,
}

/// Lays out every place this year. `roads` are the road and track lines in
/// metres with their half-widths; `water` answers for rivers and the sea.
pub fn plan(v: &ViewState, roads: &[(Vec<Vec2>, f32, bool)], water: &dyn Water) -> Plan {
    let t = &v.terrain;
    let g = t.grid;
    let cell_m = g.cell_km * 1000.0;
    let snap = &v.snapshot;
    let mut space = Space::default();
    // Roads are obstacles everywhere; tracks only where a hamlet or village
    // takes them for its street (towns draw over them, R11).
    for (pts, hw, road) in roads {
        if *road {
            space.add_street(pts, *hw);
        }
    }
    let bounds: Vec<(Vec2, Vec2)> = roads
        .iter()
        .map(|(pts, _, _)| {
            pts.iter().fold(
                (Vec2::splat(f32::MAX), Vec2::splat(f32::MIN)),
                |(lo, hi), p| (lo.min(*p), hi.max(*p)),
            )
        })
        .collect();
    let mut plan = Plan::default();
    let largest = snap
        .settlements
        .iter()
        .map(|s| s.pop)
        .fold(0.0f32, f32::max);
    // Largest first, so a town's plots are not crowded out by a hamlet's.
    let mut order: Vec<usize> = (0..snap.settlements.len()).collect();
    order.sort_by(|&a, &b| {
        let (sa, sb) = (&snap.settlements[a], &snap.settlements[b]);
        sb.pop.total_cmp(&sa.pop).then(sa.id.cmp(&sb.id))
    });
    for k in order {
        let s = &snap.settlements[k];
        let (x, y) = g.xy(s.anchor as usize);
        let anchor = Vec2::new((x as f32 + 0.5) * cell_m, (y as f32 + 0.5) * cell_m);
        let walled = s.rank == Rank::City;
        let cathedral = s.rank == Rank::City && (s.pop >= 8000.0 || s.pop >= largest);
        let place = lay_out(
            t, s.id, anchor, s.pop, s.rank, walled, cathedral, roads, &bounds, water, &v.look,
            &mut space, &mut plan,
        );
        plan.places.push(place);
    }
    plan.shapes = space.shapes;
    plan
}

#[allow(clippy::too_many_arguments)]
fn lay_out(
    t: &Terrain,
    id: u32,
    anchor: Vec2,
    pop: f32,
    rank: Rank,
    walled: bool,
    cathedral: bool,
    roads: &[(Vec<Vec2>, f32, bool)],
    bounds: &[(Vec2, Vec2)],
    water: &dyn Water,
    look: &Look,
    space: &mut Space,
    plan: &mut Plan,
) -> Place {
    // Sizes are true metres times `z`, the look's building scale; the place's
    // own lanes are `rs` times their true width (R12).
    let z = look.buildings;
    let rs = look.roads;
    // Open places (market places, greens, churchyards) and the great buildings
    // grow more slowly than houses, so a larger look does not open huge
    // squares in the middle of a place (R13).
    let zo = z.powf(0.6);
    let seed = id as usize;
    let start_shapes = space.shapes.len();
    let start_streets = plan.streets.len();
    let households = (pop / look.per_house).round().max(1.0) as u32;
    let (front0, depth0) = plot_size(pop);
    // Lots widen with the buildings but deepen only as the root of their scale,
    // so a larger look keeps its lots built over rather than empty (R13).
    let (front0, depth0) = (front0 * z, depth0 * z.sqrt());
    // Plots needed: fewer than households where the core holds tenements.
    let plots_needed = households as f32 / (1.0 + 0.5 * smoothstep(4000.0, 15000.0, pop));
    let r_est = (plots_needed * front0 * depth0 * 1.5 / std::f32::consts::PI).sqrt();
    // How far plots may run from the centre along the roads. A large city's
    // core is dense (tenements and courts) and its lanes many, so it stays
    // compact; suburbs along the roads stay short.
    let reach_max = (2.4 * r_est).max(160.0 * z);
    let cell_m = t.grid.cell_km * 1000.0;
    let (cx, cy) = ((anchor.x / cell_m) as i32, (anchor.y / cell_m) as i32);
    let ci = t.grid.idx(
        cx.clamp(0, t.grid.w as i32 - 1),
        cy.clamp(0, t.grid.h as i32 - 1),
    );
    let upland = t.elevation[ci] > 250.0;
    let poor = t.food_yield[ci] < 0.35;

    // Main streets: the roads within reach. The centre is the nearest point on
    // them to the anchor (the roads were kept off the rivers), or the anchor.
    let mut streets: Vec<Poly> = Vec::new();
    // Towns take only the roads for their main streets (land trade off the
    // roads gives way to them); smaller places, and towns no road reaches,
    // their tracks too.
    let near = |lo: &Vec2, hi: &Vec2| {
        let r = Vec2::splat(reach_max);
        !(anchor.x + r.x < lo.x
            || anchor.x - r.x > hi.x
            || anchor.y + r.y < lo.y
            || anchor.y - r.y > hi.y)
    };
    let has_road = roads.iter().zip(bounds).any(|((pts, _, road), (lo, hi))| {
        *road && near(lo, hi) && !clip_disc(pts, anchor, reach_max).is_empty()
    });
    for ((pts, hw, road), (lo, hi)) in roads.iter().zip(bounds) {
        if !road && pop >= 500.0 && has_road {
            continue;
        }
        if !near(lo, hi) {
            continue;
        }
        for run in clip_disc(pts, anchor, reach_max) {
            if !road {
                space.add_street(&run, *hw);
            }
            streets.push(Poly::new(run, *hw, true));
        }
    }
    // On dry land: a place whose anchor is on its river (a bridge town) has
    // its centre, and its market, on the nearer bank.
    let mut centre = anchor;
    let mut best = f32::MAX;
    let dry = |p: Vec2| water.land(p) && water.river_gap(p) > 30.0;
    for st in &streets {
        let mut s = 0.0;
        while s <= st.len() {
            let p = st.at(s).0;
            let d = (p - anchor).length();
            if d < best && d < 400.0 && dry(p) {
                best = d;
                centre = p;
            }
            s += 10.0;
        }
    }
    // A place on no road gets a lane of its own through it.
    // It runs from the nearest dry ground, the way that keeps longest on it.
    if streets.is_empty() {
        let half = r_est.clamp(60.0 * z, 300.0 * z);
        let mut start = anchor;
        'find: for r in [0.0f32, 20.0, 40.0, 70.0, 100.0, 150.0] {
            for k in 0..12 {
                let p = anchor + dir(k as f32 / 12.0 * std::f32::consts::TAU) * r;
                if water.land(p) && water.river_gap(p) > 30.0 {
                    start = p;
                    break 'find;
                }
            }
        }
        let a0 = h01(seed, 11) * std::f32::consts::PI;
        let pts = (0..6)
            .map(|k| {
                let a = a0 + k as f32 / 6.0 * std::f32::consts::PI;
                let pts = vec![start - dir(a) * half, start, start + dir(a) * half];
                truncate_both(pts, start, water)
            })
            .max_by_key(|p| p.len())
            .unwrap_or_default();
        if pts.len() >= 2 {
            space.add_street(&pts, 2.5 * rs);
            plan.streets.push(Street {
                pts: pts.clone(),
                hw: 2.5 * rs,
            });
            // It is the place's high street, and lanes branch off it.
            streets.push(Poly::new(pts, 2.5 * rs, true));
        }
    }

    // The market place, where the main street passes the centre (towns), with a
    // market hall in the larger; or a green where a village's roads meet.
    let mut reserved: Vec<Shape> = Vec::new();
    let main_near: Option<(usize, f32)> = streets
        .iter()
        .enumerate()
        .filter(|(_, st)| st.main)
        .map(|(i, st)| (i, st.nearest(centre)))
        .min_by(|a, b| a.1 .0.total_cmp(&b.1 .0))
        .map(|(i, (_, s))| (i, s));
    if pop >= 500.0 {
        if let Some((i, s)) = main_near {
            let (p, tan) = streets[i].at(s);
            let len = (38.0 * (pop / 500.0).sqrt()).clamp(38.0, 150.0) * zo;
            let wid = (0.42 * len).clamp(18.0 * zo, 60.0 * zo) + 2.0 * streets[i].hw;
            let m = Shape {
                c: p,
                h: Vec2::new(len * 0.5, wid * 0.5),
                angle: tan.y.atan2(tan.x),
                kind: MARKET,
                tone: h01(seed, 13),
                extra: 0.0,
                height: 0.0,
                q: None,
            };
            if water.land(m.c) {
                reserved.push(m);
                if pop >= 1500.0 {
                    let side = if h01(seed, 14) < 0.5 { 1.0 } else { -1.0 };
                    let off = perp(tan) * side * (wid * 0.5 - 8.0 * z);
                    reserved.push(Shape {
                        c: p + off,
                        h: Vec2::new(9.0 + 4.0 * h01(seed, 15), 4.5) * z,
                        angle: m.angle,
                        kind: HALL,
                        tone: h01(seed, 16),
                        extra: 1.0,
                        height: 9.0 * z,
                        q: None,
                    });
                }
            }
        }
    } else if pop >= 100.0 {
        let arms = streets
            .iter()
            .filter(|st| st.main)
            .filter(|st| {
                (st.pts[0] - centre).length() < 90.0 * z
                    || (st.pts[st.pts.len() - 1] - centre).length() < 90.0 * z
            })
            .count();
        if arms >= 3 {
            let r = (30.0 + 25.0 * h01(seed, 17)) * zo;
            let g = Shape {
                c: centre,
                h: Vec2::new(r, r * (0.6 + 0.3 * h01(seed, 18))),
                angle: h01(seed, 19) * std::f32::consts::PI,
                kind: GREEN,
                tone: h01(seed, 20),
                extra: 0.0,
                height: 0.0,
                q: None,
            };
            if water.land(g.c) {
                reserved.push(g);
            }
        }
    }

    // Side streets in towns, grown in generations: lanes off the main
    // streets, then lanes off those in the larger towns, each kept a block's
    // depth from every street but the one it leaves, so they join into blocks
    // of about 1.5 plot depths; and back lanes behind the main streets' plots.
    // Villages of 150 or more have a lane or two of their own (R13).
    if pop >= 150.0 {
        let village = pop < 350.0;
        // A town on only one or two roads (often by a river, which takes half
        // its frontage) grows more lanes, further out, to house its people.
        let mains = streets.iter().filter(|st| st.main).count();
        let few = mains <= 2 && !village;
        let generations = if pop < 1500.0 {
            1
        } else if pop < 5000.0 {
            2
        } else {
            4
        } + if few { 2 } else { 0 };
        let spacing =
            (130.0 - 60.0 * smoothstep(500.0, 8000.0, pop)) * z * if village { 1.3 } else { 1.0 };
        let reach = r_est * if few { 1.7 } else { 1.25 };
        let hw =
            (1.6 + 1.0 * smoothstep(800.0, 5000.0, pop)) * rs * if village { 0.8 } else { 1.0 };
        let mut from = 0usize;
        for gen in 0..generations {
            let upto = streets.len();
            let mut made: Vec<(usize, Vec<Vec2>, bool)> = Vec::new();
            for (i, st) in streets.iter().enumerate().take(upto).skip(from) {
                if gen == 0 && !st.main {
                    continue;
                }
                let (_, s0) = st.nearest(centre);
                let chance = if village {
                    0.25
                } else if gen == 0 {
                    0.5
                } else {
                    0.4
                };
                for sgn in [1.0f32, -1.0] {
                    let mut s = s0 + sgn * spacing * 0.6;
                    let mut n = 0usize;
                    while s > 0.0 && s < st.len() && (st.at(s).0 - centre).length() < reach {
                        let (p, tan) = st.at(s);
                        for side in [1.0f32, -1.0] {
                            let key = seed * 7919
                                + i * 131
                                + n * 2
                                + (side > 0.0) as usize
                                + gen * 100_003;
                            if h01(key, 21) > chance {
                                continue;
                            }
                            let a = (perp(tan) * side).y.atan2((perp(tan) * side).x)
                                + (h01(key, 22) - 0.5) * 0.5;
                            let len = depth0 * 1.6 + (30.0 + 120.0 * h01(key, 23)) * z;
                            let start = p + perp(tan) * side * (st.hw + 0.5);
                            // A lane bends as it goes, one way overall and
                            // wandering a little along its length (R13).
                            let bend = (h01(key, 24) - 0.5) * 2.0;
                            let pts: Vec<Vec2> = (0..=20u32)
                                .scan((start, a), |st8, j| {
                                    let (p0, a0) = *st8;
                                    let p1 = p0 + dir(a0) * len / 20.0;
                                    let turn =
                                        bend * 0.07 + (h01(key * 31 + j as usize, 26) - 0.5) * 0.16;
                                    *st8 = (p1, a0 + turn);
                                    Some(p0)
                                })
                                .collect();
                            made.push((i, pts, true));
                        }
                        s += sgn * spacing * (0.8 + 0.4 * h01(seed + n + gen * 77, 25));
                        n += 1;
                    }
                }
                // Back lanes behind the main streets' plots.
                if gen == 0 && pop >= 800.0 {
                    for side in [1.0f32, -1.0] {
                        let off = st.hw + depth0 * 0.85 + 3.0 * z;
                        let pts: Vec<Vec2> = (0..)
                            .map(|k| s0 - 1.4 * r_est + k as f32 * 20.0)
                            .take_while(|&s| s <= s0 + 1.4 * r_est)
                            .filter(|&s| s > 0.0 && s < st.len())
                            .map(|s| {
                                let (p, tan) = st.at(s);
                                p + perp(tan) * side * off
                            })
                            .collect();
                        if pts.len() >= 3 {
                            made.push((i, pts, false));
                        }
                    }
                }
            }
            for (own, pts, side_street) in made {
                // How far a point keeps from street j: the block's depth from
                // any street but the lane's own (which it leaves, or which a
                // back lane follows at its plots' depth).
                // Err(Some(j)) when it comes too near street j.
                let ok =
                    |p: Vec2, from_start: f32, streets: &Vec<Poly>| -> Result<(), Option<usize>> {
                        if !water.land(p)
                            || water.river_gap(p) < 4.0
                            || (p - centre).length() > reach_max
                        {
                            return Err(None);
                        }
                        if reserved.iter().any(|s| {
                            let (u, v) = s.axes();
                            let l = p - s.c;
                            l.dot(u).abs() < s.h.x + 2.0 && l.dot(v).abs() < s.h.y + 2.0
                        }) {
                            return Err(None);
                        }
                        for (j, st) in streets.iter().enumerate() {
                            let keep = if j == own {
                                if side_street {
                                    continue;
                                }
                                st.hw + depth0 * 0.5
                            } else if from_start < 12.0 * z {
                                continue;
                            } else {
                                st.hw + hw + depth0 * 0.7
                            };
                            if st.nearest(p).0 < keep {
                                return Err(Some(j));
                            }
                        }
                        Ok(())
                    };
                let mut runs: Vec<Vec<Vec2>> = Vec::new();
                let mut run: Vec<Vec2> = Vec::new();
                let mut along = 0.0f32;
                'walk: for w in pts.windows(2) {
                    let n = ((w[1] - w[0]).length() / 5.0).ceil().max(1.0) as usize;
                    for k in 0..n {
                        let p = w[0] + (w[1] - w[0]) * (k as f32 / n as f32);
                        let here = ok(p, along, &streets);
                        if here.is_ok() {
                            run.push(p);
                        } else {
                            // A side street that comes up to another street
                            // mostly runs on to join it, closing a block (R13).
                            if let (true, Err(Some(j)), true) = (side_street, here, run.len() >= 2)
                            {
                                if h01(own * 7 + j * 131 + seed, 27) < 0.75 {
                                    let to = &streets[j];
                                    let q = to.at(to.nearest(p).1).0;
                                    let last = *run.last().unwrap_or(&p);
                                    let clear = (1..8).all(|t| {
                                        let x = last + (q - last) * (t as f32 / 8.0);
                                        water.land(x) && water.river_gap(x) > 4.0
                                    });
                                    if clear && (q - last).length() < 3.0 * depth0 {
                                        run.push(q);
                                    }
                                }
                            }
                            if run.len() >= 2 {
                                runs.push(std::mem::take(&mut run));
                            }
                            run.clear();
                            if side_street {
                                break 'walk;
                            }
                        }
                        along += (w[1] - w[0]).length() / n as f32;
                    }
                }
                if run.len() >= 2 {
                    runs.push(run);
                }
                if side_street {
                    runs.truncate(1);
                }
                for r in runs {
                    let len: f32 = r.windows(2).map(|w| (w[1] - w[0]).length()).sum();
                    if len < 40.0 * z {
                        continue;
                    }
                    // Thin the points to about every 15 m.
                    let mut pts: Vec<Vec2> = r.iter().step_by(3).copied().collect();
                    if pts.last() != r.last() {
                        pts.push(*r.last().unwrap());
                    }
                    space.add_street(&pts, hw);
                    plan.streets.push(Street {
                        pts: pts.clone(),
                        hw,
                    });
                    streets.push(Poly::new(pts, hw, false));
                }
            }
            from = upto;
        }
    }

    for r in &reserved {
        space.add_shape(*r);
    }

    // Churches: one to about a thousand people, in a churchyard beside a street
    // near the centre, the others spread out; a cathedral in its close in the
    // greatest city; friaries at the edge of cities.
    if pop >= 100.0 {
        let n = ((pop / 1000.0).round() as usize).max(1);
        for c in 0..n {
            let big = cathedral && c == 0;
            let want = if c == 0 {
                40.0 * z
            } else {
                r_est * (0.35 + 0.6 * c as f32 / n as f32)
            };
            let (yw, yd) = if big {
                (190.0 * zo, 140.0 * zo)
            } else {
                (
                    (42.0 + 10.0 * smoothstep(500.0, 3000.0, pop)) * zo,
                    38.0 * zo,
                )
            };
            place_church(
                seed * 31 + c,
                centre,
                want,
                yw,
                yd,
                big,
                pop,
                zo,
                &streets,
                water,
                space,
            );
        }
        if walled {
            let friaries = ((pop / 4000.0).floor() as usize).max(1);
            for f in 0..friaries {
                place_friary(
                    seed * 37 + f,
                    centre,
                    r_est * 0.85,
                    zo,
                    &streets,
                    water,
                    space,
                );
            }
        }
    }

    // Plots, from the centre outward along every street.
    let first_shape = space.shapes.len();
    let wall = if walled {
        // A first layout finds the core; the wall goes round about half the
        // houses, then the plots are laid again clear of it.
        let mark = space.shapes.len();
        place_plots(
            seed,
            pop,
            rank,
            upland,
            upland && poor,
            households,
            centre,
            reach_max,
            z,
            &streets,
            water,
            space,
            None,
        );
        let core: Vec<Shape> = space.shapes[mark..]
            .iter()
            .filter(|s| s.kind == PLOT || s.kind == COURT)
            .take(
                space.shapes[mark..]
                    .iter()
                    .filter(|s| s.kind == PLOT || s.kind == COURT)
                    .count()
                    / 2,
            )
            .copied()
            .collect();
        let line = wall_line(centre, &core, z);
        // Undo the first layout.
        space.shapes.truncate(mark);
        for v in space.shape_at.values_mut() {
            v.retain(|&i| (i as usize) < mark);
        }
        Some(line)
    } else {
        None
    };
    let placed = place_plots(
        seed,
        pop,
        rank,
        upland,
        upland && poor,
        households,
        centre,
        reach_max,
        z,
        &streets,
        water,
        space,
        wall.as_deref(),
    );
    // The core of a town packed with buildings behind its fronts (R15).
    if pop >= 500.0 {
        infill(
            seed,
            pop,
            upland,
            centre,
            r_est,
            z,
            water,
            space,
            start_shapes,
            wall.as_deref(),
        );
    }
    let mut reach = 0.0f32;
    for s in &space.shapes[first_shape..] {
        if s.kind == PLOT || s.kind == COURT {
            reach = reach.max((s.c - centre).length() + s.h.x.max(s.h.y));
        }
    }
    if let Some(line) = &wall {
        build_wall(seed, line, &streets, space, z);
    }
    let bound = boundary(centre, &space.shapes[start_shapes..], z);
    // The place's own lanes end where its plots do: a lane is cut back to its
    // last point within the boundary, so none runs on into the fields.
    let inside = |p: Vec2| {
        let d = p - centre;
        let a = d.y.atan2(d.x) / std::f32::consts::TAU;
        let f = (a + 1.0).fract() * BOUND as f32;
        let k = f as usize % BOUND;
        let r = bound[k] + (bound[(k + 1) % BOUND] - bound[k]) * f.fract();
        d.length() < r - 8.0 * z
    };
    for st in &mut plan.streets[start_streets..] {
        let mut last = 0;
        for (k, &p) in st.pts.iter().enumerate() {
            if inside(p) {
                last = k;
            }
        }
        st.pts.truncate(last + 1);
    }
    plan.streets.retain(|st| st.pts.len() >= 2);
    Place {
        id,
        households,
        houses: placed,
        reach,
        walled,
        bound,
    }
}

/// Keeps a lane only where it is on dry land, out from the centre each way.
fn truncate_both(pts: Vec<Vec2>, centre: Vec2, water: &dyn Water) -> Vec<Vec2> {
    let ok = |p: Vec2| water.land(p) && water.river_gap(p) > 4.0;
    let mut out = Vec::new();
    for w in pts.windows(2) {
        let n = ((w[1] - w[0]).length() / 5.0).ceil().max(1.0) as usize;
        for k in 0..=n {
            let p = w[0] + (w[1] - w[0]) * (k as f32 / n as f32);
            if (ok(p) || (p - centre).length() < 1.0)
                && out.last().is_none_or(|q: &Vec2| (*q - p).length() > 4.0)
            {
                out.push(p);
            }
        }
    }
    out
}

#[allow(clippy::too_many_arguments)]
fn place_church(
    key: usize,
    centre: Vec2,
    want: f32,
    yw: f32,
    yd: f32,
    big: bool,
    pop: f32,
    z: f32,
    streets: &[Poly],
    water: &dyn Water,
    space: &mut Space,
) {
    // Try spots along the streets, nearest to the wanted distance first.
    let mut spots: Vec<(f32, usize, f32, f32)> = Vec::new();
    for (i, st) in streets.iter().enumerate() {
        let mut s = 10.0;
        while s < st.len() {
            let (p, _) = st.at(s);
            let d = (p - centre).length();
            for side in [1.0f32, -1.0] {
                spots.push((
                    (d - want).abs() + 20.0 * z * h01(key + i, s as u32 + side as u32),
                    i,
                    s,
                    side,
                ));
            }
            s += 15.0;
        }
    }
    spots.sort_by(|a, b| a.0.total_cmp(&b.0));
    for &(_, i, s, side) in spots.iter().take(300) {
        let st = &streets[i];
        let (p, tan) = st.at(s);
        let n = perp(tan) * side;
        let yard = Shape {
            c: p + n * (st.hw + 2.0 * z + yd * 0.5),
            h: Vec2::new(yw * 0.5, yd * 0.5),
            angle: tan.y.atan2(tan.x),
            kind: if big { PRECINCT } else { CHURCHYARD },
            tone: h01(key, 31),
            extra: 0.0,
            height: 0.0,
            q: None,
        };
        if !yard
            .probes()
            .iter()
            .all(|&q| water.land(q) && water.river_gap(q) > 3.0)
        {
            continue;
        }
        if !space.clear(&yard, 1.0) {
            continue;
        }
        space.add_shape(yard);
        // Churches face east, more or less.
        let a = (h01(key, 32) - 0.5) * 0.25;
        let (nave, wide) = if big {
            (110.0 * z, 13.0 * z)
        } else {
            let l = 18.0 + 14.0 * smoothstep(100.0, 3000.0, pop);
            (l * z, (4.0 + 1.5 * smoothstep(100.0, 3000.0, pop)) * z)
        };
        let body = Shape {
            c: yard.c,
            h: Vec2::new(nave * 0.5, wide),
            angle: a,
            kind: CHURCH,
            tone: h01(key, 33),
            extra: if pop >= 1500.0 { 1.0 } else { 0.0 },
            height: if big { 30.0 } else { 11.0 } * z,
            q: None,
        };
        space.shapes.push(body);
        // The tower at the west end; transepts on a cathedral; its cloister.
        let west = yard.c - dir(a) * (nave * 0.5 + wide * 0.6);
        space.shapes.push(Shape {
            c: west,
            h: Vec2::splat(wide * if big { 0.9 } else { 0.85 }),
            angle: a,
            kind: TOWER,
            tone: h01(key, 34),
            extra: 0.0,
            height: if big { 60.0 } else { 22.0 } * z,
            q: None,
        });
        if big {
            space.shapes.push(Shape {
                c: yard.c + dir(a) * 15.0 * z,
                h: Vec2::new(wide, 32.0 * z),
                angle: a,
                kind: CHURCH,
                tone: h01(key, 35),
                extra: 1.0,
                height: 28.0 * z,
                q: None,
            });
            space.shapes.push(Shape {
                c: yard.c + (perp(dir(a)) * 38.0 + dir(a) * 10.0) * z,
                h: Vec2::splat(20.0 * z),
                angle: a,
                kind: CLOISTER,
                tone: h01(key, 36),
                extra: 1.0,
                height: 9.0 * z,
                q: None,
            });
        }
        return;
    }
}

fn place_friary(
    key: usize,
    centre: Vec2,
    want: f32,
    z: f32,
    streets: &[Poly],
    water: &dyn Water,
    space: &mut Space,
) {
    let mut spots: Vec<(f32, usize, f32, f32)> = Vec::new();
    for (i, st) in streets.iter().enumerate() {
        let mut s = 10.0;
        while s < st.len() {
            let d = (st.at(s).0 - centre).length();
            for side in [1.0f32, -1.0] {
                spots.push(((d - want).abs() + 30.0 * h01(key + i, s as u32), i, s, side));
            }
            s += 20.0;
        }
    }
    spots.sort_by(|a, b| a.0.total_cmp(&b.0));
    for &(_, i, s, side) in spots.iter().take(200) {
        let st = &streets[i];
        let (p, tan) = st.at(s);
        let n = perp(tan) * side;
        let yard = Shape {
            c: p + n * (st.hw + 52.0 * z),
            h: Vec2::new(60.0, 50.0) * z,
            angle: tan.y.atan2(tan.x),
            kind: PRECINCT,
            tone: h01(key, 41),
            extra: 0.0,
            height: 0.0,
            q: None,
        };
        if !yard
            .probes()
            .iter()
            .all(|&q| water.land(q) && water.river_gap(q) > 3.0)
            || !space.clear(&yard, 1.0)
        {
            continue;
        }
        space.add_shape(yard);
        let a = (h01(key, 42) - 0.5) * 0.2;
        space.shapes.push(Shape {
            c: yard.c - perp(dir(a)) * 12.0 * z,
            h: Vec2::new(24.0, 6.0) * z,
            angle: a,
            kind: CHURCH,
            tone: h01(key, 43),
            extra: 1.0,
            height: 14.0 * z,
            q: None,
        });
        space.shapes.push(Shape {
            c: yard.c + perp(dir(a)) * 14.0 * z,
            h: Vec2::splat(15.0 * z),
            angle: a,
            kind: CLOISTER,
            tone: h01(key, 44),
            extra: 1.0,
            height: 8.0 * z,
            q: None,
        });
        return;
    }
}

/// Places one plot per household along the streets, nearest the centre first,
/// each with its house and often a barn or back range. Returns how many.
#[allow(clippy::too_many_arguments)]
fn place_plots(
    seed: usize,
    pop: f32,
    rank: Rank,
    upland: bool,
    scattered: bool,
    households: u32,
    centre: Vec2,
    reach_max: f32,
    z: f32,
    streets: &[Poly],
    water: &dyn Water,
    space: &mut Space,
    wall: Option<&[Vec2]>,
) -> u32 {
    let (front0, depth0) = plot_size(pop);
    // Lots widen with the buildings but deepen only as the root of their scale,
    // so a larger look keeps its lots built over rather than empty (R13).
    let (front0, depth0) = (front0 * z, depth0 * z.sqrt());
    let town = smoothstep(300.0, 1500.0, pop);
    let mut slots: Vec<Slot> = Vec::new();
    for (i, st) in streets.iter().enumerate() {
        let (_, s0) = st.nearest(centre);
        for sgn in [1.0f32, -1.0] {
            let mut s = s0;
            let mut n = 0u32;
            loop {
                let (p, _) = st.at(s);
                let d = (p - centre).length();
                if d > reach_max {
                    break;
                }
                // Plots widen towards the edge of the place.
                let grow = 1.0 + 0.6 * smoothstep(0.2 * reach_max, 0.8 * reach_max, d);
                // Village lots vary more in width than a town's (R13).
                let vary = h01(seed * 977 + i * 31 + n as usize, 51);
                let f = front0
                    * grow
                    * (0.8 + 0.45 * vary + (1.0 - town) * (0.4 * vary - 0.15))
                    * if scattered { 1.4 } else { 1.0 };
                let mid = s + sgn * f * 0.5;
                if mid < 0.0 || mid > st.len() {
                    break;
                }
                for side in [1.0f32, -1.0] {
                    let pri = (st.at(mid).0 - centre).length()
                        + if st.main { 0.0 } else { 10.0 * z }
                        + 15.0 * z * h01(seed + i * 7 + n as usize * 3 + (side > 0.0) as usize, 53);
                    slots.push(Slot {
                        pri,
                        street: i,
                        s: mid,
                        side,
                        front: f,
                    });
                }
                s += sgn * f;
                n += 1;
            }
        }
    }
    slots.sort_by(|a, b| a.pri.total_cmp(&b.pri));
    let inside_wall = |p: Vec2| wall.is_some_and(|w| point_in(p, w));
    let near_wall = |p: Vec2| {
        wall.is_some_and(|w| {
            (0..w.len()).any(|k| seg_dist(p, w[k], w[(k + 1) % w.len()]) < 8.0 * z)
        })
    };
    let mut placed = 0u32;
    for (n, sl) in slots.iter().enumerate() {
        if placed >= households {
            break;
        }
        let key = seed * 104729 + n;
        if scattered && h01(key, 55) < 0.35 {
            continue;
        }
        // Villages leave gaps between their lots, more of them towards the
        // edge, where the ground is the village's own closes and trees (R13).
        let d_mid = (streets[sl.street].at(sl.s).0 - centre).length();
        let gap = (1.0 - town) * (0.1 + 0.3 * smoothstep(0.25 * reach_max, 0.9 * reach_max, d_mid));
        if h01(key, 56) < gap {
            continue;
        }
        let st = &streets[sl.street];
        let (p, tan) = st.at(sl.s);
        let nrm = perp(tan) * sl.side;
        let angle = tan.y.atan2(tan.x);
        let setback = (0.6 + (1.0 - town) * 1.5) * z;
        let mut ok = None;
        // Plots run back further towards the edge of a town; near its middle,
        // where land was dear, they are shorter.
        let d_c = (p - centre).length();
        let core = 0.6 + 0.4 * smoothstep(0.1 * reach_max, 0.6 * reach_max, d_c);
        let core = if town > 0.5 { core } else { 1.0 };
        for depth_f in [1.0f32, 0.75, 0.55] {
            let depth = depth0 * depth_f * core * (0.65 + 0.7 * h01(key, 57));
            // The plot's angle is turned so its local y runs from the street
            // to the back, whichever side of the street it is on.
            let plot = Shape {
                c: p + nrm * (st.hw + setback + depth * 0.5),
                h: Vec2::new(sl.front * 0.5 - 0.15 * z, depth * 0.5),
                angle: if sl.side < 0.0 {
                    angle + std::f32::consts::PI
                } else {
                    angle
                },
                kind: PLOT,
                tone: h01(key, 59),
                extra: rank as u8 as f32,
                height: 0.0,
                q: None,
            };
            let probes = plot.probes();
            if !probes
                .iter()
                .all(|&q| water.land(q) && water.river_gap(q) > 2.0)
            {
                continue;
            }
            if wall.is_some() {
                let inside = inside_wall(probes[0]);
                if probes
                    .iter()
                    .any(|&q| inside_wall(q) != inside || near_wall(q))
                {
                    continue;
                }
            }
            // Streets are tested against the plot set back a little from its
            // front, so the street it faces, curving, does not graze it.
            let back = perp(dir(plot.angle));
            let inset = Shape {
                c: plot.c + back * 1.5 * z,
                h: Vec2::new(plot.h.x - 0.3 * z, plot.h.y - 1.5 * z),
                ..plot
            };
            if !space.clear_of_shapes(&plot) || !space.off_streets(&inset, 0.3) {
                continue;
            }
            ok = Some((plot, depth));
            break;
        }
        let Some((mut plot, depth)) = ok else {
            continue;
        };
        // In the core of a large town a plot holds several households: the
        // front house is let as tenements and its yard built up as a court of
        // cottages down one side, with an alley beside them.
        let dense = town
            * smoothstep(4000.0, 15000.0, pop)
            * (1.0 - smoothstep(0.1 * reach_max, 0.4 * reach_max, d_c));
        let mut hh = (1.0 + dense * 3.0 * (0.6 + 0.8 * h01(key, 79))).round() as u32;
        if depth < 22.0 * z {
            hh = hh.min(2);
        }
        let hh = hh.clamp(1, 4).min(households - placed);
        let court = hh >= 3;
        if court {
            plot.kind = COURT;
        }
        space.add_shape(plot);
        placed += hh;
        let roof = if upland && h01(key, 61) < 0.6 {
            2.0
        } else if h01(key, 62) < 0.1 + 0.8 * smoothstep(500.0, 3000.0, pop) {
            1.0
        } else {
            0.0
        };
        let front_edge = p + nrm * (st.hw + setback);
        let core_t = town * (1.0 - smoothstep(0.15 * reach_max, 0.6 * reach_max, d_c));
        let lot = Lot {
            key,
            plot,
            front_edge,
            tan,
            nrm,
            front: sl.front,
            depth,
            hh,
            town: town > 0.5,
            core: core_t,
            roof,
            z,
        };
        let mut built: Vec<Shape> = Vec::new();
        lot.build(&mut built);
        space.shapes.extend(built);
    }
    placed
}

/// One lot and what stands on it (R13), after the hand-drawn village and town
/// maps the owner likes. In a village each house has its own size, is turned a
/// little from the street and set back by its own amount; many have a barn or
/// byre making an L with the house, or standing apart behind it, and a shed. In
/// a town the fronts make a row of uneven depth and height, some with an entry
/// beside them, built up behind in the core.
struct Lot {
    key: usize,
    plot: Shape,
    /// The middle of the lot's front edge, the street's direction there, and
    /// the way into the lot.
    front_edge: Vec2,
    tan: Vec2,
    nrm: Vec2,
    front: f32,
    depth: f32,
    /// Households it holds (more than one for tenements and courts).
    hh: u32,
    town: bool,
    /// 1 in the core of a town, 0 at its edge and in villages.
    core: f32,
    roof: f32,
    z: f32,
}

impl Lot {
    fn h(&self, k: u32) -> f32 {
        h01(self.key, k)
    }

    /// A point `along` metres from the middle of the frontage and `back` into the lot.
    fn at(&self, along: f32, back: f32) -> Vec2 {
        self.front_edge + self.tan * along + self.nrm * back
    }

    /// Whether b lies within the lot, a little in from its edge.
    fn within(&self, b: &Shape) -> bool {
        let m = 0.4 * self.z;
        let (u, w) = self.plot.axes();
        b.corners().iter().all(|&q| {
            let l = q - self.plot.c;
            l.dot(u).abs() < self.plot.h.x - m && l.dot(w).abs() < self.plot.h.y - m
        })
    }

    /// Adds b if it lies within the lot and clear of what stands there.
    fn add(&self, b: Shape, out: &mut Vec<Shape>) -> bool {
        if self.within(&b) && out.iter().all(|o| !o.overlaps(&b)) {
            out.push(b);
            true
        } else {
            false
        }
    }

    fn build(&self, out: &mut Vec<Shape>) {
        if self.town {
            self.build_town(out);
        } else {
            self.build_village(out);
        }
    }

    fn build_village(&self, out: &mut Vec<Shape>) {
        let z = self.z;
        let ang = self.tan.y.atan2(self.tan.x);
        let half_pi = std::f32::consts::FRAC_PI_2;
        let end_on = self.h(65) < 0.3;
        let len = (8.0 + 8.0 * self.h(63)) * z;
        let wid = (4.5 + 2.0 * self.h(64)) * z;
        let turn = (self.h(66) - 0.5) * 0.35;
        let set = (1.0 + 7.0 * self.h(67) * self.h(67)) * z;
        // The largest fit of the house, turned or (failing that) square to the street.
        let mut house = None;
        'fit: for shrink in [1.0f32, 0.8, 0.62] {
            for t in [turn, 0.0] {
                let (hx, hy) = (0.5 * len * shrink, 0.5 * wid);
                // Half its extent along the street, and into the lot.
                let (span, reach_in) = if end_on { (hy, hx) } else { (hx, hy) };
                let slack = (0.5 * self.front - span - 0.6 * z).max(0.0);
                let along = (self.h(68) - 0.5) * 2.0 * slack;
                let b = Shape {
                    c: self.at(along, set + reach_in + span * t.abs()),
                    h: Vec2::new(hx, hy),
                    angle: ang + t + if end_on { half_pi } else { 0.0 },
                    kind: HOUSE,
                    tone: self.h(69),
                    extra: self.roof,
                    height: (5.5 + 1.5 * self.h(70)) * z,
                    q: None,
                };
                if self.add(b, out) {
                    house = Some(b);
                    break 'fit;
                }
            }
        }
        let Some(house) = house else {
            return;
        };
        if self.depth < 22.0 * z {
            return;
        }
        // The house's axis that runs most nearly back into the lot, and the
        // one across it, with the house's half sizes along them.
        let (u, w) = house.axes();
        let (b_ax, s_ax, hb, hs) = if u.dot(self.nrm).abs() > w.dot(self.nrm).abs() {
            (u * u.dot(self.nrm).signum(), w, house.h.x, house.h.y)
        } else {
            (w * w.dot(self.nrm).signum(), u, house.h.y, house.h.x)
        };
        let pick = self.h(71);
        if pick < 0.35 {
            // A barn or byre from one end of the house, back into the yard: an L.
            let side = if self.h(72) < 0.5 { 1.0 } else { -1.0 };
            let blen = (7.0 + 7.0 * self.h(73)) * z;
            let bw = (3.5 + 1.5 * self.h(74)).min(hs / z) * z;
            let barn = Shape {
                c: house.c + s_ax * side * (hs - 0.5 * bw) + b_ax * (hb + 0.5 * blen),
                h: Vec2::new(0.5 * blen, 0.5 * bw),
                angle: b_ax.y.atan2(b_ax.x),
                kind: BARN,
                tone: self.h(75),
                extra: self.roof,
                height: (5.0 + 1.5 * self.h(76)) * z,
                q: None,
            };
            self.add(barn, out);
        } else if pick < 0.75 {
            // A barn standing apart behind the house, turned its own way.
            let blen = (6.0 + 8.0 * self.h(73)) * z;
            let bw = (3.5 + 2.0 * self.h(74)) * z;
            let a = ang + if self.h(77) < 0.5 { 0.0 } else { half_pi } + (self.h(78) - 0.5) * 0.3;
            for f in [1.0f32, 0.75] {
                let barn = Shape {
                    c: self.at(
                        (self.h(79) - 0.5) * 0.5 * self.front,
                        self.depth * (0.45 + 0.25 * self.h(81)) * f,
                    ),
                    h: Vec2::new(0.5 * blen, 0.5 * bw),
                    angle: a,
                    kind: BARN,
                    tone: self.h(75),
                    extra: self.roof,
                    height: (5.5 + 1.5 * self.h(76)) * z,
                    q: None,
                };
                if self.add(barn, out) {
                    break;
                }
            }
        }
        // A shed or pigsty somewhere in the yard.
        if self.h(82) < 0.45 {
            let shed = Shape {
                c: self.at(
                    (self.h(83) - 0.5) * 0.8 * self.front,
                    set + 2.0 * hb + (3.0 + 10.0 * self.h(84)) * z,
                ),
                h: Vec2::new(1.3 + 0.8 * self.h(85), 1.0 + 0.5 * self.h(86)) * z,
                angle: ang + (self.h(87) - 0.5) * 0.6,
                kind: BARN,
                tone: self.h(88),
                extra: 0.0,
                height: 3.0 * z,
                q: None,
            };
            self.add(shed, out);
        }
    }

    fn build_town(&self, out: &mut Vec<Shape>) {
        let z = self.z;
        let ang = self.tan.y.atan2(self.tan.x);
        let half_pi = std::f32::consts::FRAC_PI_2;
        let tall = if self.hh >= 2 { 1.0 } else { 0.0 };
        // Some fronts leave an entry beside them into the yard behind.
        let entry = if self.h(90) < 0.3 {
            (0.9 + 0.8 * self.h(91)) * z
        } else {
            0.0
        };
        let side = if self.h(92) < 0.5 { 1.0 } else { -1.0 };
        let hl = (0.5 * self.front - 0.15 * z - 0.5 * entry).max(1.5 * z);
        let hd = (4.0 + 2.5 * self.h(63) + 2.0 * tall) * z;
        // Fronts stand a little forward or back of their neighbours.
        let ragged = 0.7 * self.h(93) * z;
        let house = Shape {
            c: self.at(-side * 0.5 * entry, ragged + hd),
            h: Vec2::new(hl, hd),
            angle: ang,
            kind: HOUSE,
            tone: self.h(68),
            extra: self.roof,
            height: (8.0 + 3.5 * self.h(64) + 3.0 * tall) * z,
            q: None,
        };
        out.push(house);
        let start = ragged + 2.0 * hd;
        if self.hh >= 3 {
            // A court: cottages one behind another down one side.
            let across = (self.front - 2.2 * z).clamp(2.2 * z, 3.4 * z);
            for j in 0..(self.hh - 1) {
                let k = self.key + j as usize;
                let len = (4.0 + 1.5 * h01(k, 81)) * z;
                let cot = Shape {
                    c: self.at(
                        side * (0.5 * self.front - 0.5 * across - 0.3 * z),
                        start + 2.5 * z + j as f32 * 6.0 * z + 0.5 * len,
                    ),
                    h: Vec2::new(0.5 * len, 0.5 * across),
                    angle: ang + half_pi,
                    kind: HOUSE,
                    tone: h01(k, 82),
                    extra: self.roof,
                    height: (5.0 + 2.0 * h01(k, 83)) * z,
                    q: None,
                };
                self.add(cot, out);
            }
            return;
        }
        if self.depth < 20.0 * z {
            return;
        }
        // Back ranges down the sides of the lot, one behind another: in the
        // core they fill most of its depth, so the blocks read as built over;
        // towards the edge fewer, with gardens behind.
        let w = (0.5 * self.front - 0.6 * z).clamp(1.5 * z, 3.0 * z);
        let mut at = start + (1.0 + 4.0 * self.h(71)) * z;
        let mut side = side;
        for j in 0..6u32 {
            let k = self.key * 7 + j as usize;
            if h01(k, 69) > 0.3 + 0.6 * self.core - 0.12 * j as f32 {
                break;
            }
            let len = (4.0 + 5.0 * h01(k, 70) + 6.0 * self.core * h01(k, 94)) * z;
            let b = Shape {
                c: self.at(side * (0.5 * self.front - w - 0.3 * z), at + len),
                h: Vec2::new(len, w),
                angle: ang + half_pi,
                kind: BARN,
                tone: h01(k, 73),
                extra: self.roof,
                height: (5.0 + 2.5 * self.core * h01(k, 72)) * z,
                q: None,
            };
            if !self.add(b, out) {
                break;
            }
            at += 2.0 * len + (1.5 + 4.0 * h01(k, 71)) * z;
            if h01(k, 74) < 0.4 {
                side = -side;
            }
        }
        // In the core, a workshop or warehouse across the back of the plot.
        if self.h(95) < 0.45 * self.core {
            let d = (3.0 + 2.5 * self.h(96)) * z;
            let b = Shape {
                c: self.at(0.0, self.depth - d - 1.0 * z),
                h: Vec2::new(0.5 * self.front - 0.6 * z, d),
                angle: ang,
                kind: BARN,
                tone: self.h(97),
                extra: self.roof,
                height: 6.0 * z,
                q: None,
            };
            self.add(b, out);
        }
    }
}

/// Packs the core of a town with buildings behind its street fronts (R15), as
/// the cores of hand-drawn town maps are: from the centre outward, wherever a
/// building fits clear of the streets, the other buildings, the open places
/// (market, greens, churchyards, courts), the water and the wall. Each is
/// turned to the nearest street, and many stand with no frontage of their own,
/// reached through the yards. They may cover the back of a lot, not its front.
/// The packing thins out away from the centre and grows with the town.
#[allow(clippy::too_many_arguments)]
fn infill(
    seed: usize,
    pop: f32,
    upland: bool,
    centre: Vec2,
    r_est: f32,
    z: f32,
    water: &dyn Water,
    space: &mut Space,
    start: usize,
    wall: Option<&[Vec2]>,
) {
    let amount = smoothstep(300.0, 2500.0, pop);
    let core_r = r_est * (0.55 + 0.35 * amount);
    // What a new building must keep clear of: every building and open place
    // of this place (its lots' ground excepted), indexed by bucket.
    let mut block: Vec<Shape> = space.shapes[start..]
        .iter()
        .filter(|s| s.kind != PLOT)
        .copied()
        .collect();
    let mut at: BTreeMap<(i32, i32), Vec<u32>> = BTreeMap::new();
    let index = |at: &mut BTreeMap<(i32, i32), Vec<u32>>, s: &Shape, i: usize| {
        let (lo, hi) = s.bounds();
        for b in buckets(lo, hi) {
            at.entry(b).or_default().push(i as u32);
        }
    };
    for (i, s) in block.iter().enumerate() {
        index(&mut at, s, i);
    }
    // The place's lots: infill stands on their ground or within a short way
    // of it (the middles of blocks deeper than two lots), never out in the
    // fields.
    let lots: Vec<Shape> = space.shapes[start..]
        .iter()
        .filter(|s| s.kind == PLOT)
        .copied()
        .collect();
    let mut lot_at: BTreeMap<(i32, i32), Vec<u32>> = BTreeMap::new();
    let reach = 20.0 * z;
    for (i, s) in lots.iter().enumerate() {
        let (lo, hi) = s.bounds();
        for b in buckets(lo - Vec2::splat(reach), hi + Vec2::splat(reach)) {
            lot_at.entry(b).or_default().push(i as u32);
        }
    }
    let on_lots = |q: Vec2| {
        let b = ((q.x / BUCKET).floor() as i32, (q.y / BUCKET).floor() as i32);
        lot_at.get(&b).is_some_and(|ids| {
            ids.iter().any(|&i| {
                let l = &lots[i as usize];
                let (u, w) = l.axes();
                let d = q - l.c;
                d.dot(u).abs() <= l.h.x + reach && d.dot(w).abs() <= l.h.y + reach
            })
        })
    };
    let clear = |block: &[Shape], at: &BTreeMap<(i32, i32), Vec<u32>>, s: &Shape| {
        let (lo, hi) = s.bounds();
        for b in buckets(lo, hi) {
            if let Some(ids) = at.get(&b) {
                if ids.iter().any(|&i| block[i as usize].overlaps(s)) {
                    return false;
                }
            }
        }
        true
    };
    // Lot fronts stay open: no infill within this of a street's edge.
    let front_gap = 1.5 * z;
    let inside_wall = |p: Vec2| wall.is_some_and(|w| point_in(p, w));
    let near_wall = |p: Vec2| {
        wall.is_some_and(|w| {
            (0..w.len()).any(|k| seg_dist(p, w[k], w[(k + 1) % w.len()]) < 8.0 * z)
        })
    };
    // Candidate spots on a jittered grid, nearest the centre first.
    let step = 4.0 * z;
    let n = (core_r / step).ceil() as i32;
    let mut spots: Vec<(f32, Vec2, usize)> = Vec::new();
    for gy in -n..=n {
        for gx in -n..=n {
            let k = seed * 7_368_787 + ((gy + n) * (2 * n + 1) + gx + n) as usize;
            let p = centre
                + Vec2::new(gx as f32 + h01(k, 121) - 0.5, gy as f32 + h01(k, 122) - 0.5) * step;
            let d = (p - centre).length() / core_r;
            // Fewer tries further out, none past the core.
            if d >= 1.0 || h01(k, 123) > amount * (1.0 - smoothstep(0.5, 1.0, d)) {
                continue;
            }
            spots.push((d + 0.15 * h01(k, 124), p, k));
        }
    }
    spots.sort_by(|a, b| a.0.total_cmp(&b.0));
    let roof_of = |k: usize| {
        if upland && h01(k, 127) < 0.6 {
            2.0
        } else if h01(k, 128) < 0.1 + 0.8 * smoothstep(500.0, 3000.0, pop) {
            1.0
        } else {
            0.0
        }
    };
    let fits = |b: &Shape, block: &[Shape], at: &BTreeMap<(i32, i32), Vec<u32>>| -> bool {
        if !b.corners().iter().all(|&q| on_lots(q)) {
            return false;
        }
        let probes = b.probes();
        if !probes
            .iter()
            .all(|&q| water.land(q) && water.river_gap(q) > 2.0)
        {
            return false;
        }
        if wall.is_some() {
            let inside = inside_wall(probes[0]);
            if probes
                .iter()
                .any(|&q| inside_wall(q) != inside || near_wall(q))
            {
                return false;
            }
        }
        space.off_streets(b, front_gap) && clear(block, at, b)
    };
    // Whether a point is taken: off the place's ground, in water, across the
    // wall, by a street, or within a hair of a building or open place.
    let gap = 0.35 * z;
    let blocked = |q: Vec2, from: Vec2, block: &[Shape], at: &BTreeMap<(i32, i32), Vec<u32>>| {
        if !on_lots(q) || !water.land(q) || water.river_gap(q) < 2.0 {
            return true;
        }
        if wall.is_some() && (inside_wall(q) != inside_wall(from) || near_wall(q)) {
            return true;
        }
        let b = ((q.x / BUCKET).floor() as i32, (q.y / BUCKET).floor() as i32);
        if let Some(ids) = space.seg_at.get(&b) {
            if ids.iter().any(|&i| {
                let (a, c, hw) = space.segs[i as usize];
                seg_dist(q, a, c) < hw + front_gap
            }) {
                return true;
            }
        }
        at.get(&b)
            .is_some_and(|ids| ids.iter().any(|&i| block[i as usize].sd(q) < gap))
    };
    // A house grown from seed p, turned to `ang`: in each of eight directions
    // it reaches out until it meets something or its own size, and its four
    // corners take the room found, so it fits its gap, wedge or skewed as the
    // gap is, rather than standing as a box square to the street (R16).
    let grow = |p: Vec2,
                ang: f32,
                k: usize,
                block: &[Shape],
                at: &BTreeMap<(i32, i32), Vec<u32>>|
     -> Option<Shape> {
        if blocked(p, p, block, at) {
            return None;
        }
        let (u, w) = (dir(ang), perp(dir(ang)));
        // Longer than wide (R17): houses, not squares.
        let lu = (5.0 + 6.0 * h01(k, 129)) * z;
        let lw = (2.5 + 2.0 * h01(k, 130)) * z;
        let step = 0.6 * z;
        let reach = |d: Vec2, cap: f32| {
            let mut t = 0.0;
            while t + step <= cap && !blocked(p + d * (t + step), p, block, at) {
                t += step;
            }
            t
        };
        let mut corner = [Vec2::ZERO; 4];
        for (i, (sx, sy)) in [(1.0f32, 1.0f32), (1.0, -1.0), (-1.0, -1.0), (-1.0, 1.0)]
            .into_iter()
            .enumerate()
        {
            let fx = reach(u * sx, lu);
            let fy = reach(w * sy, lw);
            // The corner itself, drawn in towards the seed until it is clear:
            // a corner that meets something leans in, the others stand square.
            let jit = 0.9 + 0.1 * h01(k, 133 + i as u32);
            let (mut x, mut y) = (fx * jit, fy * jit);
            let mut free = false;
            for _ in 0..8 {
                if !blocked(p + u * (sx * x) + w * (sy * y), p, block, at) {
                    free = true;
                    break;
                }
                x *= 0.88;
                y *= 0.88;
            }
            if !free || x < 1.2 * z || y < 1.2 * z {
                return None;
            }
            corner[i] = Vec2::new(sx * x, sy * y);
        }
        // At least 1.6 times as long as wide, along whichever way the gap
        // ran longer: the narrower way is drawn in further (which keeps it
        // clear), so the roof has a ridge rather than a point.
        let ax = corner.iter().map(|q| q.x.abs()).sum::<f32>() / 4.0;
        let ay = corner.iter().map(|q| q.y.abs()).sum::<f32>() / 4.0;
        const LONG: f32 = 1.6;
        if ax >= ay && ax < LONG * ay {
            let f = ax / (LONG * ay);
            corner.iter_mut().for_each(|q| q.y *= f);
        } else if ay > ax && ay < LONG * ax {
            let f = ay / (LONG * ax);
            corner.iter_mut().for_each(|q| q.x *= f);
        }
        if corner
            .iter()
            .any(|q| q.x.abs() < 1.2 * z || q.y.abs() < 1.2 * z)
        {
            return None;
        }
        // Convex, and not too small.
        let mut area = 0.0;
        for i in 0..4 {
            let (a, b, c) = (corner[i], corner[(i + 1) % 4], corner[(i + 2) % 4]);
            let cr = (b - a).x * (c - b).y - (b - a).y * (c - b).x;
            if cr > 0.0 {
                return None;
            }
            area += a.x * b.y - a.y * b.x;
        }
        if area.abs() * 0.5 < 16.0 * z * z {
            return None;
        }
        let world = corner.map(|q| p + u * q.x + w * q.y);
        let mid = (world[0] + world[1] + world[2] + world[3]) * 0.25;
        let hx = corner.iter().map(|q| q.x.abs()).fold(0.0f32, f32::max);
        let hy = corner.iter().map(|q| q.y.abs()).fold(0.0f32, f32::max);
        Some(Shape {
            c: mid,
            h: Vec2::new(hx, hy),
            angle: ang,
            kind: QHOUSE,
            tone: h01(k, 131),
            extra: roof_of(k),
            height: (6.0 + 4.0 * h01(k, 132)) * z,
            q: Some(world),
        })
    };
    let mut added: Vec<Shape> = Vec::new();
    // Try a grown house, shrinking it towards its seed if it grazes anything.
    let mut place =
        |b: Shape, block: &mut Vec<Shape>, at: &mut BTreeMap<(i32, i32), Vec<u32>>| -> bool {
            let mut b = b;
            for _ in 0..3 {
                if fits(&b, block, at) {
                    index(at, &b, block.len());
                    block.push(b);
                    added.push(b);
                    return true;
                }
                let q = b.q.unwrap_or_else(|| b.corners());
                b.q = Some(q.map(|p| b.c + (p - b.c) * 0.85));
            }
            false
        };
    // First by aggregation: each building in the core seeds others against
    // its sides, so the blocks fill from their street fronts inward, as the
    // cores of hand-drawn towns do.
    let mut queue: Vec<usize> = (0..block.len())
        .filter(|&i| {
            let s = &block[i];
            (s.kind == HOUSE || s.kind == BARN) && (s.c - centre).length() < core_r
        })
        .collect();
    queue.sort_by(|&i, &j| {
        (block[i].c - centre)
            .length()
            .total_cmp(&(block[j].c - centre).length())
    });
    let mut qi = 0;
    while qi < queue.len() && qi < 20_000 {
        let base = block[queue[qi]];
        let d = (base.c - centre).length() / core_r;
        let keep = amount * (1.0 - smoothstep(0.45, 1.0, d));
        let (u, w) = base.axes();
        let bc = base.corners();
        for (side, n) in [u, -u, w, -w].into_iter().enumerate() {
            let k = seed * 104_723 + qi * 4 + side;
            if h01(k, 141) > keep {
                continue;
            }
            let reach_n = bc
                .iter()
                .map(|c| (*c - base.c).dot(n))
                .fold(f32::MIN, f32::max);
            let reach_t = bc
                .iter()
                .map(|c| (*c - base.c).dot(perp(n)).abs())
                .fold(0.0f32, f32::max);
            let p = base.c
                + n * (reach_n + gap + 2.0 * z)
                + perp(n) * (h01(k, 144) - 0.5) * 1.6 * reach_t;
            let ang = base.angle
                + if h01(k, 142) < 0.25 {
                    std::f32::consts::FRAC_PI_2
                } else {
                    0.0
                }
                + (h01(k, 143) - 0.5) * 0.4;
            if let Some(b) = grow(p, ang, k, &block, &at) {
                if place(b, &mut block, &mut at) {
                    queue.push(block.len() - 1);
                }
            }
        }
        qi += 1;
    }
    // Then any gaps left, on a jittered grid, nearest the centre first.
    for (_, p, k) in spots {
        // The nearest street, for the building's angle.
        let mut best = (f32::MAX, Vec2::new(1.0, 0.0));
        for b in buckets(p - Vec2::splat(BUCKET), p + Vec2::splat(BUCKET)) {
            if let Some(ids) = space.seg_at.get(&b) {
                for &i in ids {
                    let (a, c, _) = space.segs[i as usize];
                    let d = seg_dist(p, a, c);
                    if d < best.0 {
                        best = (d, (c - a).normalized());
                    }
                }
            }
        }
        if best.0 == f32::MAX {
            continue;
        }
        let ang = best.1.y.atan2(best.1.x)
            + if h01(k, 125) < 0.35 {
                std::f32::consts::FRAC_PI_2
            } else {
                0.0
            }
            + (h01(k, 126) - 0.5) * 0.5;
        if let Some(b) = grow(p, ang, k, &block, &at) {
            place(b, &mut block, &mut at);
        }
    }
    space.shapes.extend(added);
}

/// Whether p lies inside the closed polygon w.
fn point_in(p: Vec2, w: &[Vec2]) -> bool {
    let mut inside = false;
    let n = w.len();
    for k in 0..n {
        let (a, b) = (w[k], w[(k + 1) % n]);
        if (a.y > p.y) != (b.y > p.y) && p.x < a.x + (b.x - a.x) * (p.y - a.y) / (b.y - a.y) {
            inside = !inside;
        }
    }
    inside
}

/// The line of a city's wall round its core plots: the farthest corner in each
/// of 48 directions, with room for a lane inside, smoothed.
fn wall_line(centre: Vec2, core: &[Shape], z: f32) -> Vec<Vec2> {
    const N: usize = 48;
    let mut r = [0.0f32; N];
    for s in core {
        for c in s.corners() {
            let d = c - centre;
            let k = ((d.y.atan2(d.x) / std::f32::consts::TAU + 1.0) * N as f32) as usize % N;
            r[k] = r[k].max(d.length() + 14.0 * z);
        }
    }
    let base = r.iter().copied().fold(0.0f32, f32::max).max(80.0 * z) * 0.5;
    for x in r.iter_mut() {
        if *x == 0.0 {
            *x = base;
        }
    }
    let r: Vec<f32> = (0..N)
        .map(|k| (r[(k + N - 1) % N] + 2.0 * r[k] + r[(k + 1) % N]) / 4.0)
        .map(|x| x.max(base * 0.6))
        .collect();
    let mut pts: Vec<Vec2> = (0..N)
        .map(|k| centre + dir(k as f32 / N as f32 * std::f32::consts::TAU) * r[k])
        .collect();
    // Two rounds of closed corner cutting.
    for _ in 0..2 {
        let n = pts.len();
        let mut out = Vec::with_capacity(2 * n);
        for k in 0..n {
            let (a, b) = (pts[k], pts[(k + 1) % n]);
            out.push(a + (b - a) * 0.25);
            out.push(a + (b - a) * 0.75);
        }
        pts = out;
    }
    pts
}

/// The wall itself: stretches of curtain wall, round towers about every 55 m,
/// a gatehouse where each street passes through, and a ditch outside.
fn build_wall(seed: usize, line: &[Vec2], streets: &[Poly], space: &mut Space, z: f32) {
    let n = line.len();
    // Where streets cross the wall: (index along the wall, point, street tangent).
    // Gatehouses where the roads pass; where a lane meets the wall, a
    // postern: a gap in the wall, without a gatehouse.
    let mut gates: Vec<(Vec2, Vec2)> = Vec::new();
    let mut posterns: Vec<Vec2> = Vec::new();
    for st in streets {
        for w in st.pts.windows(2) {
            for k in 0..n {
                let (a, b) = (line[k], line[(k + 1) % n]);
                if let Some(x) = intersect(w[0], w[1], a, b) {
                    if !st.main {
                        posterns.push(x);
                    } else if gates.iter().all(|g| (g.0 - x).length() > 25.0 * z) {
                        gates.push((x, (w[1] - w[0]).normalized()));
                    }
                }
            }
        }
    }
    let near_gate = |p: Vec2, r: f32| {
        gates.iter().any(|g| (g.0 - p).length() < r)
            || posterns.iter().any(|q| (*q - p).length() < r * 0.5)
    };
    let mut along = 0.0f32;
    let mut next_tower = 30.0 * z;
    for k in 0..n {
        let (a, b) = (line[k], line[(k + 1) % n]);
        let len = (b - a).length();
        let t = (b - a) / len.max(1e-6);
        let mid = (a + b) * 0.5;
        // Outward: away from the polygon's centre.
        let centroid = line.iter().fold(Vec2::ZERO, |s, p| s + *p) / n as f32;
        let out = if perp(t).dot(mid - centroid) > 0.0 {
            perp(t)
        } else {
            -perp(t)
        };
        if !near_gate(mid, 12.0 * z) {
            space.shapes.push(Shape {
                c: mid,
                h: Vec2::new(len * 0.5 + 0.4, 1.3 * z),
                angle: t.y.atan2(t.x),
                kind: WALL,
                tone: h01(seed + k, 81),
                extra: 0.0,
                height: 8.0 * z,
                q: None,
            });
            space.add_shape(Shape {
                c: mid + out * 9.0 * z,
                h: Vec2::new(len * 0.5 + 0.4 + 1.5 * z, 4.5 * z),
                angle: t.y.atan2(t.x),
                kind: DITCH,
                tone: h01(seed + k, 82),
                extra: 0.0,
                height: 0.0,
                q: None,
            });
        }
        along += len;
        if along >= next_tower {
            if !near_gate(b, 20.0 * z) {
                space.shapes.push(Shape {
                    c: b + out * z,
                    h: Vec2::splat(4.2 * z),
                    angle: 0.0,
                    kind: WALL_TOWER,
                    tone: h01(seed + k, 83),
                    extra: 0.0,
                    height: 11.0 * z,
                    q: None,
                });
            }
            next_tower = along + (50.0 + 15.0 * h01(seed + k, 84)) * z;
        }
    }
    for (k, (p, t)) in gates.iter().enumerate() {
        // The gatehouse spans the street, its passage open along it.
        space.shapes.push(Shape {
            c: *p,
            h: Vec2::new(6.0, 7.5) * z,
            angle: t.y.atan2(t.x),
            kind: GATE,
            tone: h01(seed + k, 85),
            extra: 0.0,
            height: 13.0 * z,
            q: None,
        });
    }
}

fn intersect(p1: Vec2, p2: Vec2, q1: Vec2, q2: Vec2) -> Option<Vec2> {
    let r = p2 - p1;
    let s = q2 - q1;
    let den = r.x * s.y - r.y * s.x;
    if den.abs() < 1e-9 {
        return None;
    }
    let t = ((q1 - p1).x * s.y - (q1 - p1).y * s.x) / den;
    let u = ((q1 - p1).x * r.y - (q1 - p1).y * r.x) / den;
    ((0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u)).then(|| p1 + r * t)
}

/// The shapes as textures: the shapes, four texels each (the last two the
/// corners of a four-cornered house, from its middle, R16); per map cell, where its
/// squares start in the square table (-1 if it has none); per square, its
/// entries in the index list, how much of it is built over, and whether it is
/// town land; the index.
pub struct ShapeData {
    pub shp: Vec<f32>,
    pub shp_size: (i32, i32),
    pub cell: Vec<f32>,
    pub sq: Vec<f32>,
    pub sq_size: (i32, i32),
    pub idx: Vec<f32>,
    pub idx_size: (i32, i32),
    /// Squares per cell along a side.
    pub sub: f32,
    pub most: usize,
}

const ROW: usize = super::ways::ROW;
/// Squares of about this many metres index the shapes.
const SQUARE_M: f32 = 62.5;

pub fn layout(t: &Terrain, shapes: &[Shape], z: f32) -> ShapeData {
    let g = t.grid;
    let cell_m = g.cell_km * 1000.0;
    let sub = (cell_m / SQUARE_M).round().max(1.0) as i32;
    let sq_m = cell_m / sub as f32;
    let mut lists: BTreeMap<(i32, i32), Vec<u32>> = BTreeMap::new();
    let mut cover: BTreeMap<(i32, i32), f32> = BTreeMap::new();
    let (wmax, hmax) = (g.w as i32 * sub, g.h as i32 * sub);
    for (i, s) in shapes.iter().enumerate() {
        let (lo, hi) = s.bounds();
        // Reach: the shadow, and for plots the streets beside them (so the
        // shader knows it is in town).
        let m = Vec2::splat(if s.kind == PLOT || s.kind == COURT {
            10.0 * z
        } else {
            2.0 + 0.8 * s.height
        });
        let (x0, y0) = (
            ((lo.x - m.x) / sq_m).floor() as i32,
            ((lo.y - m.y) / sq_m).floor() as i32,
        );
        let (x1, y1) = (
            ((hi.x + m.x) / sq_m).floor() as i32,
            ((hi.y + m.y) / sq_m).floor() as i32,
        );
        for y in y0.max(0)..=y1.min(hmax - 1) {
            for x in x0.max(0)..=x1.min(wmax - 1) {
                lists.entry((x, y)).or_default().push(i as u32);
            }
        }
        // How much of each square the place covers, by points 5 m apart.
        if s.kind < BUILDING && s.kind != DITCH {
            let (u, w) = s.axes();
            let (nu, nw) = ((s.h.x / 2.5).ceil() as i32, (s.h.y / 2.5).ceil() as i32);
            let area = 4.0 * s.h.x * s.h.y / ((2 * nu) as f32 * (2 * nw) as f32);
            for a in -nu..nu {
                for b in -nw..nw {
                    let p = s.c
                        + u * ((a as f32 + 0.5) / nu as f32 * s.h.x)
                        + w * ((b as f32 + 0.5) / nw as f32 * s.h.y);
                    let key = ((p.x / sq_m).floor() as i32, (p.y / sq_m).floor() as i32);
                    *cover.entry(key).or_insert(0.0) += area / (sq_m * sq_m);
                }
            }
        }
    }
    // The town's land: squares with any of a place's ground in them, and the
    // squares round those (about 60 m out). The shader takes the 0.5 line of
    // this, smoothed, as the edge of the town: fields stop there, and within it
    // the land is the place's own closes, so no field runs under a house.
    // Stored as a distance field (1 on the place's squares, falling to 0 three
    // squares out), so its 0.5 line rounds off instead of following squares.
    let mut land: BTreeMap<(i32, i32), f32> = BTreeMap::new();
    for (&(x, y), &c) in &cover {
        if c > 0.03 {
            for dy in -3i32..=3 {
                for dx in -3i32..=3 {
                    let d = ((dx * dx + dy * dy) as f32).sqrt();
                    let v = (1.0 - d / 2.6).max(0.0);
                    if v > 0.0 {
                        let e = land.entry((x + dx, y + dy)).or_insert(0.0);
                        *e = e.max(v);
                    }
                }
            }
        }
    }
    // Cells with any square listed get a block of sub x sub squares.
    let mut cell = vec![-1.0f32; t.n() * 4];
    let mut sq: Vec<f32> = Vec::new();
    let mut idx: Vec<f32> = Vec::new();
    let mut most = 0;
    let cells: std::collections::BTreeSet<(i32, i32)> = lists
        .keys()
        .chain(cover.keys())
        .chain(land.keys())
        .map(|&(x, y)| (x.div_euclid(sub), y.div_euclid(sub)))
        .filter(|&(cx, cy)| cx >= 0 && cy >= 0 && cx < g.w as i32 && cy < g.h as i32)
        .collect();
    for (cx, cy) in cells {
        let ci = g.idx(cx, cy);
        cell[ci * 4] = (sq.len() / 4) as f32;
        for y in 0..sub {
            for x in 0..sub {
                let key = (cx * sub + x, cy * sub + y);
                let list = lists.get(&key).map(|v| v.as_slice()).unwrap_or(&[]);
                let cov = cover.get(&key).copied().unwrap_or(0.0).min(1.0);
                let town = land.get(&key).copied().unwrap_or(0.0);
                sq.extend_from_slice(&[idx.len() as f32, list.len() as f32, cov, town]);
                idx.extend(list.iter().map(|&i| i as f32));
                most = most.max(list.len());
            }
        }
    }
    let mut shp = Vec::with_capacity(shapes.len() * 16);
    for s in shapes {
        let q = s.q.map_or([Vec2::ZERO; 4], |q| q.map(|p| p - s.c));
        // Height in whole metres beside the extra (0..3): extra + 4 x height.
        shp.extend_from_slice(&[
            s.c.x,
            s.c.y,
            s.h.x,
            s.h.y,
            s.angle,
            s.kind,
            s.tone,
            s.extra + 4.0 * s.height.round(),
            q[0].x,
            q[0].y,
            q[1].x,
            q[1].y,
            q[2].x,
            q[2].y,
            q[3].x,
            q[3].y,
        ]);
    }
    let pad = |v: &mut Vec<f32>| -> (i32, i32) {
        let rows = v.len().div_ceil(4).div_ceil(ROW).max(1);
        v.resize(rows * ROW * 4, 0.0);
        (ROW as i32, rows as i32)
    };
    let shp_size = pad(&mut shp);
    let sq_size = pad(&mut sq);
    let idx_size = pad(&mut idx);
    ShapeData {
        shp,
        shp_size,
        cell,
        sq,
        sq_size,
        idx,
        idx_size,
        sub: sub as f32,
        most,
    }
}
