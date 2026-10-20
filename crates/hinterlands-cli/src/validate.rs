//! The validation table from the foundation document, run headless over several
//! maps and seeds in parallel. Definitions are in `docs/validation.md`.

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
    /// A settlement over 5,000 within 10 km of the main river mouth.
    pub mouth_city: bool,
    pub ms_per_year: f64,
    pub hash: u64,
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
        target: "2-5",
        ok: |r| (2..=5).contains(&r.m.over_5000),
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
    for _ in 0..years {
        sim.step(1);
        let tot = sim.state.total_pop();
        if sim.state.year > 100 && prev > 0.0 {
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
    let mouth_city = t.river_mouth.is_some_and(|mouth| {
        snap.settlements
            .iter()
            .any(|s| s.pop > 5000.0 && t.grid.dist_km(s.anchor as usize, mouth as usize) <= 10.0)
    });
    RunResult {
        map_seed,
        sim_seed,
        m,
        max_change,
        largest_anchor,
        mouth_city,
        ms_per_year,
        hash: snap.hash(),
    }
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
                let r = run_one(t, &params, *map_seed, seed, years);
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
                    _ => r.max_change,
                }
            };
            let mut vals: Vec<f32> = rs.iter().map(|r| value(r)).collect();
            vals.sort_by(|a, b| a.total_cmp(b));
            let med = vals.get(vals.len() / 2).copied().unwrap_or(f32::NAN);
            let canon_ok = canon.is_some_and(|r| (c.ok)(r));
            println!(
                "{:<20} {:>12} {:>10.3} {:>10.3} {:>7}/{:<2} {:>8}",
                c.name,
                c.target,
                canon.map_or(f32::NAN, |r| value(r)),
                med,
                pass,
                rs.len(),
                if canon_ok { "ok" } else { "FAIL" }
            );
            all_ok &= canon_ok;
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
        let mouth = rs.iter().filter(|r| r.mouth_city).count() as f32 / rs.len().max(1) as f32;
        let contingency_ok = differs >= 0.3 && mouth >= 0.8;
        let ms = rs.iter().map(|r| r.ms_per_year).fold(0.0, f64::max);
        println!(
            "{:<20} {:>12} {:>10.2} {:>10.2} {:>10} {:>8}",
            "contingency",
            ">=0.30 / >=0.80",
            differs,
            mouth,
            "",
            if contingency_ok { "ok" } else { "FAIL" }
        );
        println!(
            "runs passing every check: {full}/{}   slowest run {:.2} ms/year (native)",
            rs.len(),
            ms
        );
        all_ok &= contingency_ok;
    }
    println!("\nvalidation {}", if all_ok { "PASSED" } else { "FAILED" });
    if !all_ok {
        std::process::exit(1);
    }
}
