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
