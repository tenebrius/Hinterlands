//! Grain by the price equilibrium (v6 S2, D77–D78; Samuelson 1952; Takayama &
//! Judge 1971; Wilson 1970), in place of the gravity grain trade when
//! `market_grain` is on. Part of the trade system; called by `trade::run`.
//!
//! Prices are in value units: one person-year of grain at the reference price
//! 1. Rent is taken in kind: the lord's `rent_share × H` goes to market, and
//! the farmers eat from the rest first, at their own price, and offer what is
//! left. Each place's people buy, beyond that, `need · min(b^(−ε), c_max)` at
//! its buying price `b`, within its budget. Along each stored path the flow is
//! `m_j exp(θ (b_j − s_i − t_ij))`, `s_i` the supply price, with an own arc of
//! no cost and `m_j = W_j^α` the market premium (decision 4; 1 with
//! `grain_premium` off). A supplier whose clearing price would fall below
//! `p_floor` sells at the floor what the flows take, and the rest is unsold
//! and lost (until stores, S3). The outside ports buy their fixed grain at up
//! to the world price `port_world_price`, and less above it.
//!
//! The budget of a place (D77, D78): the farmers' sales at its supply price,
//! the rents it receives (its lords' sales, shared by the gravity weights
//! `W^α exp(−β c)`, its own market by `home_weight`), its goods sales and
//! last year's fee income, less its goods bills and, per unit bought from
//! others, last year's fee rate. Prices are solved by alternating exact
//! log-space solves, suppliers then buyers, warm-started from last year and
//! re-levelled by the closed-form regional level; each pass also shifts every
//! price by a common amount, which leaves the flows as they are, so that the
//! region's demand meets its sales (supply prices stop at the floor). There
//! are `market_passes` passes, plus up to 8 more while the residual is above
//! 0.05. Flows are each supplier's sales shared over its outlets, so no grain
//! is created; a buyer whose bill exceeds its budget, or whose grain would
//! pass `c_max × need`, takes less, and the rest is unsold. Writes
//! settlements' `price[0]`, `grain_sup`, `income`, `capped`, and
//! `grain_stats`, `grain_level`, `port_grain`, `market_arcs`,
//! `outside_trade.0`.

use crate::params::Params;
use crate::state::{port_of, Flow, State, NONE};

/// One arc: supplier, buyer node, transport price, ln premium, supplier's
/// partner index (NONE for the own arc).
struct Arc {
    from: usize,
    to: usize,
    t: f32,
    lm: f32,
    k: u32,
}

/// log Σ exp over a list, max-subtracted.
fn lse(v: &[f32]) -> f32 {
    let top = v.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    if !top.is_finite() {
        return f32::NEG_INFINITY;
    }
    top + libm::logf(v.iter().map(|&x| libm::expf(x - top)).sum::<f32>())
}

/// Grain by the equilibrium. `attract` is each settlement's `W^α`; `bills`
/// what each paid for goods and imports this year (value units).
#[allow(clippy::too_many_arguments)]
pub fn trade(
    state: &mut State,
    p: &Params,
    idx_of: &[u32],
    attract: &[f32],
    bills: &[f32],
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
    let nodes = m + ports;
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
    let eps = p.elast_grain;
    let cmax = p.food_cap;
    let harvest: Vec<f32> = state
        .settlements
        .iter()
        .map(|s| s.food_prod.max(0.0))
        .collect();
    let need: Vec<f32> = (0..nodes)
        .map(|j| {
            if j < m {
                state.settlements[j].need(p)
            } else {
                p.outside_grain
            }
        })
        .collect();
    let lm_of = |j: usize| -> f32 {
        if j < m && p.grain_premium {
            libm::logf(attract[j].max(1e-9))
        } else {
            0.0
        }
    };

    // Arcs, grouped by supplier (contiguous), and by buyer.
    let mut arcs: Vec<Arc> = Vec::new();
    let mut from_start = vec![0usize; m + 1];
    for (i, s) in state.settlements.iter().enumerate() {
        from_start[i] = arcs.len();
        if harvest[i] <= 0.0 {
            continue;
        }
        if need[i] > 0.0 {
            arcs.push(Arc {
                from: i,
                to: i,
                t: 0.0,
                lm: lm_of(i),
                k: NONE,
            });
        }
        for (k, q) in s.reach.partners.iter().enumerate() {
            let Some(j) = node_of(q.id) else { continue };
            if j == i || need[j] <= 0.0 {
                continue;
            }
            arcs.push(Arc {
                from: i,
                to: j,
                t: p.market_tau * q.cost,
                lm: lm_of(j),
                k: k as u32,
            });
        }
    }
    from_start[m] = arcs.len();
    let mut to_count = vec![0usize; nodes + 1];
    for a in &arcs {
        to_count[a.to + 1] += 1;
    }
    for j in 0..nodes {
        to_count[j + 1] += to_count[j];
    }
    let mut to_list = vec![0usize; arcs.len()];
    let mut fill = to_count.clone();
    for (k, a) in arcs.iter().enumerate() {
        to_list[fill[a.to]] = k;
        fill[a.to] += 1;
    }

    // Rent weights by payer: (receiver, share), home market first.
    let mut rent_to: Vec<Vec<(usize, f32)>> = vec![Vec::new(); m];
    if p.rent_share > 0.0 {
        for (i, s) in state.settlements.iter().enumerate() {
            let mut ws = vec![(i, attract[i] * p.home_weight)];
            for q in &s.reach.partners {
                if let Some(j) = node_of(q.id).filter(|&j| j < m && j != i) {
                    ws.push((j, attract[j] * libm::expf(-p.beta * q.cost)));
                }
            }
            let total: f32 = ws.iter().map(|w| w.1).sum();
            if total > 0.0 {
                ws.iter_mut().for_each(|w| w.1 /= total);
                rent_to[i] = ws;
            }
        }
    }
    let goods_sales: Vec<f32> = state.settlements.iter().map(|s| s.goods_in).collect();
    let fee_rate: Vec<f32> = state.settlements.iter().map(|s| s.fee_rate).collect();
    let fee_earned: Vec<f32> = state.settlements.iter().map(|s| s.fee_earned).collect();

    // Warm start, re-levelled by the closed-form regional level (D78).
    let total_h: f32 = harvest.iter().sum();
    let total_need: f32 = need[..m].iter().sum::<f32>().max(1e-6);
    let level = libm::powf((total_h / total_need).max(1e-3), -1.0 / eps.max(0.05));
    let relevel = if state.grain_level > 0.0 {
        level / state.grain_level
    } else {
        1.0
    };
    state.grain_level = level;
    let start = |v: f32| if v > 0.0 { v * relevel } else { level };
    let mut buy: Vec<f32> = (0..nodes)
        .map(|j| {
            let v = if j < m {
                state.settlements[j].price[0]
            } else {
                state.port_grain.get(j - m).copied().unwrap_or(0.0)
            };
            start(v).clamp(1e-3, 1e3)
        })
        .collect();
    let mut sup: Vec<f32> = state
        .settlements
        .iter()
        .map(|s| start(s.grain_sup).clamp(p.p_floor, 1e3))
        .collect();
    let mut sold = vec![0f32; m];
    let mut capped = vec![false; m];
    let mut scratch: Vec<f32> = Vec::new();

    // People and establishment (D79). The farmers are the people beyond the
    // market's size W; the market's establishment (Wilson's facility cost:
    // its people's food, malt, fodder, households) wants `K × W` grain at the
    // reference price, with elasticity ε and no satiation.
    let farm_need: Vec<f32> = state
        .settlements
        .iter()
        .map(|s| (s.pop - s.market.max(0.0)).max(0.0) * p.food_per_person)
        .collect();
    let estab_need: Vec<f32> = state
        .settlements
        .iter()
        .map(|s| p.k_maint * s.market.max(0.0).min(s.pop) * p.food_per_person)
        .collect();
    // Grain the farmers would eat at price `b`.
    let eat = |j: usize, b: f32| farm_need[j] * libm::powf(b, -eps).min(cmax);
    // Rent is taken in kind (D78): the lord's `rent_share × H` goes to market;
    // the farmers eat from the rest first, at their own price, and offer what
    // is left over. A place's market supply is its harvest less that.
    let own_of = |i: usize, b: f32| eat(i, b).min(harvest[i] * (1.0 - p.rent_share));
    // The farmers' share of a place's market sales; the rest is rent.
    let farm_share = |i: usize, supply: f32| {
        if supply > 0.0 {
            ((supply - harvest[i] * p.rent_share) / supply).max(0.0)
        } else {
            0.0
        }
    };
    let mut own: Vec<f32> = (0..m).map(|i| own_of(i, buy[i])).collect();
    // Budgets at the current supply prices and sales: the farmers' sales pay
    // for the farmers' grain; rents received, goods sales and fees, less goods
    // bills, pay for the establishment's (D79).
    let mut farm_inc = vec![0f32; m];
    let mut estab_inc = vec![0f32; m];
    let incomes = |sup: &[f32], sold: &[f32], own: &[f32], fi: &mut [f32], ei: &mut [f32]| {
        for i in 0..m {
            fi[i] = farm_share(i, harvest[i] - own[i]) * sup[i] * sold[i];
            ei[i] = goods_sales[i] + fee_earned[i] - bills[i];
        }
        for i in 0..m {
            let fs = farm_share(i, harvest[i] - own[i]);
            let rent = (1.0 - fs) * sup[i] * sold[i];
            for &(j, w) in &rent_to[i] {
                ei[j] += rent * w;
            }
        }
    };
    // The two parts of a place's market demand at a buying price, each within
    // its budget at `b` plus the fee per unit.
    let parts = |j: usize, b: f32, fi: &[f32], ei: &[f32]| -> (f32, f32) {
        let unit = b + fee_rate[j];
        let farm = (eat(j, b) - own_of(j, b))
            .min(fi[j].max(0.0) / unit)
            .max(0.0);
        let estab = (estab_need[j] * libm::powf(b, -eps)).min(ei[j].max(0.0) / unit);
        (farm, estab)
    };
    let demand = |j: usize, b: f32, fi: &[f32], ei: &[f32]| -> f32 {
        if j >= m {
            // The outside world buys at most its fixed grain, and less where
            // grain costs more than the world price (D78).
            return need[j] * libm::powf(b / p.port_world_price, -p.port_elast).min(1.0);
        }
        let (a, e) = parts(j, b, fi, ei);
        a + e
    };

    let mut residual = f32::INFINITY;
    let mut passes = 0u32;
    let max_passes = p.market_passes + 8;
    while passes < max_passes {
        passes += 1;
        // Suppliers: θ s_i + ln S_i = ln Σ_j exp(θ (b_j − t_ij) + ln m_j),
        // S_i the market supply after the farmers' own grain.
        for i in 0..m {
            own[i] = own_of(i, buy[i]);
            let supply = harvest[i] - own[i];
            let (a0, a1) = (from_start[i], from_start[i + 1]);
            if a0 == a1 || supply <= 1e-6 {
                sold[i] = 0.0;
                continue;
            }
            scratch.clear();
            scratch.extend(
                arcs[a0..a1]
                    .iter()
                    .map(|a| theta * (buy[a.to] - a.t) + a.lm),
            );
            let a = lse(&scratch);
            let s = ((a - libm::logf(supply)) / theta).max(p.p_floor);
            sup[i] = s.min(1e3);
            sold[i] = libm::expf((a - theta * sup[i]).min(60.0)).min(supply);
        }
        incomes(&sup, &sold, &own, &mut farm_inc, &mut estab_inc);
        // The price level: a common shift δ of every price leaves the flows
        // as they are, so choose it (by bisection) for the region's demand to
        // equal its sales; budgets move with it linearly (D78).
        let total_sold: f32 = sold.iter().sum();
        if total_sold > 0.0 {
            let mut slope_f = vec![0f32; m];
            let mut slope_e = vec![0f32; m];
            for i in 0..m {
                let fs = farm_share(i, harvest[i] - own[i]);
                slope_f[i] += sold[i] * fs;
                for &(j, w) in &rent_to[i] {
                    slope_e[j] += (1.0 - fs) * sold[i] * w;
                }
            }
            let mut fi = farm_inc.clone();
            let mut ei = estab_inc.clone();
            let mut want = |d: f32| -> f32 {
                let mut tot = 0f32;
                for j in 0..nodes {
                    if to_count[j] == to_count[j + 1] {
                        continue;
                    }
                    if j < m {
                        fi[j] = farm_inc[j] + d * slope_f[j];
                        ei[j] = estab_inc[j] + d * slope_e[j];
                    }
                    tot += demand(j, (buy[j] + d).max(1e-3), &fi, &ei);
                }
                tot
            };
            // Supply prices stop at the floor: in a glut the level falls only
            // until the dearest supplier reaches it, and the rest is unsold.
            let lowest = buy.iter().copied().fold(f32::INFINITY, f32::min);
            let dearest = sup.iter().copied().fold(p.p_floor, f32::max);
            let (mut lo, mut hi) = ((1e-3 - lowest).max(p.p_floor - dearest), 5.0f32);
            if lo < hi {
                for _ in 0..20 {
                    let mid = 0.5 * (lo + hi);
                    if want(mid) > total_sold {
                        lo = mid;
                    } else {
                        hi = mid;
                    }
                }
                let d = 0.5 * (lo + hi);
                buy.iter_mut().for_each(|b| *b = (*b + d).clamp(1e-3, 1e3));
                for i in 0..m {
                    sup[i] = (sup[i] + d).clamp(p.p_floor, 1e3);
                }
                incomes(&sup, &sold, &own, &mut farm_inc, &mut estab_inc);
            }
        }
        // Buyers: θ b_j + ln m_j + ln Σ_i exp(−θ (s_i + t_ij)) = ln D(b_j),
        // by Newton with a numerical slope of ln D.
        for j in 0..nodes {
            let (k0, k1) = (to_count[j], to_count[j + 1]);
            if k0 == k1 {
                continue;
            }
            scratch.clear();
            scratch.extend(to_list[k0..k1].iter().map(|&k| {
                let a = &arcs[k];
                -theta * (sup[a.from] + a.t)
            }));
            let e = lse(&scratch) + lm_of(j);
            let mut b = buy[j];
            for _ in 0..6 {
                let ld = libm::logf(demand(j, b, &farm_inc, &estab_inc).max(1e-9));
                let h = 1e-3 * b.max(1e-3);
                let ld2 = libm::logf(demand(j, b + h, &farm_inc, &estab_inc).max(1e-9));
                let slope = ((ld - ld2) / h).clamp(0.0, 50.0);
                let f = theta * b + e - ld;
                b = (b - f / (theta + slope)).clamp(1e-3, 1e3);
            }
            buy[j] = b;
        }
        // Residual at the buyers: flows as each supplier shares its sales.
        if passes >= p.market_passes {
            let inflow = shares(
                &arcs,
                &from_start,
                &buy,
                &sold,
                theta,
                m,
                nodes,
                &mut scratch,
            )
            .1;
            let (mut gap, mut tot) = (0f32, 0f32);
            for j in 0..nodes {
                if to_count[j] == to_count[j + 1] {
                    continue;
                }
                let d = demand(j, buy[j], &farm_inc, &estab_inc);
                gap += (inflow[j] - d).abs();
                tot += d;
            }
            residual = gap / tot.max(1.0);
            if residual <= 0.05 {
                break;
            }
        }
    }

    // Final flows: each supplier's sales shared over its outlets.
    let (mut flow, mut inflow) = shares(
        &arcs,
        &from_start,
        &buy,
        &sold,
        theta,
        m,
        nodes,
        &mut scratch,
    );
    // What each place takes is split between farmers and establishment in
    // proportion to their demands; each part is capped by its budget, and the
    // farmers' by satiation (D78, D79). The rest is unsold. Three rounds,
    // since a seller's lost sales cut its budgets.
    let mut take = vec![1f32; nodes];
    let mut to_estab = vec![0f32; m];
    for _ in 0..3 {
        let mut sales = vec![0f32; m];
        for (k, a) in arcs.iter().enumerate() {
            sales[a.from] += flow[k] * take[a.to];
        }
        incomes(&sup, &sales, &own, &mut farm_inc, &mut estab_inc);
        for j in 0..m {
            let got = inflow[j];
            if got <= 0.0 {
                take[j] = 1.0;
                continue;
            }
            let b = buy[j];
            let unit = b + fee_rate[j];
            let wf = (eat(j, b) - own[j]).max(0.0);
            let we = estab_need[j] * libm::powf(b, -eps);
            let share_e = if wf + we > 0.0 { we / (wf + we) } else { 0.0 };
            let (mut gf, mut ge) = (got * (1.0 - share_e), got * share_e);
            let rf = (farm_inc[j].max(0.0) / unit).min((cmax * farm_need[j] - own[j]).max(0.0));
            let re = estab_inc[j].max(0.0) / unit;
            if gf > rf {
                capped[j] |= rf < farm_inc[j].max(0.0) / unit;
                gf = rf;
            }
            if ge > re {
                capped[j] = true;
                ge = re;
            }
            take[j] = ((gf + ge) / got).min(1.0);
            to_estab[j] = ge;
        }
    }
    for (k, a) in arcs.iter().enumerate() {
        flow[k] *= take[a.to];
    }
    inflow.iter_mut().zip(&take).for_each(|(x, t)| *x *= t);

    // Accounts: grain eaten, sales, real income towards W, flows.
    let mut sales = vec![0f32; m];
    let mut home = vec![0f32; m];
    let mut to_ports = 0f32;
    for (k, a) in arcs.iter().enumerate() {
        let q = flow[k];
        if q <= 0.0 {
            continue;
        }
        sales[a.from] += q;
        if a.from == a.to {
            home[a.from] += q;
            continue;
        }
        if a.to >= m {
            to_ports += q;
        } else {
            food_in[a.to] += q;
        }
        if q >= p.grain_flow_min {
            let s = &state.settlements[a.from];
            flows.push(Flow {
                from: s.id,
                to: s.reach.partners[a.k as usize].id,
                amount: q,
                food: q,
                partner_idx: a.k,
                good: None,
                payment: false,
                import: false,
                relief: false,
            });
        }
    }
    incomes(&sup, &sales, &own, &mut farm_inc, &mut estab_inc);
    let mut rent_in = vec![0f32; m];
    for i in 0..m {
        let fs = farm_share(i, harvest[i] - own[i]);
        let rent = (1.0 - fs) * sup[i] * sales[i];
        for &(j, w) in &rent_to[i] {
            rent_in[j] += rent * w;
        }
    }
    let mut unsold = 0f32;
    let mut capped_n = 0u32;
    let (mut est_big, mut all_big) = (0f32, 0f32);
    for i in 0..m {
        // Food: the farmers' own and bought grain, and the market people's
        // share of the establishment's, one K-th of it (D79). What the
        // establishment uses beyond its people's food leaves the accounts.
        // `inflow` is all grain bought, its own market's included (`home`).
        let got = inflow[i];
        let estab = to_estab[i].min(got);
        let used = estab - estab / p.k_maint.max(1.0);
        food_out[i] += harvest[i] - own[i] - home[i] + used;
        unsold += harvest[i] - own[i] - sales[i];
        let b = buy[i].max(1e-3);
        // Real market income (D79): rents, goods sales and fees.
        market_in[i] += (rent_in[i] + goods_sales[i]) / b;
        if state.settlements[i].pop > 2000.0 {
            est_big += estab;
            all_big += got;
        }
        let s = &mut state.settlements[i];
        s.price[0] = buy[i];
        s.grain_sup = if harvest[i] > 0.0 { sup[i] } else { 0.0 };
        s.income = farm_inc[i] + estab_inc[i];
        s.capped = capped[i];
        capped_n += capped[i] as u32;
    }
    state.port_grain = buy[m..].to_vec();
    state.outside_trade.0 = to_ports;
    state.grain_stats = crate::state::GrainStats {
        residual,
        passes,
        unsold,
        harvest: total_h,
        capped: capped_n,
        estab_share: if all_big > 0.0 {
            est_big / all_big
        } else {
            f32::NAN
        },
    };
    state.market_arcs = arcs
        .iter()
        .zip(&flow)
        .filter(|(a, &q)| a.from != a.to && a.to < m && q >= 1.0)
        .map(|(a, &q)| {
            (
                state.settlements[a.from].id,
                state.settlements[a.to].id,
                q,
                buy[a.to] - sup[a.from],
                a.t,
            )
        })
        .collect();
}

/// Each supplier's sales shared over its outlets by a softmax of
/// `θ (b_j − t) + ln m_j`; returns the flow on each arc and each buyer's inflow.
#[allow(clippy::too_many_arguments)]
fn shares(
    arcs: &[Arc],
    from_start: &[usize],
    buy: &[f32],
    sold: &[f32],
    theta: f32,
    m: usize,
    nodes: usize,
    scratch: &mut Vec<f32>,
) -> (Vec<f32>, Vec<f32>) {
    let mut flow = vec![0f32; arcs.len()];
    let mut inflow = vec![0f32; nodes];
    for i in 0..m {
        let (a0, a1) = (from_start[i], from_start[i + 1]);
        if a0 == a1 || sold[i] <= 0.0 {
            continue;
        }
        scratch.clear();
        scratch.extend(
            arcs[a0..a1]
                .iter()
                .map(|a| theta * (buy[a.to] - a.t) + a.lm),
        );
        let top = scratch.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        let total: f32 = scratch.iter().map(|&x| libm::expf(x - top)).sum();
        for (k, &x) in (a0..a1).zip(scratch.iter()) {
            flow[k] = sold[i] * libm::expf(x - top) / total;
            inflow[arcs[k].to] += flow[k];
        }
    }
    (flow, inflow)
}
