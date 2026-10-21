//! Inspector lookups over a snapshot and terrain: what is under a cell, what a
//! settlement trades, why a site scores what it does. Shared by the viewer and CLI.

use crate::cost::{CostGrid, Dijkstra};
use crate::params::Params;
use crate::snapshot::{SettlementView, Snapshot};
use crate::state::NONE;
use crate::terrain::{Resource, Terrain, Water};

/// The settlement anchored at, or farming, `cell`.
pub fn settlement_at<'a>(snap: &'a Snapshot, cell: usize) -> Option<&'a SettlementView> {
    snap.settlements
        .iter()
        .find(|s| s.anchor as usize == cell)
        .or_else(|| {
            (snap.owner[cell] != NONE)
                .then(|| snap.settlement(snap.owner[cell]))
                .flatten()
        })
}

/// The settlement anchored nearest to `cell` within `radius` cells, for picking.
pub fn settlement_near<'a>(
    snap: &'a Snapshot,
    t: &Terrain,
    cell: usize,
    radius_km: f32,
) -> Option<&'a SettlementView> {
    snap.settlements
        .iter()
        .map(|s| (t.grid.dist_km(s.anchor as usize, cell), s))
        .filter(|(d, _)| *d <= radius_km)
        .min_by(|a, b| a.0.total_cmp(&b.0).then(a.1.id.cmp(&b.1.id)))
        .map(|(_, s)| s)
}

pub fn describe_cell(t: &Terrain, cell: usize) -> String {
    let water = match t.water[cell] {
        Water::Sea => "sea",
        Water::NavigableRiver => "navigable river",
        Water::Stream => "stream",
        Water::None => {
            if t.slope[cell] > 0.08 {
                "steep land"
            } else if t.food_yield[cell] > 0.6 {
                "good farmland"
            } else if t.food_yield[cell] > 0.3 {
                "farmland"
            } else {
                "poor land"
            }
        }
    };
    let mut s = water.to_string();
    if t.ford[cell] {
        s.push_str(", ford");
    }
    if t.harbour[cell] {
        s.push_str(", harbour");
    }
    match t.resource[cell] {
        Resource::None => {}
        Resource::Ore => s.push_str(", ore"),
        Resource::Salt => s.push_str(", salt"),
        Resource::Stone => s.push_str(", stone"),
    }
    s
}

/// A settlement's largest trading partners by flow, both directions: (partner id, flow).
pub fn main_partners(snap: &Snapshot, id: u32, n: usize) -> Vec<(u32, f32)> {
    let mut acc: Vec<(u32, f32)> = Vec::new();
    for f in &snap.flows {
        let other = if f.from == id {
            f.to
        } else if f.to == id {
            f.from
        } else {
            continue;
        };
        match acc.iter_mut().find(|e| e.0 == other) {
            Some(e) => e.1 += f.amount,
            None => acc.push((other, f.amount)),
        }
    }
    acc.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
    acc.truncate(n);
    acc
}

/// Flows whose path passes through `cell`: (from, to, amount, by water here).
pub fn routes_through(
    snap: &Snapshot,
    t: &Terrain,
    cell: usize,
    n: usize,
) -> Vec<(u32, u32, f32, bool)> {
    let cells = t.n() as u32;
    let mut v: Vec<(u32, u32, f32, bool)> = snap
        .flows
        .iter()
        .filter_map(|f| {
            f.path
                .iter()
                .find(|&&node| (node % cells) as usize == cell)
                .map(|&node| (f.from, f.to, f.amount, node >= cells))
        })
        .collect();
    v.sort_by(|a, b| b.2.total_cmp(&a.2));
    v.truncate(n);
    v
}

/// Why a cell scores what it does as a founding site, recomputed from the
/// snapshot: (part, value) pairs, plus travel cost to the nearest settlement.
pub struct SiteReport {
    /// Weighted score parts, without the food term (normalised per step) and noise.
    pub parts: Vec<(&'static str, f32)>,
    pub total: f32,
    /// Free yield within `r_cult`, in prime-cell units.
    pub free_food: f32,
    pub nearest: Option<(u32, f32)>,
}

pub fn site_report(snap: &Snapshot, t: &Terrain, p: &Params, cell: usize) -> SiteReport {
    let g = CostGrid::build(t, &snap.road, &snap.transport);
    let mut dj = Dijkstra::new(t.n());
    dj.run(&g, cell as u32, p.c_max, false);
    let mut shadow = 0.0;
    let mut access = 0.0;
    let mut nearest: Option<(u32, f32)> = None;
    for s in &snap.settlements {
        let c = dj.dist[s.anchor as usize];
        if !c.is_finite() {
            continue;
        }
        if nearest.is_none_or(|n| c < n.1) {
            nearest = Some((s.id, c));
        }
        if crate::systems::founding::in_shadow(t, p, s.anchor as usize, cell, c) {
            shadow += p.shadow_weight * s.pop / 1000.0;
        } else {
            access += p.access_weight * s.pop / 1000.0 * libm::expf(-p.access_decay * c);
        }
    }
    // Free yield reachable on foot within the cultivation radius, in prime cells.
    dj.run(&g, cell as u32, p.r_cult, true);
    let free_food: f32 = dj
        .touched()
        .iter()
        .map(|&n| n as usize)
        .filter(|&c| snap.owner[c] == NONE && t.food_yield[c] >= p.min_field_yield)
        .map(|c| t.food_yield[c])
        .sum();
    let flag = |b: bool| if b { 1.0 } else { 0.0 };
    let parts = vec![
        ("hub", p.w_hub * flag(snap.hub[cell])),
        ("river", p.w_river * flag(t.touches(cell, |j| t.is_nav(j)))),
        ("coast", p.w_coast * flag(t.is_coastal(cell))),
        ("road", p.w_road * flag(t.touches(cell, |j| snap.road[j]))),
        ("shadow", -shadow),
        ("access", access),
    ];
    let total = parts.iter().map(|x| x.1).sum();
    SiteReport {
        parts,
        total,
        free_food,
        nearest,
    }
}
