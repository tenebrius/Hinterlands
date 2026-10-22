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
            // Sold in the wool market below (D2, D128).
            let lb_cell = acres * p.fleece_lb * (1.0 - murrain);
            sellers.push(Seller {
                i,
                k1,
                lb_p: lb_cell * p.sheep_per_acre * cells as f32,
                lb_f: lb_cell * p.fallow_share * p.sheep_per_fallow_acre * field_cells as f32,
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
        wool_market(state, p, &route, &sellers, fall);
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

/// One place's clip under D2: lb, pasture and fallow.
struct Seller {
    i: usize,
    k1: u32,
    lb_p: f32,
    lb_f: f32,
}

/// The wool market (D2, D128). Wool has three outlets, split by the S1
/// entropy form as the goods are: home use, the ports and the mills.
///
/// - Home: every place needs `cloth_yd_adult × home_lb_per_yd` lb a year for
///   each adult, spun and woven at home (no coin to the mills), and buys it
///   from the sellers in its reach, its own clip first, with elasticity
///   `elast_cloth`.
/// - Ports: they take any amount at the world price less the export duties
///   (the ancient custom from 1275, the subsidy from 1342), at the nearest
///   port by port reach.
/// - Mills (mill settlements with W, up to `cloth_lb_per_w` × W): they bid
///   what export cloth fetches (the world cloth price less the custom and
///   carriage to the nearest port) less the cost of making it: their
///   capacity at or below the bid, falling with elasticity `elast_mill` above
///   it. They sell all their cloth to the ports.
///
/// Quantities are coin at the world wool price. Buyers pay their price, the
/// sellers get theirs, and carriage and margin go to the buyer's market purse
/// (to the seller's for the ports). The duty and the cloth custom leave the
/// region; what the ports pay for wool and cloth enters it. A mill's margin
/// over its wool counts to its market's inflow.
fn wool_market(
    state: &mut State,
    p: &Params,
    route: &[Option<(f32, u32, u32)>],
    sellers: &[Seller],
    fall: f32,
) {
    let m = state.settlements.len();
    let y = state.year;
    let to_coin = p.port_world_price / (p.wheat_s_bu * 12.0 * p.bu_per_person).max(1e-6);
    // Coin a lb: wool and cloth at the world prices, the duties and the
    // cost of making cloth.
    let vw = (p.wool_price_sack * (1.0 - fall) * 240.0 / 364.0 * to_coin).max(1e-9);
    let vc = p.cloth_d_lb * to_coin;
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
        p.cloth_custom_d / p.cloth_lb.max(1.0) * to_coin
    } else {
        0.0
    };
    let make = p.cloth_cost_d_lb * to_coin;
    let tau_w = p.market_tau / p.rho_wool.max(1e-6);
    let tau_c = p.market_tau / p.rho_cloth.max(1e-6);
    let theta = p.beta / p.market_tau.max(1e-9);
    let home_pp = p.cloth_yd_adult * p.adult_share * p.home_lb_per_yd;
    let port_price = (1.0 - duty_lb / vw).max(0.0);
    for s in state.settlements.iter_mut() {
        s.cloth_made = 0.0;
        s.cloth_wool_cost = 0.0;
        s.cloth_margin = 0.0;
    }
    let mut idx_of = vec![usize::MAX; state.next_id as usize];
    for (i, s) in state.settlements.iter().enumerate() {
        idx_of[s.id as usize] = i;
    }
    // Buyer nodes: 0..m home, m..2m mills, 2m the ports (fixed price).
    let port = 2 * m;
    let mut d0 = vec![0f32; 2 * m];
    let mut elast = vec![p.elast_cloth; 2 * m];
    // A mill's cloth value a lb at its port, and its port route.
    let mut cloth_net = vec![0f32; m];
    let mut mill_bid = vec![0f32; m];
    for (j, s) in state.settlements.iter().enumerate() {
        d0[j] = home_pp * s.pop.max(0.0) * vw;
        if !s.mill || s.market <= 0.0 || p.cloth_lb_per_w <= 0.0 {
            continue;
        }
        let Some((c, _, _)) = route[j] else { continue };
        cloth_net[j] = vc * (1.0 - tau_c * c).max(0.0) - custom_lb;
        let bid = (cloth_net[j] - make) / vw;
        if bid > 0.0 {
            // D(p) = cap min(1, (bid / p)^e): its capacity up to its bid.
            elast[m + j] = p.elast_mill;
            mill_bid[j] = bid;
            d0[m + j] = p.cloth_lb_per_w * s.market * vw;
        }
    }
    // Arcs: (seller, buyer node, transport).
    let mut arcs: Vec<(usize, usize, f32)> = Vec::new();
    let cap: Vec<f32> = sellers.iter().map(|x| (x.lb_p + x.lb_f) * vw).collect();
    for (k, x) in sellers.iter().enumerate() {
        if cap[k] <= 0.0 {
            continue;
        }
        let s = &state.settlements[x.i];
        if d0[x.i] > 0.0 {
            arcs.push((k, x.i, 0.0));
        }
        if d0[m + x.i] > 0.0 {
            arcs.push((k, m + x.i, 0.0));
        }
        for q in &s.reach.partners {
            let Some(&j) = idx_of.get(q.id as usize) else {
                continue;
            };
            if j == usize::MAX {
                continue;
            }
            let t = tau_w * q.cost;
            if d0[j] > 0.0 {
                arcs.push((k, j, t));
            }
            if d0[m + j] > 0.0 {
                arcs.push((k, m + j, t));
            }
        }
        if let Some((c, _, _)) = route[x.i] {
            arcs.push((k, port, tau_w * c));
        }
    }
    if arcs.is_empty() {
        return;
    }
    let mut by_from: Vec<Vec<usize>> = vec![Vec::new(); sellers.len()];
    let mut by_to: Vec<Vec<usize>> = vec![Vec::new(); 2 * m];
    for (a, &(k, b, _)) in arcs.iter().enumerate() {
        by_from[k].push(a);
        if b < port {
            by_to[b].push(a);
        }
    }
    let lse = |v: &[f32]| -> f32 {
        let top = v.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        if !top.is_finite() {
            return f32::NEG_INFINITY;
        }
        top + libm::logf(v.iter().map(|&x| libm::expf(x - top)).sum::<f32>())
    };
    let mut sup = vec![1f32; sellers.len()];
    let mut buy = vec![1f32; 2 * m + 1];
    buy[port] = port_price;
    // Four times the goods' passes, and the sellers last, so that each
    // seller's flows sum to its clip.
    let passes = 4 * p.market_passes;
    for pass in 0..=passes {
        // Sellers sell their whole clip: θ p + ln cap = ln Σ exp(θ (p_b − t)).
        for k in 0..sellers.len() {
            if by_from[k].is_empty() {
                continue;
            }
            let v: Vec<f32> = by_from[k]
                .iter()
                .map(|&a| theta * (buy[arcs[a].1] - arcs[a].2))
                .collect();
            sup[k] = ((lse(&v) - libm::logf(cap[k].max(1e-9))) / theta).clamp(-1e3, 1e3);
        }
        if pass == passes {
            break;
        }
        // Buyers: θ p + ln Σ exp(−θ (p_k + t)) = ln D0 − e ln p.
        for b in 0..2 * m {
            if by_to[b].is_empty() || d0[b] <= 0.0 {
                continue;
            }
            let v: Vec<f32> = by_to[b]
                .iter()
                .map(|&a| -theta * (sup[arcs[a].0] + arcs[a].2))
                .collect();
            let lb = lse(&v);
            let ld = libm::logf(d0[b].max(1e-9));
            let e = elast[b];
            let mut pb = buy[b];
            if b >= m {
                // A mill: ln D = ln cap − e max(0, ln p − ln bid).
                let lbid = libm::logf(mill_bid[b - m].max(1e-6));
                for _ in 0..8 {
                    let over = libm::logf(pb.max(1e-6)) > lbid;
                    let f = theta * pb + lb - ld
                        + if over {
                            e * (libm::logf(pb.max(1e-6)) - lbid)
                        } else {
                            0.0
                        };
                    let df = theta + if over { e / pb.max(1e-6) } else { 0.0 };
                    pb = (pb - f / df).clamp(1e-3, 1e3);
                }
            } else {
                for _ in 0..4 {
                    let f = theta * pb + lb - ld + e * libm::logf(pb.max(1e-6));
                    pb = (pb - f / (theta + e / pb.max(1e-6))).clamp(1e-3, 1e3);
                }
            }
            buy[b] = pb;
        }
    }
    // The flows and their coin.
    let mut receipts = vec![0f32; sellers.len()];
    let mut milled = vec![0f32; m];
    let mut mill_paid = vec![0f32; m];
    let food_of = |s: &crate::state::Settlement| -> f32 {
        let b = if s.price[0] > 0.0 { s.price[0] } else { 1.0 };
        crate::systems::marketplace::market_demand(p, s.market.max(0.0).min(s.pop), b) * b
    };
    // The flows; a mill takes no more than its capacity (D130): what is over
    // goes, seller by seller, to that seller's port at the port's price.
    let mut flow: Vec<f32> = arcs
        .iter()
        .map(|&(k, b, t)| libm::expf((theta * (buy[b] - sup[k] - t)).min(60.0)))
        .collect();
    let mut port_arc = vec![usize::MAX; sellers.len()];
    for (a, &(k, b, _)) in arcs.iter().enumerate() {
        if b == port {
            port_arc[k] = a;
        }
    }
    let mut inflow = vec![0f32; 2 * m];
    for (a, &(k, b, _)) in arcs.iter().enumerate() {
        if b >= m && b < port && port_arc[k] != usize::MAX && flow[a] > 0.0 {
            inflow[b] += flow[a];
        }
    }
    for (a, &(k, b, _)) in arcs.iter().enumerate() {
        if b < m || b >= port || port_arc[k] == usize::MAX || !(flow[a] > 0.0) {
            continue;
        }
        if inflow[b] > d0[b] {
            let over = flow[a] * (1.0 - d0[b] / inflow[b]);
            flow[a] -= over;
            flow[port_arc[k]] += over;
        }
    }
    for (a, &(k, b, t)) in arcs.iter().enumerate() {
        let _ = t;
        let q = flow[a];
        if !(q > 0.0) {
            continue;
        }
        let i = sellers[k].i;
        let lb = q / vw;
        let want = buy[b] * q;
        if b == port {
            // The port pays the world price; the duty leaves the region.
            let r = (sup[k] * q).clamp(0.0, want);
            receipts[k] += r;
            state.settlements[i].mpurse += want - r;
            state.settlements[i].wool_in += want - r;
            state.money.wool_in += q;
            state.money.duties_out += q - want;
            state.cloth_stats[2] += lb;
            state.wool_ship.push((
                state.settlements[i].id,
                sellers[k].k1,
                q / p.port_world_price.max(1e-6),
            ));
            continue;
        }
        let (j, mill) = if b >= m { (b - m, true) } else { (b, false) };
        if j == i && !mill {
            // Own use: no coin moves.
            state.cloth_stats[3] += lb;
            continue;
        }
        // The buyer pays from what its market people's food leaves (D93), by
        // head with its farmers for home wool, from the market for a mill.
        let bs = &state.settlements[j];
        let share_m = if mill {
            1.0
        } else {
            (bs.market.max(0.0) / bs.pop.max(1.0)).clamp(0.0, 1.0)
        };
        let food = food_of(bs);
        let from_m = (want * share_m).min((bs.mpurse - food).max(0.0));
        let from_f = (want * (1.0 - share_m)).min(bs.purse.max(0.0));
        let paid = from_m + from_f;
        let r = (sup[k] * q).clamp(0.0, paid);
        receipts[k] += r;
        {
            let bs = &mut state.settlements[j];
            bs.mpurse -= from_m;
            bs.purse -= from_f;
            bs.mpurse += paid - r;
        }
        if mill {
            milled[j] += lb;
            mill_paid[j] += r;
            state.cloth_stats[1] += lb;
        } else {
            state.cloth_stats[3] += lb;
        }
        // On the seller's path to the buyer, for road loads.
        if let Some(pi) = state.settlements[i]
            .reach
            .partners
            .iter()
            .position(|q| q.id == state.settlements[j].id)
        {
            state.wool_ship.push((
                state.settlements[i].id,
                pi as u32,
                q / p.port_world_price.max(1e-6),
            ));
        }
    }
    // Sellers: the lords' share of the pasture's part, the farmers' the rest
    // less the toll.
    for (k, x) in sellers.iter().enumerate() {
        let lb = x.lb_p + x.lb_f;
        if lb <= 0.0 {
            continue;
        }
        state.cloth_stats[0] += lb;
        let r = receipts[k];
        let lords = p.market_share * r * x.lb_p / lb;
        let farmers_share = r - lords;
        let toll = p.toll * farmers_share;
        let s = &mut state.settlements[x.i];
        s.mpurse += lords + toll;
        s.purse += farmers_share - toll;
        s.wool_in += lords + toll;
        state.fallow_wool += r * x.lb_f / lb;
    }
    state.cloth_stats[4] = d0[..m].iter().sum::<f32>() / vw;
    // Mills make export cloth and sell it at the nearest port.
    for j in 0..m {
        if milled[j] <= 0.0 {
            continue;
        }
        let gross = vc * milled[j];
        let custom = custom_lb * milled[j];
        let id = state.settlements[j].id;
        let k1 = route[j].map(|r| r.1);
        let s = &mut state.settlements[j];
        s.cloth_made = milled[j];
        s.cloth_wool_cost = mill_paid[j];
        s.mpurse += gross - custom;
        s.cloth_margin = (gross - custom - mill_paid[j]).max(0.0);
        state.money.cloth_in += gross;
        state.money.duties_out += custom;
        if let Some(k1) = k1 {
            state
                .wool_ship
                .push((id, k1, gross / p.port_world_price.max(1e-6)));
        }
    }
}
