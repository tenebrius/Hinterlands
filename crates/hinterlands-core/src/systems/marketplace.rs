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
//! Goods are paid as in S1. Writes food_in, food_out, market_in (grain sold
//! at the market), flows, settlements' `price[0]` and `capped`,
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
    let mut rent_in = vec![0f32; m];
    for (i, s) in state.settlements.iter().enumerate() {
        let h = s.food_prod.max(0.0);
        let farm_need = (s.pop - s.market.max(0.0)).max(0.0) * fpp;
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
            rent_in[j] += arrived * rent_frac;
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

    // Market people's demand at price `b`, within their budget in grain:
    // rents sold here, the toll and last year's fees.
    let need_w: Vec<f32> = state
        .settlements
        .iter()
        .map(|s| s.market.max(0.0).min(s.pop) * fpp)
        .collect();
    let budget: Vec<f32> = (0..m)
        .map(|j| rent_in[j] + p.toll * turnover[j] + state.settlements[j].fee_earned)
        .collect();
    let want = |j: usize, b: f32| -> f32 {
        (need_w[j] * libm::powf(b.max(1e-3), -eps).min(cap)).min(budget[j].max(0.0))
    };

    // 2. Large markets, and each small market's nearest large one in reach.
    let big: Vec<bool> = state
        .settlements
        .iter()
        .map(|s| s.market >= p.market_min)
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
    // Each node's supply (turnover plus its small markets' exports) and
    // demand (its people plus its small markets' imports), at its price.
    let position = |bi: usize, b: f32| -> (f32, f32) {
        let j = nodes_settle[bi];
        let (mut s, mut d) = (turnover[j], want(j, b));
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
            return p.outside_grain * libm::powf(b / p.port_world_price, -p.port_elast).min(1.0);
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
        sold_here[j] = turnover[j] * frac;
        capped[j] = need_w[j] * libm::powf(b, -eps).min(cap) > budget[j] + 1e-3;
        for &c in &children[bi] {
            let (t, k) = parent[c].map_or((0.0, NONE), |x| (x.1, x.2));
            let (net, pc) = small_net(turnover[c], b, t, p.p_floor, &|q| want(c, q));
            price[c] = pc;
            capped[c] = need_w[c] * libm::powf(pc, -eps).min(cap) > budget[c] + 1e-3;
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
        capped[j] = need_w[j] * libm::powf(pc, -eps).min(cap) > budget[j] + 1e-3;
    }
    let mut to_ports = 0f32;
    for (a, &(from, to, _, k)) in arcs.iter().enumerate() {
        let q = flow[a];
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
    let mut capped_n = 0u32;
    for (j, s) in state.settlements.iter_mut().enumerate() {
        food_in[j] += eaten_w[j];
        market_in[j] += sold_here[j];
        s.price[0] = price[j];
        s.capped = capped[j];
        capped_n += capped[j] as u32;
    }
    state.port_grain = buy[nb..].to_vec();
    state.outside_trade.0 = to_ports;
    state.grain_stats = crate::state::GrainStats {
        residual: if nb > 0 { residual } else { 0.0 },
        passes,
        unsold,
        harvest: state.settlements.iter().map(|s| s.food_prod).sum(),
        capped: capped_n,
        estab_share: f32::NAN,
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
