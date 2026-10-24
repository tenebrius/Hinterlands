//! Growth (D15): rural natural increase from the food ratio, at most `r_grow` a
//! year (with `growth_ceiling`, D165: from its ten-year average, at most
//! `r_grow_pc`), plus the Rihll–Wilson term `epsilon * (D_j - K * W_j)` (Rihll & Wilson
//! 1987; Harris & Wilson 1978) on the non-farming population `W_j`, where the
//! inflow `D_j` is net food imports plus `through_weight` times non-food trade
//! received and trade passing through. A positive term is met by migrants drawn
//! from the town's partners, so villages grow by demography and towns by
//! migration, and small places near big ones are drained. With `gross_inflow`
//! (D24) the inflow is the gross market inflow instead of net food imports, as in
//! the published model: a market grows on what is sold there whether or not the
//! place itself is short of food. Under v2 it also keeps ten-year averages of
//! goods sales and inflow, and turns a specialist whose sales have fallen below
//! `specialist_share` of its inflow into an ordinary settlement (D44). With `market_state` (D26) `W_j` is the
//! settlement's own market size, not its derived non-farming population, and
//! moves with the people the market gains or loses; the inflow is gross. With
//! `handling_income` (D27) a place also earns a margin on the trade passing its
//! anchor and on the trade transshipped there (break of bulk), in place of the
//! through-traffic weight.
//! For `resettle_years` after a plague, people leave places on poorer land than
//! their partners' for the partner with the most vacant land per head (D61).
//! Writes: pop, food_ratio, ratio_avg, last_fed, low_years, smoothed_loss, history, market;
//! under v2 also avg_goods_in, avg_inflow, kind and the log entry for it.

use crate::params::Params;
use crate::state::State;
use crate::terrain::Terrain;
use crate::Rng;

pub fn run(state: &mut State, t: &Terrain, p: &Params, _rng: &mut Rng) {
    let year = state.year;
    let m = state.settlements.len();
    // Pass 1: food ratios, natural change, and each place's Rihll–Wilson pull.
    let mut natural = vec![0f32; m];
    // Read-only here; growth does not write it (D170).
    let state_ever = std::mem::take(&mut state.ever_owned);
    // The region's free share (D170 exploration): farmable land never cleared.
    let region_free = if p.free_land_growth && p.free_land_region && !state_ever.is_empty() {
        let (mut all, mut free) = (0u32, 0u32);
        for c in 0..t.n() {
            if t.is_land(c) && t.food_yield[c] >= p.min_field_yield {
                all += 1;
                if !state_ever[c] {
                    free += 1;
                }
            }
        }
        Some(if all > 0 { free as f32 / all as f32 } else { 1.0 })
    } else {
        None
    };
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
        // D165: under `growth_ceiling` births answer the food ratio over ten
        // years, and natural growth is at most `r_grow_pc` a year.
        // D170: a frontier's rate on land never cleared, a settled country's
        // where it has been taken, by the share of the place's reach still free.
        let r_free = if let Some(share) = region_free {
            Some(p.r_grow_pc + (p.r_grow - p.r_grow_pc) * share)
        } else if p.free_land_growth && !state_ever.is_empty() {
            let (mut all, mut free) = (0u32, 0u32);
            for &c in &s.reach.field_order {
                let ci = c as usize;
                if t.is_land(ci) && t.food_yield[ci] >= p.min_field_yield {
                    all += 1;
                    if !state_ever[ci] {
                        free += 1;
                    }
                }
            }
            let share = if all > 0 { free as f32 / all as f32 } else { 1.0 };
            Some(p.r_grow_pc + (p.r_grow - p.r_grow_pc) * share)
        } else {
            None
        };
        let (ratio_g, r_max) = if let Some(r) = r_free {
            (ratio, r)
        } else if p.settled_start {
            // D182: after the spin-up, births follow the place's own food
            // ratio at a settled rate (the spin-up runs without the switch).
            (ratio, p.r_grow_pc)
        } else if p.growth_ceiling {
            s.ratio_avg = if s.ratio_avg > 0.0 {
                s.ratio_avg + (ratio - s.ratio_avg) / 10.0
            } else {
                ratio
            };
            (s.ratio_avg, p.r_grow_pc)
        } else {
            (ratio, p.r_grow)
        };
        let capped = ratio_g.min(p.food_cap);
        let gain = if capped > 1.0 {
            r_max * (capped - 1.0) / (p.food_cap - 1.0).max(1e-6)
        } else {
            0.0
        };
        if p.shed_unfed {
            // D34: an underfed place sheds a third of the people its food does
            // not feed each year; the leavers are lost. People endure a
            // moderate shortfall: only the gap below `hunger_tolerance` sheds
            // (D101; Livi-Bacci 1991).
            let tol = p.hunger_tolerance.min(1.0);
            let loss = if ratio < tol {
                (tol - ratio) * s.pop / 3.0
            } else {
                0.0
            };
            s.smoothed_loss = loss;
            natural[i] = gain * s.pop - loss;
        } else {
            let loss = if ratio < 1.0 {
                p.r_shrink * (1.0 - ratio) * s.pop
            } else {
                0.0
            };
            s.smoothed_loss += (loss - s.smoothed_loss) / p.loss_years.max(1.0);
            natural[i] = gain * s.pop - s.smoothed_loss;
        }

        let urban = if p.market_state {
            s.market.max(p.market_floor)
        } else {
            crate::systems::trade::urban_pop(s, p)
        };
        let market = if p.gross_inflow || p.market_state {
            s.market_in
        } else {
            (s.food_in - s.food_out).max(0.0)
        };
        let inflow = if p.market_grain {
            // Under S2 (D79) the inflow is real market income: rents and goods
            // sales (in `market_in`) and fees, over the grain price.
            let fees = if p.handling_income {
                p.handling_margin * s.through + p.transship_margin * s.transship
            } else {
                0.0
            };
            market + fees / s.price[0].max(1e-3)
        } else if p.handling_income {
            market
                + p.through_weight * s.trade_in
                + p.handling_margin * s.through
                + p.transship_margin * s.transship
        } else {
            market + p.through_weight * (s.trade_in + s.through)
        };
        if p.goods_enabled {
            s.avg_goods_in += (s.goods_in - s.avg_goods_in) / 10.0;
            s.avg_inflow += (inflow - s.avg_inflow) / 10.0;
        }
        // W2 (D85): the pull answers the ten-year average inflow, not the year's.
        let inflow = if p.pull_smoothed && p.goods_enabled {
            s.avg_inflow
        } else {
            inflow
        };
        pull[i] = (p.epsilon * (inflow - p.k_maint * p.food_per_person * urban))
            .min(p.max_growth * s.pop);
    }

    state.ever_owned = state_ever;

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
    // Only trade partners drain, so the neighbour must also be within reach.
    if p.growth_enabled && p.shadow_drain > 0.0 {
        for i in 0..m {
            let si = &state.settlements[i];
            let mut lost = 0.0;
            let mut takers: Vec<(usize, f32)> = Vec::new();
            for q in &si.reach.partners {
                let Some(&k) = idx_of.get(q.id as usize) else {
                    continue;
                };
                if k == u32::MAX {
                    continue;
                }
                let sk = &state.settlements[k as usize];
                // Distance into the shadow: straight-line km within
                // `drain_radius_km` with `shadow_km` (D52, D53), else travel cost.
                let (d, radius) = if p.shadow_km {
                    (
                        t.grid.dist_km(si.anchor as usize, sk.anchor as usize),
                        p.drain_radius_km,
                    )
                } else {
                    (q.cost, p.shadow_radius)
                };
                if d > radius {
                    continue;
                }
                let pk = sk.pop;
                if pk >= p.shadow_size_ratio * si.pop {
                    let rate = p.shadow_drain * pk / 1000.0 * (1.0 - d / radius);
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
    // Resettlement after a plague (D61; Dyer): for `resettle_years` after it
    // strikes, a place whose local yield is below its partners' median sends
    // `resettle_rate` of its people to the partner with the most free food per
    // head within its cultivation reach.
    let since = year.wrapping_sub(state.last_plague);
    if p.growth_enabled
        && p.events_mode != 0
        && state.last_plague != crate::state::NONE
        && since < p.resettle_years
    {
        let local: Vec<f32> = state
            .settlements
            .iter()
            .map(|s| local_yield(t, s.anchor as usize))
            .collect();
        let free_per_head = |k: usize| {
            let s = &state.settlements[k];
            let free: f32 = s
                .reach
                .field_order
                .iter()
                .map(|&c| c as usize)
                .filter(|&c| state.owner[c] == crate::state::NONE && t.is_land(c))
                .map(|c| t.food_yield[c])
                .sum();
            free / s.pop.max(1.0)
        };
        for i in 0..m {
            let mut ks: Vec<usize> = state.settlements[i]
                .reach
                .partners
                .iter()
                .filter_map(|q| idx_of.get(q.id as usize).copied())
                .filter(|&k| k != u32::MAX && k as usize != i)
                .map(|k| k as usize)
                .collect();
            if ks.is_empty() {
                continue;
            }
            ks.sort_unstable();
            ks.dedup();
            let mut ys: Vec<f32> = ks.iter().map(|&k| local[k]).collect();
            ys.sort_by(|a, b| a.total_cmp(b));
            let median = ys[ys.len() / 2];
            if local[i] >= median {
                continue;
            }
            let best = ks
                .iter()
                .map(|&k| (free_per_head(k), k))
                .max_by(|a, b| a.0.total_cmp(&b.0).then(b.1.cmp(&a.1)));
            if let Some((f, k)) = best {
                if f > 0.0 {
                    let moved = p.resettle_rate * state.settlements[i].pop;
                    delta[i] -= moved;
                    delta[k] += moved;
                }
            }
        }
    }
    // The people a market actually gains or loses change its size.
    let mut market_delta = vec![0f32; m];
    for j in 0..m {
        if pull[j] < 0.0 {
            delta[j] += pull[j];
            market_delta[j] += pull[j];
        }
        for &(i, a) in &asked[j] {
            let moved = a * scale[i];
            delta[i] -= moved;
            delta[j] += moved;
            market_delta[j] += moved;
        }
    }

    for (i, s) in state.settlements.iter_mut().enumerate() {
        if p.growth_enabled {
            s.pop = (s.pop + delta[i]).max(0.0);
            if p.market_state {
                // Labour guard (D92): market people are those the fields do
                // not need. The fields need the farmers they are sized to
                // feed, after rent, at the cultivation margin.
                let field_need = s
                    .fields
                    .iter()
                    .map(|&c| t.food_yield[c as usize])
                    .sum::<f32>()
                    * p.people_per_prime_cell
                    * (1.0 - p.market_share)
                    / (p.cult_margin.max(1e-3) * p.food_per_person.max(1e-3))
                    // Shepherds are counted with the farmers (L1, D105).
                    + p.pasture_labour
                        * t.grid.cell_km
                        * t.grid.cell_km
                        * s.pasture.len() as f32;
                let room = (s.pop - field_need).max(p.market_floor);
                s.market = (s.market + market_delta[i])
                    .min(s.pop)
                    .min(room)
                    .max(p.market_floor);
            }
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

    // The kind follows the economy (D44): a specialist whose goods sales have
    // fallen below `specialist_share` of its inflow, over ten years, becomes an
    // ordinary settlement for good. It keeps working its site.
    if p.goods_enabled {
        let mut outgrew = Vec::new();
        for s in state.settlements.iter_mut() {
            let good = match s.kind {
                crate::state::Kind::Agrarian => continue,
                crate::state::Kind::Mine => crate::state::Good::Iron,
                crate::state::Kind::Saltern => crate::state::Good::Salt,
                crate::state::Kind::Quarry => crate::state::Good::Stone,
            };
            if year.saturating_sub(s.founded) >= 10
                && s.avg_goods_in < p.specialist_share * s.avg_inflow
            {
                s.kind = crate::state::Kind::Agrarian;
                outgrew.push(crate::state::LogEntry {
                    year,
                    settlement: s.id,
                    name: s.name.clone(),
                    anchor: s.anchor,
                    pop: s.pop,
                    kind: crate::state::LogKind::Outgrew { good },
                });
            }
        }
        state.log.extend(outgrew);
    }
}

/// Mean yield of the land within 3 km of a cell (D58, D61).
fn local_yield(t: &Terrain, cell: usize) -> f32 {
    let (mut sum, mut n) = (0.0f32, 0u32);
    for j in t.grid.disk(cell, 3.0) {
        if t.is_land(j) {
            sum += t.food_yield[j];
            n += 1;
        }
    }
    sum / n.max(1) as f32
}
