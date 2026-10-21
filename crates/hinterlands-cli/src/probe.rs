//! Site diagnostics: what happens at a few named cells over a run. Read-only:
//! the run itself is stepped exactly as `run` steps it.
//!
//! `probe --cells x,y;x,y --radius 3 --at 100,250,500` prints
//! - every settlement founded (or present at the start) within `radius` km of a
//!   watched cell: founding year, peak population, and its final population or
//!   failure year and cause;
//! - at each founding step in `--at`, the founding score at the watched cells,
//!   split into its parts, against the best score of that step and the
//!   threshold. The score is taken on a copy of the run advanced through that
//!   year's systems up to founding, so it is the score founding saw, without
//!   the noise term.

use crate::{die, make_sim, Args};
use hinterlands_core::reach;
use hinterlands_core::state::{FailCause, LogKind};
use hinterlands_core::systems::{self, founding};

struct Tracked {
    id: u32,
    name: String,
    anchor: usize,
    founded: u32,
    peak: f32,
    end: Option<(u32, FailCause)>,
    last: f32,
}

pub fn cmd_probe(args: &Args) {
    let mut sim = make_sim(args);
    let g = sim.terrain.grid;
    let cells: Vec<usize> = args
        .str("cells")
        .unwrap_or_else(|| die("--cells x,y;x,y"))
        .split(';')
        .map(|xy| {
            let (x, y) = xy.split_once(',').unwrap_or_else(|| die("--cells x,y;x,y"));
            g.idx(x.trim().parse().unwrap(), y.trim().parse().unwrap())
        })
        .collect();
    let radius: f32 = args.str("radius").map_or(3.0, |v| v.parse().unwrap());
    let at: Vec<u32> = args
        .str("at")
        .unwrap_or("100,250,500")
        .split(',')
        .map(|v| v.trim().parse().unwrap())
        .collect();
    let years = args.u64("years", 500) as u32;
    let seed = sim.sim_seed;
    let near = |a: usize| cells.iter().any(|&c| g.dist_km(a, c) <= radius);

    let mut tracked: Vec<Tracked> = sim
        .state
        .settlements
        .iter()
        .filter(|s| near(s.anchor as usize))
        .map(|s| Tracked {
            id: s.id,
            name: s.name.clone(),
            anchor: s.anchor as usize,
            founded: 0,
            peak: s.pop,
            end: None,
            last: s.pop,
        })
        .collect();

    println!("score seed year cell x y candidate food water shadow access total best threshold noise_max");
    loop {
        let y = sim.state.year;
        if at.contains(&y) && y % sim.params.founding_interval.max(1) == 0 {
            score_report(&sim, &cells, seed);
        }
        if y >= years {
            break;
        }
        sim.step(1);
        for e in &sim.state.log {
            match &e.kind {
                LogKind::Founded { .. } if near(e.anchor as usize) => tracked.push(Tracked {
                    id: e.settlement,
                    name: e.name.clone(),
                    anchor: e.anchor as usize,
                    founded: e.year,
                    peak: e.pop,
                    end: None,
                    last: e.pop,
                }),
                LogKind::Failed { cause, .. } => {
                    if let Some(t) = tracked.iter_mut().find(|t| t.id == e.settlement) {
                        t.end = Some((e.year, *cause));
                    }
                }
                _ => {}
            }
        }
        for t in tracked.iter_mut().filter(|t| t.end.is_none()) {
            if let Some(s) = sim.state.settlements.iter().find(|s| s.id == t.id) {
                t.peak = t.peak.max(s.pop);
                t.last = s.pop;
            }
        }
    }
    println!("site seed name x y founded peak end");
    for t in &tracked {
        let (x, y) = g.xy(t.anchor);
        let end = match t.end {
            Some((yr, cause)) => format!("failed {yr} {cause:?}"),
            None => format!("alive {:.0}", t.last),
        };
        println!(
            "site {seed} {} {x} {y} {} {:.0} {end}",
            t.name, t.founded, t.peak
        );
    }
}

/// The founding score at `cells` as founding sees it this year.
fn score_report(sim: &hinterlands_core::Sim, cells: &[usize], seed: u64) {
    let (t, p) = (&sim.terrain, &sim.params);
    let mut state = sim.state.clone();
    let mut rng = sim.rng.clone();
    reach::refresh_due(&mut state, t, p);
    systems::cultivation::run(&mut state, t, p, &mut rng);
    systems::trade::run(&mut state, t, p, &mut rng);
    systems::growth::run(&mut state, t, p, &mut rng);
    // Founding normalises food by the best free food among all candidates.
    let cands: Vec<usize> = (0..t.n())
        .filter(|&c| founding::is_candidate(&state, t, p, c))
        .collect();
    let max_food = cands
        .iter()
        .map(|&c| founding::free_food_near(&mut state, t, p, c))
        .fold(0.0f32, f32::max)
        .max(1e-6);
    let mut rows = Vec::new();
    for &c in cells {
        let cand = founding::is_candidate(&state, t, p, c);
        let food = founding::free_food_near(&mut state, t, p, c);
        let mut parts = founding::site_parts(&state, t, p, c, food);
        parts.food /= max_food;
        rows.push((c, cand, parts));
    }
    let mut after = state.clone();
    systems::founding::run(&mut after, t, p, &mut rng);
    let best = after.tally.last_best_score;
    for (c, cand, parts) in rows {
        let (x, y) = t.grid.xy(c);
        println!(
            "score {seed} {} {c} {x} {y} {cand} {:.3} {:.3} {:.3} {:.3} {:.3} {best:.3} {:.3} {:.3}",
            state.year,
            p.w_food * parts.food,
            parts.water,
            parts.shadow,
            parts.access,
            parts.total(p),
            p.found_threshold,
            p.noise
        );
    }
}
