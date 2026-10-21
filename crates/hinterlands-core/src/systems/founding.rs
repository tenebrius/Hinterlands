//! Founding (Bosker & Buringh 2017 site weights; Christaller 1933 and Krugman 1991
//! urban shadow; Bleakley & Lin 2012 break-of-bulk via the hub term). Every
//! `founding_interval` years each free land cell with water nearby and no
//! settlement close is scored, and the best few become hamlets, peopled by
//! migrants from the nearest well-fed settlement.
//! Writes: settlements (adds), site_score, tally, log.

use crate::params::Params;
use crate::state::{LogEntry, LogKind, State, NONE};
use crate::terrain::Terrain;
use crate::Rng;
use rand::Rng as _;

/// The parts of a site score, kept for the inspector.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SiteParts {
    pub hub: f32,
    pub river: f32,
    pub coast: f32,
    pub road: f32,
    pub food: f32,
    pub shadow: f32,
    pub access: f32,
    /// Water-supply bonus, already weighted (D29).
    pub water: f32,
}

impl SiteParts {
    pub fn total(&self, p: &Params) -> f32 {
        p.w_hub * self.hub
            + p.w_river * self.river
            + p.w_coast * self.coast
            + p.w_road * self.road
            + p.w_food * self.food
            - self.shadow
            + self.access
            + self.water
    }
}

/// A cell a village can draw water from: a brook or larger stream, or the sea.
fn brook_or_sea(t: &Terrain, p: &Params, j: usize) -> bool {
    t.is_sea(j) || (t.is_land(j) && t.flux[j] >= p.brook_area)
}

/// Whether a cell has a water supply for a settlement. v1: any stream, river or
/// sea within `water_radius_km`. With `spring_water` (D29): a brook of at least
/// `brook_area` or the coast within `supply_radius_km`.
pub fn has_water_supply(t: &Terrain, p: &Params, cell: usize) -> bool {
    if p.spring_water {
        t.grid
            .disk(cell, p.supply_radius_km)
            .any(|j| brook_or_sea(t, p, j))
    } else {
        t.water_within(cell, p.water_radius_km)
    }
}

/// The water-supply score term (D29): a brook or river in the cell, or one or
/// the sea within 1 km.
pub fn water_bonus(t: &Terrain, p: &Params, cell: usize) -> f32 {
    if !p.spring_water {
        0.0
    } else if t.flux[cell] >= p.brook_area {
        p.w_water_cell
    } else if t.grid.disk(cell, 1.0).any(|j| brook_or_sea(t, p, j)) {
        p.w_water_near
    } else {
        0.0
    }
}

/// Cells that may receive a new settlement.
pub fn is_candidate(state: &State, t: &Terrain, p: &Params, cell: usize) -> bool {
    t.is_land(cell)
        && state.owner[cell] == NONE
        && has_water_supply(t, p, cell)
        && !state
            .settlements
            .iter()
            .any(|s| t.grid.dist_km(s.anchor as usize, cell) < p.min_spacing_km)
}

/// Free yield reachable on foot within `r_cult`, in prime-cell units.
pub fn free_food_near(state: &mut State, t: &Terrain, p: &Params, cell: usize) -> f32 {
    let mut dj = std::mem::take(&mut state.scratch);
    dj.run(&state.cost, cell as u32, p.r_cult, true);
    let total = dj
        .touched()
        .iter()
        .map(|&n| n as usize)
        .filter(|&c| state.owner[c] == NONE && t.food_yield[c] >= p.min_field_yield)
        .map(|c| t.food_yield[c])
        .sum();
    state.scratch = dj;
    total
}

/// Least cost per km of any walking edge (land to land) in the cost graph.
fn min_walk_per_km(state: &State, t: &Terrain) -> f32 {
    let g = t.grid;
    let n = t.n();
    let mut best = f32::INFINITY;
    for u in 0..n {
        for &(v, c) in state.cost.edges(u as u32) {
            if (v as usize) < n && v as usize != u {
                let km = g.dist_km(u, v as usize).max(1e-6);
                best = best.min(c / km);
            }
        }
    }
    best
}

/// Bring the free-food cache up to date before a founding step. A cell's value
/// depends on owners and walking costs within `r_cult`; a walking path costs at
/// least its length times the cheapest walking cost per km, so a changed owner
/// or road only stales cells within `r_cult / min_per_km` of it. Anything else
/// that changes costs clears the cache. Cached values are exact, so results are
/// identical to computing every cell afresh.
fn refresh_food_cache(state: &mut State, t: &Terrain, p: &Params) {
    let n = t.n();
    let key = (p.r_cult.to_bits(), p.min_field_yield.to_bits());
    let fc = &state.food_cache;
    if fc.value.len() != n || fc.transport != Some(state.transport) || fc.key != key {
        let min_per_km = min_walk_per_km(state, t);
        state.food_cache = crate::state::FoodCache {
            value: vec![f32::NAN; n],
            owner: state.owner.clone(),
            road: state.road.clone(),
            transport: Some(state.transport),
            key,
            min_per_km,
        };
        return;
    }
    let roads_changed = (0..n).any(|i| state.road[i] != fc.road[i]);
    let mut min_per_km = fc.min_per_km;
    if roads_changed {
        min_per_km = min_per_km.min(min_walk_per_km(state, t));
    }
    let radius = p.r_cult / min_per_km.max(1e-6) + t.grid.cell_km;
    let changed: Vec<usize> = (0..n)
        .filter(|&i| state.owner[i] != fc.owner[i] || state.road[i] != fc.road[i])
        .collect();
    let fc = &mut state.food_cache;
    for &x in &changed {
        for j in t.grid.disk(x, radius) {
            fc.value[j] = f32::NAN;
        }
    }
    fc.owner.copy_from_slice(&state.owner);
    fc.road.copy_from_slice(&state.road);
    fc.min_per_km = min_per_km;
}

/// Free food near a cell, from the cache when it is current.
fn cached_food(state: &mut State, t: &Terrain, p: &Params, cell: usize) -> f32 {
    let v = state.food_cache.value[cell];
    if !v.is_nan() {
        return v;
    }
    let v = free_food_near(state, t, p, cell);
    state.food_cache.value[cell] = v;
    v
}

/// Score parts for one cell; `food` is left unnormalised.
pub fn site_parts(state: &State, t: &Terrain, p: &Params, cell: usize, food: f32) -> SiteParts {
    let mut parts = site_parts_without_neighbours(state, t, p, cell, food);
    for s in &state.settlements {
        let c = s.reach.cost_to_cell(cell);
        if in_shadow(t, p, s.anchor as usize, cell, c) {
            parts.shadow += p.shadow_weight * s.pop / 1000.0;
        } else if c <= p.c_max {
            parts.access += p.access_weight * s.pop / 1000.0 * libm::expf(-p.access_decay * c);
        }
    }
    parts
}

/// Whether a settlement at `anchor`, `cost` away, shadows `cell`: within
/// `shadow_radius_km` in a straight line with `shadow_km` (D52), else within
/// `shadow_radius` of travel cost. Access is counted only outside the shadow.
pub fn in_shadow(t: &Terrain, p: &Params, anchor: usize, cell: usize, cost: f32) -> bool {
    if p.shadow_km {
        t.grid.dist_km(anchor, cell) <= p.shadow_radius_km
    } else {
        cost <= p.shadow_radius
    }
}

/// Score parts that do not depend on other settlements (shadow and access 0).
fn site_parts_without_neighbours(
    state: &State,
    t: &Terrain,
    p: &Params,
    cell: usize,
    food: f32,
) -> SiteParts {
    SiteParts {
        hub: if state.hub[cell] { 1.0 } else { 0.0 },
        river: if t.touches(cell, |j| t.is_nav(j)) {
            1.0
        } else {
            0.0
        },
        coast: if t.is_coastal(cell) { 1.0 } else { 0.0 },
        road: if t.touches(cell, |j| state.road[j]) {
            1.0
        } else {
            0.0
        },
        food,
        shadow: 0.0,
        access: 0.0,
        water: water_bonus(t, p, cell),
    }
}

pub fn run(state: &mut State, t: &Terrain, p: &Params, rng: &mut Rng) {
    if !p.founding_enabled || state.year == 0 || state.year % p.founding_interval.max(1) != 0 {
        return;
    }
    let n = t.n();
    refresh_food_cache(state, t, p);
    // Cells too close to a settlement, stamped once (same test as `is_candidate`).
    let mut crowded = vec![false; n];
    for st in &state.settlements {
        let a = st.anchor as usize;
        for j in t.grid.disk(a, p.min_spacing_km) {
            if t.grid.dist_km(a, j) < p.min_spacing_km {
                crowded[j] = true;
            }
        }
    }
    let cells: Vec<usize> = (0..n)
        .filter(|&c| {
            !crowded[c] && t.is_land(c) && state.owner[c] == NONE && has_water_supply(t, p, c)
        })
        .collect();
    // Shadow and access with settlements outermost; each cell still sums its
    // settlements in the same order as `site_parts`, so the results are identical.
    let mut shadow = vec![0f32; cells.len()];
    let mut access = vec![0f32; cells.len()];
    for st in &state.settlements {
        for (k, &cell) in cells.iter().enumerate() {
            let c = st.reach.cost_to_cell(cell);
            if in_shadow(t, p, st.anchor as usize, cell, c) {
                shadow[k] += p.shadow_weight * st.pop / 1000.0;
            } else if c <= p.c_max {
                access[k] += p.access_weight * st.pop / 1000.0 * libm::expf(-p.access_decay * c);
            }
        }
    }
    let mut cands: Vec<(usize, SiteParts)> = Vec::with_capacity(cells.len());
    for (k, &cell) in cells.iter().enumerate() {
        let food = cached_food(state, t, p, cell);
        let mut parts = site_parts_without_neighbours(state, t, p, cell, food);
        parts.shadow = shadow[k];
        parts.access = access[k];
        cands.push((cell, parts));
    }
    let max_food = cands
        .iter()
        .map(|c| c.1.food)
        .fold(0.0f32, f32::max)
        .max(1e-6);
    state.site_score.iter_mut().for_each(|x| *x = f32::NAN);
    let mut scored: Vec<(f32, usize)> = Vec::with_capacity(cands.len());
    for (cell, mut parts) in cands {
        parts.food /= max_food;
        let score = parts.total(p) + rng.gen::<f32>() * p.noise;
        state.site_score[cell] = score;
        scored.push((score, cell));
    }
    scored.sort_by(|a, b| b.0.total_cmp(&a.0).then(a.1.cmp(&b.1)));
    state.tally.last_best_score = scored.first().map_or(f32::NAN, |s| s.0);

    let mut founded: Vec<usize> = Vec::new();
    for &(score, cell) in &scored {
        if founded.len() as u32 >= p.max_found_per_step || score < p.found_threshold {
            break;
        }
        if founded
            .iter()
            .any(|&f| t.grid.dist_km(f, cell) < p.min_spacing_km)
        {
            continue;
        }
        // Migrants come from the nearest settlement that can spare them.
        let donor = state
            .settlements
            .iter()
            .enumerate()
            .filter(|(_, s)| s.food_ratio > p.donor_min_ratio && s.pop > 2.0 * p.found_pop)
            .map(|(i, s)| (s.reach.cost_to_cell(cell), i))
            .filter(|(c, _)| c.is_finite())
            .min_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
        let Some((_, di)) = donor else { continue };
        state.settlements[di].pop -= p.found_pop;
        let donor_id = state.settlements[di].id;
        let id = state.add_settlement(t, p, cell as u32, p.found_pop, false);
        state.tally.founded += 1;
        state.log.push(LogEntry {
            year: state.year,
            settlement: id,
            name: state.name_of(id),
            anchor: cell as u32,
            pop: p.found_pop,
            kind: LogKind::Founded {
                score,
                donor: donor_id,
            },
        });
        founded.push(cell);
    }

    if p.goods_enabled {
        found_works(state, t, p);
    }
}

/// Specialist settlements (v2, D40): for each good, the free resource site with
/// the most reachable demand, `Σ demand_k × exp(−β_g c)`, gets a hamlet of
/// `found_pop` on the cell if that demand would feed the founders. At most one
/// per good per step; migrants come from the nearest settlement that can spare
/// them, as for any founding.
fn found_works(state: &mut State, t: &Terrain, p: &Params) {
    use crate::state::{Good, Kind};
    use crate::systems::trade;
    for good in Good::ALL {
        let b = trade::beta(p, good);
        let best = state
            .sites
            .iter()
            .filter(|site| site.good == good)
            .map(|site| site.cell as usize)
            .filter(|&cell| {
                t.is_land(cell)
                    && !state
                        .settlements
                        .iter()
                        .any(|s| t.grid.dist_km(s.anchor as usize, cell) < p.min_spacing_km)
            })
            .map(|cell| {
                let reach: f32 = state
                    .settlements
                    .iter()
                    .map(|s| {
                        let c = s.reach.cost_to_cell(cell);
                        if c.is_finite() {
                            // Under goods by price (D72) demand counts at its
                            // value: the buyer's latest price for the good, or
                            // 1 where no working site reaches it.
                            let price = if p.market_goods {
                                let v = s.price[1 + good.index()];
                                if v > 0.0 {
                                    v
                                } else {
                                    1.0
                                }
                            } else {
                                1.0
                            };
                            price * trade::demand(s, p, good) * libm::expf(-b * c)
                        } else {
                            0.0
                        }
                    })
                    .sum();
                (reach, cell)
            })
            .filter(|&(reach, _)| reach >= p.found_pop * p.food_per_person)
            .max_by(|a, b| a.0.total_cmp(&b.0).then(b.1.cmp(&a.1)));
        let Some((demand, cell)) = best else { continue };
        let donor = state
            .settlements
            .iter()
            .enumerate()
            .filter(|(_, s)| s.food_ratio > p.donor_min_ratio && s.pop > 2.0 * p.found_pop)
            .map(|(i, s)| (s.reach.cost_to_cell(cell), i))
            .filter(|(c, _)| c.is_finite())
            .min_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
        let Some((_, di)) = donor else { continue };
        state.settlements[di].pop -= p.found_pop;
        let donor_id = state.settlements[di].id;
        let id = state.add_settlement(t, p, cell as u32, p.found_pop, false);
        if let Some(i) = state.index_of(id) {
            state.settlements[i].kind = match good {
                Good::Salt => Kind::Saltern,
                Good::Iron => Kind::Mine,
                Good::Stone => Kind::Quarry,
            };
        }
        state.tally.founded += 1;
        state.log.push(LogEntry {
            year: state.year,
            settlement: id,
            name: state.name_of(id),
            anchor: cell as u32,
            pop: p.found_pop,
            kind: LogKind::Works {
                good,
                demand,
                donor: donor_id,
            },
        });
    }
}
