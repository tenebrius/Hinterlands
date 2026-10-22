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
                    println!(
                        "outgrew {map} {seed} {} {good:?} {x} {yy} {}",
                        e.year, e.name
                    )
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
/// built, the share of settlements with a mill, and with `--mask` (a 0/1 grid,
/// e.g. Watling Street) the mask's cells still road.
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
    let mask: Option<Vec<bool>> = args.str("mask").map(|path| {
        let text = std::fs::read_to_string(path).unwrap_or_else(|e| die(&format!("{path}: {e}")));
        text.lines()
            .filter(|l| l.starts_with(|c: char| c.is_ascii_digit()))
            .flat_map(|l| l.split_whitespace())
            .map(|v| v == "1")
            .collect()
    });
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
        // Cells of a 0/1 mask (e.g. Watling Street) still road.
        let masked = mask.as_ref().map_or(String::new(), |m| {
            let all = (0..t.n()).filter(|&i| m[i]).count();
            let kept = (0..t.n()).filter(|&i| m[i] && st.road[i]).count();
            format!(" mask_kept {kept}/{all}")
        });
        let initial = t.initial_road.iter().filter(|&&r| r).count();
        let kept = (0..t.n())
            .filter(|&i| t.initial_road.get(i) == Some(&true) && st.road[i])
            .count();
        println!(
            "roads {map} {seed} {y} cells {cells} land_share {:.3} initial_kept {kept}/{initial} edges {} condition p10 {:.2} p50 {:.2} ever {} abandoned {} bridges {} mill_share {:.2}{masked}",
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

/// Desertion report (v4): for each settlement alive the year before the
/// plague, `place seed id pop fields field_yield local_yield failed cause`,
/// where local yield is the mean over land within 3 km of the anchor and
/// failed is the year it was removed before `plague_year + 52`, or 0; last,
/// the distance in km to the nearest place over 2,000.
pub fn cmd_desertion(args: &Args) {
    let mut sim = make_sim(args);
    let seed = sim.sim_seed;
    let map = args.u64("map-seed", 1);
    let py = sim.params.plague_year;
    while sim.state.year < py {
        sim.step(1);
    }
    let t = sim.terrain.clone();
    let big: Vec<usize> = sim
        .state
        .settlements
        .iter()
        .filter(|s| s.pop > 2000.0)
        .map(|s| s.anchor as usize)
        .collect();
    let rows: Vec<(u32, f32, usize, f32, f32, f32)> = sim
        .state
        .settlements
        .iter()
        .map(|s| {
            let fy = if s.fields.is_empty() {
                f32::NAN
            } else {
                s.fields
                    .iter()
                    .map(|&c| t.food_yield[c as usize])
                    .sum::<f32>()
                    / s.fields.len() as f32
            };
            let near: Vec<usize> = t
                .grid
                .disk(s.anchor as usize, 3.0)
                .filter(|&j| t.is_land(j))
                .collect();
            let ly = near.iter().map(|&j| t.food_yield[j]).sum::<f32>() / near.len().max(1) as f32;
            let d = big
                .iter()
                .filter(|&&b| b != s.anchor as usize)
                .map(|&b| t.grid.dist_km(b, s.anchor as usize))
                .fold(f32::INFINITY, f32::min);
            (s.id, s.pop, s.fields.len(), fy, ly, d)
        })
        .collect();
    let mut failed: std::collections::BTreeMap<u32, (u32, FailCause)> = Default::default();
    while sim.state.year < py + 52 {
        sim.step(1);
        for e in &sim.state.log {
            if let LogKind::Failed { cause, .. } = &e.kind {
                failed.insert(e.settlement, (e.year, *cause));
            }
        }
    }
    for (id, pop, nf, fy, ly, d) in rows {
        let (y, c) = failed
            .get(&id)
            .map_or((0, String::from("-")), |(y, c)| (*y, format!("{c:?}")));
        println!("place {map} {seed} {id} {pop:.0} {nf} {fy:.3} {ly:.3} {y} {c} {d:.1}");
    }
}

/// Shadow prices (v6 S0): every settlement each year from `--from` (default 1),
/// `price seed year id x y grain salt iron stone imports yield pop`, where
/// yield is the harvest per cultivated cell; each year a line
/// `market seed year residuals... unmet salt iron stone imports` (unmet
/// goods demand with `market_goods`); each settlement's line ends with its
/// supply prices of salt, iron and stone, its kind, its whole harvest and its
/// fields' mean yield multiplier for the year (1 without local harvests, v6
/// W1); and at `--arcs` years
/// the grain arcs,
/// `arc seed year from to flow gap transport`.
pub fn cmd_prices(args: &Args) {
    let mut sim = make_sim(args);
    if !sim.params.market_shadow && !sim.params.market_goods && !sim.params.market_grain {
        die("prices needs --params with market_shadow or market_goods true");
    }
    let years = args.u64("years", 500) as u32;
    let from = args.u64("from", 1) as u32;
    let arcs_at: Vec<u32> = args
        .str("arcs")
        .unwrap_or("")
        .split(',')
        .filter(|v| !v.is_empty())
        .map(|v| v.trim().parse().unwrap())
        .collect();
    let seed = sim.sim_seed;
    while sim.state.year < years {
        sim.step(1);
        let y = sim.state.year;
        if y < from {
            continue;
        }
        let g = sim.terrain.grid;
        let mut r = sim.state.market_residual;
        {
            let gs = sim.state.grain_stats;
            r[0] = gs.residual;
            println!(
                "grain {seed} {y} residual {:.4} passes {} unsold {:.0} harvest {:.0} capped {} arrived {:.0} local {:.0} markets {} toll_share {:.3}",
                gs.residual, gs.passes, gs.unsold, gs.harvest, gs.capped, gs.arrived, gs.unsold_local, gs.markets, gs.toll_share
            );
        }
        let u = sim.state.market_unmet;
        println!(
            "market {seed} {y} {:.4} {:.4} {:.4} {:.4} {:.4} unmet {:.0} {:.0} {:.0} {:.0}",
            r[0], r[1], r[2], r[3], r[4], u[1], u[2], u[3], u[4]
        );
        for s in &sim.state.settlements {
            let (x, yy) = g.xy(s.anchor as usize);
            let yld = s.food_prod / s.fields.len().max(1) as f32;
            // Market prices where the model trades by them, else the shadow's.
            let mut pr = s.shadow_price;
            if sim.params.market_goods || sim.params.market_grain {
                pr[1..].copy_from_slice(&s.price[1..]);
            }
            if !sim.params.market_shadow {
                pr[0] = s.price[0];
            }
            let sp = s.supply_price;
            let hv = &sim.state.harvest;
            let m = if hv.is_empty() || s.fields.is_empty() {
                1.0
            } else {
                let fy = |c: &u32| sim.terrain.food_yield[*c as usize];
                s.fields
                    .iter()
                    .map(|c| fy(c) * hv[*c as usize])
                    .sum::<f32>()
                    / s.fields.iter().map(fy).sum::<f32>().max(1e-9)
            };
            println!(
                "price {seed} {y} {} {x} {yy} {:.5} {:.5} {:.5} {:.5} {:.5} {:.3} {:.0} supply {:.4} {:.4} {:.4} kind {:?} harvest {:.1} m {:.4} grain {:.4} {:.1} {} eaten {:.1} in {:.1}",
                s.id, pr[0], pr[1], pr[2], pr[3], pr[4], yld, s.pop, sp[0], sp[1], sp[2], s.kind, s.food_prod, m,
                s.grain_sup, s.income, s.capped as u8, s.food_prod + s.food_in - s.food_out, s.market_in
            );
        }
        if arcs_at.contains(&y) {
            let arcs = if sim.params.market_shadow {
                &sim.state.shadow_arcs
            } else {
                &sim.state.market_arcs
            };
            for &(a, b, x, gap, t) in arcs {
                println!("arc {seed} {y} {a} {b} {x:.2} {gap:.5} {t:.5}");
            }
        }
    }
}

/// The largest places' accounts at `--at` years (default 100,200,300):
/// `town seed year id pop market market_in through transship food_ratio
/// income grain_price capped`.
pub fn cmd_towns(args: &Args) {
    let mut sim = make_sim(args);
    let at: Vec<u32> = args
        .str("at")
        .unwrap_or("100,200,300")
        .split(',')
        .map(|v| v.trim().parse().unwrap())
        .collect();
    let top = args.u64("top", 8) as usize;
    let seed = sim.sim_seed;
    let last = at.iter().copied().max().unwrap_or(0);
    while sim.state.year < last {
        sim.step(1);
        if !at.contains(&sim.state.year) {
            continue;
        }
        let mut ss: Vec<_> = sim.state.settlements.iter().collect();
        ss.sort_by(|a, b| b.pop.total_cmp(&a.pop));
        for s in ss.iter().take(top) {
            println!(
                "town {seed} {} {} {:.0} {:.0} {:.0} {:.0} {:.0} {:.3} {:.0} {:.3} {} harvest {:.0} store {:.0} need {:.0} mstore {:.0} avgp {:.3}",
                sim.state.year,
                s.id,
                s.pop,
                s.market,
                s.market_in,
                s.through,
                s.transship,
                s.food_ratio,
                s.income,
                s.price[0],
                s.capped as u8,
                s.food_prod,
                s.store,
                s.need(&sim.params),
                s.mstore,
                s.avg_price
            );
        }
    }
}

/// v7 M0 (marketplaces diagnostic; no feedback). Runs with the shadow market
/// on (inert, D77) and at each `--at` year computes, per settlement:
/// - turnover under seller routing: each place sells its rent
///   (`market_share × harvest`) plus what its people do not eat
///   (`(1 − market_share) × harvest − need`, if positive), routed to markets by
///   `W^α exp(−β c)` with its own market by `home_weight`, valued at the
///   market's shadow grain price;
/// - rents received, the same rents valued at the payer's shadow price and
///   routed by the same weights;
/// - fees (`handling_margin × through + transship_margin × transship`) and
///   goods sales, valued at its own price;
/// - its food bill: what its people need beyond its own fields after rent, at
///   its own shadow price.
///
/// Prints `mkt seed year id pop market_in turnover rents fees goods bill` per
/// settlement.
pub fn cmd_marketplaces(args: &Args) {
    let mut sim = make_sim(args);
    sim.params.market_shadow = true;
    let at: Vec<u32> = args
        .str("at")
        .unwrap_or("300,450")
        .split(',')
        .map(|v| v.trim().parse().unwrap())
        .collect();
    let seed = sim.sim_seed;
    let map = args.u64("map-seed", 1);
    let last = at.iter().copied().max().unwrap_or(0);
    while sim.state.year < last {
        sim.step(1);
        let y = sim.state.year;
        if !at.contains(&y) {
            continue;
        }
        let (st, p) = (&sim.state, &sim.params);
        let m = st.settlements.len();
        let attract: Vec<f32> = st
            .settlements
            .iter()
            .map(|s| s.market.max(p.market_floor).powf(p.alpha))
            .collect();
        let price: Vec<f32> = st
            .settlements
            .iter()
            .map(|s| {
                if s.shadow_price[0] > 0.0 {
                    s.shadow_price[0]
                } else {
                    1.0
                }
            })
            .collect();
        let mut turnover = vec![0f32; m];
        let mut rents = vec![0f32; m];
        for (i, s) in st.settlements.iter().enumerate() {
            let h = s.food_prod;
            let rent = p.market_share * h;
            let surplus = ((1.0 - p.market_share) * h - s.need(p)).max(0.0);
            let sale = rent + surplus;
            let mut ws: Vec<(usize, f32)> = vec![(i, attract[i] * p.home_weight)];
            for q in &s.reach.partners {
                if let Some(j) = st.index_of(q.id) {
                    ws.push((j, attract[j] * (-p.beta * q.cost).exp()));
                }
            }
            let total: f32 = ws.iter().map(|w| w.1).sum();
            if total <= 0.0 {
                continue;
            }
            for (j, w) in ws {
                let share = w / total;
                turnover[j] += sale * share * price[j];
                rents[j] += rent * share * price[i];
            }
        }
        for (i, s) in st.settlements.iter().enumerate() {
            let fees =
                (p.handling_margin * s.through + p.transship_margin * s.transship) * price[i];
            let goods = s.goods_in;
            let bill = (s.need(p) - (1.0 - p.market_share) * s.food_prod).max(0.0) * price[i];
            println!(
                "mkt {map} {seed} {y} {} {:.0} {:.1} {:.1} {:.1} {:.1} {:.1} {:.1}",
                s.id, s.pop, s.market_in, turnover[i], rents[i], fees, goods, bill
            );
        }
    }
}

/// D97 money: each year the coin stock, the farmers' and markets' purses,
/// coin in and out by channel, the conservation residual, and the regional
/// grain price (median over large markets) against the world price.
pub fn cmd_money(args: &Args) {
    let mut sim = make_sim(args);
    let years = args.u64("years", 500) as u32;
    let every = args.u64("every", 1) as u32;
    let seed = sim.sim_seed;
    println!("money seed year pop stock F M port_in planted imports luxury wear removed residual price world ports dearth_met unsold capped wool_in pasture");
    while sim.state.year < years {
        let before = sim.state.money_stock();
        sim.step(1);
        let y = sim.state.year;
        let st = &sim.state;
        let l = st.money;
        let after = st.money_stock();
        let resid =
            after - before - (l.port_in + l.wool_in + l.planted_in - l.imports_out - l.luxury_out - l.wear - l.removed);
        if y % every != 0 {
            continue;
        }
        let mut ps: Vec<f32> = st
            .settlements
            .iter()
            .filter(|s| s.market >= sim.params.market_min && s.price[0] > 0.0)
            .map(|s| s.price[0])
            .collect();
        ps.sort_by(|a, b| a.total_cmp(b));
        let price = ps.get(ps.len() / 2).copied().unwrap_or(f32::NAN);
        let port = if st.port_grain.is_empty() {
            f32::NAN
        } else {
            st.port_grain.iter().sum::<f32>() / st.port_grain.len() as f32
        };
        let f: f32 = st.settlements.iter().map(|s| s.purse).sum();
        let m: f32 = st.settlements.iter().map(|s| s.mpurse).sum();
        let gs = st.grain_stats;
        println!(
            "money {seed} {y} {:.0} {:.0} {:.0} {:.0} {:.1} {:.1} {:.1} {:.1} {:.1} {:.1} {:.4} {:.3} {:.3} {:.3} {:.3} {:.0} {} {:.0} {}",
            st.total_pop(),
            after,
            f,
            m,
            l.port_in,
            l.planted_in,
            l.imports_out,
            l.luxury_out,
            l.wear,
            l.removed,
            resid / after.max(1.0),
            price,
            sim.params.port_world_price,
            port,
            gs.dearth_bought / gs.dearth_need.max(1e-6),
            gs.unsold,
            gs.capped,
            l.wool_in,
            st.settlements.iter().map(|s| s.pasture.len()).sum::<usize>()
        );
    }
}

/// L0 (D105): wool and land use, a diagnostic with no feedback. At the given
/// years, for every claimed cell and every free land cell within a place's
/// cultivation reach, the arable and pasture rents at this year's grain
/// prices and at each trial wool price (pounds sterling a sack), with:
/// - A: free cells with a positive pasture rent, nearest first, within the
///   farmers' labour left over from arable;
/// - B: claimed cells where pasture out-rents arable;
/// - the pasture share by travel cost to the nearest market and by yield;
/// - the break-even wool price;
/// - wool's coin at the ports against grain's (ten-year mean), and under B the
///   grain lost against grain arriving at markets.
/// Rents, per cell (proposal section 5):
///   arable  = yield × ppc × p × (1 − τ c_market) − arable_labour × fpp × p
///   pasture = wool × (1 − τ / ρ_wool × c_port) − pasture_labour × fpp × p
/// with `wool` the cell's fleece at the port in coin: acres × sheep_per_acre
/// × fleece_lb / 364 sacks at the trial price, turned into grain at the
/// wheat price (`wheat_s_bu`, `bu_per_person`) and valued at
/// `port_world_price`.
pub fn cmd_wool(args: &Args) {
    let mut sim = make_sim(args);
    let years: Vec<u32> = args
        .str("at")
        .unwrap_or("200,240,300,400")
        .split(',')
        .map(|v| v.trim().parse().unwrap())
        .collect();
    let prices: Vec<f32> = args
        .str("wool")
        .unwrap_or("4,5,9")
        .split(',')
        .map(|v| v.trim().parse().unwrap())
        .collect();
    let f = |k: &str, d: f32| args.str(k).map_or(d, |v| v.parse().unwrap());
    let sheep_per_acre = f("sheep-per-acre", 1.5);
    let fleece_lb = f("fleece-lb", 1.5);
    let wheat_s_bu = f("wheat-s-bu", 0.60);
    let bu_per_person = f("bu-per-person", 6.0);
    // `model` (the default): the farmers per claimed cell in this year's run.
    let arable_labour_arg = args.str("arable-labour").unwrap_or("model").to_string();
    let pasture_labour = f("pasture-labour", 5.0);
    let rho_wool = f("rho-wool", 5.0);
    let last = *years.iter().max().unwrap_or(&0);
    let seed = sim.sim_seed;
    let mut port_hist: Vec<f32> = Vec::new();
    while sim.state.year < last {
        sim.step(1);
        port_hist.push(sim.state.money.port_in);
        let y = sim.state.year;
        if !years.contains(&y) {
            continue;
        }
        let (t, st, p) = (&sim.terrain, &sim.state, &sim.params);
        let n = t.n();
        let acres = t.grid.cell_km * t.grid.cell_km * 247.1;
        let sacks = acres * sheep_per_acre * fleece_lb / 364.0;
        // Coin at the port per cell, per pound sterling of the sack price.
        let coin_per_pound = sacks * 20.0 / wheat_s_bu / bu_per_person * p.port_world_price;
        let fpp = p.food_per_person;
        let ppc = p.people_per_prime_cell;
        let arable_labour = if arable_labour_arg == "model" {
            let farmers: f32 = st.settlements.iter().map(|s| (s.pop - s.market.max(0.0)).max(0.0)).sum();
            let fields: usize = st.settlements.iter().map(|s| s.fields.len()).sum();
            farmers / fields.max(1) as f32
        } else {
            arable_labour_arg.parse().unwrap()
        };
        let big: Vec<usize> = (0..st.settlements.len())
            .filter(|&i| st.settlements[i].market >= p.market_min)
            .collect();
        let port_cost: Vec<f32> = st
            .settlements
            .iter()
            .map(|s| {
                s.reach
                    .partners
                    .iter()
                    .filter(|q| hinterlands_core::state::port_of(q.id).is_some())
                    .map(|q| q.cost)
                    .fold(f32::INFINITY, f32::min)
            })
            .collect();
        let mut prices_all: Vec<f32> = st
            .settlements
            .iter()
            .filter(|s| s.price[0] > 0.0)
            .map(|s| s.price[0])
            .collect();
        prices_all.sort_by(|a, b| a.total_cmp(b));
        let p_reg = prices_all.get(prices_all.len() / 2).copied().unwrap_or(p.port_world_price);
        // Candidate cells: claimed (B) and free land within any field reach (A).
        let mut in_reach = vec![false; n];
        for s in &st.settlements {
            for &c in &s.reach.field_order {
                in_reach[c as usize] = true;
            }
        }
        struct Cell {
            yield_: f32,
            c_mkt: f32,
            claimed: bool,
            r_arable: f32,
            c_port: f32,
            p: f32,
        }
        let mut cells: Vec<(usize, Cell)> = Vec::new();
        for c in 0..n {
            if !t.is_land(c) {
                continue;
            }
            let claimed = st.owner[c] != hinterlands_core::state::NONE;
            if !claimed && !in_reach[c] {
                continue;
            }
            let (mut c_mkt, mut pl) = (f32::INFINITY, p_reg);
            for &i in &big {
                let k = st.settlements[i].reach.cost_to_cell(c);
                if k < c_mkt {
                    c_mkt = k;
                    pl = st.settlements[i].price[0].max(1e-3);
                }
            }
            let mut c_port = f32::INFINITY;
            for (i, s) in st.settlements.iter().enumerate() {
                if port_cost[i].is_finite() {
                    let k = s.reach.cost_to_cell(c);
                    if k.is_finite() {
                        c_port = c_port.min(k + port_cost[i]);
                    }
                }
            }
            let yield_ = t.food_yield[c];
            let cm = if c_mkt.is_finite() { c_mkt } else { 1.0 / p.market_tau };
            let r_arable = yield_ * ppc * fpp * pl * (1.0 - p.market_tau * cm).max(0.0)
                - arable_labour * fpp * pl;
            cells.push((
                c,
                Cell {
                    yield_,
                    c_mkt,
                    claimed,
                    r_arable,
                    c_port,
                    p: pl,
                },
            ));
        }
        // Travel-cost bands: quintiles over the candidate cells with a market in reach.
        let mut costs: Vec<f32> = cells
            .iter()
            .map(|x| x.1.c_mkt)
            .filter(|v| v.is_finite())
            .collect();
        costs.sort_by(|a, b| a.total_cmp(b));
        let edges: Vec<f32> = (1..5)
            .map(|k| costs.get(costs.len() * k / 5).copied().unwrap_or(f32::INFINITY))
            .collect();
        let band_c = |v: f32| -> usize {
            if !v.is_finite() {
                5
            } else {
                edges.iter().filter(|&&e| v >= e).count()
            }
        };
        let band_y = |v: f32| -> usize {
            if v < 0.3 {
                0
            } else if v < 0.5 {
                1
            } else if v < 0.7 {
                2
            } else {
                3
            }
        };
        let port_mean = {
            let k = port_hist.len().min(10);
            port_hist[port_hist.len() - k..].iter().sum::<f32>() / k.max(1) as f32
        };
        let harvest: f32 = st.settlements.iter().map(|s| s.food_prod).sum();
        let arrived = st.grain_stats.arrived;
        println!(
            "wool {seed} {y} pop {:.0} arable_labour {arable_labour:.1} cells claimed {} free {} p_reg {:.3} grain_coin {:.0} harvest {:.0} arrived {:.0} cost_edges {:.0} {:.0} {:.0} {:.0}",
            st.total_pop(),
            cells.iter().filter(|x| x.1.claimed).count(),
            cells.iter().filter(|x| !x.1.claimed).count(),
            p_reg,
            port_mean,
            harvest,
            arrived,
            edges[0],
            edges[1],
            edges[2],
            edges[3]
        );
        let tau_w = p.market_tau / rho_wool.max(1e-6);
        let pasture = |x: &Cell, w: f32| -> f32 {
            let cp = if x.c_port.is_finite() { x.c_port } else { 1.0 / tau_w };
            w * (1.0 - tau_w * cp).max(0.0) - pasture_labour * fpp * x.p
        };
        // Break-even sack prices (pounds), per cell.
        let mut be_a: Vec<f32> = Vec::new();
        let mut be_b: Vec<f32> = Vec::new();
        for (_, x) in &cells {
            let cp = if x.c_port.is_finite() { x.c_port } else { 1.0 / tau_w };
            let net = (1.0 - tau_w * cp).max(1e-6) * coin_per_pound;
            let lab = pasture_labour * fpp * x.p;
            if x.claimed {
                be_b.push((x.r_arable + lab).max(0.0) / net);
            } else {
                be_a.push(lab / net);
            }
        }
        be_a.sort_by(|a, b| a.total_cmp(b));
        be_b.sort_by(|a, b| a.total_cmp(b));
        let q = |v: &Vec<f32>, f: f32| v.get(((v.len() as f32 * f) as usize).min(v.len().saturating_sub(1))).copied().unwrap_or(f32::NAN);
        println!(
            "wool_be {seed} {y} A min {:.2} med {:.2} | B min {:.2} p10 {:.2} p25 {:.2} med {:.2} (pounds a sack)",
            q(&be_a, 0.0),
            q(&be_a, 0.5),
            q(&be_b, 0.0),
            q(&be_b, 0.1),
            q(&be_b, 0.25),
            q(&be_b, 0.5)
        );
        for &pw in &prices {
            let w = pw * coin_per_pound;
            // A: free cells with a positive pasture rent, nearest first per
            // settlement, within its farmers' labour left after arable.
            let mut taken = vec![false; n];
            let mut cell_ix = vec![usize::MAX; n];
            for (k, (c, _)) in cells.iter().enumerate() {
                cell_ix[*c] = k;
            }
            let (mut a_cells, mut a_free_pos) = (0u32, 0u32);
            let mut a_band_c = [[0u32; 2]; 6];
            let mut a_band_y = [[0u32; 2]; 4];
            for (_, x) in &cells {
                if !x.claimed {
                    let pos = pasture(x, w) > 0.0;
                    a_free_pos += pos as u32;
                    a_band_c[band_c(x.c_mkt)][1] += 1;
                    a_band_y[band_y(x.yield_)][1] += 1;
                }
            }
            let mut a_coin = 0f32;
            for s in &st.settlements {
                let farmers = (s.pop - s.market.max(0.0)).max(0.0);
                let spare = farmers - arable_labour * s.fields.len() as f32;
                let mut can = (spare / pasture_labour.max(1e-6)).floor().max(0.0) as u32;
                for &c in &s.reach.field_order {
                    if can == 0 {
                        break;
                    }
                    let c = c as usize;
                    let k = cell_ix[c];
                    if k == usize::MAX || taken[c] {
                        continue;
                    }
                    let x = &cells[k].1;
                    if x.claimed || pasture(x, w) <= 0.0 {
                        continue;
                    }
                    taken[c] = true;
                    can -= 1;
                    a_cells += 1;
                    a_coin += w;
                    a_band_c[band_c(x.c_mkt)][0] += 1;
                    a_band_y[band_y(x.yield_)][0] += 1;
                }
            }
            // B: claimed cells where pasture out-rents arable.
            let (mut b_cells, mut b_lost, mut b_coin) = (0u32, 0f32, 0f32);
            let mut b_band_c = [[0u32; 2]; 6];
            let mut b_band_y = [[0u32; 2]; 4];
            let mut claimed_n = 0u32;
            for (_, x) in &cells {
                if !x.claimed {
                    continue;
                }
                claimed_n += 1;
                let conv = pasture(x, w) > x.r_arable;
                b_band_c[band_c(x.c_mkt)][1] += 1;
                b_band_y[band_y(x.yield_)][1] += 1;
                if conv {
                    b_cells += 1;
                    b_lost += x.yield_ * ppc * fpp;
                    b_coin += w;
                    b_band_c[band_c(x.c_mkt)][0] += 1;
                    b_band_y[band_y(x.yield_)][0] += 1;
                }
            }
            let share = |b: &[[u32; 2]]| -> String {
                b.iter()
                    .map(|v| {
                        if v[1] == 0 {
                            "-".to_string()
                        } else {
                            format!("{:.2}", v[0] as f32 / v[1] as f32)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            };
            println!(
                "wool_A {seed} {y} price {pw} coin_per_cell {w:.1} free_positive {a_free_pos} pasture {a_cells} coin {a_coin:.0} vs_grain {:.2} by_cost {} by_yield {}",
                a_coin / port_mean.max(1.0),
                share(&a_band_c),
                share(&a_band_y)
            );
            println!(
                "wool_B {seed} {y} price {pw} converted {b_cells}/{claimed_n} share {:.3} coin {b_coin:.0} vs_grain {:.2} grain_lost {b_lost:.0} of_harvest {:.3} of_arrived {:.3} by_cost {} by_yield {}",
                b_cells as f32 / claimed_n.max(1) as f32,
                b_coin / port_mean.max(1.0),
                b_lost / harvest.max(1.0),
                b_lost / arrived.max(1.0),
                share(&b_band_c),
                share(&b_band_y)
            );
        }
    }
}
