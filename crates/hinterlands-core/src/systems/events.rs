//! Events (v4, D57): which shocks fall due this year. In `Historical` mode
//! (`events_mode` 1, year 0 = 1100) the calendar is the Great Famine
//! (`famine_year`, for `famine_years`), the Black Death (`plague_year`) and
//! two recurrences (`plague_recur_1`, `plague_recur_2`) at `plague_recur_share`
//! of its mortality, and one harbour silting in a year drawn within
//! `silt_from` .. `silt_from + silt_window` (D60). In `Random` mode (2) each
//! kind is drawn yearly from the run's `Rng` at a mean interval. In either
//! mode a lord plants a town every `plant_interval` years (D62). Off (0) does
//! nothing and draws nothing.
//! Writes: the event queue only.

use crate::event::Event;
use crate::params::Params;
use crate::state::State;
use crate::terrain::Terrain;
use crate::Rng;
use rand::Rng as _;

pub fn run(state: &mut State, t: &Terrain, p: &Params, rng: &mut Rng) {
    let y = state.year;
    let (famine, plague) = match p.events_mode {
        1 => {
            let plague = if y == p.plague_year {
                Some(p.plague_mortality)
            } else if y == p.plague_recur_1 || y == p.plague_recur_2 {
                Some(p.plague_mortality * p.plague_recur_share)
            } else {
                None
            };
            (y == p.famine_year, plague)
        }
        2 if y > 0 => {
            let famine = rng.gen::<f32>() * p.famine_interval.max(1.0) < 1.0;
            let plague = rng.gen::<f32>() * p.plague_interval.max(1.0) < 1.0;
            (famine, plague.then_some(p.plague_mortality))
        }
        _ => (false, None),
    };
    // Harbour silting (D60): in historical mode one, in a year drawn when the
    // window opens; in random mode at a rate.
    let silt = match p.events_mode {
        1 => {
            if y == p.silt_from && p.silt_window > 0 {
                state.silt_year = p.silt_from + rng.gen_range(0..=p.silt_window);
            } else if y == p.silt_from {
                state.silt_year = p.silt_from;
            }
            y >= p.silt_from && y == state.silt_year
        }
        2 if y > 0 => rng.gen::<f32>() * p.silt_interval.max(1.0) < 1.0,
        _ => false,
    };
    if silt {
        if let Some(id) = busiest_shallow_port(state, t, p) {
            state.events.push(Event::Silt {
                settlement: id,
                cost: p.silt_cost,
            });
        }
    }
    // Planted towns (D62): every `plant_interval` years with events on.
    if p.events_mode != 0 && p.plant_interval > 0 && y > 0 && y % p.plant_interval == 0 {
        if let Some(cell) = plant_site(state, t, p) {
            state.events.push(Event::Plant {
                cell: cell as u32,
                pop: p.plant_pop,
                market: p.plant_market * p.market_w0,
            });
        }
    }
    if famine {
        state.events.push(Event::Famine {
            years: p.famine_years,
            loss: p.famine_loss,
        });
    }
    if let Some(mean) = plague {
        // The share is drawn per settlement, uniform within the spread around
        // the mean (scaled down with it for a recurrence), and taken in equal
        // yearly parts: `1 − (1 − m)^(1/years)` a year.
        let spread = p.plague_spread * mean / p.plague_mortality.max(1e-6);
        let years = p.plague_years.max(1) as f32;
        let shares = state
            .settlements
            .iter()
            .map(|s| {
                let m = (mean + spread * (2.0 * rng.gen::<f32>() - 1.0)).clamp(0.0, 0.95);
                (s.id, 1.0 - libm::powf(1.0 - m, 1.0 / years))
            })
            .collect();
        state.events.push(Event::Plague {
            shares,
            years: p.plague_years,
            first: true,
        });
    }
}

/// The port with the most transshipped trade whose harbour faces a river mouth
/// or a shallow coast (D60): its anchor, or a cell next to it, borders sea no
/// deeper than `silt_depth_m`, or a navigable river cell that flows into the
/// sea. Ties go to the lower id.
fn busiest_shallow_port(state: &State, t: &Terrain, p: &Params) -> Option<u32> {
    let g = t.grid;
    let near = |a: usize| std::iter::once(a).chain((0..8).filter_map(move |d| g.neighbor(a, d)));
    let faces = |c: usize| {
        (0..8).filter_map(|d| g.neighbor(c, d)).any(|j| {
            (t.is_sea(j) && t.elevation[j] >= -p.silt_depth_m)
                || (t.is_nav(j) && t.downstream(j).is_some_and(|k| t.is_sea(k)))
        })
    };
    state
        .settlements
        .iter()
        .filter(|s| s.transship > 0.0)
        .filter(|s| state.silt.get(s.anchor as usize).is_none_or(|&c| c == 0.0))
        .filter(|s| near(s.anchor as usize).any(faces))
        .max_by(|a, b| a.transship.total_cmp(&b.transship).then(b.id.cmp(&a.id)))
        .map(|s| s.id)
}

/// Where a lord plants a town (D62): among the cells a settlement could be
/// founded on that are on or next to a road or a navigable river, and at least
/// `plant_spacing_km` from any place over 500, the one with the highest market
/// access, Σ pop/1000 × exp(−access_decay × cost) over settlements within
/// `c_max`. Ties go to the lower cell.
fn plant_site(state: &State, t: &Terrain, p: &Params) -> Option<usize> {
    let big: Vec<usize> = state
        .settlements
        .iter()
        .filter(|s| s.pop > 500.0)
        .map(|s| s.anchor as usize)
        .collect();
    let mut best: Option<(f32, usize)> = None;
    for c in 0..t.n() {
        if !t.touches(c, |j| state.road[j] || t.is_nav(j))
            || big.iter().any(|&b| t.grid.dist_km(b, c) < p.plant_spacing_km)
            || !crate::systems::founding::is_candidate(state, t, p, c)
        {
            continue;
        }
        let access: f32 = state
            .settlements
            .iter()
            .map(|s| (s.pop, s.reach.cost_to_cell(c)))
            .filter(|&(_, cost)| cost <= p.c_max)
            .map(|(pop, cost)| pop / 1000.0 * libm::expf(-p.access_decay * cost))
            .sum();
        if best.is_none_or(|(a, _)| access > a) {
            best = Some((access, c));
        }
    }
    best.map(|(_, c)| c)
}
