//! The travel-cost graph (Tobler 1993; ORBIS mode ratios) and a bounded Dijkstra.
//!
//! The graph has two layers. Every land cell has a land node (`cell`); every sea
//! and navigable-river cell has a water node (`n + cell`). Land edges walk or
//! cart between cells; water edges sail the sea or follow a river's flow links;
//! a transshipment edge joins a river cell's two nodes, and coastal land cells
//! step straight onto the neighbouring sea node. Transshipment is what makes
//! ports, and heads of navigation fall out of where the water layer ends.

use crate::grid::{opposite, Grid, DIRS};
use crate::params::Params;
use crate::terrain::{Terrain, Water};
use serde::{Deserialize, Serialize};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub const INF: f32 = f32::INFINITY;
pub const NO_NODE: u32 = u32::MAX;

/// Transport cost ratios, copied from `Params` into `State` at the start of a run
/// so that a later event or technology can change them mid-run.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transport {
    pub land: f32,
    pub road: f32,
    pub river_down: f32,
    pub river_up: f32,
    pub sea: f32,
    pub river_cross: f32,
    pub stream_cross: f32,
    pub transship: f32,
    pub river_sea_transship: f32,
    pub open_coast_factor: f32,
    pub tobler: bool,
}

impl Transport {
    pub fn from_params(p: &Params) -> Self {
        Self {
            land: p.cost_land,
            road: p.cost_road,
            river_down: p.cost_river_down,
            river_up: p.cost_river_up,
            sea: p.cost_sea,
            river_cross: p.cost_river_cross,
            stream_cross: p.cost_stream_cross,
            transship: p.cost_transship,
            river_sea_transship: p.cost_river_sea_transship,
            open_coast_factor: p.open_coast_factor,
            tobler: p.tobler,
        }
    }
}

/// Tobler's hiking function as a cost multiplier, 1.0 on the flat.
#[inline]
pub fn tobler_factor(slope: f32) -> f32 {
    libm::expf(3.5 * libm::fabsf(slope + 0.05)) / libm::expf(0.175)
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CostGrid {
    pub n: usize,
    /// Edge cost from a cell's land node in each direction (INF = no edge).
    pub land: Vec<[f32; 8]>,
    /// Edge cost from a cell's water node in each direction.
    pub water: Vec<[f32; 8]>,
    /// Bit d set: land edge d lands on the neighbour's water node (coast to sea).
    pub land_to_water: Vec<u8>,
    /// Bit d set: water edge d lands on the neighbour's land node (sea to shore).
    pub water_to_land: Vec<u8>,
    /// Cost between a cell's own land and water nodes (river quays).
    pub transship: Vec<f32>,
    /// From a land node onto the boat of a neighbouring navigable river cell.
    pub board: Vec<[f32; 8]>,
    /// From a river boat onto the bank of a neighbouring land cell.
    pub alight: Vec<[f32; 8]>,
    /// Neighbour index offset per direction.
    offs: [isize; 8],
    /// Compact adjacency of every finite edge: node `u`'s edges are
    /// `adj[adj_start[u]..adj_start[u + 1]]`, as (target node, cost).
    adj_start: Vec<u32>,
    adj: Vec<(u32, f32)>,
    /// Cheapest edge in the graph; the bucket width of the bucket queue.
    min_edge: f32,
}

#[inline]
fn has_water_node(t: &Terrain, i: usize) -> bool {
    matches!(t.water[i], Water::Sea | Water::NavigableRiver)
}

impl CostGrid {
    pub fn build(t: &Terrain, road: &[bool], tr: &Transport) -> Self {
        let q: Vec<f32> = road.iter().map(|&r| if r { 1.0 } else { 0.0 }).collect();
        Self::build_with_condition(t, &q, &[], &[], tr)
    }

    /// As `build`, with a road condition per cell (D54): a step between two
    /// road cells costs `land + q × (road − land)` per km, `q` the lower of the
    /// two conditions; a cell with `q` 0 has no road. `bridge` marks cells the
    /// model has bridged (D55), crossed free as at a ford; `silt` is the extra
    /// cost of every step between land and water at a silted harbour cell
    /// (D60). Either may be empty.
    pub fn build_with_condition(
        t: &Terrain,
        road_q: &[f32],
        bridge: &[bool],
        silt: &[f32],
        tr: &Transport,
    ) -> Self {
        let g = t.grid;
        let n = g.n();
        let mut land = vec![[INF; 8]; n];
        let mut water = vec![[INF; 8]; n];
        let mut land_to_water = vec![0u8; n];
        let mut water_to_land = vec![0u8; n];
        let mut transship = vec![INF; n];
        let mut board = vec![[INF; 8]; n];
        let mut alight = vec![[INF; 8]; n];
        let silted = |i: usize| -> f32 { silt.get(i).copied().unwrap_or(0.0) };
        let coast_ts = |i: usize| {
            silted(i)
                + if t.harbour[i] {
                    tr.transship
                } else {
                    tr.transship * tr.open_coast_factor
                }
        };
        let cross = |j: usize| -> f32 {
            if t.free_crossing(j) || bridge.get(j).copied().unwrap_or(false) {
                0.0
            } else {
                match t.water[j] {
                    Water::NavigableRiver => tr.river_cross,
                    Water::Stream => tr.stream_cross,
                    _ => 0.0,
                }
            }
        };

        for i in 0..n {
            if t.is_nav(i) {
                transship[i] = tr.transship + silted(i);
            }
            for d in 0..8 {
                let Some(j) = g.neighbor(i, d) else { continue };
                let km = g.step_km(d);
                if t.is_land(i) {
                    if t.is_sea(j) {
                        land[i][d] = coast_ts(i) + tr.sea * km;
                        land_to_water[i] |= 1 << d;
                    } else {
                        let q = road_q[i].min(road_q[j]);
                        let base = if q >= 1.0 {
                            tr.road
                        } else if q > 0.0 {
                            tr.land + q * (tr.road - tr.land)
                        } else {
                            tr.land
                        };
                        let mut c = base * km;
                        if tr.tobler {
                            let s =
                                (t.elevation[j].max(0.0) - t.elevation[i].max(0.0)) / (km * 1000.0);
                            c *= tobler_factor(s);
                        }
                        // Crossing a river: entering a river cell from off the river,
                        // or slipping diagonally between two linked river cells.
                        if t.is_river(j) && !(t.is_river(i) && t.flow_linked(i, j)) {
                            c += cross(j);
                        } else if d % 2 == 1 {
                            let (dx, dy) = DIRS[d];
                            let (x, y) = g.xy(i);
                            let a = g.idx(x + dx, y);
                            let b = g.idx(x, y + dy);
                            if t.is_river(a) && t.is_river(b) && t.flow_linked(a, b) {
                                c += cross(a).min(cross(b));
                            }
                        }
                        land[i][d] = c;
                        if t.is_nav(j) {
                            board[i][d] = tr.transship + tr.land * km + silted(i).max(silted(j));
                        }
                    }
                }
                if t.is_nav(i) && t.is_land(j) && !t.is_nav(j) {
                    alight[i][d] = tr.transship + tr.land * km + silted(i).max(silted(j));
                }
                if has_water_node(t, i) {
                    if t.is_sea(i) {
                        if t.is_sea(j) {
                            water[i][d] = tr.sea * km;
                        } else if t.is_nav(j) && t.downstream(j) == Some(i) {
                            // Sea ship to river barge: break of bulk at the mouth.
                            water[i][d] = tr.river_up * km + tr.river_sea_transship;
                        } else {
                            water[i][d] = tr.sea * km + coast_ts(j);
                            water_to_land[i] |= 1 << d;
                        }
                    } else if t.downstream(i) == Some(j) && has_water_node(t, j) {
                        let bulk = if t.is_sea(j) {
                            tr.river_sea_transship
                        } else {
                            0.0
                        };
                        water[i][d] = tr.river_down * km + bulk;
                    } else if t.is_nav(j) && t.downstream(j) == Some(i) {
                        water[i][d] = tr.river_up * km;
                    }
                }
            }
        }
        let w = g.w as isize;
        let mut offs = [0isize; 8];
        for (d, &(dx, dy)) in DIRS.iter().enumerate() {
            offs[d] = dy as isize * w + dx as isize;
        }
        Self {
            n,
            land,
            water,
            land_to_water,
            water_to_land,
            transship,
            board,
            alight,
            offs,
            adj_start: Vec::new(),
            adj: Vec::new(),
            min_edge: 0.0,
        }
        .with_adjacency()
    }

    fn with_adjacency(mut self) -> Self {
        let nodes = 2 * self.n;
        let mut start = Vec::with_capacity(nodes + 1);
        let mut adj = Vec::new();
        for u in 0..nodes as u32 {
            start.push(adj.len() as u32);
            self.for_each_edge_scan(u, |v, c| adj.push((v, c)));
        }
        start.push(adj.len() as u32);
        self.min_edge = adj.iter().map(|e| e.1).fold(INF, f32::min);
        self.adj_start = start;
        self.adj = adj;
        self
    }

    #[inline]
    pub fn node_cell(&self, node: u32) -> usize {
        let node = node as usize;
        if node >= self.n {
            node - self.n
        } else {
            node
        }
    }

    #[inline]
    pub fn is_water_node(&self, node: u32) -> bool {
        node as usize >= self.n
    }

    /// Outgoing edges of `node` as (target node, cost).
    #[inline]
    pub fn edges(&self, node: u32) -> &[(u32, f32)] {
        let u = node as usize;
        &self.adj[self.adj_start[u] as usize..self.adj_start[u + 1] as usize]
    }

    /// Visit every outgoing edge of `node` as (target node, cost).
    #[inline]
    pub fn for_each_edge(&self, node: u32, mut f: impl FnMut(u32, f32)) {
        for &(v, c) in self.edges(node) {
            f(v, c);
        }
    }

    /// Derive a node's edges from the per-direction arrays.
    fn for_each_edge_scan(&self, node: u32, mut f: impl FnMut(u32, f32)) {
        let n = self.n;
        let u = node as usize;
        if u < n {
            let costs = &self.land[u];
            let mask = self.land_to_water[u];
            for d in 0..8 {
                let c = costs[d];
                if c < INF {
                    let j = (u as isize + self.offs[d]) as usize;
                    let t = if mask & (1 << d) != 0 { n + j } else { j };
                    f(t as u32, c);
                }
            }
            let ts = self.transship[u];
            if ts < INF {
                f((n + u) as u32, ts);
            }
            for d in 0..8 {
                let c = self.board[u][d];
                if c < INF {
                    f((n as isize + u as isize + self.offs[d]) as u32, c);
                }
            }
        } else {
            let i = u - n;
            let costs = &self.water[i];
            let mask = self.water_to_land[i];
            for d in 0..8 {
                let c = costs[d];
                if c < INF {
                    let j = (i as isize + self.offs[d]) as usize;
                    let t = if mask & (1 << d) != 0 { j } else { n + j };
                    f(t as u32, c);
                }
            }
            let ts = self.transship[i];
            if ts < INF {
                f(i as u32, ts);
            }
            for d in 0..8 {
                let c = self.alight[i][d];
                if c < INF {
                    f((i as isize + self.offs[d]) as u32, c);
                }
            }
        }
    }
}

/// Reusable buffers for a bounded single-source Dijkstra over the two-layer graph.
#[derive(Clone, Debug, Default)]
pub struct Dijkstra {
    pub dist: Vec<f32>,
    pub pred: Vec<u32>,
    touched: Vec<u32>,
    heap: BinaryHeap<Reverse<(u32, u32)>>,
    buckets: Vec<Vec<(u32, f32)>>,
}

impl Dijkstra {
    pub fn new(cells: usize) -> Self {
        Self {
            dist: vec![INF; 2 * cells],
            pred: vec![NO_NODE; 2 * cells],
            touched: Vec::new(),
            heap: BinaryHeap::new(),
            buckets: Vec::new(),
        }
    }

    fn reset(&mut self) {
        for &u in &self.touched {
            self.dist[u as usize] = INF;
            self.pred[u as usize] = NO_NODE;
        }
        self.touched.clear();
        self.heap.clear();
    }

    /// Nodes reached by the last run, in the order they were first touched.
    pub fn touched(&self) -> &[u32] {
        &self.touched
    }

    /// Run from `src` over nodes within `cap`. `land_only` keeps to the land layer.
    pub fn run(&mut self, g: &CostGrid, src: u32, cap: f32, land_only: bool) {
        if self.dist.len() != 2 * g.n {
            *self = Self::new(g.n);
        }
        self.reset();
        self.dist[src as usize] = 0.0;
        self.touched.push(src);
        let width = g.min_edge;
        if width >= 1e-3 && cap.is_finite() && cap / width < 200_000.0 {
            self.run_buckets(g, cap, land_only, width, src);
        } else {
            self.run_heap(g, cap, land_only, src);
        }
    }

    /// Dial's algorithm: with buckets no wider than the cheapest edge, nodes in
    /// one bucket cannot improve each other, so the result is exact.
    fn run_buckets(&mut self, g: &CostGrid, cap: f32, land_only: bool, width: f32, src: u32) {
        let nb = (cap / width) as usize + 2;
        if self.buckets.len() < nb {
            self.buckets.resize_with(nb, Vec::new);
        }
        let n = g.n as u32;
        let inv = 1.0 / width;
        self.buckets[0].push((src, 0.0));
        let mut b = 0;
        let mut pending = 1usize;
        let mut cur: Vec<(u32, f32)> = Vec::new();
        while pending > 0 && b < nb {
            std::mem::swap(&mut cur, &mut self.buckets[b]);
            pending -= cur.len();
            for &(u, du) in &cur {
                if du != self.dist[u as usize] {
                    continue;
                }
                for &(v, c) in g.edges(u) {
                    if land_only && v >= n {
                        continue;
                    }
                    let nd = du + c;
                    let dv = &mut self.dist[v as usize];
                    if nd <= cap && nd < *dv {
                        if *dv == INF {
                            self.touched.push(v);
                        }
                        *dv = nd;
                        self.pred[v as usize] = u;
                        // Rounding can place `nd` in the current bucket; defer it by one.
                        let k = ((nd * inv) as usize).max(b + 1).min(nb - 1);
                        self.buckets[k].push((v, nd));
                        pending += 1;
                    }
                }
            }
            cur.clear();
            b += 1;
        }
        for bucket in self.buckets.iter_mut().take(nb) {
            bucket.clear();
        }
    }

    fn run_heap(&mut self, g: &CostGrid, cap: f32, land_only: bool, src: u32) {
        // Non-negative f32 costs order the same as their bit patterns.
        self.heap.push(Reverse((0f32.to_bits(), src)));
        let n = g.n as u32;
        while let Some(Reverse((dbits, u))) = self.heap.pop() {
            let du = f32::from_bits(dbits);
            if du > self.dist[u as usize] {
                continue;
            }
            for &(v, c) in g.edges(u) {
                if land_only && v >= n {
                    continue;
                }
                let nd = du + c;
                let dv = &mut self.dist[v as usize];
                if nd <= cap && nd < *dv {
                    if *dv == INF {
                        self.touched.push(v);
                    }
                    *dv = nd;
                    self.pred[v as usize] = u;
                    self.heap.push(Reverse((nd.to_bits(), v)));
                }
            }
        }
    }

    /// Run from `src` with no bound until the first of `targets` is settled;
    /// returns it, or `None` if none is reachable (D1b, D124).
    pub fn run_nearest(&mut self, g: &CostGrid, src: u32, targets: &[u32]) -> Option<u32> {
        if self.dist.len() != 2 * g.n {
            *self = Self::new(g.n);
        }
        self.reset();
        self.dist[src as usize] = 0.0;
        self.touched.push(src);
        self.heap.push(Reverse((0f32.to_bits(), src)));
        let mut found = None;
        while let Some(Reverse((dbits, u))) = self.heap.pop() {
            let du = f32::from_bits(dbits);
            if du > self.dist[u as usize] {
                continue;
            }
            if targets.contains(&u) {
                found = Some(u);
                break;
            }
            for &(v, c) in g.edges(u) {
                let nd = du + c;
                let dv = &mut self.dist[v as usize];
                if nd < *dv {
                    if *dv == INF {
                        self.touched.push(v);
                    }
                    *dv = nd;
                    self.pred[v as usize] = u;
                    self.heap.push(Reverse((nd.to_bits(), v)));
                }
            }
        }
        self.heap.clear();
        found
    }

    /// Node path from the last source to `target`, source first. Empty if unreached.
    pub fn path_to(&self, target: u32) -> Vec<u32> {
        if self.dist[target as usize] == INF {
            return Vec::new();
        }
        let mut path = vec![target];
        let mut u = target;
        while self.pred[u as usize] != NO_NODE {
            u = self.pred[u as usize];
            path.push(u);
        }
        path.reverse();
        path
    }

    /// Cheapest cost to reach a cell on either layer.
    #[inline]
    pub fn cell_cost(&self, n: usize, cell: usize) -> f32 {
        self.dist[cell].min(self.dist[n + cell])
    }
}

/// Direction of travel between two consecutive path nodes, for hub detection:
/// 0..8 for a move between cells, 8 for a change of layer in place.
pub fn step_dir(grid: &Grid, g: &CostGrid, a: u32, b: u32) -> usize {
    let ca = g.node_cell(a);
    let cb = g.node_cell(b);
    if ca == cb {
        8
    } else {
        grid.dir_between(ca, cb).unwrap_or(8)
    }
}

/// Mirror a direction bit for the incoming side of a step.
#[inline]
pub fn back_dir(d: usize) -> usize {
    if d == 8 {
        8
    } else {
        opposite(d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::Grid;
    use crate::terrain::Terrain;

    fn tr() -> Transport {
        Transport::from_params(&Params::default())
    }

    fn cost(t: &Terrain, from: usize, to: u32) -> (f32, Vec<u32>) {
        let g = CostGrid::build(t, &vec![false; t.n()], &tr());
        let mut dj = Dijkstra::new(t.n());
        dj.run(&g, from as u32, 1000.0, false);
        (dj.dist[to as usize], dj.path_to(to))
    }

    /// Sea in column 0; a navigable river along row `ry` flowing west into it.
    fn coast_with_river(w: u32, h: u32, ry: i32) -> Terrain {
        let mut t = Terrain::flat(w, h, 1.0);
        let g = t.grid;
        for y in 0..h as i32 {
            let i = g.idx(0, y);
            t.water[i] = Water::Sea;
            t.elevation[i] = -10.0;
        }
        for x in 1..w as i32 {
            let i = g.idx(x, ry);
            t.water[i] = Water::NavigableRiver;
            t.flow_dir[i] = 4; // west
        }
        t
    }

    #[test]
    fn flat_land_costs_one_per_km() {
        let t = Terrain::flat(10, 3, 1.0);
        let g = t.grid;
        let (c, _) = cost(&t, g.idx(0, 1), g.idx(9, 1) as u32);
        assert!((c - 9.0).abs() < 1e-4, "{c}");
        let t2 = Terrain::flat(10, 3, 2.0);
        let (c2, _) = cost(&t2, g.idx(0, 1), g.idx(9, 1) as u32);
        assert!((c2 - 18.0).abs() < 1e-4, "costs scale with cell_km: {c2}");
    }

    #[test]
    fn tobler_makes_uphill_dearer_than_downhill() {
        let mut t = Terrain::flat(10, 1, 1.0);
        for x in 0..10 {
            t.elevation[x] = 50.0 * x as f32;
        }
        let up = cost(&t, 0, 9).0;
        let down = cost(&t, 9, 0).0;
        assert!(up > down && down < 9.0 && up > 9.0, "up {up} down {down}");
    }

    #[test]
    fn inland_to_coast_prefers_river_and_transships_once() {
        let t = coast_with_river(40, 5, 2);
        let g = t.grid;
        let from = g.idx(38, 1);
        let to = (t.n() + g.idx(0, 2)) as u32; // the sea at the river mouth
        let (c, path) = cost(&t, from, to);
        let n = t.n() as u32;
        let layer_changes = path.windows(2).filter(|w| (w[0] < n) != (w[1] < n)).count();
        assert_eq!(layer_changes, 1, "boards once: {path:?}");
        // Board from the bank (transship + one step), sail down, transfer to a
        // sea ship at the mouth.
        let expected = 3.0 + 1.0 + 0.2 * 38.0 + Params::default().cost_river_sea_transship;
        assert!(
            (c - expected).abs() < 0.3,
            "cost {c}, expected about {expected}"
        );
        assert!(c < 38.0, "the river must beat walking");
    }

    #[test]
    fn upstream_costs_double_downstream() {
        let t = coast_with_river(40, 5, 2);
        let g = t.grid;
        let n = t.n();
        let a = (n + g.idx(30, 2)) as u32;
        let b = (n + g.idx(10, 2)) as u32;
        let gr = CostGrid::build(&t, &vec![false; n], &tr());
        let mut dj = Dijkstra::new(n);
        dj.run(&gr, a, 1000.0, false);
        let down = dj.dist[b as usize];
        dj.run(&gr, b, 1000.0, false);
        let up = dj.dist[a as usize];
        assert!(
            (down - 4.0).abs() < 1e-3 && (up - 8.0).abs() < 1e-3,
            "down {down} up {up}"
        );
    }

    #[test]
    fn crossing_a_river_costs_unless_at_a_ford() {
        let mut t = coast_with_river(12, 5, 2);
        let g = t.grid;
        let (from, to) = (g.idx(6, 0), g.idx(6, 4) as u32);
        let walk = cost(&t, from, to).0;
        assert!(walk > 4.0 + 4.0, "crossing pays the fixed cost: {walk}");
        t.ford[g.idx(6, 2)] = true;
        let ford = cost(&t, from, to).0;
        assert!((ford - 4.0).abs() < 1e-3, "free at a ford: {ford}");
    }

    #[test]
    fn crossing_is_free_at_a_bridge() {
        let mut t = coast_with_river(12, 5, 2);
        let g = t.grid;
        let (from, to) = (g.idx(6, 0), g.idx(6, 4) as u32);
        t.bridge = vec![false; t.n()];
        t.bridge[g.idx(6, 2)] = true;
        let bridge = cost(&t, from, to).0;
        assert!((bridge - 4.0).abs() < 1e-3, "free at a bridge: {bridge}");
    }

    #[test]
    fn harbours_transship_cheaper_than_open_coast() {
        let mut t = Terrain::flat(6, 3, 1.0);
        let g = t.grid;
        for y in 0..3 {
            let i = g.idx(0, y);
            t.water[i] = Water::Sea;
        }
        let from = g.idx(1, 1);
        let to = (t.n() + g.idx(0, 1)) as u32;
        let open = cost(&t, from, to).0;
        t.harbour[from] = true;
        let harbour = cost(&t, from, to).0;
        assert!(
            (harbour - (3.0 + 0.08)).abs() < 1e-4 && (open - (6.0 + 0.08)).abs() < 1e-4,
            "{harbour} {open}"
        );
    }

    #[test]
    fn bucket_queue_matches_heap() {
        let mut t = coast_with_river(30, 12, 5);
        for i in 0..t.n() {
            let (x, y) = t.grid.xy(i);
            if t.is_land(i) {
                t.elevation[i] = 10.0 + ((x * 37 + y * 91) % 23) as f32 * 7.0;
            }
        }
        let g = CostGrid::build(&t, &vec![false; t.n()], &tr());
        let (mut a, mut b) = (Dijkstra::new(t.n()), Dijkstra::new(t.n()));
        a.run(&g, 40, 50.0, false);
        b.reset();
        b.dist[40] = 0.0;
        b.touched.push(40);
        b.run_heap(&g, 50.0, false, 40);
        for i in 0..2 * t.n() {
            assert!(
                (a.dist[i] - b.dist[i]).abs() < 1e-4
                    || (a.dist[i].is_infinite() && b.dist[i].is_infinite()),
                "node {i}"
            );
        }
    }

    #[test]
    fn dijkstra_respects_the_cap() {
        let t = Terrain::flat(50, 1, 1.0);
        let gr = CostGrid::build(&t, &vec![false; t.n()], &tr());
        let mut dj = Dijkstra::new(t.n());
        dj.run(&gr, 0, 10.0, false);
        assert!(dj.dist[10] <= 10.0 && dj.dist[11].is_infinite());
        let _ = Grid::new(1, 1, 1.0);
    }
}
