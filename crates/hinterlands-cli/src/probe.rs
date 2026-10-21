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

/// Every removal in a run: `fail seed year cause peak final`, where peak is
/// the settlement's largest population over its life.
pub fn cmd_failures(args: &Args) {
    let mut sim = make_sim(args);
    let years = args.u64("years", 500) as u32;
    let seed = sim.sim_seed;
    let map = args.u64("map-seed", 1);
    let mut peak: std::collections::BTreeMap<u32, f32> = sim
        .state
        .settlements
        .iter()
        .map(|s| (s.id, s.pop))
        .collect();
    while sim.state.year < years {
        sim.step(1);
        for e in &sim.state.log {
            if let LogKind::Failed { cause, .. } = &e.kind {
                let pk = peak.remove(&e.settlement).unwrap_or(e.pop).max(e.pop);
                println!(
                    "fail {map} {seed} {} {cause:?} {pk:.0} {:.0}",
                    e.year, e.pop
                );
            }
        }
        for s in &sim.state.settlements {
            let v = peak.entry(s.id).or_insert(s.pop);
            *v = v.max(s.pop);
        }
    }
}

/// Goods report (v2): at each year in `--at`, every resource site with its
/// holder and output, and for working iron sites the forest within 5 km
/// against the mean forest on land; and every specialist founded or outgrown
/// (`works` and `outgrew` lines).
pub fn cmd_goods(args: &Args) {
    let mut sim = make_sim(args);
    let at: Vec<u32> = args
        .str("at")
        .unwrap_or("200,500")
        .split(',')
        .map(|v| v.trim().parse().unwrap())
        .collect();
    let years = *at.iter().max().unwrap_or(&500);
    let seed = sim.sim_seed;
    let map = args.u64("map-seed", 1);
    while sim.state.year < years {
        sim.step(1);
        let y = sim.state.year;
        for e in &sim.state.log {
            let (x, yy) = sim.terrain.grid.xy(e.anchor as usize);
            match &e.kind {
                LogKind::Works { good, .. } => {
                    println!("works {map} {seed} {} {good:?} {x} {yy} {}", e.year, e.name)
                }
                LogKind::Outgrew { good } => {
                    println!("outgrew {map} {seed} {} {good:?} {x} {yy} {}", e.year, e.name)
                }
                _ => {}
            }
        }
        if !at.contains(&y) {
            continue;
        }
        let t = &sim.terrain;
        let st = &sim.state;
        let land: Vec<usize> = (0..t.n()).filter(|&i| t.is_land(i)).collect();
        let land_mean = land.iter().map(|&i| st.forest[i]).sum::<f32>() / land.len().max(1) as f32;
        flow_summary(map, seed, y, &sim);
        for s in &st.sites {
            let (x, yy) = t.grid.xy(s.cell as usize);
            let near: Vec<usize> = t
                .grid
                .disk(s.cell as usize, 5.0)
                .filter(|&j| t.is_land(j))
                .collect();
            let forest = near.iter().map(|&j| st.forest[j]).sum::<f32>() / near.len().max(1) as f32;
            println!(
                "site {map} {seed} {y} {:?} {x} {yy} held {} output {:.1} potential {:.1} fuel {:.2}/{:.2} forest5km {:.3} land {:.3} sold {:.1}",
                s.good,
                s.holder != hinterlands_core::state::NONE,
                s.output,
                s.potential,
                s.fuel_got,
                s.fuel_want,
                forest,
                land_mean,
                s.sold
            );
        }
    }
}

/// Payments against goods received, and the amount-weighted median route cost
/// of each kind of flow (G2 check).
fn flow_summary(map: u64, seed: u64, y: u32, sim: &hinterlands_core::Sim) {
    let st = &sim.state;
    let p = &sim.params;
    let cost_of = |f: &hinterlands_core::state::Flow| {
        st.get(f.from)
            .map_or(f32::NAN, |s| s.reach.partners[f.partner_idx as usize].cost)
    };
    let (mut goods_value, mut paid, mut arrived) = (0.0f64, 0.0f64, 0.0f64);
    let mut classes: std::collections::BTreeMap<&str, Vec<(f32, f32)>> = Default::default();
    for f in &st.flows {
        let c = cost_of(f);
        let class = match (f.good, f.payment) {
            (Some(hinterlands_core::state::Good::Salt), _) => "salt",
            (Some(hinterlands_core::state::Good::Iron), _) => "iron",
            (Some(hinterlands_core::state::Good::Stone), _) => "stone",
            (None, true) => "payment",
            (None, false) => "grain",
        };
        if f.good.is_some() {
            goods_value += f.amount as f64;
        }
        if f.payment {
            paid += f.amount as f64;
            arrived += (f.amount * (1.0 - p.transport_loss * c).max(0.0)) as f64;
        }
        if f.amount > 0.0 && c.is_finite() {
            classes.entry(class).or_default().push((c, f.amount));
        }
    }
    let median = |v: &mut Vec<(f32, f32)>| {
        v.sort_by(|a, b| a.0.total_cmp(&b.0));
        let half = v.iter().map(|x| x.1).sum::<f32>() / 2.0;
        let mut acc = 0.0;
        for &(c, a) in v.iter() {
            acc += a;
            if acc >= half {
                return c;
            }
        }
        f32::NAN
    };
    let mut line = format!(
        "flows {map} {seed} {y} goods_value {goods_value:.0} paid {paid:.0} arrived {arrived:.0}"
    );
    for (k, v) in classes.iter_mut() {
        let total: f32 = v.iter().map(|x| x.1).sum();
        line += &format!(" {k}:{:.2}@{total:.0}", median(v));
    }
    println!("{line}");
}

/// Roads report (v3): at each year in `--at`, the road cells and edges, the
/// share of land that is road, the initial road cells still road, the road
/// condition (p10, p50), the cells ever road and since abandoned, the bridges
/// built and the share of settlements with a mill.
pub fn cmd_roads(args: &Args) {
    let mut sim = make_sim(args);
    let at: Vec<u32> = args
        .str("at")
        .unwrap_or("100,250,500")
        .split(',')
        .map(|v| v.trim().parse().unwrap())
        .collect();
    let years = *at.iter().max().unwrap_or(&500);
    let seed = sim.sim_seed;
    let map = args.u64("map-seed", 1);
    let mut ever = sim.state.road.clone();
    while sim.state.year < years {
        sim.step(1);
        for (e, &r) in ever.iter_mut().zip(&sim.state.road) {
            *e |= r;
        }
        let y = sim.state.year;
        if !at.contains(&y) {
            continue;
        }
        let t = &sim.terrain;
        let st = &sim.state;
        let land = (0..t.n()).filter(|&i| t.is_land(i)).count().max(1);
        let cells = st.road.iter().filter(|&&r| r).count();
        let mut qs: Vec<f32> = (0..t.n())
            .filter(|&i| st.road[i])
            .map(|i| st.road_q.get(i).copied().unwrap_or(1.0))
            .collect();
        qs.sort_by(f32::total_cmp);
        let q = |f: f32| {
            qs.get(((qs.len() as f32 - 1.0) * f) as usize)
                .copied()
                .unwrap_or(0.0)
        };
        let initial = t.initial_road.iter().filter(|&&r| r).count();
        let kept = (0..t.n())
            .filter(|&i| t.initial_road.get(i) == Some(&true) && st.road[i])
            .count();
        println!(
            "roads {map} {seed} {y} cells {cells} land_share {:.3} initial_kept {kept}/{initial} edges {} condition p10 {:.2} p50 {:.2} ever {} abandoned {} bridges {} mill_share {:.2}",
            cells as f32 / land as f32,
            st.road_edges.len(),
            q(0.1),
            q(0.5),
            ever.iter().filter(|&&e| e).count(),
            (0..t.n()).filter(|&i| ever[i] && !st.road[i]).count(),
            st.bridge.iter().filter(|&&b| b).count(),
            st.settlements.iter().filter(|s| s.mill).count() as f32
                / st.settlements.len().max(1) as f32
        );
    }
}
