//! Parameter sweeps for tuning: the cartesian product of `--set name=a,b,c`
//! options, over a few seeds, in parallel; one line of metrics per combination.

use crate::validate::{run_one, CHECKS};
use crate::{die, load_map_params, load_params, Args};
use hinterlands_core::Params;
use hinterlands_mapgen::generate;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

pub fn cmd_sweep(raw: &[String], args: &Args) {
    let mut axes: Vec<(String, Vec<f64>)> = Vec::new();
    let mut it = raw.iter();
    while let Some(a) = it.next() {
        if a == "--set" {
            let spec = it.next().unwrap_or_else(|| die("--set name=v1,v2"));
            let (name, vals) = spec
                .split_once('=')
                .unwrap_or_else(|| die("--set name=v1,v2"));
            if Params::default().get(name).is_none() {
                die(&format!("unknown parameter {name}"));
            }
            let vals = vals
                .split(',')
                .map(|v| v.parse().unwrap_or_else(|_| die("bad value")))
                .collect();
            axes.push((name.to_string(), vals));
        }
    }
    let base = load_params(args);
    let years = args.u64("years", 500) as u32;
    let seeds = args.u64("seeds", 2);
    let map_seed = args.u64("map-seed", 1);
    let maps = args.u64("maps", 1);
    let terrains: Vec<(u64, hinterlands_core::Terrain)> = (map_seed..map_seed + maps)
        .map(|s| (s, generate(s, &load_map_params(args))))
        .collect();

    let mut combos: Vec<Vec<(String, f64)>> = vec![Vec::new()];
    for (name, vals) in &axes {
        combos = combos
            .into_iter()
            .flat_map(|c| {
                vals.iter().map(move |&v| {
                    let mut c = c.clone();
                    c.push((name.clone(), v));
                    c
                })
            })
            .collect();
    }
    let jobs: Vec<(usize, usize, u64)> = (0..combos.len())
        .flat_map(|c| (0..terrains.len()).flat_map(move |m| (1..=seeds).map(move |s| (c, m, s))))
        .collect();
    let next = AtomicUsize::new(0);
    let results = Mutex::new(Vec::new());
    let threads = std::thread::available_parallelism().map_or(4, |n| n.get());
    std::thread::scope(|scope| {
        for _ in 0..threads {
            scope.spawn(|| loop {
                let j = next.fetch_add(1, Ordering::Relaxed);
                let Some(&(c, m, s)) = jobs.get(j) else { break };
                let mut p = base.clone();
                for (name, v) in &combos[c] {
                    p.set(name, *v);
                }
                let (ms, t) = &terrains[m];
                let r = run_one(t, &p, *ms, s, years);
                results.lock().unwrap().push((c, r));
            });
        }
    });
    let mut results = results.into_inner().unwrap();
    results.sort_by_key(|(c, r)| (*c, r.map_seed, r.sim_seed));
    println!(
        "{:<40} {:>4} {:>5} {:>7} {:>6} {:>4} {:>4} {:>5} {:>6} {:>5} {:>5} {:>5} {:>5} {:>5} {:>5} {:>4}",
        "params", "map", "seed", "total", "max", ">2k", ">5k", "share", "slope", "space", "sited", "fail", "frst", "stab", "med", "pass"
    );
    for (c, r) in &results {
        let label: Vec<String> = combos[*c].iter().map(|(n, v)| format!("{n}={v}")).collect();
        let pass = CHECKS.iter().filter(|k| (k.ok)(r)).count();
        println!(
            "{:<40} {:>4} {:>5} {:>7.0} {:>6.0} {:>4} {:>4} {:>5.2} {:>6.2} {:>5.1} {:>5.2} {:>5.2} {:>5.2} {:>5.3} {:>5.0} {:>2}/{}",
            label.join(" "),
            r.map_seed,
            r.sim_seed,
            r.m.total_pop,
            r.m.largest,
            r.m.over_2000,
            r.m.over_5000,
            r.m.largest_share,
            r.m.rank_slope,
            r.m.spacing_median_km,
            r.m.siting_share,
            r.m.failure_rate,
            r.m.forest_near_cities,
            r.max_change,
            r.median_pop,
            pass,
            CHECKS.len()
        );
        let _ = r.m.count;
    }
}

/// How far a run is from the validation targets: 0 when every check passes,
/// growing with each metric's distance outside its range (scaled by the range).
pub fn miss(r: &crate::validate::RunResult) -> f32 {
    let out = |v: f32, lo: f32, hi: f32| {
        if !v.is_finite() {
            return 1.0;
        }
        let w = (hi - lo).max(1e-6);
        if v < lo {
            ((lo - v) / w).min(3.0)
        } else if v > hi {
            ((v - hi) / w).min(3.0)
        } else {
            0.0
        }
    };
    let m = &r.m;
    out(m.count as f32, 150.0, 350.0)
        + out(m.over_2000 as f32, 5.0, 12.0)
        + out(m.over_5000 as f32, 1.0, 5.0)
        + out(m.largest_share, 0.0, 0.25)
        + out(m.rank_slope, -1.3, -0.8)
        + out(m.spacing_median_km, 8.0, 15.0)
        + out(m.siting_share, 0.7, 1.0)
        + out(m.failure_rate, 0.4, 0.7)
        + out(m.forest_near_cities, 0.0, 0.3)
        + out(r.max_change, 0.0, 0.05)
}

/// Random search: `--range name=lo:hi` (repeatable), `--n` samples around the
/// base params, each run on `--seeds` seeds; prints the best by mean miss.
pub fn cmd_search(raw: &[String], args: &Args) {
    let mut ranges: Vec<(String, f64, f64)> = Vec::new();
    let mut it = raw.iter();
    while let Some(a) = it.next() {
        if a == "--range" {
            let spec = it.next().unwrap_or_else(|| die("--range name=lo:hi"));
            let (name, r) = spec
                .split_once('=')
                .unwrap_or_else(|| die("--range name=lo:hi"));
            let (lo, hi) = r
                .split_once(':')
                .unwrap_or_else(|| die("--range name=lo:hi"));
            if Params::default().get(name).is_none() {
                die(&format!("unknown parameter {name}"));
            }
            ranges.push((name.to_string(), lo.parse().unwrap(), hi.parse().unwrap()));
        }
    }
    let base = load_params(args);
    let n = args.u64("n", 40) as usize;
    let seeds = args.u64("seeds", 1);
    let years = args.u64("years", 500) as u32;
    let map_seed = args.u64("map-seed", 1);
    let maps = args.u64("maps", 1);
    let terrains: Vec<(u64, hinterlands_core::Terrain)> = (map_seed..map_seed + maps)
        .map(|s| (s, generate(s, &load_map_params(args))))
        .collect();
    // A small deterministic generator for the samples.
    let mut state = args
        .u64("search-seed", 7)
        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
        | 1;
    let mut rand01 = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        (state >> 11) as f64 / (1u64 << 53) as f64
    };
    let mut samples: Vec<Params> = vec![base.clone()];
    for _ in 1..n {
        let mut p = base.clone();
        for (name, lo, hi) in &ranges {
            p.set(name, lo + (hi - lo) * rand01());
        }
        samples.push(p);
    }
    let jobs: Vec<(usize, usize, u64)> = (0..samples.len())
        .flat_map(|c| (0..terrains.len()).flat_map(move |m| (1..=seeds).map(move |s| (c, m, s))))
        .collect();
    let next = AtomicUsize::new(0);
    let results = Mutex::new(Vec::new());
    let threads = std::thread::available_parallelism().map_or(4, |n| n.get());
    std::thread::scope(|scope| {
        for _ in 0..threads {
            scope.spawn(|| loop {
                let j = next.fetch_add(1, Ordering::Relaxed);
                let Some(&(c, m, s)) = jobs.get(j) else { break };
                let (ms, t) = &terrains[m];
                let r = run_one(t, &samples[c], *ms, s, years);
                results.lock().unwrap().push((c, r));
            });
        }
    });
    let results = results.into_inner().unwrap();
    let mut scored: Vec<(f32, usize, usize)> = (0..samples.len())
        .map(|c| {
            let rs: Vec<_> = results.iter().filter(|(k, _)| *k == c).collect();
            let mut mean = rs.iter().map(|(_, r)| miss(r)).sum::<f32>() / rs.len().max(1) as f32;
            // Contingency per map, when there are enough seeds to judge it.
            if seeds >= 3 {
                for (ms, t) in &terrains {
                    let g: Vec<_> = rs
                        .iter()
                        .filter(|(_, r)| r.map_seed == *ms)
                        .map(|(_, r)| r)
                        .collect();
                    let anchors: Vec<usize> = g.iter().filter_map(|r| r.largest_anchor).collect();
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
                    let mouth =
                        g.iter().filter(|r| r.corridor_city).count() as f32 / g.len().max(1) as f32;
                    mean += ((0.3 - differs).max(0.0) / 0.3 + (0.8 - mouth).max(0.0) / 0.8)
                        / terrains.len() as f32;
                }
            }
            let passes = rs
                .iter()
                .map(|(_, r)| CHECKS.iter().filter(|k| (k.ok)(r)).count())
                .min()
                .unwrap_or(0);
            (mean, passes, c)
        })
        .collect();
    scored.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.2.cmp(&b.2)));
    for &(mean, passes, c) in scored.iter().take(8) {
        let p = &samples[c];
        let vals: Vec<String> = ranges
            .iter()
            .map(|(name, _, _)| format!("{name}={:.4}", p.get(name).unwrap()))
            .collect();
        println!(
            "miss {:.2}  min passes {}/{}  {}",
            mean,
            passes,
            CHECKS.len(),
            vals.join(" ")
        );
        for (_, r) in results.iter().filter(|(k, _)| *k == c) {
            println!(
                "    map {} seed {} corridor {}: n {} >2k {} >5k {} share {:.2} slope {:.2} space {:.1} sited {:.2} fail {:.2} forest {:.2} stab {:.3} total {:.0}",
                r.map_seed, r.sim_seed, r.corridor_city as u8, r.m.count, r.m.over_2000, r.m.over_5000, r.m.largest_share, r.m.rank_slope,
                r.m.spacing_median_km, r.m.siting_share, r.m.failure_rate, r.m.forest_near_cities, r.max_change, r.m.total_pop
            );
        }
    }
    if let Some(path) = args.str("best-out") {
        let best = &samples[scored[0].2];
        std::fs::write(path, serde_json::to_string_pretty(best).unwrap())
            .unwrap_or_else(|e| die(&e.to_string()));
    }
}
