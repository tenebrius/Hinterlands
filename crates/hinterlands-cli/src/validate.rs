//! The validation table from the foundation document, run headless over several
//! maps and seeds in parallel. Definitions are in `docs/validation.md`. A check
//! passes when it holds in at least 75% of seeds on every map; map 1 seed 1 is
//! the determinism golden.

use crate::{load_map_params, load_params, Args};
use hinterlands_core::metrics::{self, Metrics};
use hinterlands_core::{Sim, Terrain};
use hinterlands_mapgen::generate;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

pub struct RunResult {
    pub map_seed: u64,
    pub sim_seed: u64,
    pub m: Metrics,
    /// Largest year-on-year change in total population after year 100.
    pub max_change: f32,
    pub largest_anchor: Option<usize>,
    /// Anchors of the ten largest settlements, largest first.
    pub top10: Vec<usize>,
    /// A settlement over 5,000 on the main river corridor or the coast within 15 km of it.
    pub corridor_city: bool,
    /// Distance of the largest settlement from the main river mouth, km (reported only).
    pub largest_mouth_km: f32,
    pub ms_per_year: f64,
    pub hash: u64,
    pub median_pop: f32,
    /// v2 (goods on; neutral values with goods off).
    pub goods: GoodsResult,
    /// v3 (traffic roads, D54): change in road length from year 300 to the
    /// last year, as a share; NaN with traffic roads off.
    pub road_change: f32,
    /// Share of cells ever road that are no longer road at the end.
    pub road_abandoned: f32,
    /// v3 (D55, D56): bridges built; share of places of 1,000 or more, and of
    /// all settlements, within 2 km of one; share of settlements with a mill.
    pub bridges: u32,
    pub near_bridge_big: f32,
    pub near_bridge_all: f32,
    pub mill_share: f32,
    /// v4 (D57), with historical events; `None` with events off.
    pub shock: Option<ShockResult>,
}

/// The v4 gate's measures for one run with historical events.
#[derive(Clone, Debug)]
pub struct ShockResult {
    /// Largest fall in total population within five years of the plague, as a
    /// share of the total the year before it.
    pub plague_drop: f32,
    /// Total at the end against the highest total before the plague.
    pub recovery: f32,
    /// Failure rates, years plague..plague+52, of the settlements alive the
    /// year before the plague in the lowest and the highest quarter of local
    /// yield.
    pub fail_low: f32,
    pub fail_high: f32,
    /// Deepest fall of the total during the famine and the years after, as a
    /// share, and the years until the pre-famine total is regained.
    pub famine_dip: f32,
    pub famine_years: f32,
    /// Share of this run's top ten within 5 km of the same seed's top ten with
    /// events off; filled in by `cmd_validate`.
    pub overlap_off: f32,
}

/// The v2 gate's measures at the final year.
#[derive(Clone, Debug)]
pub struct GoodsResult {
    /// Share of resource sites that sold anything (1 with goods off or no sites).
    pub sites_share: f32,
    pub largest_specialist: bool,
    pub specialist_pops: Vec<f32>,
    /// Payment grain arriving at producers as a share of all market inflow.
    pub payment_share: f32,
    /// Mean forest within 5 km of working iron sites, and on land.
    pub iron_forest: (f32, f32),
    /// Share of settlements over 2,000 on or next to a route carrying goods.
    pub goods_route_share: f32,
}

fn goods_result(sim: &Sim) -> GoodsResult {
    use hinterlands_core::state::{Good, Kind};
    let (st, t, p) = (&sim.state, &sim.terrain, &sim.params);
    let neutral = GoodsResult {
        sites_share: 1.0,
        largest_specialist: false,
        specialist_pops: Vec::new(),
        payment_share: f32::NAN,
        iron_forest: (f32::NAN, f32::NAN),
        goods_route_share: f32::NAN,
    };
    if !p.goods_enabled {
        return neutral;
    }
    let sites_share = if st.sites.is_empty() {
        1.0
    } else {
        st.sites.iter().filter(|s| s.sold > 0.0).count() as f32 / st.sites.len() as f32
    };
    let largest_specialist = st
        .settlements
        .iter()
        .max_by(|a, b| a.pop.total_cmp(&b.pop))
        .is_some_and(|s| s.kind != Kind::Agrarian);
    let specialist_pops = st
        .settlements
        .iter()
        .filter(|s| s.kind != Kind::Agrarian)
        .map(|s| s.pop)
        .collect();
    let mut arrived = 0.0f32;
    let mut on_goods = vec![false; t.n()];
    for f in &st.flows {
        let Some(s) = st.get(f.from) else { continue };
        let q = &s.reach.partners[f.partner_idx as usize];
        if f.payment {
            arrived += f.amount * (1.0 - p.transport_loss * q.cost).max(0.0);
        }
        if f.good.is_some() {
            for &node in &q.path {
                on_goods[st.cost.node_cell(node)] = true;
            }
        }
    }
    let market: f32 = st.settlements.iter().map(|s| s.market_in).sum();
    let land: Vec<usize> = (0..t.n()).filter(|&i| t.is_land(i)).collect();
    let land_mean = land.iter().map(|&i| st.forest[i]).sum::<f32>() / land.len().max(1) as f32;
    let near: Vec<usize> = st
        .sites
        .iter()
        .filter(|s| s.good == Good::Iron && s.sold > 0.0)
        .flat_map(|s| t.grid.disk(s.cell as usize, 5.0).filter(|&j| t.is_land(j)))
        .collect();
    let iron = if near.is_empty() {
        f32::NAN
    } else {
        near.iter().map(|&j| st.forest[j]).sum::<f32>() / near.len() as f32
    };
    let big: Vec<_> = st.settlements.iter().filter(|s| s.pop > 2000.0).collect();
    let goods_route_share = if big.is_empty() {
        f32::NAN
    } else {
        big.iter()
            .filter(|s| t.touches(s.anchor as usize, |j| on_goods[j]))
            .count() as f32
            / big.len() as f32
    };
    GoodsResult {
        sites_share,
        largest_specialist,
        specialist_pops,
        payment_share: if market > 0.0 {
            arrived / market
        } else {
            f32::NAN
        },
        iron_forest: (iron, land_mean),
        goods_route_share,
    }
}

pub struct Check {
    pub name: &'static str,
    pub target: &'static str,
    pub ok: fn(&RunResult) -> bool,
}

pub const CHECKS: &[Check] = &[
    Check {
        name: "settlements",
        target: "150-350",
        ok: |r| (150..=350).contains(&r.m.count),
    },
    Check {
        name: "over 2,000",
        target: "5-12",
        ok: |r| (5..=12).contains(&r.m.over_2000),
    },
    Check {
        name: "over 5,000",
        target: "1-5",
        ok: |r| (1..=5).contains(&r.m.over_5000),
    },
    Check {
        name: "largest share",
        target: "<0.25",
        ok: |r| r.m.largest_share < 0.25,
    },
    Check {
        name: "rank-size slope",
        target: "-1.3..-0.8",
        ok: |r| (-1.3..=-0.8).contains(&r.m.rank_slope),
    },
    Check {
        name: "market spacing",
        target: "8-15 km",
        ok: |r| (8.0..=15.0).contains(&r.m.spacing_median_km),
    },
    Check {
        name: "siting",
        target: ">=0.70",
        ok: |r| r.m.siting_share >= 0.7,
    },
    Check {
        name: "failure rate",
        target: "0.40-0.70",
        ok: |r| (0.4..=0.7).contains(&r.m.failure_rate),
    },
    Check {
        name: "forest near cities",
        target: "<0.30",
        ok: |r| r.m.forest_near_cities < 0.3,
    },
    Check {
        name: "stability",
        target: "<5%/yr",
        ok: |r| r.max_change < 0.05,
    },
    // v2 gate (pass trivially with goods off).
    Check {
        name: "sites producing",
        target: ">=0.67",
        ok: |r| r.goods.sites_share >= 2.0 / 3.0,
    },
    Check {
        name: "largest not specialist",
        target: "true",
        ok: |r| !r.goods.largest_specialist,
    },
    // v4 gate (D57; passes trivially with events off).
    Check {
        name: "plague drop",
        target: ">=25%",
        ok: |r| r.shock.as_ref().is_none_or(|s| s.plague_drop >= 0.25),
    },
    Check {
        name: "recovery by 500",
        target: ">=0.90",
        ok: |r| r.shock.as_ref().is_none_or(|s| s.recovery >= 0.9),
    },
    Check {
        name: "poor land deserted",
        target: "low>high",
        ok: |r| r.shock.as_ref().is_none_or(|s| s.fail_low > s.fail_high),
    },
    Check {
        name: "top-10 vs off",
        target: ">=0.50",
        ok: |r| r.shock.as_ref().is_none_or(|s| s.overlap_off >= 0.5),
    },
    // v3 gate (D54; passes trivially with traffic roads off).
    Check {
        name: "road length 300-500",
        target: "<10%",
        ok: |r| r.road_change.is_nan() || r.road_change < 0.10,
    },
];

pub fn run_one(
    t: &Terrain,
    params: &hinterlands_core::Params,
    map_seed: u64,
    sim_seed: u64,
    years: u32,
) -> RunResult {
    let mut sim = Sim::new(t.clone(), params.clone(), sim_seed);
    let t0 = Instant::now();
    let mut prev = sim.state.total_pop();
    let mut max_change = 0f32;
    let mut ever = sim.state.road.clone();
    let mut road_300 = f32::NAN;
    let road_len = |s: &Sim| s.state.road.iter().filter(|&&r| r).count() as f32;
    let shocks = params.events_mode == 1;
    let mut totals: Vec<f32> = vec![sim.state.total_pop()];
    let mut quartile: std::collections::BTreeMap<u32, u8> = Default::default();
    let mut failed: std::collections::BTreeSet<u32> = Default::default();
    for _ in 0..years {
        if shocks && sim.state.year == params.plague_year {
            quartile = yield_quarters(&sim);
        }
        sim.step(1);
        if shocks {
            totals.push(sim.state.total_pop());
            let y = sim.state.year;
            if y > params.plague_year && y <= params.plague_year + 52 {
                for e in &sim.state.log {
                    if matches!(e.kind, hinterlands_core::state::LogKind::Failed { .. }) {
                        failed.insert(e.settlement);
                    }
                }
            }
        }
        for (e, &r) in ever.iter_mut().zip(&sim.state.road) {
            *e |= r;
        }
        if sim.state.year == 300 {
            road_300 = road_len(&sim);
        }
        let tot = sim.state.total_pop();
        // With events, the event years and the one after are not steady state
        // (D58).
        let shocked = shocks && {
            let y = sim.state.year;
            let fy = params.famine_year;
            let in_famine = y >= fy && y <= fy + params.famine_years + 1;
            let in_plague = [params.plague_year, params.plague_recur_1, params.plague_recur_2]
                .iter()
                .any(|&py| y >= py && y <= py + params.plague_years + 1);
            in_famine || in_plague
        };
        if sim.state.year > 100 && prev > 0.0 && !shocked {
            max_change = max_change.max((tot - prev).abs() / prev);
        }
        prev = tot;
    }
    let ms_per_year = t0.elapsed().as_secs_f64() * 1000.0 / years.max(1) as f64;
    let snap = sim.snapshot();
    let m = metrics::compute(&snap, t);
    let largest_anchor = snap
        .settlements
        .iter()
        .max_by(|a, b| a.pop.total_cmp(&b.pop))
        .map(|s| s.anchor as usize);
    let mut by_size: Vec<_> = snap.settlements.iter().collect();
    by_size.sort_by(|a, b| b.pop.total_cmp(&a.pop).then(a.id.cmp(&b.id)));
    let top10 = by_size.iter().take(10).map(|s| s.anchor as usize).collect();
    let corridor_city = metrics::corridor_city(&snap, t, &metrics::main_river_corridor(t));
    let largest_mouth_km = match (largest_anchor, t.river_mouth) {
        (Some(a), Some(m)) => t.grid.dist_km(a, m as usize),
        _ => f32::NAN,
    };
    let mut pops: Vec<f32> = snap.settlements.iter().map(|s| s.pop).collect();
    pops.sort_by(|a, b| a.total_cmp(b));
    let median_pop = pops.get(pops.len() / 2).copied().unwrap_or(0.0);
    RunResult {
        map_seed,
        sim_seed,
        m,
        max_change,
        largest_anchor,
        top10,
        corridor_city,
        largest_mouth_km,
        ms_per_year,
        hash: snap.hash(),
        median_pop,
        goods: goods_result(&sim),
        shock: shocks.then(|| shock_result(params, &totals, &quartile, &failed)),
        road_change: if params.traffic_roads {
            (road_len(&sim) - road_300).abs() / road_300.max(1.0)
        } else {
            f32::NAN
        },
        bridges: snap.bridges.len() as u32,
        near_bridge_big: near_bridge(&snap, t, 1000.0),
        near_bridge_all: near_bridge(&snap, t, 0.0),
        mill_share: snap.settlements.iter().filter(|s| s.mill).count() as f32
            / snap.settlements.len().max(1) as f32,
        road_abandoned: {
            let ever_n = ever.iter().filter(|&&e| e).count();
            let gone = ever
                .iter()
                .zip(&sim.state.road)
                .filter(|(&e, &r)| e && !r)
                .count();
            gone as f32 / ever_n.max(1) as f32
        },
    }
}

/// The quarter (0 lowest .. 3 highest) of each settlement's local yield: the
/// mean yield of the land within 3 km of its anchor (D58).
fn yield_quarters(sim: &Sim) -> std::collections::BTreeMap<u32, u8> {
    let t = &sim.terrain;
    let mut v: Vec<(f32, u32)> = sim
        .state
        .settlements
        .iter()
        .map(|s| {
            let near: Vec<usize> = t
                .grid
                .disk(s.anchor as usize, 3.0)
                .filter(|&j| t.is_land(j))
                .collect();
            let y: f32 = near.iter().map(|&j| t.food_yield[j]).sum();
            (y / near.len().max(1) as f32, s.id)
        })
        .collect();
    v.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
    let n = v.len().max(1);
    v.iter()
        .enumerate()
        .map(|(k, &(_, id))| (id, (k * 4 / n) as u8))
        .collect()
}

fn shock_result(
    p: &hinterlands_core::Params,
    totals: &[f32],
    quartile: &std::collections::BTreeMap<u32, u8>,
    failed: &std::collections::BTreeSet<u32>,
) -> ShockResult {
    let at = |y: u32| totals.get(y as usize).copied().unwrap_or(f32::NAN);
    let py = p.plague_year;
    let before = at(py);
    let low = (py + 1..=py + 5).map(at).fold(f32::INFINITY, f32::min);
    let peak = (0..=py).map(at).fold(0.0f32, f32::max);
    let rate = |q: u8| {
        let ids: Vec<u32> = quartile
            .iter()
            .filter(|(_, &k)| k == q)
            .map(|(&id, _)| id)
            .collect();
        ids.iter().filter(|id| failed.contains(id)).count() as f32 / ids.len().max(1) as f32
    };
    let fy = p.famine_year;
    let pre = at(fy);
    let dip_low = (fy + 1..=fy + p.famine_years + 5).map(at).fold(f32::INFINITY, f32::min);
    let back = (fy + 1..py)
        .find(|&y| at(y) >= pre)
        .map_or(f32::NAN, |y| (y - fy) as f32);
    ShockResult {
        plague_drop: 1.0 - low / before.max(1.0),
        recovery: totals.last().copied().unwrap_or(0.0) / peak.max(1.0),
        fail_low: rate(0),
        fail_high: rate(3),
        famine_dip: 1.0 - dip_low / pre.max(1.0),
        famine_years: back,
        overlap_off: f32::NAN,
    }
}

/// Share of settlements of at least `min_pop` within 2 km of a model bridge.
fn near_bridge(snap: &hinterlands_core::Snapshot, t: &Terrain, min_pop: f32) -> f32 {
    let places: Vec<_> = snap.settlements.iter().filter(|s| s.pop >= min_pop).collect();
    let near = places
        .iter()
        .filter(|s| {
            snap.bridges
                .iter()
                .any(|&b| t.grid.dist_km(s.anchor as usize, b as usize) <= 2.0)
        })
        .count();
    near as f32 / places.len().max(1) as f32
}

fn passes(r: &RunResult) -> bool {
    CHECKS.iter().all(|c| (c.ok)(r))
}

pub fn cmd_validate(args: &Args) {
    let maps = args.u64("maps", 3);
    let seeds = args.u64("seeds", 20);
    let years = args.u64("years", 500) as u32;
    let first_map = args.u64("map-seed", 1);
    let threads = args.u64(
        "threads",
        std::thread::available_parallelism().map_or(4, |n| n.get()) as u64,
    ) as usize;
    let params = load_params(args);
    let mp = load_map_params(args);

    let terrains: Vec<(u64, Terrain)> = (first_map..first_map + maps)
        .map(|s| (s, generate(s, &mp)))
        .collect();
    let jobs: Vec<(usize, u64)> = (0..terrains.len())
        .flat_map(|m| (1..=seeds).map(move |s| (m, s)))
        .collect();
    let next = AtomicUsize::new(0);
    let results = Mutex::new(Vec::new());
    std::thread::scope(|scope| {
        for _ in 0..threads.max(1) {
            scope.spawn(|| loop {
                let j = next.fetch_add(1, Ordering::Relaxed);
                let Some(&(mi, seed)) = jobs.get(j) else {
                    break;
                };
                let (map_seed, t) = &terrains[mi];
                let mut r = run_one(t, &params, *map_seed, seed, years);
                // With events, the same seed without them, for the overlap.
                if r.shock.is_some() {
                    let mut off = params.clone();
                    off.events_mode = 0;
                    let o = run_one(t, &off, *map_seed, seed, years);
                    let near = r
                        .top10
                        .iter()
                        .filter(|&&x| o.top10.iter().any(|&y| t.grid.dist_km(x, y) <= 5.0))
                        .count();
                    let share = near as f32 / r.top10.len().max(1) as f32;
                    if let Some(sh) = r.shock.as_mut() {
                        sh.overlap_off = share;
                    }
                }
                results.lock().unwrap().push(r);
            });
        }
    });
    let mut results = results.into_inner().unwrap();
    results.sort_by_key(|r| (r.map_seed, r.sim_seed));

    // Determinism: rerun the canonical run of the first map.
    let det = run_one(&terrains[0].1, &params, terrains[0].0, 1, years);
    let det_ok = results
        .iter()
        .find(|r| r.map_seed == terrains[0].0 && r.sim_seed == 1)
        .is_some_and(|r| r.hash == det.hash);

    let mut all_ok = det_ok;
    println!(
        "determinism (map {}, seed 1): {}",
        terrains[0].0,
        if det_ok { "ok" } else { "FAIL" }
    );
    for (map_seed, t) in &terrains {
        let rs: Vec<&RunResult> = results.iter().filter(|r| r.map_seed == *map_seed).collect();
        println!("\nmap {map_seed}: {} runs of {years} years", rs.len());
        println!(
            "{:<20} {:>12} {:>10} {:>10} {:>10} {:>8}",
            "check", "target", "seed 1", "median", "pass", ""
        );
        let canon = rs.iter().find(|r| r.sim_seed == 1).copied();
        for c in CHECKS {
            let pass = rs.iter().filter(|r| (c.ok)(r)).count();
            let value = |r: &RunResult| -> f32 {
                match c.name {
                    "settlements" => r.m.count as f32,
                    "over 2,000" => r.m.over_2000 as f32,
                    "over 5,000" => r.m.over_5000 as f32,
                    "largest share" => r.m.largest_share,
                    "rank-size slope" => r.m.rank_slope,
                    "market spacing" => r.m.spacing_median_km,
                    "siting" => r.m.siting_share,
                    "failure rate" => r.m.failure_rate,
                    "forest near cities" => r.m.forest_near_cities,
                    "sites producing" => r.goods.sites_share,
                    "largest not specialist" => {
                        if r.goods.largest_specialist {
                            0.0
                        } else {
                            1.0
                        }
                    }
                    "road length 300-500" => r.road_change,
                    "plague drop" => r.shock.as_ref().map_or(f32::NAN, |s| s.plague_drop),
                    "recovery by 500" => r.shock.as_ref().map_or(f32::NAN, |s| s.recovery),
                    "poor land deserted" => r
                        .shock
                        .as_ref()
                        .map_or(f32::NAN, |s| s.fail_low - s.fail_high),
                    "top-10 vs off" => r.shock.as_ref().map_or(f32::NAN, |s| s.overlap_off),
                    _ => r.max_change,
                }
            };
            let mut vals: Vec<f32> = rs.iter().map(|r| value(r)).collect();
            vals.sort_by(|a, b| a.total_cmp(b));
            let med = vals.get(vals.len() / 2).copied().unwrap_or(f32::NAN);
            // v1 rule (D21): each check passes in at least 75% of the seeds on every map.
            let rate_ok = pass * 4 >= rs.len() * 3;
            println!(
                "{:<20} {:>12} {:>10.3} {:>10.3} {:>7}/{:<2} {:>8}",
                c.name,
                c.target,
                canon.map_or(f32::NAN, |r| value(r)),
                med,
                pass,
                rs.len(),
                if rate_ok { "ok" } else { "FAIL" }
            );
            all_ok &= rate_ok;
        }
        let full = rs.iter().filter(|r| passes(r)).count();
        let anchors: Vec<usize> = rs.iter().filter_map(|r| r.largest_anchor).collect();
        let modal = anchors
            .iter()
            .map(|&a| {
                anchors
                    .iter()
                    .filter(|&&b| t.grid.dist_km(a, b) <= 5.0)
                    .count()
            })
            .max()
            .unwrap_or(0);
        let differs = 1.0 - modal as f32 / anchors.len().max(1) as f32;
        let corridor =
            rs.iter().filter(|r| r.corridor_city).count() as f32 / rs.len().max(1) as f32;
        // Contingency is gated on the middle tier (D35): the ten largest must
        // differ between seeds. The varying largest city (gated until v1.2) and
        // the river-corridor share (D30) are reported.
        let overlap = top10_overlap(t, &rs);
        let contingency_ok = overlap <= 0.8;
        let mut over5: Vec<u32> = rs.iter().map(|r| r.m.over_5000).collect();
        over5.sort();
        let over5_median = over5.get(over5.len() / 2).copied().unwrap_or(0);
        // Reported, not gated, since v2 (D48); the per-seed 1–5 check stays gated.
        let over5_ok = true;
        let mut mouth_km: Vec<f32> = rs.iter().map(|r| r.largest_mouth_km).collect();
        mouth_km.sort_by(|a, b| a.total_cmp(b));
        let ms = rs.iter().map(|r| r.ms_per_year).fold(0.0, f64::max);
        println!(
            "{:<20} {:>12} {:>10} {:>10.2} {:>10} {:>8}",
            "top-10 overlap",
            "<=.80",
            "",
            overlap,
            "",
            if contingency_ok { "ok" } else { "FAIL" }
        );
        println!(
            "{:<20} {:>12} {:>10} {:>10.2} {:>10} {:>8}",
            "largest varies", "(reported)", "", differs, "", ""
        );
        println!(
            "{:<20} {:>12} {:>10} {:>10.2} {:>10} {:>8}",
            "river corridor city", "(reported)", "", corridor, "", ""
        );
        println!(
            "{:<20} {:>12} {:>10} {:>10} {:>10} {:>8}",
            "over 5,000 median", "(reported)", "", over5_median, "", ""
        );
        println!(
            "{:<20} {:>12} {:>10} {:>10.1} {:>10} {:>8}",
            "largest to mouth km",
            "(reported)",
            "",
            mouth_km
                .get(mouth_km.len() / 2)
                .copied()
                .unwrap_or(f32::NAN),
            "",
            ""
        );
        if params_goods(&rs) {
            let med = |mut v: Vec<f32>| {
                v.retain(|x| x.is_finite());
                v.sort_by(|a, b| a.total_cmp(b));
                v.get(v.len() / 2).copied().unwrap_or(f32::NAN)
            };
            let mut pops: Vec<f32> = rs
                .iter()
                .flat_map(|r| r.goods.specialist_pops.clone())
                .collect();
            pops.sort_by(|a, b| a.total_cmp(b));
            let q = |f: f32| {
                pops.get(((pops.len() as f32 - 1.0) * f) as usize)
                    .copied()
                    .unwrap_or(f32::NAN)
            };
            println!(
                "{:<20} {:>12} specialists per run {:.1}; size p10 {:.0}, median {:.0}, p90 {:.0}, max {:.0}",
                "v2 (reported)",
                "",
                pops.len() as f32 / rs.len().max(1) as f32,
                q(0.1),
                q(0.5),
                q(0.9),
                pops.last().copied().unwrap_or(f32::NAN)
            );
            println!(
                "{:<20} {:>12} payment share of market inflow {:.3}; iron forest {:.2} vs land {:.2}; >2k on goods route {:.2}",
                "",
                "",
                med(rs.iter().map(|r| r.goods.payment_share).collect()),
                med(rs.iter().map(|r| r.goods.iron_forest.0).collect()),
                med(rs.iter().map(|r| r.goods.iron_forest.1).collect()),
                med(rs.iter().map(|r| r.goods.goods_route_share).collect())
            );
        }
        if rs.iter().any(|r| !r.road_change.is_nan()) {
            let mut v: Vec<f32> = rs.iter().map(|r| r.road_abandoned).collect();
            v.sort_by(|a, b| a.total_cmp(b));
            println!(
                "{:<20} {:>12} share of road cells abandoned: median {:.2}, max {:.2}",
                "v3 roads (reported)",
                "",
                v[v.len() / 2],
                v[v.len() - 1]
            );
        }
        let med = |mut v: Vec<f32>| {
            v.sort_by(|a, b| a.total_cmp(b));
            v.get(v.len() / 2).copied().unwrap_or(f32::NAN)
        };
        if rs.iter().any(|r| r.shock.is_some()) {
            let sh: Vec<&ShockResult> = rs.iter().filter_map(|r| r.shock.as_ref()).collect();
            println!(
                "{:<20} {:>12} plague drop {:.2}; recovery {:.2}; failure 248-300 low {:.2} / high {:.2} quarter; famine dip {:.2}, regained in {:.0} years",
                "v4 shocks",
                "(reported)",
                med(sh.iter().map(|s| s.plague_drop).collect()),
                med(sh.iter().map(|s| s.recovery).collect()),
                med(sh.iter().map(|s| s.fail_low).collect()),
                med(sh.iter().map(|s| s.fail_high).collect()),
                med(sh.iter().map(|s| s.famine_dip).collect()),
                med(sh.iter().map(|s| s.famine_years).filter(|x| x.is_finite()).collect())
            );
        }
        if rs.iter().any(|r| r.bridges > 0) {
            println!(
                "{:<20} {:>12} bridges per run {:.0}; within 2 km of one: places >=1,000 {:.2}, all settlements {:.2}",
                "v3 bridges",
                "(reported)",
                med(rs.iter().map(|r| r.bridges as f32).collect()),
                med(rs.iter().map(|r| r.near_bridge_big).collect()),
                med(rs.iter().map(|r| r.near_bridge_all).collect())
            );
        }
        if rs.iter().any(|r| r.mill_share > 0.0) {
            println!(
                "{:<20} {:>12} share of settlements with a mill {:.2}",
                "v3 mills",
                "(reported)",
                med(rs.iter().map(|r| r.mill_share).collect())
            );
        }
        println!(
            "runs passing every check: {full}/{}   slowest run {:.2} ms/year (native)",
            rs.len(),
            ms
        );
        all_ok &= contingency_ok && over5_ok;
    }
    println!("\nvalidation {}", if all_ok { "PASSED" } else { "FAILED" });
    if !all_ok {
        std::process::exit(1);
    }
}

/// Mean over ordered pairs of runs of the share of one run's ten largest
/// settlements with one of the other run's ten largest within 5 km (D35).
pub fn top10_overlap(t: &hinterlands_core::Terrain, rs: &[&RunResult]) -> f32 {
    let (mut sum, mut pairs) = (0.0f32, 0u32);
    for (i, a) in rs.iter().enumerate() {
        for (j, b) in rs.iter().enumerate() {
            if i == j || a.top10.is_empty() {
                continue;
            }
            let near = a
                .top10
                .iter()
                .filter(|&&x| b.top10.iter().any(|&y| t.grid.dist_km(x, y) <= 5.0))
                .count();
            sum += near as f32 / a.top10.len() as f32;
            pairs += 1;
        }
    }
    if pairs == 0 {
        f32::NAN
    } else {
        sum / pairs as f32
    }
}

/// Whether these runs had goods on (v2 measures are then reported).
fn params_goods(rs: &[&RunResult]) -> bool {
    rs.iter().any(|r| r.goods.payment_share.is_finite())
}
