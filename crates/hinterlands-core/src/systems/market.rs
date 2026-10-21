//! Shadow prices (v6 S0, D69; Samuelson 1952; Takayama & Judge 1971; Wilson
//! 1970). With `market_shadow`, after trade, an entropy-regularised spatial
//! price equilibrium is solved for grain and the four goods (salt, iron,
//! stone, imports) and the prices are recorded. Nothing feeds back.
//!
//! Each place has one price per good, in grain units. Along each stored path
//! from `i` to a partner `j` the would-be flow is
//! `x_ij = m_j · exp(θ (p_j − p_i − t_ij))`, with `t_ij = τ_g · cost`,
//! `τ_g = market_tau / ρ_g` and `θ = β / market_tau`, so that at equal prices
//! the flows fall off with cost as the gravity trade does (`exp(−β c / ρ_g)`).
//! For grain the destination's market size is a premium, `m_j = W_j^α`
//! (decision 4); for goods `m_j = 1`. Each place clears its supply and its
//! demand `D0 · p^(−ε)` against its outflow and inflow. Prices are solved by
//! `market_passes` Furness (Sinkhorn) passes: each supplier's supply is shared
//! among its outlets by a softmax of `θ p_j − θ t_ij + ln m_j`, and each buyer
//! takes a damped Newton step towards `inflow = D0 · p^(−ε)`. Before each pass
//! the price level is set so that the region's demand equals its supply, since
//! a buyer's own step only moves its price against its rivals'. Supplier prices
//! follow in closed form. Prices are warm-started from last year. Goods supply
//! is what the sites sold this year. Outside ports sell imports without limit
//! at a price of 1, and buy their fixed grain at the price that clears it.
//! Writes: settlements' `price`, `market_shadow` (residuals and grain arcs).

use crate::params::Params;
use crate::state::{port_of, Good, State};
use crate::terrain::Terrain;
use crate::Rng;

/// Goods priced: grain, then salt, iron, stone, imports.
pub const PRICED: usize = 5;

/// One directed arc: supplier index, buyer index, transport price, premium.
/// Indices at or above the settlement count are outside ports.
struct Arc {
    from: usize,
    to: usize,
    t: f32,
    premium: f32,
}

pub fn run(state: &mut State, _t: &Terrain, p: &Params, _rng: &mut Rng) {
    if !p.market_shadow {
        return;
    }
    let m = state.settlements.len();
    let ports = state.outside.len();
    let nodes = m + ports;
    let mut idx_of = vec![u32::MAX; state.next_id as usize];
    for (i, s) in state.settlements.iter().enumerate() {
        idx_of[s.id as usize] = i as u32;
    }
    let node_of = |id: u32| -> Option<usize> {
        if let Some(k) = port_of(id) {
            return (k < ports).then_some(m + k);
        }
        idx_of
            .get(id as usize)
            .copied()
            .filter(|&j| j != u32::MAX)
            .map(|j| j as usize)
    };
    let theta = p.beta / p.market_tau.max(1e-9);
    // The market premium on grain, W^α, as the trade uses it.
    let premium_of: Vec<f32> = state
        .settlements
        .iter()
        .map(|s| {
            let w = if p.market_state {
                s.market.max(p.market_floor)
            } else {
                crate::systems::trade::urban_pop(s, p) + p.urban_seed
            };
            libm::powf(w, p.alpha)
        })
        .collect();
    let mut residual = [0f32; PRICED];
    let mut grain_arcs: Vec<(u32, u32, f32, f32, f32)> = Vec::new();
    for g in 0..PRICED {
        let rho = match g {
            0 => 1.0,
            1 => p.rho_salt,
            2 => p.rho_iron,
            3 => p.rho_stone,
            _ => p.rho_import,
        };
        let tau = p.market_tau / rho.max(1e-6);
        let elast = match g {
            0 => p.elast_grain,
            1 => p.elast_salt,
            2 => p.elast_iron,
            3 => p.elast_stone,
            _ => p.elast_import,
        };
        // Supply and reference demand at each node.
        let mut supply = vec![0f32; nodes];
        let mut demand = vec![0f32; nodes];
        for (i, s) in state.settlements.iter().enumerate() {
            match g {
                0 => {
                    supply[i] = s.food_prod;
                    demand[i] = s.need(p);
                }
                4 => {
                    demand[i] = p.import_demand
                        * if p.market_state {
                            s.market.max(0.0)
                        } else {
                            crate::systems::trade::urban_pop(s, p)
                        };
                }
                _ => {
                    let good = Good::ALL[g - 1];
                    demand[i] = crate::systems::trade::demand(s, p, good);
                }
            }
        }
        if (1..=3).contains(&g) {
            let good = Good::ALL[g - 1];
            for site in state.sites.iter().filter(|x| x.good == good) {
                if let Some(i) = node_of(site.holder) {
                    supply[i] += site.sold.max(0.0);
                }
            }
        }
        if g == 0 && p.outside_ports {
            for k in 0..ports {
                demand[m + k] = p.outside_grain;
            }
        }
        // Arcs: from every supplier to itself (local use) and along its
        // stored paths; for imports, from the ports along the buyers' paths.
        let mut arcs: Vec<Arc> = Vec::new();
        let premium = |j: usize| {
            if g == 0 && j < m {
                premium_of[j]
            } else {
                1.0
            }
        };
        for (i, s) in state.settlements.iter().enumerate() {
            if g == 4 {
                for q in &s.reach.partners {
                    if let Some(k) = node_of(q.id).filter(|&k| k >= m) {
                        arcs.push(Arc {
                            from: k,
                            to: i,
                            t: tau * q.cost,
                            premium: 1.0,
                        });
                    }
                }
                continue;
            }
            if supply[i] <= 0.0 {
                continue;
            }
            arcs.push(Arc {
                from: i,
                to: i,
                t: 0.0,
                premium: premium(i),
            });
            for q in &s.reach.partners {
                let Some(j) = node_of(q.id) else { continue };
                if j == i || demand[j] <= 0.0 || (j >= m && g != 0) {
                    continue;
                }
                arcs.push(Arc {
                    from: i,
                    to: j,
                    t: tau * q.cost,
                    premium: premium(j),
                });
            }
        }
        // Buyers' prices: last year's, else 1. For imports the price is kept
        // as the premium over the ports' price of 1.
        let base = if g == 4 { 1.0 } else { 0.0 };
        let mut price: Vec<f32> = (0..nodes)
            .map(|i| {
                let v = if i < m {
                    state.settlements[i].price[g]
                } else {
                    state.port_price.get(i - m).copied().unwrap_or(1.0)
                };
                if v > 0.0 {
                    v
                } else {
                    1.0
                }
            })
            .collect();
        // Arcs grouped by supplier, for the shares.
        let mut by_from: Vec<Vec<usize>> = vec![Vec::new(); nodes];
        for (k, a) in arcs.iter().enumerate() {
            by_from[a.from].push(k);
        }
        let mut inflow = vec![0f32; nodes];
        let mut flow = vec![0f32; arcs.len()];
        let mut sell = vec![0f32; nodes];
        let total_supply: f32 = supply.iter().sum();
        for pass in 0..p.market_passes {
            // The price level first: all supply is sold, so the region's
            // demand at the scaled prices must equal it (solved by bisection
            // on the log of the scale). Buyers no supplier reaches are left
            // out; on the first pass, before flows, all buyers count.
            if g != 4 && total_supply > 0.0 && elast > 0.0 {
                let counted = |j: usize| demand[j] > 0.0 && (pass == 0 || inflow[j] > 0.0);
                let fixed_demand: f32 = (m..nodes).filter(|&j| counted(j)).map(|j| demand[j]).sum();
                let want = |l: f32| -> f32 {
                    (0..m)
                        .filter(|&j| counted(j))
                        .map(|j| demand[j] * libm::powf(price[j] * libm::expf(l), -elast))
                        .sum::<f32>()
                        + fixed_demand
                };
                let (mut lo, mut hi) = (-7.0f32, 7.0f32);
                for _ in 0..16 {
                    let mid = 0.5 * (lo + hi);
                    if want(mid) > total_supply {
                        lo = mid;
                    } else {
                        hi = mid;
                    }
                }
                let scale = libm::expf(0.5 * (lo + hi));
                for pj in price.iter_mut().take(m) {
                    *pj = (*pj * scale).clamp(1e-3, 1e3);
                }
            }
            inflow.iter_mut().for_each(|x| *x = 0.0);
            if g == 4 {
                // Ports sell without limit: x = exp(θ (q_j − t)), q the premium.
                for (k, a) in arcs.iter().enumerate() {
                    let q = price[a.to] - base;
                    flow[k] = libm::expf((theta * (q - a.t)).clamp(-80.0, 60.0));
                    inflow[a.to] += flow[k];
                }
            } else {
                for (i, ks) in by_from.iter().enumerate() {
                    if ks.is_empty() {
                        continue;
                    }
                    // Shares by softmax of θ p_j − θ t + ln m_j, max-subtracted.
                    let score = |a: &Arc| {
                        theta * (price[a.to] - a.t) + libm::logf(a.premium.max(1e-9))
                    };
                    let top = ks
                        .iter()
                        .map(|&k| score(&arcs[k]))
                        .fold(f32::NEG_INFINITY, f32::max);
                    let total: f32 = ks
                        .iter()
                        .map(|&k| libm::expf(score(&arcs[k]) - top))
                        .sum();
                    for &k in ks {
                        flow[k] = supply[i] * libm::expf(score(&arcs[k]) - top) / total;
                        inflow[arcs[k].to] += flow[k];
                    }
                    // The supplier's price: what the marginal buyer pays, less
                    // carriage, on the log-sum of its outlets.
                    sell[i] = (top + libm::logf(total) - libm::logf(supply[i].max(1e-9))) / theta;
                }
            }
            // Each buyer moves its price towards clearing: ln F + θ Δp = ln D(p).
            for j in 0..nodes {
                if demand[j] <= 0.0 || inflow[j] <= 0.0 {
                    continue;
                }
                let e = if j >= m { 0.0 } else { elast };
                let pj = price[j];
                let f = libm::logf(inflow[j]) - libm::logf(demand[j])
                    + e * libm::logf(pj.max(1e-6));
                let d = theta + e / pj.max(1e-6);
                let step = (-f / d).clamp(-p.market_step, p.market_step);
                price[j] = (pj + p.market_damping * step).clamp(1e-3, 1e3);
            }
        }
        // Residual: unmatched demand at the buyers some supplier reaches, as a
        // share of their demand; demand no supplier reaches is not counted.
        let reached = |j: usize| demand[j] > 0.0 && inflow[j] > 0.0;
        let total: f32 = (0..nodes)
            .filter(|&j| reached(j))
            .map(|j| demand[j])
            .sum::<f32>()
            .max(1.0);
        residual[g] = (0..nodes)
            .filter(|&j| reached(j))
            .map(|j| {
                let e = if j >= m { 0.0 } else { elast };
                (inflow[j] - demand[j] * libm::powf(price[j], -e)).abs()
            })
            .sum::<f32>()
            / total;
        for (i, s) in state.settlements.iter_mut().enumerate() {
            s.price[g] = if demand[i] > 0.0 { price[i] } else { sell[i] };
        }
        if g == 0 {
            state.port_price = price[m..].to_vec();
            for (k, a) in arcs.iter().enumerate() {
                if a.from >= m || a.to >= m || a.from == a.to || flow[k] < 1.0 {
                    continue;
                }
                grain_arcs.push((
                    state.settlements[a.from].id,
                    state.settlements[a.to].id,
                    flow[k],
                    price[a.to] - sell[a.from],
                    a.t,
                ));
            }
        }
    }
    state.market_residual = residual;
    state.market_arcs = grain_arcs;
}

/// Goods by the equilibrium (v6 S1, D71), in place of the shopping model
/// (D41) when `market_goods` is on. Grain is the numeraire at 1.
///
/// For salt, iron and stone the suppliers are the settlements holding working
/// sites, each with supply `capacity × min(1, p_i)`: a site sells all it can
/// at a supply price of 1 or more, the old value, and holds back below it. For
/// imports the outside ports sell without limit at 1. Buyers demand
/// `D0 · p_j^(−ε)`, `D0` being the v2 demand (per head for salt and iron, per
/// market person for stone and imports). Along each stored path the flow is
/// `x_ij = exp(θ (p_j − p_i − t_ij))`, with a holder's own use on an arc of no
/// cost. Prices are solved by alternating exact log-space solves, suppliers
/// then buyers (Furness), for `market_passes` passes, warm-started.
///
/// A buyer pays `p_j × q` out of its marketed share, capped there as in D41.
/// The producer receives `p_i × q` as a grain flow, and the rest (carriage
/// and the entropy margin) is lost. Own use moves no money. Goods and
/// payments ride the stored paths, as in D41. Writes sites' `sold`,
/// settlements' `price` (goods) and `goods_in`, `outside_trade`'s imports,
/// and the goods' unmet demand to `market_unmet`. Returns the grain each
/// settlement paid.
#[allow(clippy::too_many_arguments)]
pub fn goods_trade(
    state: &mut State,
    p: &Params,
    idx_of: &[u32],
    food_in: &mut [f32],
    food_out: &mut [f32],
    market_in: &mut [f32],
    flows: &mut Vec<crate::state::Flow>,
) -> Vec<f32> {
    use crate::state::Flow;
    let m = state.settlements.len();
    let ports = state.outside.len();
    let node_of = |id: u32| -> Option<usize> {
        if let Some(k) = port_of(id) {
            return (k < ports).then_some(m + k);
        }
        idx_of
            .get(id as usize)
            .copied()
            .filter(|&j| j != u32::MAX)
            .map(|j| j as usize)
    };
    let theta = p.beta / p.market_tau.max(1e-9);
    // Everything bought: (buyer, supplier node, quantity, buyer price,
    // supplier price, transport, buyer's partner index or NONE for own use,
    // good index 1..=4).
    struct Buy {
        buyer: usize,
        from: usize,
        q: f32,
        pj: f32,
        pi: f32,
        k: u32,
        g: usize,
    }
    let mut buys: Vec<Buy> = Vec::new();
    let mut unmet = [0f32; PRICED];
    let mut sold_by = vec![[0f32; 3]; m];
    for g in 1..PRICED {
        let rho = match g {
            1 => p.rho_salt,
            2 => p.rho_iron,
            3 => p.rho_stone,
            _ => p.rho_import,
        };
        let tau = p.market_tau / rho.max(1e-6);
        let elast = match g {
            1 => p.elast_salt,
            2 => p.elast_iron,
            3 => p.elast_stone,
            _ => p.elast_import,
        };
        let demand: Vec<f32> = state
            .settlements
            .iter()
            .map(|s| match g {
                4 => {
                    p.import_demand
                        * if p.market_state {
                            s.market.max(0.0)
                        } else {
                            crate::systems::trade::urban_pop(s, p)
                        }
                }
                _ => crate::systems::trade::demand(s, p, Good::ALL[g - 1]),
            })
            .collect();
        let mut cap = vec![0f32; m];
        if g < 4 {
            let good = Good::ALL[g - 1];
            for site in state.sites.iter().filter(|x| x.good == good) {
                if let Some(i) = node_of(site.holder).filter(|&i| i < m) {
                    cap[i] += site.output.max(0.0);
                }
            }
        }
        // Arcs (supplier node, buyer, transport, buyer's partner index).
        let mut arcs: Vec<(usize, usize, f32, u32)> = Vec::new();
        for (j, s) in state.settlements.iter().enumerate() {
            if demand[j] <= 0.0 {
                continue;
            }
            if g < 4 && cap[j] > 0.0 {
                arcs.push((j, j, 0.0, crate::state::NONE));
            }
            for (k, q) in s.reach.partners.iter().enumerate() {
                let Some(i) = node_of(q.id) else { continue };
                let ok = if g == 4 { i >= m } else { i < m && cap[i] > 0.0 };
                if ok {
                    arcs.push((i, j, tau * q.cost, k as u32));
                }
            }
        }
        if arcs.is_empty() {
            unmet[g] = demand.iter().sum();
            continue;
        }
        let mut by_from: Vec<Vec<usize>> = vec![Vec::new(); m + ports];
        let mut by_to: Vec<Vec<usize>> = vec![Vec::new(); m];
        for (a, &(i, j, _, _)) in arcs.iter().enumerate() {
            by_from[i].push(a);
            by_to[j].push(a);
        }
        // Prices: buyers' delivered prices and suppliers' supply prices,
        // warm-started; ports' supply price is 1.
        let mut buy_p: Vec<f32> = state
            .settlements
            .iter()
            .map(|s| if s.price[g] > 0.0 { s.price[g] } else { 1.0 })
            .collect();
        let mut sup_p: Vec<f32> = (0..m + ports)
            .map(|i| {
                // Ports sell imports at 1; only salt, iron and stone have
                // settlement suppliers.
                if i >= m || g == 4 {
                    1.0
                } else {
                    let v = state.settlements[i].supply_price[g - 1];
                    if v > 0.0 {
                        v
                    } else {
                        1.0
                    }
                }
            })
            .collect();
        // log Σ exp(v) over a list, max-subtracted.
        let lse = |vals: &mut dyn Iterator<Item = f32>| -> f32 {
            let v: Vec<f32> = vals.collect();
            let top = v.iter().copied().fold(f32::NEG_INFINITY, f32::max);
            if !top.is_finite() {
                return f32::NEG_INFINITY;
            }
            top + libm::logf(v.iter().map(|&x| libm::expf(x - top)).sum::<f32>())
        };
        for _ in 0..p.market_passes {
            // Suppliers: θ p_i + ln S(p_i) = ln Σ_j exp(θ (p_j − t_ij)).
            for i in 0..m {
                if by_from[i].is_empty() {
                    continue;
                }
                let a = lse(&mut by_from[i].iter().map(|&k| theta * (buy_p[arcs[k].1] - arcs[k].2)));
                let lc = libm::logf(cap[i].max(1e-9));
                let mut pi = (a - lc) / theta;
                if pi < 1.0 {
                    // Below 1 supply is cap × p: θ p + ln p = a − ln cap.
                    for _ in 0..4 {
                        let f = theta * pi + libm::logf(pi.max(1e-6)) + lc - a;
                        pi = (pi - f / (theta + 1.0 / pi.max(1e-6))).clamp(1e-3, 1.0);
                    }
                }
                sup_p[i] = pi.clamp(1e-3, 1e3);
            }
            // Buyers: θ p_j + ln Σ_i exp(−θ (p_i + t_ij)) = ln D0 − ε ln p_j.
            for j in 0..m {
                if by_to[j].is_empty() {
                    continue;
                }
                let b = lse(&mut by_to[j].iter().map(|&k| -theta * (sup_p[arcs[k].0] + arcs[k].2)));
                let ld = libm::logf(demand[j].max(1e-9));
                let mut pj = buy_p[j];
                for _ in 0..4 {
                    let f = theta * pj + b - ld + elast * libm::logf(pj.max(1e-6));
                    pj = (pj - f / (theta + elast / pj.max(1e-6))).clamp(1e-3, 1e3);
                }
                buy_p[j] = pj;
            }
        }
        // The flows at the solved prices.
        let mut got = vec![0f32; m];
        for &(i, j, t, k) in &arcs {
            let q = libm::expf((theta * (buy_p[j] - sup_p[i] - t)).min(60.0));
            if !(q > 0.0) {
                continue;
            }
            got[j] += q;
            if i < m && g < 4 {
                sold_by[i][g - 1] += q;
            }
            buys.push(Buy {
                buyer: j,
                from: i,
                q,
                pj: buy_p[j],
                pi: sup_p[i],
                k,
                g,
            });
        }
        unmet[g] = (0..m).map(|j| (demand[j] - got[j]).max(0.0)).sum();
        for (j, s) in state.settlements.iter_mut().enumerate() {
            s.price[g] = buy_p[j];
            if g < 4 {
                s.supply_price[g - 1] = sup_p[j];
            }
        }
    }
    // Goods arcs for the report: (supplier id, buyer id, quantity, price gap,
    // transport), settlement suppliers only, own use left out.
    state.market_arcs = buys
        .iter()
        .filter(|b| b.from < m && b.from != b.buyer && b.q >= 0.1)
        .map(|b| {
            let t = b.pj - b.pi - libm::logf(b.q.max(1e-9)) / theta;
            (
                state.settlements[b.from].id,
                state.settlements[b.buyer].id,
                b.q,
                b.pj - b.pi,
                t,
            )
        })
        .collect();
    // Bills, capped at each buyer's marketed share (own use is free).
    let mut bill = vec![0f32; m];
    for b in buys.iter().filter(|b| b.from != b.buyer) {
        bill[b.buyer] += b.pj * b.q;
    }
    let scale: Vec<f32> = (0..m)
        .map(|i| {
            let budget = state.settlements[i].food_prod * p.market_share;
            if bill[i] > budget {
                budget / bill[i]
            } else {
                1.0
            }
        })
        .collect();
    let mut paid = vec![0f32; m];
    let mut goods_in = vec![0f32; m];
    let mut imported = 0.0f32;
    for b in &buys {
        if b.from == b.buyer {
            continue;
        }
        let buyer = &state.settlements[b.buyer];
        let pay = b.pj * b.q * scale[b.buyer];
        paid[b.buyer] += pay;
        food_out[b.buyer] += pay;
        let to = buyer.reach.partners[b.k as usize].id;
        let good = (b.g < 4).then(|| Good::ALL[b.g - 1]);
        // The goods come in on the buyer's path; the payment goes out on it.
        flows.push(Flow {
            from: buyer.id,
            to,
            amount: b.pj * b.q,
            food: 0.0,
            partner_idx: b.k,
            good,
            payment: false,
            import: b.g == 4,
        });
        flows.push(Flow {
            from: buyer.id,
            to,
            amount: pay,
            food: pay,
            partner_idx: b.k,
            good: None,
            payment: true,
            import: false,
        });
        if b.from >= m {
            imported += pay;
            continue;
        }
        // The producer gets the supply price; carriage and margin are lost.
        let received = b.pi * b.q * scale[b.buyer];
        food_in[b.from] += received;
        market_in[b.from] += received;
        goods_in[b.from] += received;
    }
    // Each holder's sales shared over its sites by capacity.
    for site in state.sites.iter_mut() {
        site.sold = 0.0;
    }
    let mut cap_of = vec![[0f32; 3]; m];
    for site in state.sites.iter() {
        if let Some(i) = idx_of.get(site.holder as usize).copied().filter(|&i| i != u32::MAX) {
            cap_of[i as usize][site.good.index()] += site.output.max(0.0);
        }
    }
    for site in state.sites.iter_mut() {
        if let Some(i) = idx_of.get(site.holder as usize).copied().filter(|&i| i != u32::MAX) {
            let (i, g) = (i as usize, site.good.index());
            if cap_of[i][g] > 0.0 {
                site.sold = sold_by[i][g] * site.output.max(0.0) / cap_of[i][g];
            }
        }
    }
    for (s, g) in state.settlements.iter_mut().zip(goods_in) {
        s.goods_in = g;
    }
    state.outside_trade.1 = imported;
    state.market_unmet = unmet;
    paid
}
