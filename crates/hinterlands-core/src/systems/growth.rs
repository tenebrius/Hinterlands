//! Growth (D15): rural natural increase from the food ratio, at most `r_grow` a
//! year, plus the Rihll–Wilson term `epsilon * (D_j - K * W_j)` (Rihll & Wilson
//! 1987; Harris & Wilson 1978) on the non-farming population `W_j`, where the
//! inflow `D_j` is net food imports plus `through_weight` times non-food trade
//! received and trade passing through. A positive term is met by migrants drawn
//! from the town's partners, so villages grow by demography and towns by
//! migration, and small places near big ones are drained.
//! Writes: pop, food_ratio, last_fed, low_years, smoothed_loss, history.

use crate::params::Params;
use crate::state::State;
use crate::terrain::Terrain;
use crate::Rng;

pub fn run(state: &mut State, _t: &Terrain, p: &Params, _rng: &mut Rng) {
    let year = state.year;
    let m = state.settlements.len();
    // Pass 1: food ratios, natural change, and each place's Rihll–Wilson pull.
    let mut natural = vec![0f32; m];
    let mut pull = vec![0f32; m];
    for (i, s) in state.settlements.iter_mut().enumerate() {
        let need = s.need(p).max(1e-6);
        let food = s.food_prod + s.food_in - s.food_out;
        let ratio = food / need;
        s.food_ratio = ratio;
        if ratio >= p.fed_ratio {
            s.last_fed = year;
        }
        if !p.growth_enabled {
            continue;
        }
        // Natural term: r_grow at a full surplus, zero when exactly fed, and a
        // shortfall loss spread over `loss_years`.
        let capped = ratio.min(p.food_cap);
        let gain = if capped > 1.0 {
            p.r_grow * (capped - 1.0) / (p.food_cap - 1.0).max(1e-6)
        } else {
            0.0
        };
        let loss = if ratio < 1.0 {
            p.r_shrink * (1.0 - ratio) * s.pop
        } else {
            0.0
        };
        s.smoothed_loss += (loss - s.smoothed_loss) / p.loss_years.max(1.0);
        natural[i] = gain * s.pop - s.smoothed_loss;

        let urban = crate::systems::trade::urban_pop(s, p);
        let net_food = (s.food_in - s.food_out).max(0.0);
        let inflow = net_food + p.through_weight * (s.trade_in + s.through);
        pull[i] = (p.epsilon * (inflow - p.k_maint * p.food_per_person * urban))
            .min(p.max_growth * s.pop);
    }

    // Pass 2: towns that pull draw migrants from their partners, in proportion to
    // the partner's size and closeness; no source gives more than `migration_cap`.
    let mut idx_of = vec![u32::MAX; state.next_id as usize];
    for (i, s) in state.settlements.iter().enumerate() {
        idx_of[s.id as usize] = i as u32;
    }
    let mut asked: Vec<Vec<(usize, f32)>> = vec![Vec::new(); m];
    let mut asked_of = vec![0f32; m];
    if p.growth_enabled {
        for j in 0..m {
            if pull[j] <= 0.0 {
                continue;
            }
            let s = &state.settlements[j];
            let mut ws: Vec<(usize, f32)> = Vec::new();
            for q in &s.reach.partners {
                let Some(&i) = idx_of.get(q.id as usize) else {
                    continue;
                };
                if i != u32::MAX {
                    ws.push((
                        i as usize,
                        state.settlements[i as usize].pop * libm::expf(-p.beta * q.cost),
                    ));
                }
            }
            let total: f32 = ws.iter().map(|w| w.1).sum();
            if total <= 0.0 {
                continue;
            }
            for (i, w) in ws {
                let a = pull[j] * w / total;
                asked[j].push((i, a));
                asked_of[i] += a;
            }
        }
    }
    let scale: Vec<f32> = (0..m)
        .map(|i| {
            let cap = p.migration_cap * state.settlements[i].pop;
            if asked_of[i] > cap {
                cap / asked_of[i]
            } else {
                1.0
            }
        })
        .collect();
    let mut delta = natural;
    // Urban shadow (Bosker & Buringh 2017; Christaller 1933): a place near a much
    // larger one loses people to it, more the larger and closer the neighbour.
    if p.growth_enabled && p.shadow_drain > 0.0 {
        for i in 0..m {
            let si = &state.settlements[i];
            let mut lost = 0.0;
            let mut takers: Vec<(usize, f32)> = Vec::new();
            for q in &si.reach.partners {
                if q.cost > p.shadow_radius {
                    continue;
                }
                let Some(&k) = idx_of.get(q.id as usize) else {
                    continue;
                };
                if k == u32::MAX {
                    continue;
                }
                let pk = state.settlements[k as usize].pop;
                if pk >= p.shadow_size_ratio * si.pop {
                    let rate = p.shadow_drain * pk / 1000.0 * (1.0 - q.cost / p.shadow_radius);
                    takers.push((k as usize, rate));
                    lost += rate;
                }
            }
            if lost <= 0.0 {
                continue;
            }
            let total = (lost.min(p.migration_cap.max(0.01) * 5.0)) * si.pop;
            for (k, rate) in takers {
                let moved = total * rate / lost;
                delta[i] -= moved;
                delta[k] += moved;
            }
        }
    }
    for j in 0..m {
        if pull[j] < 0.0 {
            delta[j] += pull[j];
        }
        for &(i, a) in &asked[j] {
            let moved = a * scale[i];
            delta[i] -= moved;
            delta[j] += moved;
        }
    }

    for (i, s) in state.settlements.iter_mut().enumerate() {
        if p.growth_enabled {
            s.pop = (s.pop + delta[i]).max(0.0);
        }
        if s.pop < p.fail_pop {
            s.low_years += 1;
        } else {
            s.low_years = 0;
        }
        s.history.push(s.pop);
        if s.history.len() > 51 {
            s.history.remove(0);
        }
    }
}
