//! Wool on spare land (L1, D105, D107): Option A.
//!
//! Arable is claimed by need, as before, and has first call on the land: a
//! grazed cell that cultivation claims stops being pasture. Beyond its arable,
//! a place grazes the free land within its cultivation reach that is not
//! worth ploughing at its distance (D108): land whose net arable output,
//! `yield × ppc × (1 − τ c_market)`, feeds fewer than the `arable_labour` it
//! would take to work. The poorest and farthest land goes to grass (von
//! Thünen's order applied to spare land). Its shepherds, `pasture_labour` a
//! km², come from the place's own people: they are counted with the farmers
//! in the labour guard (D92).
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
//! With `cloth` (D2 step 1, D123) the fallow flock adds `fallow_share ×
//! sheep_per_fallow_acre` sheep an acre of each place's fields, sold the same
//! way; that wool is the farmers' own (no rent), less the toll.
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
    state.fallow_wool = 0.0;
    state.cloth_stats = [0.0; 5];
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
        if g != NONE && (state.owner[c] != NONE || !alive.get(g as usize).copied().unwrap_or(false))
        {
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
    // Wool fell against wheat after the plague, in a straight line from
    // `wool_fall_from` to `wool_fall_to` (Clark 2004; D109).
    let fall = if state.year <= p.wool_fall_from {
        0.0
    } else if state.year >= p.wool_fall_to {
        p.wool_fall
    } else {
        p.wool_fall * (state.year - p.wool_fall_from) as f32
            / (p.wool_fall_to - p.wool_fall_from).max(1) as f32
    };
    // The murrain of 1315-22 (L2, D112): flocks cut in the Historical calendar.
    let since = state.year.wrapping_sub(p.murrain_year);
    let murrain = if p.murrain && p.events_mode == 1 && state.year >= p.murrain_year {
        if since < p.murrain_years {
            p.murrain_loss
        } else if since < p.murrain_years + p.murrain_late_years {
            p.murrain_late_loss
        } else {
            0.0
        }
    } else {
        0.0
    };
    let price_sack = p.wool_price_sack * (1.0 - fall) * (1.0 - murrain);
    let wool_coin = acres * p.sheep_per_acre * p.fleece_lb / 364.0 * price_sack * 20.0
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
        idx_of
            .get(id as usize)
            .copied()
            .filter(|&i| i != usize::MAX)
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
            // From D2 (D124) wool goes to its nearest port at the full travel
            // cost, as the port goods do, not by two hops.
            if p.cloth {
                return direct[i].map(|(c, k)| (c, k, NONE)).or_else(|| {
                    s.reach
                        .port
                        .as_ref()
                        .map(|q| (q.cost, crate::state::FAR_PORT, NONE))
                });
            }
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
                .filter(|q| {
                    settlement(q.id).is_some_and(|j| state.settlements[j].market >= p.market_min)
                })
                .map(|q| q.cost)
                .fold(f32::INFINITY, f32::min)
        })
        .collect();

    let mut wool_total = 0f32;
    let mut sellers: Vec<Seller> = Vec::new();
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
                if s.price[0] > 0.0 {
                    s.price[0]
                } else {
                    p.port_world_price
                },
            )
        };
        let _ = (farmers, fields);
        // With no large market in reach the place sells at home.
        let c_mkt = if to_market[i].is_finite() {
            to_market[i]
        } else {
            0.0
        };
        // Arable rent order: yield net of carriage to market.
        let key = |s: &crate::state::Settlement, c: u32| -> f32 {
            t.food_yield[c as usize]
                * (1.0 - p.market_tau * (c_mkt + s.reach.cost_to_cell(c as usize))).max(0.0)
        };
        let shepherds = p.pasture_labour * km2 * p.food_per_person * price;
        // Land not worth ploughing at its distance: its net arable output
        // feeds fewer than the people it would take to work it (D108).
        let not_worth = |s: &crate::state::Settlement, c: u32| -> bool {
            key(s, c) * p.people_per_prime_cell < p.arable_labour * km2
        };
        {
            let s = &state.settlements[i];
            let mut cand: Vec<(f32, u32)> = s
                .reach
                .field_order
                .iter()
                .copied()
                .filter(|&c| {
                    let ci = c as usize;
                    t.is_land(ci)
                        && state.owner[ci] == NONE
                        && state.grazed[ci] == NONE
                        && not_worth(s, c)
                        && {
                            let cp = route_cost + s.reach.cost_to_cell(ci);
                            wool_coin * (1.0 - tau_w * cp).max(0.0) > shepherds
                        }
                })
                .map(|c| (key(s, c), c))
                .collect();
            cand.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
            let take: Vec<u32> = cand.iter().map(|x| x.1).collect();
            // Old fields (cleared land) going to grass make the chronicle.
            let old_fields = take
                .iter()
                .filter(|&&c| state.forest[c as usize] < p.grass_log_forest)
                .count() as u32;
            for &c in &take {
                state.grazed[c as usize] = id;
            }
            state.settlements[i].pasture.extend(take);
            let quiet = {
                let g = state.settlements[i].grass_logged;
                g == 0 || state.year >= g + p.grass_log_gap
            };
            if old_fields >= p.grass_log_cells && state.year > 0 && quiet {
                state.settlements[i].grass_logged = state.year;
                let s = &state.settlements[i];
                state.log.push(crate::state::LogEntry {
                    year: state.year,
                    settlement: s.id,
                    name: s.name.clone(),
                    anchor: s.anchor,
                    pop: s.pop,
                    kind: crate::state::LogKind::Grass { cells: old_fields },
                });
            }
        }

        // The year's clip, sold at the port.
        let s = &state.settlements[i];
        let cells = s.pasture.len();
        // The fallow flock (D2 step 1, D123): sheep on the fallow and commons
        // of the place's fields, its wool sold by the farmers with the
        // pasture's.
        let fallow_coin = if p.cloth {
            wool_coin * p.fallow_share * p.sheep_per_fallow_acre / p.sheep_per_acre.max(1e-6)
        } else {
            0.0
        };
        let field_cells = if fallow_coin > 0.0 { s.fields.len() } else { 0 };
        if cells == 0 && field_cells == 0 {
            continue;
        }
        let gate = |coin: f32, c: u32| -> f32 {
            coin * (1.0 - tau_w * (route_cost + s.reach.cost_to_cell(c as usize))).max(0.0)
        };
        let gross_fallow = fallow_coin * field_cells as f32;
        let gate_fallow: f32 = if field_cells > 0 {
            s.fields.iter().map(|&c| gate(fallow_coin, c)).sum()
        } else {
            0.0
        };
        let gross = wool_coin * cells as f32 + gross_fallow;
        let gate_pasture: f32 = s.pasture.iter().map(|&c| gate(wool_coin, c)).sum();
        if p.cloth {
            // Sold after the mills have bought (D2).
            let lb_cell = acres * p.fleece_lb * (1.0 - murrain);
            sellers.push(Seller {
                i,
                k1,
                lb_p: lb_cell * p.sheep_per_acre * cells as f32,
                lb_f: lb_cell * p.fallow_share * p.sheep_per_fallow_acre * field_cells as f32,
                gross_p: wool_coin * cells as f32,
                gross_f: gross_fallow,
                gate_p: gate_pasture,
                gate_f: gate_fallow,
            });
            continue;
        }
        let farm_gate = gate_pasture + gate_fallow;
        let carriage = gross - farm_gate;
        // Pasture pays rent; the fallow flock's wool is the farmers' own.
        let lords = p.market_share * gate_pasture;
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
        state.fallow_wool += gross_fallow;
        wool_total += gross;
        // Shipped value in grain units at the world price, for road loads.
        let value = gross / p.port_world_price.max(1e-6);
        state.wool_ship.push((id, k1, value));
        if k2 != NONE {
            state.wool_ship.push((partner_id, k2, value));
        }
    }
    let _ = wool_total;
    if p.cloth {
        mills_and_sale(state, p, &route, &sellers, murrain, fall);
    }
    if murrain > 0.0 && since == 0 {
        // Logged at the place with the most pasture.
        if let Some(s) = state
            .settlements
            .iter()
            .max_by(|a, b| a.pasture.len().cmp(&b.pasture.len()).then(b.id.cmp(&a.id)))
            .filter(|s| !s.pasture.is_empty())
        {
            let entry = crate::state::LogEntry {
                year: state.year,
                settlement: s.id,
                name: s.name.clone(),
                anchor: s.anchor,
                pop: s.pop,
                kind: crate::state::LogKind::Murrain { loss: murrain },
            };
            state.log.push(entry);
        }
    }
}

/// One place's clip under D2: lb and coin, pasture and fallow.
struct Seller {
    i: usize,
    k1: u32,
    lb_p: f32,
    lb_f: f32,
    gross_p: f32,
    gross_f: f32,
    gate_p: f32,
    gate_f: f32,
}

/// D2 (D123, D124): the mills buy wool, then the rest is exported.
///
/// Solved from the finished good to the raw one on last year's prices: a mill
/// bids for a lb of wool what its cloth fetches less the cost of making it
/// (`cloth_cost_d_lb`). Export cloth fetches the world cloth price less the
/// custom and carriage to the nearest port; home cloth, up to what the mill
/// sold at home last year, the better of that and last year's home price at
/// the mill (its supply price, or its buyers' price where none reached them). A place's wool is worth its export netback: the port's
/// price less carriage and the export duties (the ancient custom from 1275,
/// the subsidy from 1342). Pairs of seller and mill in reach are taken in
/// order of surplus, the mill's bid less carriage of the wool to it less the
/// seller's netback, within the mill's capacity (`cloth_lb_per_w` × W) and
/// purse. The seller is paid its netback either way, so its purses are as if
/// it had exported; the mill pays from its market purse. What is not milled is
/// exported; the duty leaves the region.
fn mills_and_sale(
    state: &mut State,
    p: &Params,
    route: &[Option<(f32, u32, u32)>],
    sellers: &[Seller],
    _murrain: f32,
    fall: f32,
) {
    let m = state.settlements.len();
    let y = state.year;
    let to_coin = p.port_world_price / (p.wheat_s_bu * 12.0 * p.bu_per_person).max(1e-6);
    let duty_sack = if y >= p.wool_custom_year {
        p.wool_custom_d
    } else {
        0.0
    } + if y >= p.wool_subsidy_year {
        p.wool_subsidy_d
    } else {
        0.0
    };
    let duty_lb = duty_sack / 364.0 * to_coin;
    let custom_lb = if y >= p.cloth_custom_year {
        p.cloth_custom_d / p.cloth_lb.max(1.0)
    } else {
        0.0
    } * to_coin;
    let vc = p.cloth_d_lb * to_coin;
    let make = p.cloth_cost_d_lb * to_coin;
    let vw = p.wool_price_sack * (1.0 - fall) * 240.0 / 364.0 * to_coin;
    let tau_w = p.market_tau / p.rho_wool.max(1e-6);
    let tau_c = p.market_tau / p.rho_cloth.max(1e-6);
    let home_pp = p.cloth_yd_adult * p.adult_share * p.home_lb_per_yd;
    // Mills, two tiers each: for the home market, what it sold at home last
    // year (a tenth more when it sold out at a premium, never less than its
    // own people's need) at the better of last year's home supply price and
    // export; the rest of its capacity at export. Node 2j is j's home tier,
    // 2j + 1 its export tier.
    let mut cap = vec![0f32; 2 * m];
    let mut bid = vec![f32::NEG_INFINITY; 2 * m];
    let mut purse = vec![0f32; m];
    for (j, s) in state.settlements.iter().enumerate() {
        if !s.mill || s.market <= 0.0 {
            continue;
        }
        let export = match route[j] {
            Some((c, _, _)) => vc * (1.0 - tau_c * c).max(0.0) - custom_lb,
            None => 0.0,
        };
        let total = p.cloth_lb_per_w * s.market;
        let last = if s.cloth_sup > 1.0 {
            s.cloth_home * 1.1
        } else {
            s.cloth_home
        };
        let home_q = last.max(home_pp * s.pop.max(0.0)).min(total);
        cap[2 * j] = home_q;
        bid[2 * j] = (s.cloth_sup.max(s.cloth_price).max(0.0) * vc).max(export) - make;
        cap[2 * j + 1] = total - home_q;
        bid[2 * j + 1] = export - make;
        purse[j] = s.mpurse.max(0.0);
    }
    for s in state.settlements.iter_mut() {
        s.cloth_made = 0.0;
        s.cloth_made_home = 0.0;
        s.cloth_wool_cost = 0.0;
    }
    let mut idx_of = vec![usize::MAX; state.next_id as usize];
    for (i, s) in state.settlements.iter().enumerate() {
        idx_of[s.id as usize] = i;
    }
    // Each seller's netback a lb, after the duty (never below 0).
    let lb: Vec<f32> = sellers.iter().map(|x| x.lb_p + x.lb_f).collect();
    let rec: Vec<(f32, f32)> = sellers
        .iter()
        .map(|x| {
            (
                (x.gate_p - duty_lb * x.lb_p).max(0.0),
                (x.gate_f - duty_lb * x.lb_f).max(0.0),
            )
        })
        .collect();
    let net: Vec<f32> = (0..sellers.len())
        .map(|k| {
            if lb[k] > 0.0 {
                (rec[k].0 + rec[k].1) / lb[k]
            } else {
                0.0
            }
        })
        .collect();
    // Seller-mill pairs with a surplus, best first.
    let mut cand: Vec<(f32, usize, usize)> = Vec::new();
    for (k, x) in sellers.iter().enumerate() {
        if lb[k] <= 0.0 {
            continue;
        }
        let s = &state.settlements[x.i];
        for t in [2 * x.i, 2 * x.i + 1] {
            if cap[t] > 0.0 && bid[t] - net[k] > 0.0 {
                cand.push((bid[t] - net[k], k, t));
            }
        }
        for q in &s.reach.partners {
            let Some(&j) = idx_of.get(q.id as usize) else {
                continue;
            };
            if j == usize::MAX {
                continue;
            }
            for t in [2 * j, 2 * j + 1] {
                if cap[t] <= 0.0 {
                    continue;
                }
                let sur = bid[t] - vw * tau_w * q.cost - net[k];
                if sur > 0.0 {
                    cand.push((sur, k, t));
                }
            }
        }
    }
    cand.sort_by(|a, b| b.0.total_cmp(&a.0).then(a.1.cmp(&b.1)).then(a.2.cmp(&b.2)));
    let mut left = lb.clone();
    let mut milled = vec![0f32; sellers.len()];
    for &(_, k, t) in &cand {
        if left[k] <= 0.0 || cap[t] <= 0.0 {
            continue;
        }
        let j = t / 2;
        let price = net[k];
        let afford = if price > 0.0 {
            purse[j] / price
        } else {
            f32::INFINITY
        };
        let q = left[k].min(cap[t]).min(afford);
        if q <= 0.0 {
            continue;
        }
        left[k] -= q;
        cap[t] -= q;
        purse[j] -= q * price;
        milled[k] += q;
        let s = &mut state.settlements[j];
        s.mpurse -= q * price;
        s.cloth_made += q;
        if t % 2 == 0 {
            s.cloth_made_home += q;
        }
        s.cloth_wool_cost += q * price;
    }
    // Sellers are paid their netback on all their wool; the port pays for
    // what was exported, the crown takes the duty on it.
    for (k, x) in sellers.iter().enumerate() {
        if lb[k] <= 0.0 {
            continue;
        }
        let (r_p, r_f) = rec[k];
        let lords = p.market_share * r_p;
        let farmers_share = r_p + r_f - lords;
        let toll = p.toll * farmers_share;
        let f = (1.0 - milled[k] / lb[k]).clamp(0.0, 1.0);
        let gross = (x.gross_p + x.gross_f) * f;
        let gate = (x.gate_p + x.gate_f) * f;
        let duty = gate - (r_p + r_f) * f;
        let carriage = gross - gate;
        {
            let s = &mut state.settlements[x.i];
            s.mpurse += lords + toll + carriage;
            s.purse += farmers_share - toll;
            s.wool_in += lords + toll + carriage;
        }
        state.money.wool_in += gross;
        state.money.duties_out += duty;
        state.fallow_wool += x.gross_f;
        state.cloth_stats[0] += lb[k];
        state.cloth_stats[1] += milled[k];
        state.cloth_stats[2] += lb[k] - milled[k];
        if gross > 0.0 {
            let id = state.settlements[x.i].id;
            state
                .wool_ship
                .push((id, x.k1, gross / p.port_world_price.max(1e-6)));
        }
    }
}
