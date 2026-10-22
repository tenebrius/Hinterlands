//! Marketplaces (v7 M2, D87–D88), in place of the gravity grain push when
//! `marketplaces` is on. Part of the trade system; called by `trade::run`.
//!
//! Two levels (`docs/market/m1-design.md`):
//! 1. **Village to market.** Each place's rent, `market_share × H`, goes to
//!    market in kind; its farmers (the people beyond its market size W) eat
//!    from the rest up to `food_cap × need` and sell what is left. Grain for
//!    sale is routed to markets by `W^α exp(−β c)`, its own market by
//!    `home_weight`, and arrives less the transport loss. What arrives is the
//!    market's turnover.
//! 2. **Market to market.** Markets with `W ≥ market_min` trade with each
//!    other and with the outside ports by the entropy equilibrium (S1/S2's
//!    alternating log-space solves with S2's level step and floor), over the
//!    pairs within reach. A smaller market clears locally and sends its excess
//!    to, or buys its shortfall from, its nearest large market, at that
//!    market's price less, or plus, transport.
//!
//! Buyers at a market are its market people, who buy
//! `W × food_per_person × min(p^(−ε), food_cap)` within a budget, in grain, of
//! the rents sold there, the toll on its turnover and last year's fees; and the
//! ports, up to `outside_grain` at up to the world price. Places in deficit
//! have no surplus to sell, so they rely on relief and stores (S2′, S3). Grain
//! that does not clear at the floor is lost and does not count as turnover.
//! Goods are paid as in S1. W grows on the grain brought to its market from
//! other places (D90, D92). Writes food_in, food_out, market_in (grain
//! arriving from other places), flows, settlements' `price[0]` and `capped`,
//! `grain_stats` and `outside_trade.0`.

use crate::params::Params;
use crate::state::{port_of, Flow, State, NONE};

/// log Σ exp over a list, max-subtracted.
fn lse(v: &[f32]) -> f32 {
    let top = v.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    if !top.is_finite() {
        return f32::NEG_INFINITY;
    }
    top + libm::logf(v.iter().map(|&x| libm::expf(x - top)).sum::<f32>())
}

/// A small market's net position at its large market's price `pb`, given the
/// transport price `t` between them: (net export, own price). `want(p)` is
/// its people's demand at `p`.
fn small_net(supply: f32, pb: f32, t: f32, floor: f32, want: &dyn Fn(f32) -> f32) -> (f32, f32) {
    let low = (pb - t).max(floor);
    if supply >= want(low) {
        return (supply - want(low), low);
    }
    let high = pb + t;
    if supply <= want(high) {
        return (supply - want(high), high);
    }
    // Clears locally between the two.
    let (mut lo, mut hi) = (low, high);
    for _ in 0..20 {
        let mid = 0.5 * (lo + hi);
        if want(mid) > supply {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    (0.0, 0.5 * (lo + hi))
}

/// What a market's people would buy at price `b`, before their budget: their
/// food, `W × food_per_person × min(b^(−ε), food_cap)`, and their
/// establishment's (malt, fodder, households), `(K − 1)` times that at its
/// own elasticity `elast_est`, so that in dearth it gives way first (D90, D99).
pub fn market_demand(p: &Params, people: f32, b: f32) -> f32 {
    let b = b.max(1e-3);
    let food = people * p.food_per_person;
    food * libm::powf(b, -p.elast_grain).min(p.food_cap)
        + (p.k_maint - 1.0).max(0.0) * food * libm::powf(b, -p.elast_est).min(p.food_cap)
}

#[allow(clippy::too_many_arguments)]
pub fn trade(
    state: &mut State,
    p: &Params,
    idx_of: &[u32],
    attract: &[f32],
    food_in: &mut [f32],
    food_out: &mut [f32],
    market_in: &mut [f32],
    flows: &mut Vec<Flow>,
) {
    let m = state.settlements.len();
    let ports = if p.outside_ports {
        state.outside.len()
    } else {
        0
    };
    let theta = p.beta / p.market_tau.max(1e-9);
    let eps = p.elast_grain;
    let cap = p.food_cap;
    let fpp = p.food_per_person;
    let settlement = |id: u32| -> Option<usize> {
        idx_of
            .get(id as usize)
            .copied()
            .filter(|&j| j != u32::MAX)
            .map(|j| j as usize)
    };

    // 1. Village to market.
    let mut own = vec![0f32; m];
    let mut turnover = vec![0f32; m];
    // Grain arriving from other places: a central place's draw (D92).
    let mut drawn = vec![0f32; m];
    let mut rent_in = vec![0f32; m];
    // The farmers' surplus as it arrives at each market, for their purse:
    // (seller, market, grain).
    let mut surplus_arr: Vec<(usize, usize, f32)> = Vec::new();
    // What the farmers lack below their need, for the purse to buy (D95).
    let mut deficit = vec![0f32; m];
    for (i, s) in state.settlements.iter().enumerate() {
        let h = s.food_prod.max(0.0);
        let farm_need = (s.pop - s.market.max(0.0)).max(0.0) * fpp;
        deficit[i] = (farm_need - ((1.0 - p.market_share) * h).min(cap * farm_need)).max(0.0);
        let rent = p.market_share * h;
        own[i] = ((1.0 - p.market_share) * h).min(cap * farm_need);
        let sale = h - own[i];
        food_out[i] += sale;
        if sale <= 0.0 {
            continue;
        }
        let mut ws: Vec<(usize, u32, f32, f32)> = vec![(i, NONE, attract[i] * p.home_weight, 0.0)];
        for (k, q) in s.reach.partners.iter().enumerate() {
            if let Some(j) = settlement(q.id) {
                ws.push((
                    j,
                    k as u32,
                    attract[j] * libm::expf(-p.beta * q.cost),
                    q.cost,
                ));
            }
        }
        let total: f32 = ws.iter().map(|w| w.2).sum();
        if total <= 0.0 {
            continue;
        }
        let rent_frac = rent / sale;
        for &(j, k, w, c) in &ws {
            let sent = sale * w / total;
            let arrived = sent * (1.0 - p.transport_loss * c).max(0.0);
            turnover[j] += arrived;
            if j != i {
                drawn[j] += arrived;
            }
            rent_in[j] += arrived * rent_frac;
            if rent_frac < 1.0 {
                surplus_arr.push((i, j, arrived * (1.0 - rent_frac)));
            }
            if k != NONE && sent >= p.grain_flow_min {
                flows.push(Flow {
                    from: s.id,
                    to: state.settlements[j].id,
                    amount: sent,
                    food: sent,
                    partner_idx: k,
                    good: None,
                    payment: false,
                    import: false,
                    relief: false,
                });
            }
        }
    }

    // What a market's people consume, K per market person (D90): their
    // food and their establishment's (malt, fodder, households).
    let need_w: Vec<f32> = state
        .settlements
        .iter()
        .map(|s| p.k_maint * s.market.max(0.0).min(s.pop) * fpp)
        .collect();
    // Market people buy within their market's purse, and farmers short of
    // grain within theirs (D95, D97): the coin held at the start of the
    // grain trade. This year's takings are next year's budget.
    let budget: Vec<f32> = state.settlements.iter().map(|s| s.mpurse.max(0.0)).collect();
    let purse: Vec<f32> = state.settlements.iter().map(|s| s.purse.max(0.0)).collect();
    let people: Vec<f32> = state
        .settlements
        .iter()
        .map(|s| s.market.max(0.0).min(s.pop))
        .collect();
    let food_w = |j: usize, b: f32| -> f32 {
        market_demand(p, people[j], b).min(budget[j] / b.max(1e-3))
    };
    let dearth = |j: usize, b: f32| -> f32 { deficit[j].min(purse[j] / b.max(1e-3)) };
    let want = |j: usize, b: f32| -> f32 { food_w(j, b) + dearth(j, b) };

    // 2. Large markets, and each small market's nearest large one in reach.
    // A place with an outside port in reach trades with the ships whatever
    // its size: that is where coin enters the region (D97).
    let big: Vec<bool> = state
        .settlements
        .iter()
        .map(|s| {
            s.market >= p.market_min
                || (ports > 0
                    && s.reach
                        .partners
                        .iter()
                        .any(|q| port_of(q.id).is_some_and(|k| k < ports)))
        })
        .collect();
    let mut node_of = vec![usize::MAX; m];
    let mut nodes_settle: Vec<usize> = Vec::new();
    for j in 0..m {
        if big[j] {
            node_of[j] = nodes_settle.len();
            nodes_settle.push(j);
        }
    }
    let nb = nodes_settle.len();
    let nodes = nb + ports;
    // (parent node, transport price, small market's partner index).
    let parent: Vec<Option<(usize, f32, u32)>> = state
        .settlements
        .iter()
        .enumerate()
        .map(|(j, s)| {
            if big[j] {
                return None;
            }
            s.reach
                .partners
                .iter()
                .enumerate()
                .filter_map(|(k, q)| {
                    let b = settlement(q.id)?;
                    big[b].then_some((node_of[b], p.market_tau * q.cost, k as u32, q.cost))
                })
                .min_by(|a, b| a.3.total_cmp(&b.3).then(a.0.cmp(&b.0)))
                .map(|(b, t, k, _)| (b, t, k))
        })
        .collect();
    let mut children: Vec<Vec<usize>> = vec![Vec::new(); nb];
    for j in 0..m {
        if let Some((b, _, _)) = parent[j] {
            children[b].push(j);
        }
    }
    // Arcs between large markets and to the ports: (from node, to node,
    // transport price, from settlement's partner index or NONE for own).
    let mut arcs: Vec<(usize, usize, f32, u32)> = Vec::new();
    let mut from_start = vec![0usize; nb + 1];
    for (bi, &j) in nodes_settle.iter().enumerate() {
        from_start[bi] = arcs.len();
        arcs.push((bi, bi, 0.0, NONE));
        for (k, q) in state.settlements[j].reach.partners.iter().enumerate() {
            let to = if let Some(pk) = port_of(q.id) {
                if pk < ports {
                    nb + pk
                } else {
                    continue;
                }
            } else if let Some(o) = settlement(q.id).filter(|&o| big[o]) {
                node_of[o]
            } else {
                continue;
            };
            arcs.push((bi, to, p.market_tau * q.cost, k as u32));
        }
    }
    from_start[nb] = arcs.len();
    let mut to_list: Vec<Vec<usize>> = vec![Vec::new(); nodes];
    for (a, arc) in arcs.iter().enumerate() {
        to_list[arc.1].push(a);
    }

    // Prices, warm-started, re-levelled by the regional level (D78).
    let total_t: f32 = turnover.iter().sum();
    let total_w: f32 = need_w.iter().sum::<f32>().max(1e-6);
    let level = libm::powf((total_t / total_w).max(1e-3), -1.0 / eps.max(0.05));
    let relevel = if state.grain_level > 0.0 {
        level / state.grain_level
    } else {
        1.0
    };
    state.grain_level = level;
    let start = |v: f32| if v > 0.0 { v * relevel } else { level };
    let mut buy: Vec<f32> = (0..nodes)
        .map(|n| {
            let v = if n < nb {
                state.settlements[nodes_settle[n]].price[0]
            } else {
                state.port_grain.get(n - nb).copied().unwrap_or(0.0)
            };
            start(v).clamp(1e-3, 1e3)
        })
        .collect();
    let mut sup: Vec<f32> = buy[..nb].iter().map(|&b| b.max(p.p_floor)).collect();
    // Market stores (M3, D94; Williams & Wright 1991): a large market's store
    // sells when the price is above its ten-year average and buys when it is
    // below the average less the holding margin, in proportion to the gap, at
    // most `store_cap` × its turnover, within what it holds or has room for,
    // and buying within its people's budget left after food.
    let margin = if state.year < p.store_margin_year {
        p.store_margin_early
    } else {
        p.store_margin_late
    };
    let pbar: Vec<f32> = nodes_settle
        .iter()
        .map(|&j| {
            let a = state.settlements[j].avg_price;
            if a > 0.0 {
                a
            } else {
                level
            }
        })
        .collect();
    let held: Vec<f32> = nodes_settle
        .iter()
        .map(|&j| state.settlements[j].mstore.max(0.0))
        .collect();
    // Last year's price at each large market: with `store_spike`, stores sell
    // only into a jump above it, not into a rising trend.
    let last: Vec<f32> = nodes_settle
        .iter()
        .map(|&j| state.settlements[j].price[0])
        .collect();
    let store_sell = |bi: usize, b: f32| -> f32 {
        let mut ceiling = pbar[bi] * (1.0 + p.store_sell_band * margin);
        if p.store_spike && last[bi] > 0.0 {
            ceiling = ceiling.max(last[bi] * (1.0 + margin));
        }
        if !p.market_stores || b <= ceiling {
            return 0.0;
        }
        let t = turnover[nodes_settle[bi]];
        (p.store_gain * (b - ceiling) / pbar[bi] * t)
            .min(p.store_cap * t)
            .min(held[bi])
    };
    let store_buy = |bi: usize, b: f32| -> f32 {
        let floor_price = pbar[bi] * (1.0 - margin);
        if !p.market_stores || b >= floor_price {
            return 0.0;
        }
        let j = nodes_settle[bi];
        let t = turnover[j];
        let left = (budget[j] / b.max(1e-3) - food_w(j, b)).max(0.0);
        (p.store_gain * (floor_price - b) / pbar[bi] * t)
            .min((p.store_cap * t - held[bi]).max(0.0))
            .min(left)
    };
    // Each node's supply (turnover plus its small markets' exports) and
    // demand (its people plus its small markets' imports), at its price.
    let position = |bi: usize, b: f32| -> (f32, f32) {
        let j = nodes_settle[bi];
        let (mut s, mut d) = (
            turnover[j] + store_sell(bi, b),
            want(j, b) + store_buy(bi, b),
        );
        for &c in &children[bi] {
            let t = parent[c].map_or(0.0, |x| x.1);
            let (net, _) = small_net(turnover[c], b, t, p.p_floor, &|q| want(c, q));
            if net > 0.0 {
                s += net;
            } else {
                d -= net;
            }
        }
        (s, d)
    };
    let demand = |n: usize, b: f32| -> f32 {
        if n >= nb {
            return p.outside_grain * libm::powf(b / p.port_world_price, -p.port_elast);
        }
        position(n, b).1
    };
    let mut supply: Vec<f32> = (0..nb).map(|bi| position(bi, buy[bi]).0).collect();
    let mut sold = vec![0f32; nb];
    let mut scratch: Vec<f32> = Vec::new();
    let mut residual = f32::INFINITY;
    let mut passes = 0u32;
    while passes < p.market_passes + 8 && nb > 0 {
        passes += 1;
        // Suppliers: θ s + ln S = ln Σ_k exp(θ (b_k − t)), with the floor.
        for bi in 0..nb {
            supply[bi] = position(bi, buy[bi]).0;
            let (a0, a1) = (from_start[bi], from_start[bi + 1]);
            if supply[bi] <= 1e-6 {
                sold[bi] = 0.0;
                continue;
            }
            scratch.clear();
            scratch.extend(arcs[a0..a1].iter().map(|a| theta * (buy[a.1] - a.2)));
            let a = lse(&scratch);
            sup[bi] = ((a - libm::logf(supply[bi])) / theta).clamp(p.p_floor, 1e3);
            sold[bi] = libm::expf((a - theta * sup[bi]).min(60.0)).min(supply[bi]);
        }
        // The level: a common shift that leaves flows as they are, chosen so
        // demand meets sales; supply prices stop at the floor.
        let total_sold: f32 = sold.iter().sum();
        if total_sold > 0.0 {
            let lowest = buy.iter().copied().fold(f32::INFINITY, f32::min);
            let dearest = sup.iter().copied().fold(p.p_floor, f32::max);
            let (mut lo, mut hi) = ((1e-3 - lowest).max(p.p_floor - dearest), 5.0f32);
            if lo < hi {
                for _ in 0..20 {
                    let mid = 0.5 * (lo + hi);
                    let d: f32 = (0..nodes)
                        .map(|n| demand(n, (buy[n] + mid).max(1e-3)))
                        .sum();
                    if d > total_sold {
                        lo = mid;
                    } else {
                        hi = mid;
                    }
                }
                let d = 0.5 * (lo + hi);
                buy.iter_mut().for_each(|b| *b = (*b + d).clamp(1e-3, 1e3));
                sup.iter_mut()
                    .for_each(|s| *s = (*s + d).clamp(p.p_floor, 1e3));
            }
        }
        // Buyers: θ b + ln Σ_i exp(−θ (s_i + t)) = ln D(b), Newton.
        for n in 0..nodes {
            if to_list[n].is_empty() {
                continue;
            }
            scratch.clear();
            scratch.extend(
                to_list[n]
                    .iter()
                    .map(|&a| -theta * (sup[arcs[a].0] + arcs[a].2)),
            );
            let e = lse(&scratch);
            let mut b = buy[n];
            for _ in 0..6 {
                let ld = libm::logf(demand(n, b).max(1e-9));
                let h = 1e-3 * b.max(1e-3);
                let ld2 = libm::logf(demand(n, b + h).max(1e-9));
                let slope = ((ld - ld2) / h).clamp(0.0, 50.0);
                b = (b - (theta * b + e - ld) / (theta + slope)).clamp(1e-3, 1e3);
            }
            buy[n] = b;
        }
        if passes >= p.market_passes {
            let inflow = shares(&arcs, &from_start, &buy, &sold, theta, nodes, &mut scratch).1;
            let (mut gap, mut tot) = (0f32, 0f32);
            for n in 0..nodes {
                if to_list[n].is_empty() {
                    continue;
                }
                let d = demand(n, buy[n]);
                gap += (inflow[n] - d).abs();
                tot += d;
            }
            residual = gap / tot.max(1.0);
            if residual <= 0.05 {
                break;
            }
        }
    }
    let (flow, inflow) = shares(&arcs, &from_start, &buy, &sold, theta, nodes, &mut scratch);

    // 3. Accounts. Market people eat their share of what their market takes;
    // small markets eat locally and trade the rest through their large one.
    let mut price = vec![0f32; m];
    let mut eaten_w = vec![0f32; m];
    let mut sold_here = vec![0f32; m];
    let mut capped = vec![false; m];
    let mut unsold = 0f32;
    let mut store_delta = vec![0f32; m];
    let mut store_take = vec![0f32; m];
    // Of which, at places with no large market in reach.
    let mut unsold_local = 0f32;
    for (bi, &j) in nodes_settle.iter().enumerate() {
        let b = buy[bi];
        price[j] = b;
        let frac = if supply[bi] > 0.0 {
            sold[bi] / supply[bi]
        } else {
            0.0
        };
        unsold += supply[bi] - sold[bi];
        // What arrives is shared among its people and its small markets'
        // imports in proportion to their demand.
        let mine = want(j, b);
        let d_all = demand(bi, b).max(1e-9);
        let got = inflow[bi];
        eaten_w[j] = got * mine / d_all;
        // The store's trade: what it bought arrives in store, and what it
        // offered sells as the rest of the market's supply does.
        let bought = got * store_buy(bi, b) / d_all;
        let offered = store_sell(bi, b);
        let sold_store = offered * frac;
        unsold -= offered - sold_store;
        store_delta[j] = bought - sold_store;
        store_take[j] = sold_store - bought;
        sold_here[j] = turnover[j] * frac;
        capped[j] = market_demand(p, people[j], b) > budget[j] / b.max(1e-3) + 1e-3;
        for &c in &children[bi] {
            let (t, k) = parent[c].map_or((0.0, NONE), |x| (x.1, x.2));
            let (net, pc) = small_net(turnover[c], b, t, p.p_floor, &|q| want(c, q));
            price[c] = pc;
            capped[c] = market_demand(p, people[c], pc) > budget[c] / pc.max(1e-3) + 1e-3;
            let local = turnover[c].min(want(c, pc));
            let moved = if net > 0.0 {
                // Exports share the large market's sell-through.
                eaten_w[c] = local;
                sold_here[c] = local + net * frac;
                net
            } else {
                let bought = -net * got / d_all;
                eaten_w[c] =
                    local + bought * (1.0 - p.transport_loss * t / p.market_tau.max(1e-9)).max(0.0);
                sold_here[c] = local;
                bought
            };
            if moved >= p.grain_flow_min && k != NONE {
                let s = &state.settlements[c];
                flows.push(Flow {
                    from: s.id,
                    to: s.reach.partners[k as usize].id,
                    amount: moved,
                    food: moved,
                    partner_idx: k,
                    good: None,
                    payment: false,
                    import: false,
                    relief: false,
                });
            }
        }
    }
    // Places with no large market in reach clear on their own.
    for j in 0..m {
        if big[j] || parent[j].is_some() {
            continue;
        }
        let (net, pc) = small_net(turnover[j], 1.0, 1e3, p.p_floor, &|q| want(j, q));
        price[j] = pc;
        eaten_w[j] = turnover[j] - net.max(0.0);
        sold_here[j] = eaten_w[j];
        unsold += net.max(0.0);
        unsold_local += net.max(0.0);
        capped[j] = market_demand(p, people[j], pc) > budget[j] / pc.max(1e-3) + 1e-3;
    }
    let mut to_ports = 0f32;
    let mut pairs = Vec::new();
    for (a, &(from, to, t, k)) in arcs.iter().enumerate() {
        let q = flow[a];
        if from != to && to < nb && q >= 1.0 {
            pairs.push((
                state.settlements[nodes_settle[from]].id,
                state.settlements[nodes_settle[to]].id,
                q,
                buy[to] - sup[from],
                t,
            ));
        }
        if to >= nb {
            to_ports += q;
        }
        if from == to || q < p.grain_flow_min || k == NONE {
            continue;
        }
        let s = &state.settlements[nodes_settle[from]];
        flows.push(Flow {
            from: s.id,
            to: s.reach.partners[k as usize].id,
            amount: q,
            food: q,
            partner_idx: k,
            good: None,
            payment: false,
            import: false,
            relief: false,
        });
    }
    // 4. Grain settles in coin (D96, D97). Buyers pay their price for what
    // they took, within their purses, and the ports from outside. Sellers
    // get their market's price on what it sold, less the toll: rent to the
    // market's purse, the farmers' surplus to theirs, store grain to the
    // market's. The toll goes to the market where the grain is sold, and the
    // rest of what buyers paid, carriage and handling, to the buyers' markets.
    let mut paid_m = vec![0f32; m];
    let mut paid_f = vec![0f32; m];
    let (mut dearth_need, mut dearth_got_all) = (0f32, 0f32);
    let mut dearth_got = vec![0f32; m];
    for j in 0..m {
        let all = want(j, price[j]).max(1e-9);
        dearth_got[j] = eaten_w[j] * dearth(j, price[j]) / all;
        dearth_need += deficit[j];
        dearth_got_all += dearth_got[j];
        let b = price[j];
        paid_f[j] = (dearth_got[j] * b).min(purse[j]);
        let store_cost = if big[j] { (-store_take[j]).max(0.0) * b } else { 0.0 };
        paid_m[j] = ((eaten_w[j] - dearth_got[j]) * b + store_cost).min(budget[j]);
    }
    let port_in: f32 = (nb..nodes).map(|n| inflow[n] * buy[n]).sum();
    let pool: f32 = paid_m.iter().sum::<f32>() + paid_f.iter().sum::<f32>() + port_in;
    // What each place sold, at its price.
    let mut value = vec![0f32; m];
    for j in 0..m {
        let frac = if turnover[j] > 0.0 {
            (sold_here[j] / turnover[j]).min(1.0)
        } else {
            0.0
        };
        value[j] = frac * price[j];
    }
    let mut owed_m = vec![0f32; m];
    let mut owed_f = vec![0f32; m];
    for j in 0..m {
        owed_m[j] += rent_in[j] * value[j] + store_take[j].max(0.0) * price[j];
    }
    for &(i, j, q) in &surplus_arr {
        owed_f[i] += q * value[j];
    }
    let owed: f32 = owed_m.iter().sum::<f32>() + owed_f.iter().sum::<f32>();
    // Sellers are paid in full if buyers paid that much, else pro rata.
    let scale = if owed > pool { pool / owed.max(1e-9) } else { 1.0 };
    let gap = pool - owed * scale;
    let paid_all: f32 = paid_m.iter().sum::<f32>() + paid_f.iter().sum::<f32>();
    let mut tolls = 0f32;
    let mut m_income = 0f32;
    for (j, s) in state.settlements.iter_mut().enumerate() {
        let carriage = if paid_all > 0.0 {
            gap * (paid_m[j] + paid_f[j]) / paid_all
        } else {
            0.0
        };
        // The toll on rent grain stays with the market it is paid to.
        s.mpurse += owed_m[j] * scale - paid_m[j] + carriage;
        s.purse += owed_f[j] * scale * (1.0 - p.toll) - paid_f[j];
        tolls += p.toll * owed_m[j] * scale;
        m_income += owed_m[j] * scale + carriage;
    }
    // The toll on the farmers' grain goes to the market that sold it.
    for &(_, j, q) in &surplus_arr {
        let t = p.toll * q * value[j] * scale;
        state.settlements[j].mpurse += t;
        tolls += t;
        m_income += t;
    }
    if gap > 0.0 && paid_all <= 0.0 {
        // Only the ports bought: their carriage goes to the markets that sold.
        let total: f32 = (0..m).map(|j| sold_here[j]).sum::<f32>().max(1e-9);
        for j in 0..m {
            state.settlements[j].mpurse += gap * sold_here[j] / total;
        }
    }
    state.money.port_in += port_in;

    let mut capped_n = 0u32;
    for (j, s) in state.settlements.iter_mut().enumerate() {
        let market_got = eaten_w[j] - dearth_got[j];
        // The people eat first, up to their growth target; the rest is the
        // establishment's use (D90).
        let people = s.market.max(0.0).min(s.pop) * fpp;
        food_in[j] += dearth_got[j] + market_got.min(cap * people);
        // W grows on the grain brought to its market from other places
        // (D90, D92): a central place's draw from its hinterland.
        market_in[j] += drawn[j];
        s.price[0] = price[j];
        s.capped = capped[j];
        // Stores: this year's trade, then the year's loss; the average price
        // follows (M3, D94).
        if big[j] {
            s.mstore = ((s.mstore + store_delta[j]) * (1.0 - p.store_loss)).max(0.0);
            s.store_income = store_take[j].max(0.0);
        } else {
            s.store_income = 0.0;
        }
        if price[j] > 0.0 {
            s.avg_price = if s.avg_price > 0.0 {
                s.avg_price + (price[j] - s.avg_price) / p.store_avg_years.max(1.0)
            } else {
                price[j]
            };
        }
        capped_n += capped[j] as u32;
    }

    // 5. Year end (D97). The farmers spend `purse_loss` of their purse, and
    // any excess over its cap, with the markets they sell to; the markets
    // spend `mpurse_loss` of theirs, and any excess, on the farmers whose
    // grain they take, less `luxury_out` that leaves the region. Then coin
    // wears.
    let mut sells_to: Vec<f32> = vec![0f32; m];
    for &(i, _, q) in &surplus_arr {
        sells_to[i] += q;
    }
    let mut buys_from: Vec<f32> = vec![0f32; m];
    for &(_, j, q) in &surplus_arr {
        buys_from[j] += q;
    }
    let mut out_f = vec![0f32; m];
    let mut out_m = vec![0f32; m];
    for (j, s) in state.settlements.iter().enumerate() {
        let need_f = (s.pop - s.market.max(0.0)).max(0.0) * fpp;
        let f = s.purse.max(0.0);
        out_f[j] = f * p.purse_loss + (f * (1.0 - p.purse_loss) - p.purse_cap * need_f).max(0.0);
        let mm = s.mpurse.max(0.0);
        out_m[j] = mm * p.mpurse_loss + (mm * (1.0 - p.mpurse_loss) - p.mpurse_cap * need_w[j]).max(0.0);
    }
    let mut luxury = 0f32;
    for j in 0..m {
        state.settlements[j].purse -= out_f[j];
        state.settlements[j].mpurse -= out_m[j];
        if sells_to[j] <= 0.0 {
            state.settlements[j].mpurse += out_f[j];
        }
        let lux = out_m[j] * p.luxury_out;
        luxury += lux;
        if buys_from[j] <= 0.0 {
            state.settlements[j].purse += out_m[j] - lux;
        }
    }
    for &(i, j, q) in &surplus_arr {
        if sells_to[i] > 0.0 {
            state.settlements[j].mpurse += out_f[i] * q / sells_to[i];
        }
        if buys_from[j] > 0.0 {
            state.settlements[i].purse += out_m[j] * (1.0 - p.luxury_out) * q / buys_from[j];
        }
    }
    state.money.luxury_out += luxury;
    let mut wear = 0f32;
    for s in state.settlements.iter_mut() {
        let w = (s.purse.max(0.0) + s.mpurse.max(0.0)) * p.coin_wear;
        wear += w;
        s.purse -= s.purse.max(0.0) * p.coin_wear;
        s.mpurse -= s.mpurse.max(0.0) * p.coin_wear;
    }
    state.money.wear += wear;

    state.port_grain = buy[nb..].to_vec();
    state.outside_trade.0 = to_ports;
    state.market_pairs = pairs;
    state.grain_stats = crate::state::GrainStats {
        residual: if nb > 0 { residual } else { 0.0 },
        passes,
        unsold,
        harvest: state.settlements.iter().map(|s| s.food_prod).sum(),
        capped: capped_n,
        estab_share: f32::NAN,
        arrived: total_t,
        unsold_local,
        markets: nb as u32,
        dearth_need,
        dearth_bought: dearth_got_all,
        toll_share: if m_income > 0.0 {
            tolls / m_income
        } else {
            f32::NAN
        },
    };
}

/// Each supplier's sales shared over its outlets by a softmax of
/// `θ (b_k − t)`; the flow on each arc and each buyer's inflow.
fn shares(
    arcs: &[(usize, usize, f32, u32)],
    from_start: &[usize],
    buy: &[f32],
    sold: &[f32],
    theta: f32,
    nodes: usize,
    scratch: &mut Vec<f32>,
) -> (Vec<f32>, Vec<f32>) {
    let mut flow = vec![0f32; arcs.len()];
    let mut inflow = vec![0f32; nodes];
    for i in 0..from_start.len() - 1 {
        let (a0, a1) = (from_start[i], from_start[i + 1]);
        if a0 == a1 || sold[i] <= 0.0 {
            continue;
        }
        scratch.clear();
        scratch.extend(arcs[a0..a1].iter().map(|a| theta * (buy[a.1] - a.2)));
        let top = scratch.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        let total: f32 = scratch.iter().map(|&x| libm::expf(x - top)).sum();
        for (k, &x) in (a0..a1).zip(scratch.iter()) {
            flow[k] = sold[i] * libm::expf(x - top) / total;
            inflow[arcs[k].1] += flow[k];
        }
    }
    (flow, inflow)
}
