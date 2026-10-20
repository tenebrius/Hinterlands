//! Validation metrics, computed from a snapshot so the CLI and the viewer report
//! the same numbers. Definitions are in `docs/validation.md`.

use crate::snapshot::Snapshot;
use crate::terrain::Terrain;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Metrics {
    pub year: u32,
    pub count: u32,
    pub total_pop: f32,
    pub largest: f32,
    pub over_2000: u32,
    pub over_5000: u32,
    pub largest_share: f32,
    /// OLS slope of ln(size) on ln(rank), top 30.
    pub rank_slope: f32,
    /// Median nearest-neighbour distance among settlements over 500, km.
    pub spacing_median_km: f32,
    /// Share of settlements over 2,000 touching a navigable river, the coast or a route crossing.
    pub siting_share: f32,
    /// Settlements founded during the run that have failed, as a share of those founded.
    pub failure_rate: f32,
    pub founded: u32,
    pub failed: u32,
    /// Highest mean forest cover within 5 km of any settlement over 5,000.
    pub forest_near_cities: f32,
}

pub fn rank_slope(sizes: &[f32], top: usize) -> f32 {
    let mut s: Vec<f32> = sizes.iter().copied().filter(|&x| x > 0.0).collect();
    s.sort_by(|a, b| b.total_cmp(a));
    s.truncate(top);
    if s.len() < 3 {
        return f32::NAN;
    }
    let pts: Vec<(f64, f64)> = s
        .iter()
        .enumerate()
        .map(|(i, &x)| (libm::log((i + 1) as f64), libm::log(x as f64)))
        .collect();
    let n = pts.len() as f64;
    let mx = pts.iter().map(|p| p.0).sum::<f64>() / n;
    let my = pts.iter().map(|p| p.1).sum::<f64>() / n;
    let sxy: f64 = pts.iter().map(|p| (p.0 - mx) * (p.1 - my)).sum();
    let sxx: f64 = pts.iter().map(|p| (p.0 - mx) * (p.0 - mx)).sum();
    (sxy / sxx) as f32
}

fn median(mut v: Vec<f32>) -> f32 {
    if v.is_empty() {
        return f32::NAN;
    }
    v.sort_by(|a, b| a.total_cmp(b));
    let m = v.len() / 2;
    if v.len() % 2 == 1 {
        v[m]
    } else {
        0.5 * (v[m - 1] + v[m])
    }
}

/// A road cell where roads leave in three or more directions.
pub fn is_road_crossing(snap: &Snapshot, t: &Terrain, cell: usize) -> bool {
    snap.road[cell]
        && (0..8)
            .filter(|&d| t.grid.neighbor(cell, d).is_some_and(|j| snap.road[j]))
            .count()
            >= 3
}

/// True if a settlement at `cell` sits on a transport advantage.
pub fn well_sited(snap: &Snapshot, t: &Terrain, cell: usize) -> bool {
    t.touches(cell, |j| {
        t.is_nav(j) || t.is_sea(j) || snap.hub[j] || is_road_crossing(snap, t, j)
    })
}

pub fn compute(snap: &Snapshot, t: &Terrain) -> Metrics {
    let g = t.grid;
    let sizes: Vec<f32> = snap.settlements.iter().map(|s| s.pop).collect();
    let total: f32 = sizes.iter().sum();
    let largest = sizes.iter().copied().fold(0.0f32, f32::max);
    let big: Vec<_> = snap.settlements.iter().filter(|s| s.pop > 2000.0).collect();
    let markets: Vec<_> = snap.settlements.iter().filter(|s| s.pop > 500.0).collect();
    let spacing: Vec<f32> = markets
        .iter()
        .filter_map(|a| {
            markets
                .iter()
                .filter(|b| b.id != a.id)
                .map(|b| g.dist_km(a.anchor as usize, b.anchor as usize))
                .min_by(|x, y| x.total_cmp(y))
        })
        .collect();
    let sited = big
        .iter()
        .filter(|s| well_sited(snap, t, s.anchor as usize))
        .count();
    let forest_near = snap
        .settlements
        .iter()
        .filter(|s| s.pop > 5000.0)
        .map(|s| {
            let cells: Vec<usize> = g
                .disk(s.anchor as usize, 5.0)
                .filter(|&c| t.is_land(c))
                .collect();
            cells
                .iter()
                .map(|&c| snap.forest[c] as f32 / 255.0)
                .sum::<f32>()
                / cells.len().max(1) as f32
        })
        .fold(0.0f32, f32::max);
    Metrics {
        year: snap.year,
        count: sizes.len() as u32,
        total_pop: total,
        largest,
        over_2000: big.len() as u32,
        over_5000: sizes.iter().filter(|&&x| x > 5000.0).count() as u32,
        largest_share: if total > 0.0 { largest / total } else { 0.0 },
        rank_slope: rank_slope(&sizes, 30),
        spacing_median_km: median(spacing),
        siting_share: if big.is_empty() {
            f32::NAN
        } else {
            sited as f32 / big.len() as f32
        },
        failure_rate: if snap.tally.founded > 0 {
            snap.tally.failed as f32 / snap.tally.founded as f32
        } else {
            f32::NAN
        },
        founded: snap.tally.founded,
        failed: snap.tally.failed,
        forest_near_cities: forest_near,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zipf_slope_is_minus_one() {
        let sizes: Vec<f32> = (1..=30).map(|r| 10000.0 / r as f32).collect();
        assert!((rank_slope(&sizes, 30) + 1.0).abs() < 1e-4);
    }
}
