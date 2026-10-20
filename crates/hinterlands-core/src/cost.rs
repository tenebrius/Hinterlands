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
    /// Neighbour index offset per direction.
    offs: [isize; 8],
}

#[inline]
fn has_water_node(t: &Terrain, i: usize) -> bool {
    matches!(t.water[i], Water::Sea | Water::NavigableRiver)
}

impl CostGrid {
    pub fn build(t: &Terrain, road: &[bool], tr: &Transport) -> Self {
        let g = t.grid;
        let n = g.n();
        let mut land = vec![[INF; 8]; n];
        let mut water = vec![[INF; 8]; n];
        let mut land_to_water = vec![0u8; n];
        let mut water_to_land = vec![0u8; n];
        let mut transship = vec![INF; n];
        let coast_ts = |i: usize| {
            if t.harbour[i] {
                tr.transship
            } else {
                tr.transship * tr.open_coast_factor
            }
        };
        let cross = |j: usize| -> f32 {
            if t.ford[j] {
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
                transship[i] = tr.transship;
            }
            for d in 0..8 {
                let Some(j) = g.neighbor(i, d) else { continue };
                let km = g.step_km(d);
                if t.is_land(i) {
                    if t.is_sea(j) {
                        land[i][d] = coast_ts(i) + tr.sea * km;
                        land_to_water[i] |= 1 << d;
                    } else {
                        let base = if road[i] && road[j] { tr.road } else { tr.land };
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
                    }
                }
                if has_water_node(t, i) {
                    if t.is_sea(i) {
                        if t.is_sea(j) {
                            water[i][d] = tr.sea * km;
                        } else if t.is_nav(j) && t.downstream(j) == Some(i) {
                            water[i][d] = tr.river_up * km;
                        } else {
                            water[i][d] = tr.sea * km + coast_ts(j);
                            water_to_land[i] |= 1 << d;
                        }
                    } else if t.downstream(i) == Some(j) && has_water_node(t, j) {
                        water[i][d] = tr.river_down * km;
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
            offs,
        }
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

    /// Visit every outgoing edge of `node` as (target node, cost).
    #[inline]
    pub fn for_each_edge(&self, node: u32, mut f: impl FnMut(u32, f32)) {
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
}

impl Dijkstra {
    pub fn new(cells: usize) -> Self {
        Self {
            dist: vec![INF; 2 * cells],
            pred: vec![NO_NODE; 2 * cells],
            touched: Vec::new(),
            heap: BinaryHeap::new(),
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
        // Non-negative f32 costs order the same as their bit patterns.
        self.heap.push(Reverse((0f32.to_bits(), src)));
        let n = g.n as u32;
        while let Some(Reverse((dbits, u))) = self.heap.pop() {
            let du = f32::from_bits(dbits);
            if du > self.dist[u as usize] {
                continue;
            }
            let dist = &mut self.dist;
            let pred = &mut self.pred;
            let touched = &mut self.touched;
            let heap = &mut self.heap;
            g.for_each_edge(u, |v, c| {
                if land_only && v >= n {
                    return;
                }
                let nd = du + c;
                if nd <= cap && nd < dist[v as usize] {
                    if dist[v as usize] == INF {
                        touched.push(v);
                    }
                    dist[v as usize] = nd;
                    pred[v as usize] = u;
                    heap.push(Reverse((nd.to_bits(), v)));
                }
            });
        }
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
