//! Wool on spare land (L1, D105, D107): Option A.
//!
//! Arable is claimed by need, as before, and has first call on the land: a
//! grazed cell that cultivation claims stops being pasture. Beyond its arable,
//! a place grazes free land within its cultivation reach, as many cells as
//! its farmers can herd once the arable is worked:
//! `(farmers − arable_labour × fields) / pasture_labour`, per km². It takes
//! the cells of lowest arable rent first, `yield × (1 − τ c_market)`, so the
//! poorest and farthest land goes to grass first (von Thünen's order applied
//! to spare land), and gives up its best-rented pasture first when its people
//! fall short.
//!
//! A cell carries `sheep_per_acre × fleece_lb` lb of wool a year, sold at the
//! outside ports at `wool_price_sack` a sack of 364 lb, turned into grain at
//! the wheat price and valued at `port_world_price` (the sources are in
//! `docs/wool/l0.md`). The ports pay the full price (coin in); the farm-gate
//! value, less carriage at `market_tau / rho_wool` per unit of travel cost to
//! the port, is split by the rent share: `market_share` to the place's market
//! purse, the rest to its farmers less the toll, which goes to the market
//! purse. Carriage goes to the market purse of the place on the port side.
//! The wool rides the seller's paths to the port (`wool_ship`, loaded in
//! trade). Wool does not grow W (D105, decision 3).
//!
//! Writes `grazed`, settlements' `pasture` and `wool_in`, the purses,
//! `money.wool_in` and `wool_ship`.

use crate::params::Params;
use crate::state::{port_of, State, NONE};
use crate::terrain::Terrain;
use crate::Rng;

pub fn run(state: &mut State, t: &Terrain, p: &Params, _rng: &mut Rng) {
    let n = t.n();
    if state.grazed.len() != n {
        state.grazed = vec![NONE; n];
    }
    state.wool_ship.clear();
    let m = state.settlements.len();
    for s in state.settlements.iter_mut() {
        s.wool_in = 0.0;
    }
    // Arable has first call: drop pasture that cultivation has claimed, and
    // that of places gone.
    let mut alive = vec![false; state.next_id as usize];
    for s in &state.settlements {
        alive[s.id as usize] = true;
    }
    for c in 0..n {
        let g = state.grazed[c];
        if g != NONE && (state.owner[c] != NONE || !alive.get(g as usize).copied().unwrap_or(false)) {
            state.grazed[c] = NONE;
        }
    }
    {
        let grazed = &state.grazed;
        for s in state.settlements.iter_mut() {
            let id = s.id;
            s.pasture.retain(|&c| grazed[c as usize] == id);
        }
    }

    let km2 = t.grid.cell_km * t.grid.cell_km;
    let acres = km2 * 247.1;
    // Coin a pasture cell's wool fetches at the port.
    let wool_coin = acres * p.sheep_per_acre * p.fleece_lb / 364.0 * p.wool_price_sack * 20.0
        / p.wheat_s_bu.max(1e-6)
        / p.bu_per_person.max(1e-6)
        * p.port_world_price;
    if wool_coin <= 0.0 || p.pasture_labour <= 0.0 {
        return;
    }
    let tau_w = p.market_tau / p.rho_wool.max(1e-6);
    let mut idx_of = vec![usize::MAX; state.next_id as usize];
    for (i, s) in state.settlements.iter().enumerate() {
        idx_of[s.id as usize] = i;
    }
    let settlement = |id: u32| -> Option<usize> {
        idx_of.get(id as usize).copied().filter(|&i| i != usize::MAX)
    };
    // Each place's cheapest route to a port: (cost, partner index, the
    // partner's own partner index to the port or NONE when direct).
    let direct: Vec<Option<(f32, u32)>> = state
        .settlements
        .iter()
        .map(|s| {
            s.reach
                .partners
                .iter()
                .enumerate()
                .filter(|(_, q)| port_of(q.id).is_some())
                .map(|(k, q)| (q.cost, k as u32))
                .min_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)))
        })
        .collect();
    let route: Vec<Option<(f32, u32, u32)>> = (0..m)
        .map(|i| {
            let s = &state.settlements[i];
            let mut best = direct[i].map(|(c, k)| (c, k, NONE));
            for (k, q) in s.reach.partners.iter().enumerate() {
                let Some(j) = settlement(q.id) else { continue };
                if let Some((c2, k2)) = direct[j] {
                    let c = q.cost + c2;
                    if best.is_none_or(|b| c < b.0) {
                        best = Some((c, k as u32, k2));
                    }
                }
            }
            best
        })
        .collect();
    // Travel cost from each place to its nearest large market.
    let to_market: Vec<f32> = state
        .settlements
        .iter()
        .map(|s| {
            if s.market >= p.market_min {
                return 0.0;
            }
            s.reach
                .partners
                .iter()
                .filter(|q| settlement(q.id).is_some_and(|j| state.settlements[j].market >= p.market_min))
                .map(|q| q.cost)
                .fold(f32::INFINITY, f32::min)
        })
        .collect();

    let mut wool_total = 0f32;
    for i in 0..m {
        let Some((route_cost, k1, k2)) = route[i] else {
            continue;
        };
        let (id, farmers, fields, price) = {
            let s = &state.settlements[i];
            (
                s.id,
                (s.pop - s.market.max(0.0)).max(0.0),
                s.fields.len() as f32,
                if s.price[0] > 0.0 { s.price[0] } else { p.port_world_price },
            )
        };
        let spare = farmers - p.arable_labour * km2 * fields;
        let allowed = (spare / (p.pasture_labour * km2)).floor().max(0.0) as usize;
        let c_mkt = if to_market[i].is_finite() {
            to_market[i]
        } else {
            1.0 / p.market_tau.max(1e-9)
        };
        // Arable rent order: yield net of carriage to market.
        let key = |s: &crate::state::Settlement, c: u32| -> f32 {
            t.food_yield[c as usize] * (1.0 - p.market_tau * (c_mkt + s.reach.cost_to_cell(c as usize))).max(0.0)
        };
        let shepherds = p.pasture_labour * km2 * p.food_per_person * price;
        if state.settlements[i].pasture.len() > allowed {
            // Give up the best-rented pasture first.
            let s = &state.settlements[i];
            let mut ranked: Vec<(f32, u32)> = s.pasture.iter().map(|&c| (key(s, c), c)).collect();
            ranked.sort_by(|a, b| b.0.total_cmp(&a.0).then(b.1.cmp(&a.1)));
            let drop: Vec<u32> = ranked[..s.pasture.len() - allowed].iter().map(|x| x.1).collect();
            for &c in &drop {
                state.grazed[c as usize] = NONE;
            }
            let grazed = &state.grazed;
            state.settlements[i].pasture.retain(|&c| grazed[c as usize] == id);
        } else if state.settlements[i].pasture.len() < allowed {
            let s = &state.settlements[i];
            let mut cand: Vec<(f32, u32)> = s
                .reach
                .field_order
                .iter()
                .copied()
                .filter(|&c| {
                    let ci = c as usize;
                    t.is_land(ci) && state.owner[ci] == NONE && state.grazed[ci] == NONE && {
                        let cp = route_cost + s.reach.cost_to_cell(ci);
                        wool_coin * (1.0 - tau_w * cp).max(0.0) > shepherds
                    }
                })
                .map(|c| (key(s, c), c))
                .collect();
            cand.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
            let want = allowed - s.pasture.len();
            let take: Vec<u32> = cand.iter().take(want).map(|x| x.1).collect();
            for &c in &take {
                state.grazed[c as usize] = id;
            }
            state.settlements[i].pasture.extend(take);
        }

        // The year's clip, sold at the port.
        let s = &state.settlements[i];
        let cells = s.pasture.len();
        if cells == 0 {
            continue;
        }
        let gross = wool_coin * cells as f32;
        let farm_gate: f32 = s
            .pasture
            .iter()
            .map(|&c| wool_coin * (1.0 - tau_w * (route_cost + s.reach.cost_to_cell(c as usize))).max(0.0))
            .sum();
        let carriage = gross - farm_gate;
        let lords = p.market_share * farm_gate;
        let farmers_share = farm_gate - lords;
        let toll = p.toll * farmers_share;
        // The port-side place: the seller itself, or its partner on the route.
        let port_side = if k2 == NONE {
            i
        } else {
            settlement(s.reach.partners[k1 as usize].id).unwrap_or(i)
        };
        let partner_id = s.reach.partners[k1 as usize].id;
        {
            let s = &mut state.settlements[i];
            s.mpurse += lords + toll;
            s.purse += farmers_share - toll;
            s.wool_in += lords + toll;
        }
        state.settlements[port_side].mpurse += carriage;
        state.settlements[port_side].wool_in += carriage;
        state.money.wool_in += gross;
        wool_total += gross;
        // Shipped value in grain units at the world price, for road loads.
        let value = gross / p.port_world_price.max(1e-6);
        state.wool_ship.push((id, k1, value));
        if k2 != NONE {
            state.wool_ship.push((partner_id, k2, value));
        }
    }
    let _ = wool_total;
}
