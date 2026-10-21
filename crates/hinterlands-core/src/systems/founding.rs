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

/// Score parts for one cell; `food` is left unnormalised.
pub fn site_parts(state: &State, t: &Terrain, p: &Params, cell: usize, food: f32) -> SiteParts {
    let mut shadow = 0.0;
    let mut access = 0.0;
    for s in &state.settlements {
        let c = s.reach.cost_to_cell(cell);
        if c <= p.shadow_radius {
            shadow += p.shadow_weight * s.pop / 1000.0;
        } else if c <= p.c_max {
            access += p.access_weight * s.pop / 1000.0 * libm::expf(-p.access_decay * c);
        }
    }
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
        shadow,
        access,
        water: water_bonus(t, p, cell),
    }
}

pub fn run(state: &mut State, t: &Terrain, p: &Params, rng: &mut Rng) {
    if !p.founding_enabled || state.year == 0 || state.year % p.founding_interval.max(1) != 0 {
        return;
    }
    let n = t.n();
    let mut cands: Vec<(usize, SiteParts)> = Vec::new();
    for cell in 0..n {
        if is_candidate(state, t, p, cell) {
            let food = free_food_near(state, t, p, cell);
            cands.push((cell, site_parts(state, t, p, cell, food)));
        }
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
}
