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
    /// D174: the share of year-300 road cells still road at the end; NaN with
    /// traffic roads off.
    pub road_persist: f32,
    /// The strict cell measure (D174), reported.
    pub road_persist_cells: f32,
    /// D174: median nearest-neighbour distance among market places (W at
    /// least `market_min`), km, at the end.
    pub market_spacing_km: f32,
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
    /// v5 (D64): grain bought by the outside ports plus imports sold, as a
    /// share of all gross market inflow, the last year; NaN with ports off.
    pub outside_share: f32,
    /// Exports a head c. 1300, pence (D126): grain, wool and cloth the ports
    /// paid for, the mean over years 195-204, against England's about 14 d.
    pub exports_1300: f32,
    /// Exports over output (D139): grain, wool and cloth the ports paid for,
    /// over the region's output (the harvest at its price, the wool clip at
    /// the port price, goods sold at their price, and cloth by its value
    /// added, D141), the last year; NaN with ports off.
    pub exports_output: f32,
    /// S2′ (D80): relief arrived per year, the share of deficit met outside
    /// and in the famine (1315–17), and surplus lost above `food_cap` per
    /// year; `None` with relief off.
    pub relief: Option<[f32; 6]>,
    /// S3 (D81): regional stores as a share of a year's need: mean over the
    /// ten years before the famine, at its start, at its end (1317), at years
    /// 100 and 400; `None` with stores off.
    pub stores: Option<[f32; 5]>,
    /// Total population at year 200, and (W1, filled in by `cmd_validate`)
    /// its ratio to the same seed without local harvests.
    pub pop_200: f32,
    /// v7 (D90): grain unsold at markets as a share of grain arriving, after
    /// year 200; NaN without marketplaces.
    pub waste: f32,
    /// v7 (D88): Clark's regressions on the market grain prices, years
    /// 100 to the end: `b_P`, `b_Y`, R² of the regional price; NaN when grain
    /// is the S2 record.
    pub clark: (f32, f32, f32),
    /// v7 M3 (D94): lag-1 autocorrelation of the detrended regional grain
    /// price (years 100 on); market stores after cheap years as a share of
    /// the mean harvest (mean, max); median price gap over transport on
    /// market pairs in the last year.
    pub price_ac: f32,
    pub carry: (f32, f32, f32),
    pub gap_ratio: f32,
    /// D95: share of the farmers' shortfall met by their purses (normal
    /// years, 1315–17), and with relief added in 1315–17; mean purse over need
    /// at years 200, 214, 217 and 400 for places under 500 and over 2,000.
    pub purse_met: (f32, f32, f32),
    pub purse_level: [[f32; 4]; 2],
    /// D97: the regional grain price (population-weighted median over the
    /// places with a price) at each 50-year mark, years 50 to 500; NaN
    /// without marketplaces.
    pub price_marks: [f32; 10],
    /// D97 money: stock over the region's need at the 100-year marks; the
    /// farmers' share of the stock then; coin over the run: ports in,
    /// planted, imports out, luxuries out, wear, removed.
    pub money_marks: [f32; 5],
    pub farm_share: [f32; 5],
    pub coin: [f32; 6],
    /// L1 (D107): pasture share of used land at 1340 and 1450; at 1450, the
    /// pasture share beyond and within the median travel cost to a large
    /// market, and on land below and above the median yield; wool's share of
    /// coin entering before and after 1348; median over large markets of
    /// wool coin over grain drawn at 1500 ("W with wool").
    pub pasture: [f32; 2],
    pub pasture_cost: (f32, f32),
    pub pasture_yield: (f32, f32),
    pub wool_coin: (f32, f32),
    pub w_wool: f32,
    pub weather_ratio: f32,
    /// v5 (D65): share of the people in places over 2,000 who live in coastal
    /// ones, at years 200 and 500 (1300 and 1600); and, for the top ten at
    /// year 300 (1400), the mean absolute rank change to year 500 and how many
    /// are still in the top ten.
    pub coastal: (f32, f32),
    pub top10_churn: (f32, u32),
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
    /// Years after the plague until the total is back to its level the year
    /// before it (NaN if never).
    pub plague_years: f32,
    /// Total at the end against the same seed with events off; filled in by
    /// `cmd_validate`.
    pub pop_vs_off: f32,
    /// The silted port's rank by population the year it silted and 100 years
    /// later (D60); NaN if there was no silting, or the later rank if the
    /// place failed (then one past the last).
    pub silt_rank: (f32, f32),
    /// Planted towns old enough to judge (planted 100 years or more before the
    /// end) and those of them alive 100 years after planting (D63).
    pub planted: (u32, u32),
    /// Share of this run's top ten within 5 km of the same seed's top ten with
    /// events off; filled in by `cmd_validate`.
    pub overlap_off: f32,
}

/// The v2 gate's measures at the final year.
#[derive(Clone, Debug)]
pub struct GoodsResult {
    /// Share of resource sites that sold anything (1 with goods off or no sites).
    pub sites_share: f32,
    /// Under `market_goods` (D73) the share is reported, not gated; the gate
    /// is instead that every good with a site has a working one.
    pub market: bool,
    pub every_good_worked: bool,
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
        market: false,
        every_good_worked: true,
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
    let every_good_worked = st
        .sites
        .iter()
        .all(|s| st.sites.iter().any(|o| o.good == s.good && o.sold > 0.0));
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
        let q = &s.reach.partner(f.partner_idx);
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
        market: p.market_goods,
        every_good_worked,
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

/// The share of `was` road cells with road within `r` cells of them in `now`;
/// NaN with no roads recorded (D174, D176).
fn road_near(was: &[bool], now: &[bool], t: &hinterlands_core::Terrain, r: i32) -> f32 {
    if was.is_empty() {
        return f32::NAN;
    }
    let g = t.grid;
    let (mut n, mut kept) = (0usize, 0usize);
    for (c, &w) in was.iter().enumerate() {
        if !w {
            continue;
        }
        n += 1;
        let (x, y) = g.xy(c);
        let mut near = false;
        'look: for dy in -r..=r {
            for dx in -r..=r {
                let (nx, ny) = (x + dx, y + dy);
                if nx >= 0 && ny >= 0 && (nx as u32) < g.w && (ny as u32) < g.h && now[(ny as u32 * g.w + nx as u32) as usize] {
                    near = true;
                    break 'look;
                }
            }
        }
        kept += near as usize;
    }
    if n == 0 { f32::NAN } else { kept as f32 / n as f32 }
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
    // Over 2,000 is reported, not gated, since D135.
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
        // D174: among market places (W at least market_min), not places of
        // more than 500 people; the old measure is reported.
        ok: |r| (8.0..=15.0).contains(&r.market_spacing_km),
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
        ok: |r| r.goods.market || r.goods.sites_share >= 2.0 / 3.0,
    },
    // D73: under market_goods, idle marginal sites are the equilibrium.
    Check {
        name: "every good worked",
        target: "true",
        ok: |r| r.goods.every_good_worked,
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
    // Re-founded at D168: England in 1600 stood at about 0.85 of its
    // pre-plague peak (Broadberry et al.); 1.15 is D164's ceiling.
    Check {
        name: "recovery by 500",
        target: "0.75-1.15",
        ok: |r| r.shock.as_ref().is_none_or(|s| (0.75..=1.15).contains(&s.recovery)),
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
    // v5 gate (D64; passes trivially with outside ports off).
    // Outside trade (D139): exports over the region's output, against
    // England's 6-7% c. 1300 (Broadberry et al. 2015); below 1% the region is
    // closed, above about a third an export colony. The share of market
    // inflow (3-50% from D126) is reported.
    Check {
        name: "exports / output",
        target: "1-35%",
        ok: |r| r.exports_output.is_nan() || (0.01..=0.35).contains(&r.exports_output),
    },
    // v7 gate (D88): Clark (1999) on the market grain prices.
    Check {
        name: "Clark b_P",
        target: "0.8-1.2",
        ok: |r| r.clark.0.is_nan() || (0.8..=1.2).contains(&r.clark.0),
    },
    Check {
        name: "Clark b_Y",
        target: "-0.15-0",
        ok: |r| r.clark.1.is_nan() || (-0.15..=0.0).contains(&r.clark.1),
    },
    Check {
        name: "Clark R2",
        target: ">=0.60",
        ok: |r| r.clark.2.is_nan() || r.clark.2 >= 0.6,
    },
    // v7 M3 gate (D97, D99: floor half the world price): the price level stays in range at every 50-year mark.
    Check {
        name: "price level",
        target: "0.3-2.5",
        ok: |r| {
            r.price_marks
                .iter()
                .all(|p| p.is_nan() || (0.3..=2.5).contains(p))
        },
    },
    // L1 gates (D105, D107).
    Check {
        name: "pasture outward",
        target: "far>near",
        ok: |r| r.pasture_cost.0.is_nan() || r.pasture_cost.0 > r.pasture_cost.1,
    },
    Check {
        name: "pasture on poor land",
        target: "poor>rich",
        ok: |r| r.pasture_yield.0.is_nan() || r.pasture_yield.0 > r.pasture_yield.1,
    },
    Check {
        name: "pasture 1450>1340",
        target: "1450>1340",
        ok: |r| r.pasture[1].is_nan() || r.pasture[1] > r.pasture[0],
    },
    // v7 gate (D90; passes trivially without marketplaces).
    Check {
        name: "unsold at markets",
        target: "<=10%",
        ok: |r| r.waste.is_nan() || r.waste <= 0.10,
    },
    // v3 gate (D54; passes trivially with traffic roads off).
    // D174: persistence, not length: the year-300 road cells still road at
    // the end; new roads are allowed. Net length change is reported.
    Check {
        name: "road persistence",
        target: ">=0.90 ±2",
        ok: |r| r.road_persist.is_nan() || r.road_persist >= 0.90,
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
    let mut roads_300: Vec<bool> = Vec::new();
    let road_len = |s: &Sim| s.state.road.iter().filter(|&&r| r).count() as f32;
    let shocks = params.events_mode == 1;
    let mut totals: Vec<f32> = vec![sim.state.total_pop()];
    let mut quartile: std::collections::BTreeMap<u32, u8> = Default::default();
    let mut failed: std::collections::BTreeSet<u32> = Default::default();
    let mut silt: Option<(u32, u32, f32)> = None;
    let mut planted: Vec<(u32, u32)> = Vec::new();
    let coastal_share = |sim: &Sim| {
        let big: Vec<_> = sim
            .state
            .settlements
            .iter()
            .filter(|s| s.pop > 2000.0)
            .collect();
        let all: f32 = big.iter().map(|s| s.pop).sum();
        let coast: f32 = big
            .iter()
            .filter(|s| sim.terrain.is_coastal(s.anchor as usize))
            .map(|s| s.pop)
            .sum();
        coast / all.max(1.0)
    };
    let ranks = |sim: &Sim| -> Vec<u32> {
        let mut v: Vec<_> = sim.state.settlements.iter().collect();
        v.sort_by(|a, b| b.pop.total_cmp(&a.pop).then(a.id.cmp(&b.id)));
        v.iter().map(|s| s.id).collect()
    };
    let mut coastal = (f32::NAN, f32::NAN);
    // Relief: deficit and arrived outside the famine, then in it, then all
    // arrived and all lost.
    let mut relief_acc = [0f32; 8];
    let mut purse_acc = [0f32; 5];
    let mut purse_level = [[f32::NAN; 4]; 2];
    // Regional stores over need at the end of each year (index = year).
    let mut store_share: Vec<f32> = Vec::new();
    let mut pop_200 = f32::NAN;
    let mut exports_acc = (0f64, 0f64);
    let mut waste_acc = (0f32, 0f32);
    let mut price_marks = [f32::NAN; 10];
    let mut money_marks = [f32::NAN; 5];
    let mut farm_share = [f32::NAN; 5];
    let mut coin = [0f32; 6];
    let mut pasture = [f32::NAN; 2];
    let (mut pasture_cost, mut pasture_yield) = ((f32::NAN, f32::NAN), (f32::NAN, f32::NAN));
    // Wool coin and all coin entering, before and after 1348.
    let mut wool_acc = [0f32; 4];
    let mut w_wool = f32::NAN;
    let mut clark_rows: Vec<(u32, u32, f32, f32, f32)> = Vec::new();
    // Regional price, market stores and harvest per year from year 100.
    let mut series: Vec<(f32, f32, f32)> = Vec::new();
    let mut top_300: Vec<u32> = Vec::new();
    let mut planted_alive = (0u32, 0u32);
    let mut silt_after = f32::NAN;
    let rank_of = |sim: &Sim, id: u32| -> f32 {
        let pop = sim.state.get(id).map(|s| s.pop);
        match pop {
            Some(p) => 1.0 + sim.state.settlements.iter().filter(|s| s.pop > p).count() as f32,
            None => 1.0 + sim.state.settlements.len() as f32,
        }
    };
    for _ in 0..years {
        if shocks && sim.state.year == params.plague_year {
            quartile = yield_quarters(&sim);
        }
        sim.step(1);
        if sim.state.year == 200 {
            pop_200 = sim.state.total_pop();
        }
        if (195..=204).contains(&sim.state.year) {
            let l = sim.state.money;
            let pence = (l.port_in + l.wool_in + l.cloth_in) as f64
                * (params.wheat_s_bu * 12.0 * params.bu_per_person) as f64
                / params.port_world_price.max(1e-6) as f64;
            exports_acc.0 += pence;
            exports_acc.1 += sim.state.total_pop() as f64;
        }
        if !params.market_grain && sim.state.year >= 100 {
            let y = sim.state.year;
            let rows: Vec<(u32, f32, f32, f32)> = sim
                .state
                .settlements
                .iter()
                .filter(|s| s.price[0] > 0.0 && !s.fields.is_empty() && s.food_prod > 0.0)
                .map(|s| (s.id, s.price[0], s.food_prod / s.fields.len() as f32, s.pop))
                .collect();
            let wsum: f32 = rows.iter().map(|r| r.3).sum();
            if wsum > 0.0 {
                let regional = rows.iter().map(|r| r.1 * r.3).sum::<f32>() / wsum;
                let held: f32 = sim.state.settlements.iter().map(|s| s.mstore).sum();
                let harvest: f32 = sim.state.settlements.iter().map(|s| s.food_prod).sum();
                series.push((regional, held, harvest));
                for (id, pr, yl, _) in rows {
                    clark_rows.push((id, y, pr.ln(), regional.ln(), yl.ln()));
                }
            }
        }
        if !params.market_grain {
            let y = sim.state.year;
            let l = sim.state.money;
            for (k, v) in [
                l.port_in,
                l.planted_in,
                l.imports_out,
                l.luxury_out,
                l.wear,
                l.removed,
            ]
            .into_iter()
            .enumerate()
            {
                coin[k] += v;
            }
            {
                let k = if y < 249 { 0 } else { 2 };
                if y <= 400 {
                    wool_acc[k] += l.wool_in;
                    wool_acc[k + 1] += l.wool_in + l.port_in + l.planted_in;
                }
            }
            if y == 240 || y == 350 {
                let used = sim
                    .state
                    .owner
                    .iter()
                    .filter(|&&o| o != hinterlands_core::state::NONE)
                    .count();
                let grazed = sim
                    .state
                    .grazed
                    .iter()
                    .filter(|&&g| g != hinterlands_core::state::NONE)
                    .count();
                let share = grazed as f32 / (used + grazed).max(1) as f32;
                pasture[if y == 240 { 0 } else { 1 }] = share;
                if y == 350 {
                    let st = &sim.state;
                    let big: Vec<_> = st
                        .settlements
                        .iter()
                        .filter(|s| s.market >= params.market_min)
                        .collect();
                    let mut cells: Vec<(f32, f32, bool)> = Vec::new();
                    for c in 0..st.owner.len() {
                        let past = st
                            .grazed
                            .get(c)
                            .is_some_and(|&g| g != hinterlands_core::state::NONE);
                        if st.owner[c] == hinterlands_core::state::NONE && !past {
                            continue;
                        }
                        let cost = big
                            .iter()
                            .map(|s| s.reach.cost_to_cell(c))
                            .fold(f32::INFINITY, f32::min);
                        cells.push((cost, sim.terrain.food_yield[c], past));
                    }
                    let split = |key: &dyn Fn(&(f32, f32, bool)) -> f32| -> (f32, f32) {
                        let mut v: Vec<f32> = cells.iter().map(|x| key(x)).collect();
                        v.sort_by(|a, b| a.total_cmp(b));
                        let mid = v.get(v.len() / 2).copied().unwrap_or(0.0);
                        let (mut hi, mut hin, mut lo, mut lon) = (0f32, 0f32, 0f32, 0f32);
                        for x in &cells {
                            if key(x) >= mid {
                                hi += x.2 as u32 as f32;
                                hin += 1.0;
                            } else {
                                lo += x.2 as u32 as f32;
                                lon += 1.0;
                            }
                        }
                        (hi / hin.max(1.0), lo / lon.max(1.0))
                    };
                    pasture_cost = split(&|x| x.0);
                    let (rich, poor) = split(&|x| x.1);
                    pasture_yield = (poor, rich);
                }
            }
            if y == 400 {
                let mut r: Vec<f32> = sim
                    .state
                    .settlements
                    .iter()
                    .filter(|s| s.market >= params.market_min && s.market_in > 0.0)
                    .map(|s| s.wool_in / s.price[0].max(1e-3) / s.market_in)
                    .collect();
                r.sort_by(|a, b| a.total_cmp(b));
                w_wool = r.get(r.len() / 2).copied().unwrap_or(f32::NAN);
            }
            if y % 50 == 0 && (50..=500).contains(&y) {
                let mut ps: Vec<(f32, f32)> = sim
                    .state
                    .settlements
                    .iter()
                    .filter(|s| s.price[0] > 0.0)
                    .map(|s| (s.price[0], s.pop))
                    .collect();
                ps.sort_by(|a, b| a.0.total_cmp(&b.0));
                let half = ps.iter().map(|x| x.1).sum::<f32>() / 2.0;
                let mut acc = 0.0;
                for (pr, w) in ps {
                    acc += w;
                    if acc >= half {
                        price_marks[(y / 50 - 1) as usize] = pr;
                        break;
                    }
                }
                if y % 100 == 0 {
                    let k = (y / 100 - 1) as usize;
                    let need: f32 = sim.state.settlements.iter().map(|s| s.need(params)).sum();
                    let stock = sim.state.money_stock();
                    let f: f32 = sim.state.settlements.iter().map(|s| s.purse).sum();
                    money_marks[k] = stock / need.max(1.0);
                    farm_share[k] = f / stock.max(1e-6);
                }
            }
        }
        if !params.market_grain && sim.state.year > 200 {
            waste_acc.0 += sim.state.grain_stats.unsold;
            waste_acc.1 += sim.state.grain_stats.arrived;
        }
        if params.grain_stores {
            let need: f32 = sim.state.settlements.iter().map(|s| s.need(params)).sum();
            let held: f32 = sim.state.settlements.iter().map(|s| s.store).sum();
            store_share.push(held / need.max(1.0));
        }
        if !params.market_grain {
            let gs = sim.state.grain_stats;
            let y = sim.state.year - 1;
            let famine =
                (params.famine_year..params.famine_year + params.famine_years).contains(&y);
            let k = if famine { 2 } else { 0 };
            purse_acc[k] += gs.dearth_need;
            purse_acc[k + 1] += gs.dearth_bought;
            if famine {
                purse_acc[4] += sim.state.relief_stats[2] + sim.state.relief_stats[4];
            }
            if let Some(slot) = [200u32, 214, 217, 400]
                .iter()
                .position(|&a| a == sim.state.year)
            {
                for (c, small) in [true, false].iter().enumerate() {
                    let ps: Vec<f32> = sim
                        .state
                        .settlements
                        .iter()
                        .filter(|s| {
                            if *small {
                                s.pop < 500.0
                            } else {
                                s.pop > 2000.0
                            }
                        })
                        .map(|s| s.purse / s.need(params).max(1.0))
                        .collect();
                    purse_level[c][slot] = if ps.is_empty() {
                        f32::NAN
                    } else {
                        ps.iter().sum::<f32>() / ps.len() as f32
                    };
                }
            }
        }
        if params.grain_relief {
            let [d, _, a, l, own] = sim.state.relief_stats;
            let y = sim.state.year - 1;
            let famine =
                (params.famine_year..params.famine_year + params.famine_years).contains(&y);
            let k = if famine { 2 } else { 0 };
            relief_acc[k] += d;
            relief_acc[k + 1] += a + own;
            relief_acc[4] += a;
            relief_acc[5] += l;
            relief_acc[6] += own;
            relief_acc[7] += d;
        }
        if shocks {
            totals.push(sim.state.total_pop());
            let y = sim.state.year;
            for &(id, py) in &planted {
                if y == py + 100 {
                    planted_alive.0 += 1;
                    planted_alive.1 += sim.state.get(id).is_some() as u32;
                }
            }
            for e in &sim.state.log {
                if matches!(e.kind, hinterlands_core::state::LogKind::Planted) {
                    planted.push((e.settlement, y));
                }
                if matches!(e.kind, hinterlands_core::state::LogKind::Silted) && silt.is_none() {
                    silt = Some((e.settlement, y, rank_of(&sim, e.settlement)));
                }
            }
            if let Some((id, sy, _)) = silt {
                if y == sy + 100 {
                    silt_after = rank_of(&sim, id);
                }
            }
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
            roads_300 = sim.state.road.clone();
            top_300 = ranks(&sim).into_iter().take(10).collect();
        }
        if sim.state.year == 200 {
            coastal.0 = coastal_share(&sim);
        }
        let tot = sim.state.total_pop();
        // With events, the event years and the one after are not steady state
        // (D58).
        let shocked = shocks && {
            let y = sim.state.year;
            let fy = params.famine_year;
            let in_famine = y >= fy && y <= fy + params.famine_years + 1;
            // Every calendar outbreak and the year after (D58; D170): the
            // first three, and the later list in use.
            let later: Vec<u32> = if params.plague_sourced {
                hinterlands_core::systems::events::SOURCED_PLAGUES
                    .iter()
                    .map(|&(ad, _)| ad)
                    .collect()
            } else if params.plague_later {
                hinterlands_core::systems::events::LATER_PLAGUES.to_vec()
            } else {
                Vec::new()
            };
            let in_plague = [
                params.plague_year,
                params.plague_recur_1,
                params.plague_recur_2,
            ]
            .into_iter()
            .chain(later.iter().map(|&ad| (ad + params.plague_year).saturating_sub(1348)))
            .any(|py| y >= py && y <= py + params.plague_years + 1);
            // A planting in year Y shows in the total read after it, Y + 1.
            let planting = params.plant_interval > 0
                && y > 1
                && y > params.plant_from
                && (y - 1) % params.plant_interval == 0;
            in_famine || in_plague || planting
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
    let ratio = |a: f32, d: f32| if d > 0.0 { a / d } else { f32::NAN };
    let relief = params.grain_relief.then(|| {
        [
            relief_acc[4] / years.max(1) as f32,
            ratio(relief_acc[1], relief_acc[0]),
            ratio(relief_acc[3], relief_acc[2]),
            relief_acc[5] / years.max(1) as f32,
            ratio(relief_acc[6], relief_acc[7]),
            ratio(relief_acc[4], relief_acc[7]),
        ]
    });
    let stores = params.grain_stores.then(|| {
        let at = |y: u32| store_share.get(y as usize).copied().unwrap_or(f32::NAN);
        let f = params.famine_year;
        let before: Vec<f32> = (f.saturating_sub(10)..f).map(at).collect();
        [
            before.iter().sum::<f32>() / before.len().max(1) as f32,
            at(f - 1),
            at(f + params.famine_years - 1),
            at(100),
            at(400),
        ]
    });
    let clark = if params.market_grain {
        (f32::NAN, f32::NAN, f32::NAN)
    } else {
        clark(&clark_rows)
    };
    let (price_ac, carry) = price_series(&series);
    let gaps: Vec<f32> = sim
        .state
        .market_pairs
        .iter()
        .filter(|a| a.4 > 0.0)
        .map(|a| a.3 / a.4)
        .collect();
    let gap_ratio = median_of(gaps);
    let purse_met = (
        purse_acc[1] / purse_acc[0].max(1.0),
        purse_acc[3] / purse_acc[2].max(1.0),
        (purse_acc[3] + purse_acc[4]) / purse_acc[2].max(1.0),
    );
    RunResult {
        pasture,
        pasture_cost,
        pasture_yield,
        wool_coin: (
            wool_acc[0] / wool_acc[1].max(1e-6),
            wool_acc[2] / wool_acc[3].max(1e-6),
        ),
        w_wool,
        price_marks,
        money_marks,
        farm_share,
        coin,
        purse_met,
        purse_level,
        price_ac,
        carry,
        gap_ratio,
        clark,
        waste: if !params.market_grain {
            waste_acc.0 / waste_acc.1.max(1.0)
        } else {
            f32::NAN
        },
        pop_200,
        weather_ratio: f32::NAN,
        stores,
        relief,
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
        shock: shocks.then(|| {
            let mut sh = shock_result(params, &totals, &quartile, &failed);
            sh.silt_rank = (silt.map_or(f32::NAN, |s| s.2), silt_after);
            sh.planted = planted_alive;
            sh
        }),
        road_persist_cells: if params.traffic_roads && !roads_300.is_empty() {
            let was = roads_300.iter().filter(|&&r| r).count();
            let kept = roads_300
                .iter()
                .zip(&sim.state.road)
                .filter(|(&a, &b)| a && b)
                .count();
            kept as f32 / was.max(1) as f32
        } else {
            f32::NAN
        },
        // D176: on routes, a year-300 road cell kept if road lies within two
        // cells of it at the end.
        road_persist: road_near(&roads_300, &sim.state.road, t, 2),
        market_spacing_km: {
            let g = t.grid;
            let mk: Vec<usize> = sim
                .state
                .settlements
                .iter()
                .filter(|s| s.market >= params.market_min)
                .map(|s| s.anchor as usize)
                .collect();
            let mut nn: Vec<f32> = mk
                .iter()
                .filter_map(|&a| {
                    mk.iter()
                        .filter(|&&b| b != a)
                        .map(|&b| g.dist_km(a, b))
                        .min_by(|x, y| x.total_cmp(y))
                })
                .collect();
            nn.sort_by(|a, b| a.total_cmp(b));
            nn.get(nn.len() / 2).copied().unwrap_or(f32::NAN)
        },
        road_change: if params.traffic_roads {
            (road_len(&sim) - road_300).abs() / road_300.max(1.0)
        } else {
            f32::NAN
        },
        coastal: (coastal.0, coastal_share(&sim)),
        top10_churn: {
            let now = ranks(&sim);
            let n = now.len() as f32;
            let moves: Vec<f32> = top_300
                .iter()
                .enumerate()
                .map(|(r, id)| {
                    let later = now.iter().position(|x| x == id).map_or(n, |p| p as f32);
                    (later - r as f32).abs()
                })
                .collect();
            let kept = top_300
                .iter()
                .filter(|id| now.iter().take(10).any(|x| x == *id))
                .count() as u32;
            (moves.iter().sum::<f32>() / moves.len().max(1) as f32, kept)
        },
        exports_1300: if exports_acc.1 > 0.0 {
            (exports_acc.0 / exports_acc.1) as f32
        } else {
            f32::NAN
        },
        exports_output: if params.outside_ports {
            let st = &sim.state;
            let l = st.money;
            let to_coin = params.port_world_price
                / (params.wheat_s_bu * 12.0 * params.bu_per_person).max(1e-6);
            let grain_p = |s: &hinterlands_core::state::Settlement| {
                if s.price[0] > 0.0 {
                    s.price[0]
                } else {
                    params.port_world_price
                }
            };
            let harvest: f32 = st.settlements.iter().map(|s| s.food_prod * grain_p(s)).sum();
            let goods: f32 = st
                .sites
                .iter()
                .filter_map(|x| {
                    let i = st.index_of(x.holder)?;
                    let p = st.settlements[i].price[1 + x.good.index()];
                    Some(x.sold * if p > 0.0 { p } else { 1.0 })
                })
                .sum();
            // Wool is the whole clip at the port price; cloth counts by its
            // value added, cloth less the wool it uses at the same price
            // (D141), so milled wool is counted once.
            let vw = st.cloth_trace[8] * to_coin;
            let wool = st.cloth_stats[0] * vw;
            let cloth_added = (l.cloth_in - st.cloth_stats[1] * vw).max(0.0);
            let exports = l.port_in + l.wool_in + l.cloth_in;
            exports / (harvest + goods + wool + cloth_added).max(1e-6)
        } else {
            f32::NAN
        },
        outside_share: if params.outside_ports {
            let (g, i) = sim.state.outside_trade;
            // Wool is trade with the outside too (D108), and export cloth (D2),
            // in grain units.
            let w = (sim.state.money.wool_in + sim.state.money.cloth_in)
                / params.port_world_price.max(1e-6);
            let inflow: f32 = sim.state.settlements.iter().map(|s| s.market_in).sum();
            (g + i + w) / (inflow + w).max(1.0)
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
/// mean yield of the land within 3 km of its anchor (D58), among settlements
/// more than 8 km from any place over 2,000, beyond the urban drain (D59).
fn yield_quarters(sim: &Sim) -> std::collections::BTreeMap<u32, u8> {
    let t = &sim.terrain;
    let big: Vec<usize> = sim
        .state
        .settlements
        .iter()
        .filter(|s| s.pop > 2000.0)
        .map(|s| s.anchor as usize)
        .collect();
    let mut v: Vec<(f32, u32)> = sim
        .state
        .settlements
        .iter()
        .filter(|s| {
            big.iter()
                .all(|&b| b == s.anchor as usize || t.grid.dist_km(b, s.anchor as usize) > 8.0)
        })
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
    let dip_low = (fy + 1..=fy + p.famine_years + 5)
        .map(at)
        .fold(f32::INFINITY, f32::min);
    let back = (fy + 1..py)
        .find(|&y| at(y) >= pre)
        .map_or(f32::NAN, |y| (y - fy) as f32);
    let regained = (py + 1..totals.len() as u32)
        .find(|&y| at(y) >= before)
        .map_or(f32::NAN, |y| (y - py) as f32);
    ShockResult {
        plague_years: regained,
        pop_vs_off: f32::NAN,
        silt_rank: (f32::NAN, f32::NAN),
        planted: (0, 0),
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
    let places: Vec<_> = snap
        .settlements
        .iter()
        .filter(|s| s.pop >= min_pop)
        .collect();
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

/// The accepted model's counts on 40 seeds where it is itself below 30/40
/// (D120): (map seed, check, runs passing). A stage passes such a check if it
/// is no worse. From R's two halves (`docs/retune/r.md`).
const NO_WORSE: &[(u64, &str, usize)] = &[];

/// D176: paired no-worse checks until a sourced band exists. On seeds 1-40 a
/// map passes if its count is at least the accepted default's count on the
/// same seeds minus 2. The counts are the default's (D161), read at D176.
const PAIRED: &[(u64, &str, usize)] = &[];

fn passes(r: &RunResult) -> bool {
    CHECKS.iter().all(|c| (c.ok)(r))
}

pub fn cmd_validate(args: &Args) {
    let maps = args.u64("maps", 3);
    let seeds = args.u64("seeds", 20);
    // `--first-seed N` runs seeds N..N+seeds (fresh-seed confirmation, D119).
    let first_seed = args.u64("first-seed", 1);
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
        .flat_map(|m| (first_seed..first_seed + seeds).map(move |s| (m, s)))
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
                    let vs_off = r.m.total_pop / o.m.total_pop.max(1.0);
                    if let Some(sh) = r.shock.as_mut() {
                        sh.overlap_off = share;
                        sh.pop_vs_off = vs_off;
                    }
                }
                // W1: the same seed without local harvests, to year 200.
                if params.local_harvests {
                    let mut dry = params.clone();
                    dry.local_harvests = false;
                    let o = run_one(t, &dry, *map_seed, seed, 200);
                    r.weather_ratio = r.pop_200 / o.pop_200.max(1.0);
                }
                results.lock().unwrap().push(r);
            });
        }
    });
    let mut results = results.into_inner().unwrap();
    results.sort_by_key(|r| (r.map_seed, r.sim_seed));

    // Determinism: rerun the first run of the first map.
    let det = run_one(&terrains[0].1, &params, terrains[0].0, first_seed, years);
    let det_ok = results
        .iter()
        .find(|r| r.map_seed == terrains[0].0 && r.sim_seed == first_seed)
        .is_some_and(|r| r.hash == det.hash);

    let mut all_ok = det_ok;
    println!(
        "determinism (map {}, seed {first_seed}): {}",
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
                    "over 5,000" => r.m.over_5000 as f32,
                    "largest share" => r.m.largest_share,
                    "rank-size slope" => r.m.rank_slope,
                    "market spacing" => r.market_spacing_km,
                    "siting" => r.m.siting_share,
                    "failure rate" => r.m.failure_rate,
                    "forest near cities" => r.m.forest_near_cities,
                    "sites producing" => r.goods.sites_share,
                    "every good worked" => {
                        if r.goods.every_good_worked {
                            1.0
                        } else {
                            0.0
                        }
                    }
                    "largest not specialist" => {
                        if r.goods.largest_specialist {
                            0.0
                        } else {
                            1.0
                        }
                    }
                    "road persistence" => r.road_persist,
                    "exports / output" => r.exports_output,
                    "unsold at markets" => r.waste,
                    "pasture outward" => r.pasture_cost.0 - r.pasture_cost.1,
                    "pasture on poor land" => r.pasture_yield.0 - r.pasture_yield.1,
                    "pasture 1450>1340" => r.pasture[1] - r.pasture[0],
                    "price level" => r
                        .price_marks
                        .iter()
                        .copied()
                        .filter(|p| p.is_finite())
                        .fold(f32::NAN, |a, p| {
                            // The mark furthest outside 1.0 in log terms.
                            if a.is_nan() || p.ln().abs() > a.ln().abs() {
                                p
                            } else {
                                a
                            }
                        }),
                    "Clark b_P" => r.clark.0,
                    "Clark b_Y" => r.clark.1,
                    "Clark R2" => r.clark.2,
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
            // v1 rule (D21), and D120's no-worse rule on 40 seeds: where the
            // accepted model is itself short, no worse than its count.
            let floor = NO_WORSE
                .iter()
                .find(|f| f.0 == *map_seed && f.1 == c.name && rs.len() == 40)
                .map(|f| f.2);
            let paired = PAIRED
                .iter()
                .find(|f| f.0 == *map_seed && f.1 == c.name && rs.len() == 40 && first_seed == 1)
                .map(|f| f.2);
            let rate_ok = if let Some(r) = paired {
                pass + 2 >= r
            } else {
                pass * 4 >= rs.len() * 3 || floor.is_some_and(|f| pass >= f)
            };
            let reported = c.name == "sites producing" && rs.iter().all(|r| r.goods.market);
            println!(
                "{:<20} {:>12} {:>10.3} {:>10.3} {:>7}/{:<2} {:>8}",
                c.name,
                if reported { "(reported)" } else { c.target },
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
        // Reported, not gated, since D135: the count over 2,000 and the urban
        // share against England's c. 10% c. 1300.
        let mut over2: Vec<u32> = rs.iter().map(|r| r.m.over_2000).collect();
        over2.sort();
        let mut urban: Vec<f32> = rs.iter().map(|r| r.m.urban_share).collect();
        urban.sort_by(|a, b| a.total_cmp(b));
        println!(
            "{:<20} {:>12} {:>10} {:>10} {:>10} {:>8}",
            "over 2,000",
            "(reported)",
            canon.map_or(String::new(), |r| r.m.over_2000.to_string()),
            over2.get(over2.len() / 2).copied().unwrap_or(0),
            format!("{}-{}", over2.first().unwrap_or(&0), over2.last().unwrap_or(&0)),
            ""
        );
        println!(
            "{:<20} {:>12} {:>10} {:>10.3} {:>10} {:>8}",
            "urban share >2,000",
            "(Eng. ~0.10)",
            canon.map_or(String::new(), |r| format!("{:.3}", r.m.urban_share)),
            urban.get(urban.len() / 2).copied().unwrap_or(f32::NAN),
            "",
            ""
        );
        // D174: the old measures, reported.
        {
            let med = |mut v: Vec<f32>| {
                v.retain(|x| x.is_finite());
                v.sort_by(|a, b| a.total_cmp(b));
                v.get(v.len() / 2).copied().unwrap_or(f32::NAN)
            };
            let old_sp: Vec<f32> = rs.iter().map(|r| r.m.spacing_median_km).collect();
            let old_ok = old_sp.iter().filter(|x| (8.0..=15.0).contains(*x)).count();
            println!(
                "{:<20} {:>12} {:>10.3} {:>10.3} {:>7}/{:<2} {:>8}",
                "spacing, over 500",
                "(rep. 8-15)",
                canon.map_or(f32::NAN, |r| r.m.spacing_median_km),
                med(old_sp),
                old_ok,
                rs.len(),
                ""
            );
            println!(
                "{:<20} {:>12} {:>10.3} {:>10.3} {:>10} {:>8}",
                "road persist cells",
                "(reported)",
                canon.map_or(f32::NAN, |r| r.road_persist_cells),
                med(rs.iter().map(|r| r.road_persist_cells).collect()),
                "",
                ""
            );
            let rc: Vec<f32> = rs.iter().map(|r| r.road_change).collect();
            let rc_ok = rc.iter().filter(|x| **x < 0.10).count();
            println!(
                "{:<20} {:>12} {:>10.3} {:>10.3} {:>7}/{:<2} {:>8}",
                "road length 300-500",
                "(rep. <10%)",
                canon.map_or(f32::NAN, |r| r.road_change),
                med(rc),
                rc_ok,
                rs.len(),
                ""
            );
        }
        // The share of market inflow, gated 3-50% from D126 to D139; reported.
        if rs.iter().any(|r| r.outside_share.is_finite()) {
            let mut v: Vec<f32> = rs.iter().map(|r| r.outside_share).collect();
            v.sort_by(|a, b| a.total_cmp(b));
            let inside = rs
                .iter()
                .filter(|r| (0.03..=0.50).contains(&r.outside_share))
                .count();
            println!(
                "{:<20} {:>12} {:>10.3} {:>10.3} {:>7}/{:<2} {:>8}",
                "outside trade",
                "(rep. 3-50%)",
                canon.map_or(f32::NAN, |r| r.outside_share),
                v.get(v.len() / 2).copied().unwrap_or(f32::NAN),
                inside,
                rs.len(),
                ""
            );
        }
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
                "{:<20} {:>12} plague drop {:.2}, regained in {:.0} years; recovery {:.2}; total at end vs off {:.2}; failure 248-300 low {:.2} / high {:.2} quarter; famine dip {:.2}, regained in {:.0} years",
                "v4 shocks",
                "(reported)",
                med(sh.iter().map(|s| s.plague_drop).collect()),
                med(sh.iter().map(|s| s.plague_years).filter(|x| x.is_finite()).collect()),
                med(sh.iter().map(|s| s.recovery).collect()),
                med(sh.iter().map(|s| s.pop_vs_off).collect()),
                med(sh.iter().map(|s| s.fail_low).collect()),
                med(sh.iter().map(|s| s.fail_high).collect()),
                med(sh.iter().map(|s| s.famine_dip).collect()),
                med(sh.iter().map(|s| s.famine_years).filter(|x| x.is_finite()).collect())
            );
            let (judged, alive) = sh
                .iter()
                .fold((0, 0), |a, s| (a.0 + s.planted.0, a.1 + s.planted.1));
            if judged > 0 {
                println!(
                    "{:<20} {:>12} planted towns judged {}; alive 100 years after planting {:.2}",
                    "v4 planted",
                    "(reported)",
                    judged,
                    alive as f32 / judged as f32
                );
            }
            let silted: Vec<&&ShockResult> =
                sh.iter().filter(|s| s.silt_rank.0.is_finite()).collect();
            if !silted.is_empty() {
                let worse = silted
                    .iter()
                    .filter(|s| s.silt_rank.1 > s.silt_rank.0)
                    .count();
                println!(
                    "{:<20} {:>12} silted in {} runs; rank before {:.0}, 100 years after {:.0} (medians); lost rank in {}",
                    "v4 silting",
                    "(reported)",
                    silted.len(),
                    med(silted.iter().map(|s| s.silt_rank.0).collect()),
                    med(silted.iter().map(|s| s.silt_rank.1).filter(|x| x.is_finite()).collect()),
                    worse
                );
            }
        }
        if rs.iter().any(|r| r.outside_share.is_finite()) {
            println!(
                "{:<20} {:>12} coastal share of people in places >2,000: 1300 {:.2}, 1600 {:.2}; top ten of 1400 by 1600: mean rank change {:.1}, still top ten {:.0}",
                "v5",
                "(reported)",
                med(rs.iter().map(|r| r.coastal.0).filter(|x| x.is_finite()).collect()),
                med(rs.iter().map(|r| r.coastal.1).collect()),
                med(rs.iter().map(|r| r.top10_churn.0).collect()),
                med(rs.iter().map(|r| r.top10_churn.1 as f32).collect())
            );
            println!(
                "{:<20} {:>12} exports a head c. 1300 (grain, wool, cloth to the ports): median {:.1} d, p10 {:.1}, p90 {:.1} (England about 14 d; D126)",
                "D126 exports",
                "(reported)",
                med(rs.iter().map(|r| r.exports_1300).filter(|x| x.is_finite()).collect()),
                pct(rs.iter().map(|r| r.exports_1300).filter(|x| x.is_finite()).collect(), 0.1),
                pct(rs.iter().map(|r| r.exports_1300).filter(|x| x.is_finite()).collect(), 0.9)
            );
        }
        if rs.iter().any(|r| r.relief.is_some()) {
            let at = |k: usize| {
                med(rs
                    .iter()
                    .filter_map(|r| r.relief.map(|v| v[k]))
                    .filter(|x| x.is_finite())
                    .collect())
            };
            println!(
                "{:<20} {:>12} relief arrived per year {:.0}; deficit met: normal years {:.2}, 1315-17 {:.2} (all years: own store {:.2}, relief {:.2}); surplus lost above food_cap per year {:.0}",
                "S2' relief",
                "(reported)",
                at(0),
                at(1),
                at(2),
                at(4),
                at(5),
                at(3)
            );
        }
        if rs.iter().any(|r| r.weather_ratio.is_finite()) {
            let ws: Vec<f32> = rs.iter().map(|r| r.weather_ratio).collect();
            let within = ws.iter().filter(|w| (0.8..=1.2).contains(*w)).count();
            println!(
                "{:<20} {:>12} population at year 200 against no weather: median {:.2}, within 20% in {}/{}",
                "W1 weather",
                "(reported)",
                med(ws.clone()),
                within,
                ws.len()
            );
        }
        if !params.market_grain {
            let a = |f: &dyn Fn(&RunResult) -> f32| {
                med(rs.iter().map(|r| f(r)).filter(|x| x.is_finite()).collect())
            };
            println!(
                "{:<20} {:>12} farmers' shortfall met by purses: normal years {:.2}, 1315-17 {:.2} (with relief {:.2}); purse / need under 500 at 200/1314/1317/400: {:.2} {:.2} {:.2} {:.2}; over 2,000: {:.2} {:.2} {:.2} {:.2}",
                "D95 purses",
                "(reported)",
                a(&|r| r.purse_met.0),
                a(&|r| r.purse_met.1),
                a(&|r| r.purse_met.2),
                a(&|r| r.purse_level[0][0]),
                a(&|r| r.purse_level[0][1]),
                a(&|r| r.purse_level[0][2]),
                a(&|r| r.purse_level[0][3]),
                a(&|r| r.purse_level[1][0]),
                a(&|r| r.purse_level[1][1]),
                a(&|r| r.purse_level[1][2]),
                a(&|r| r.purse_level[1][3])
            );
        }
        if rs
            .iter()
            .any(|r| r.price_marks.iter().any(|p| p.is_finite()))
        {
            let a = |f: &dyn Fn(&RunResult) -> f32| {
                med(rs.iter().map(|r| f(r)).filter(|x| x.is_finite()).collect())
            };
            let marks: Vec<String> = (0..10)
                .map(|k| format!("{:.2}", a(&|r| r.price_marks[k])))
                .collect();
            println!(
                "{:<20} {:>12} regional grain price at 50..500 (world {:.2}): {}",
                "D97 price",
                "(reported)",
                params.port_world_price,
                marks.join(" ")
            );
            let stock: Vec<String> = (0..5)
                .map(|k| format!("{:.2}", a(&|r| r.money_marks[k])))
                .collect();
            let farm: Vec<String> = (0..5)
                .map(|k| format!("{:.2}", a(&|r| r.farm_share[k])))
                .collect();
            println!(
                "{:<20} {:>12} coin stock / need at 100..500: {}; farmers' share {}; over the run, ports in {:.0}, planted {:.0}, imports out {:.0}, luxuries out {:.0}, wear {:.0}, removed {:.0}",
                "D97 money",
                "(reported)",
                stock.join(" "),
                farm.join(" "),
                a(&|r| r.coin[0]),
                a(&|r| r.coin[1]),
                a(&|r| r.coin[2]),
                a(&|r| r.coin[3]),
                a(&|r| r.coin[4]),
                a(&|r| r.coin[5])
            );
        }
        if rs.iter().any(|r| r.pasture[1].is_finite()) {
            let a = |f: &dyn Fn(&RunResult) -> f32| {
                med(rs.iter().map(|r| f(r)).filter(|x| x.is_finite()).collect())
            };
            println!(
                "{:<20} {:>12} pasture share of used land 1340 {:.3}, 1450 {:.3}; at 1450 beyond/within median cost {:.3}/{:.3}, below/above median yield {:.3}/{:.3}; wool's share of coin entering before 1348 {:.2}, after {:.2}; W with wool (wool coin / grain drawn, large markets, 1500) {:.2}",
                "L1 wool",
                "(reported)",
                a(&|r| r.pasture[0]),
                a(&|r| r.pasture[1]),
                a(&|r| r.pasture_cost.0),
                a(&|r| r.pasture_cost.1),
                a(&|r| r.pasture_yield.0),
                a(&|r| r.pasture_yield.1),
                a(&|r| r.wool_coin.0),
                a(&|r| r.wool_coin.1),
                a(&|r| r.w_wool)
            );
        }
        if rs.iter().any(|r| r.price_ac.is_finite()) {
            println!(
                "{:<20} {:>12} grain price autocorrelation (detrended) {:.2}; market stores (of a mean harvest): cheapest tenth of years {:.3}, all years {:.3}, peak {:.2}; pair price gap / transport {:.2}",
                "M3 stores",
                "(reported)",
                med(rs.iter().map(|r| r.price_ac).filter(|x| x.is_finite()).collect()),
                med(rs.iter().map(|r| r.carry.0).filter(|x| x.is_finite()).collect()),
                med(rs.iter().map(|r| r.carry.1).filter(|x| x.is_finite()).collect()),
                med(rs.iter().map(|r| r.carry.2).filter(|x| x.is_finite()).collect()),
                med(rs.iter().map(|r| r.gap_ratio).filter(|x| x.is_finite()).collect())
            );
        }
        if rs.iter().any(|r| r.stores.is_some()) {
            let at = |k: usize| {
                med(rs
                    .iter()
                    .filter_map(|r| r.stores.map(|v| v[k]))
                    .filter(|x| x.is_finite())
                    .collect())
            };
            println!(
                "{:<20} {:>12} stores / need: years 100 {:.2}, 205-214 {:.2}, 214 {:.2}, 217 {:.2} (drawn down {:.0}%), 400 {:.2}",
                "S3 stores",
                "(reported)",
                at(3),
                at(0),
                at(1),
                at(2),
                100.0 * (1.0 - at(2) / at(1).max(1e-6)),
                at(4)
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
        // Which checks bind (D119): how often each fails, and how often it is
        // a run's only failure.
        let mut fails: Vec<(&str, usize, usize)> = CHECKS
            .iter()
            .map(|c| {
                let (mut failed, mut sole) = (0usize, 0usize);
                for r in rs.iter() {
                    let r: &RunResult = r;
                    if !(c.ok)(r) {
                        failed += 1;
                        if CHECKS.iter().filter(|d| !(d.ok)(r)).count() == 1 {
                            sole += 1;
                        }
                    }
                }
                (c.name, failed, sole)
            })
            .filter(|x| x.1 > 0)
            .collect();
        fails.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
        println!(
            "failing checks (runs failing / of which the only failure): {}",
            fails
                .iter()
                .map(|(n, k, o)| format!("{n} {k}/{o}"))
                .collect::<Vec<_>>()
                .join("; ")
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

/// Clark (1999), as `tools/market/clark.py`: within-settlement OLS of the log
/// price on the log regional price and the log yield (`b_P`, `b_Y`), and the
/// pooled R² of the log price on the log regional price. Rows are
/// (settlement, year, ln p, ln P, ln Y).
fn clark(rows: &[(u32, u32, f32, f32, f32)]) -> (f32, f32, f32) {
    use std::collections::BTreeMap;
    let mut by: BTreeMap<u32, Vec<(f64, f64, f64)>> = BTreeMap::new();
    for &(id, _, lp, lpr, ly) in rows {
        by.entry(id)
            .or_default()
            .push((lp as f64, lpr as f64, ly as f64));
    }
    let (mut saa, mut sab, mut sbb, mut say, mut sby) = (0f64, 0f64, 0f64, 0f64, 0f64);
    for v in by.values().filter(|v| v.len() >= 3) {
        let n = v.len() as f64;
        let my = v.iter().map(|r| r.0).sum::<f64>() / n;
        let ma = v.iter().map(|r| r.1).sum::<f64>() / n;
        let mb = v.iter().map(|r| r.2).sum::<f64>() / n;
        for r in v {
            let (dy, da, db) = (r.0 - my, r.1 - ma, r.2 - mb);
            saa += da * da;
            sab += da * db;
            sbb += db * db;
            say += da * dy;
            sby += db * dy;
        }
    }
    let det = saa * sbb - sab * sab;
    let (bp, by_) = if det.abs() < 1e-12 {
        (f64::NAN, f64::NAN)
    } else {
        ((say * sbb - sby * sab) / det, (saa * sby - sab * say) / det)
    };
    let n = rows.len() as f64;
    let mx = rows.iter().map(|r| r.3 as f64).sum::<f64>() / n.max(1.0);
    let my = rows.iter().map(|r| r.2 as f64).sum::<f64>() / n.max(1.0);
    let (mut sxy, mut sxx, mut syy) = (0f64, 0f64, 0f64);
    for r in rows {
        let (dx, dy) = (r.3 as f64 - mx, r.2 as f64 - my);
        sxy += dx * dy;
        sxx += dx * dx;
        syy += dy * dy;
    }
    let r2 = if sxx > 0.0 && syy > 0.0 {
        sxy * sxy / (sxx * syy)
    } else {
        f64::NAN
    };
    (bp as f32, by_ as f32, r2 as f32)
}

/// Lag-1 autocorrelation of the log regional price less its centred 21-year
/// moving average, and the market stores at the end of cheap years (price
/// below 0.8 of that trend) as a share of the mean harvest: (mean, max).
fn price_series(series: &[(f32, f32, f32)]) -> (f32, (f32, f32, f32)) {
    let n = series.len();
    if n < 30 {
        return (f32::NAN, (f32::NAN, f32::NAN, f32::NAN));
    }
    let lp: Vec<f64> = series.iter().map(|s| (s.0.max(1e-6) as f64).ln()).collect();
    let mut dev = Vec::new();
    let mut trend = Vec::new();
    for t in 10..n - 10 {
        let m = lp[t - 10..=t + 10].iter().sum::<f64>() / 21.0;
        dev.push(lp[t] - m);
        trend.push((t, m.exp()));
    }
    let mean = dev.iter().sum::<f64>() / dev.len() as f64;
    let (mut num, mut den) = (0f64, 0f64);
    for k in 0..dev.len() {
        den += (dev[k] - mean).powi(2);
        if k > 0 {
            num += (dev[k] - mean) * (dev[k - 1] - mean);
        }
    }
    let ac = if den > 0.0 { num / den } else { f64::NAN };
    let mean_h = series.iter().map(|s| s.2 as f64).sum::<f64>() / n as f64;
    // Stores (D124): the mean held in the cheapest tenth of years by price
    // against trend, the mean over all years, and the peak, of a mean harvest.
    let mut by_dev: Vec<(f64, f64)> = trend
        .iter()
        .map(|(t, tr)| {
            (
                series[*t].0 as f64 / tr.max(1e-9),
                series[*t].1 as f64 / mean_h.max(1.0),
            )
        })
        .collect();
    by_dev.sort_by(|a, b| a.0.total_cmp(&b.0));
    let tenth = (by_dev.len() / 10).max(1).min(by_dev.len());
    let carry = if by_dev.is_empty() {
        (0.0, 0.0, 0.0)
    } else {
        let all: Vec<f64> = by_dev.iter().map(|x| x.1).collect();
        (
            (by_dev[..tenth].iter().map(|x| x.1).sum::<f64>() / tenth as f64) as f32,
            (all.iter().sum::<f64>() / all.len() as f64) as f32,
            all.iter().cloned().fold(0.0, f64::max) as f32,
        )
    };
    (ac as f32, carry)
}

fn median_of(mut v: Vec<f32>) -> f32 {
    if v.is_empty() {
        return f32::NAN;
    }
    v.sort_by(|a, b| a.total_cmp(b));
    v[v.len() / 2]
}

/// The `q` quantile of a list (nearest rank); NaN when empty.
fn pct(mut v: Vec<f32>, q: f32) -> f32 {
    if v.is_empty() {
        return f32::NAN;
    }
    v.sort_by(|a, b| a.total_cmp(b));
    v[((v.len() - 1) as f32 * q).round() as usize]
}
