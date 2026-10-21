//! Gravity trade (Wilson 1970; Huff 1964; Barjamovic et al. 2019), in the
//! Rihll–Wilson form: each settlement sends a fixed marketed share of its harvest,
//! plus its resource output, to markets in proportion to
//! `W_j^alpha * exp(-beta * c_ij)`, where `W_j` is the non-farming population, its own market included with weight `home_weight` (D13).
//! With `outside_ports` (v5, D64) ports at the open-sea edges buy a fixed amount
//! of grain and are the only sellers of imports, both by the shopping model;
//! the grain leaves the map along the stored paths.
//! Food arrives less a loss per unit cost. Flows are assigned to the stored
//! least-cost paths, giving the load on each cell and the route junctions (hubs).
//! Writes: flows, cell_load, hub, trade_in, food_in, food_out, through, market_in,
//! transship, goods_in, sites' sold.

use crate::cost::{back_dir, step_dir};
use crate::params::Params;
use crate::state::{port_of, Flow, Good, State};
use crate::terrain::Terrain;
use crate::Rng;

/// People beyond those the settlement's own harvest feeds: Rihll–Wilson's W.
pub fn urban_pop(s: &crate::state::Settlement, p: &Params) -> f32 {
    (s.pop - s.food_prod / p.food_per_person).max(0.0)
}

pub fn run(state: &mut State, t: &Terrain, p: &Params, _rng: &mut Rng) {
    for s in state.settlements.iter_mut() {
        s.trade_in = 0.0;
        s.food_in = 0.0;
        s.food_out = 0.0;
        s.through = 0.0;
        s.market_in = 0.0;
        s.transship = 0.0;
        s.goods_in = 0.0;
    }
    state.flows.clear();
    for site in state.sites.iter_mut() {
        site.sold = 0.0;
    }
    state.cell_load.iter_mut().for_each(|x| *x = 0.0);
    state.hub.iter_mut().for_each(|x| *x = false);
    if !p.trade_enabled {
        return;
    }

    let m = state.settlements.len();
    let mut idx_of = vec![u32::MAX; state.next_id as usize];
    for (i, s) in state.settlements.iter().enumerate() {
        idx_of[s.id as usize] = i as u32;
    }
    // Market pull is the non-farming population (Rihll–Wilson's W): everyone
    // beyond those its own harvest feeds, plus a small seed.
    let attract: Vec<f32> = state
        .settlements
        .iter()
        .map(|s| {
            let w = if p.market_state {
                s.market.max(p.market_floor)
            } else {
                urban_pop(s, p) + p.urban_seed
            };
            libm::powf(w, p.alpha)
        })
        .collect();
    let mut trade_in = vec![0f32; m];
    let mut food_in = vec![0f32; m];
    let mut food_out = vec![0f32; m];
    let mut market_in = vec![0f32; m];
    let mut flows = Vec::new();
    // (destination index, partner index, weight, cost).
    let mut weights: Vec<(usize, u32, f32, f32)> = Vec::new();

    // Goods (v2, D39): each site's output goes to consumers in proportion to
    // demand × exp(−β_g c), and they pay for it in grain out of their marketed
    // share, which travels to the producer as an ordinary food flow.
    // Grain by the equilibrium (S2, D78): goods go by the S1 equilibrium,
    // paid in value, and the ports buy grain inside the grain market.
    let s2 = p.market_grain;
    let paid = if p.goods_enabled && (p.market_goods || s2) {
        crate::systems::market::goods_trade(
            state,
            p,
            &idx_of,
            &mut food_in,
            &mut food_out,
            &mut market_in,
            &mut flows,
        )
    } else if p.goods_enabled {
        goods(
            state,
            p,
            &idx_of,
            &mut food_in,
            &mut food_out,
            &mut market_in,
            &mut flows,
        )
    } else {
        vec![0f32; m]
    };
    // The outside world (v5, D64): imports bought and grain sold to the ports,
    // out of what is left of each marketed share.
    let (paid, to_ports) = if s2 {
        (paid, vec![0f32; m])
    } else if p.outside_ports && !state.outside.is_empty() {
        outside(state, p, paid, &mut market_in, &mut food_out, &mut flows)
    } else {
        state.outside_trade = (0.0, 0.0);
        (paid, vec![0f32; m])
    };
    if s2 {
        if !p.goods_enabled {
            state.outside_trade = (0.0, 0.0);
        }
        crate::systems::grain::trade(
            state,
            p,
            &idx_of,
            &attract,
            &paid,
            &mut food_in,
            &mut food_out,
            &mut market_in,
            &mut flows,
        );
    }

    for i in (0..m).filter(|_| !s2) {
        let s = &state.settlements[i];
        // What is left of the marketed share after paying for goods and
        // imports and selling to the ports.
        let food = (s.food_prod * p.market_share - paid[i] - to_ports[i]).max(0.0);
        let out = food + s.resource_out;
        if out <= 0.0 {
            continue;
        }
        weights.clear();
        // A home-market weight keeps grain at home when markets are far or small.
        let mut total = attract[i] * p.home_weight;
        for (k, partner) in s.reach.partners.iter().enumerate() {
            let Some(&j) = idx_of.get(partner.id as usize) else {
                continue;
            };
            if j == u32::MAX {
                continue;
            }
            let w = attract[j as usize] * libm::expf(-p.beta * partner.cost);
            total += w;
            weights.push((j as usize, k as u32, w, partner.cost));
        }
        if total <= 0.0 {
            continue;
        }
        // Produce sold at the home market counts towards that market's turnover.
        market_in[i] += food * attract[i] * p.home_weight / total;
        for &(j, k, w, c) in &weights {
            let share = w / total;
            let amount = out * share;
            let sent = food * share;
            let delivered = sent * (1.0 - p.transport_loss * c).max(0.0);
            trade_in[j] += amount - sent;
            food_out[i] += sent;
            food_in[j] += delivered;
            market_in[j] += delivered;
            flows.push(Flow {
                from: s.id,
                to: state.settlements[j].id,
                amount,
                food: sent,
                partner_idx: k,
                good: None,
                payment: false,
                import: false,
                relief: false,
            });
        }
    }

    // Assign flows to paths: load per cell, and the directions routes use there.
    let grid = t.grid;
    let mut dirs = vec![0u16; t.n()];
    // Where cargo changes mode: boarding or leaving a boat, or barge to sea ship.
    let mut transship = vec![0f32; t.n()];
    // Under S2 each flow's buyer pays the handling and transshipment fees it
    // earns the places on its path (D77): per anchor passed, and per
    // transshipment within a cell of an anchor, once for each such anchor.
    let fees = s2 && p.handling_income;
    let mut anchor_at = Vec::new();
    let mut near = Vec::new();
    if fees {
        anchor_at = vec![false; t.n()];
        near = vec![0u8; t.n()];
        for s in &state.settlements {
            let a = s.anchor as usize;
            anchor_at[a] = true;
            near[a] += 1;
            for d in 0..8 {
                if let Some(j) = grid.neighbor(a, d) {
                    near[j] += 1;
                }
            }
        }
    }
    let mut fee_paid = vec![0f32; m];
    let mut flow_fee: Vec<f32> = Vec::with_capacity(flows.len());
    for f in &flows {
        let s = &state.settlements[idx_of[f.from as usize] as usize];
        let path = &s.reach.partners[f.partner_idx as usize].path;
        let mut fee = 0.0f32;
        for w in path.windows(2).filter(|_| p.handling_income) {
            let (a, b) = (w[0], w[1]);
            let (wa, wb) = (state.cost.is_water_node(a), state.cost.is_water_node(b));
            let (ca, cb) = (state.cost.node_cell(a), state.cost.node_cell(b));
            let at = if wa != wb {
                Some(if wa { cb } else { ca })
            } else if wa && t.is_nav(ca) != t.is_nav(cb) {
                Some(if t.is_nav(ca) { ca } else { cb })
            } else {
                None
            };
            if let Some(c) = at {
                transship[c] += f.amount;
                if fees {
                    fee += p.transship_margin * near[c] as f32;
                }
            }
        }
        let mark = f.amount >= p.hub_min_flow;
        for q in 1..path.len().saturating_sub(1) {
            let cell = state.cost.node_cell(path[q]);
            state.cell_load[cell] += f.amount;
            if fees && anchor_at[cell] {
                fee += p.handling_margin;
            }
            if mark {
                let din = back_dir(step_dir(&grid, &state.cost, path[q - 1], path[q]));
                let dout = step_dir(&grid, &state.cost, path[q], path[q + 1]);
                dirs[cell] |= (1 << din) | (1 << dout);
            }
        }
        flow_fee.push(fee * f.amount);
    }
    if fees {
        // Grain flows are paid by the receiver, goods by the buyer (`from`);
        // flows to the outside ports are paid from outside.
        for (f, fee) in flows.iter().zip(&flow_fee) {
            let payer = if f.good.is_none() && !f.import {
                idx_of.get(f.to as usize).copied()
            } else {
                idx_of.get(f.from as usize).copied()
            };
            if let Some(i) = payer.filter(|&i| i != u32::MAX) {
                fee_paid[i as usize] += fee;
            }
        }
    }
    for (i, &mask) in dirs.iter().enumerate() {
        state.hub[i] = t.is_land(i) && mask.count_ones() >= 3;
    }

    for (i, s) in state.settlements.iter_mut().enumerate() {
        s.trade_in = trade_in[i];
        s.food_in = food_in[i];
        s.food_out = food_out[i];
        s.market_in = market_in[i];
        s.through = state.cell_load[s.anchor as usize];
        let a = s.anchor as usize;
        s.transship = transship[a]
            + (0..8)
                .filter_map(|d| grid.neighbor(a, d))
                .map(|j| transship[j])
                .sum::<f32>();
        // Fees this year, for next year's budgets (S2, D78).
        if fees {
            s.fee_earned = p.handling_margin * s.through + p.transship_margin * s.transship;
            s.fee_rate = fee_paid[i] / food_in[i].max(1.0);
        }
    }
    state.flows = flows;
    if p.grain_relief {
        relief(state, p, &idx_of);
    } else {
        state.relief_stats = [0.0; 4];
    }
}

/// Grain relief (S2′, D80). After trade, each place's food is
/// `A = food_prod + food_in − food_out`. What a place holds above
/// `food_cap × need` is surplus, otherwise lost; what it lacks below need is
/// its deficit. Each place in deficit draws its deficit from the surplus
/// holders among its partners in proportion to `S_j exp(−β c)`, shopping-model
/// style; orders beyond a holder's surplus are scaled down in one pass, and
/// what is not met stays unmet. It arrives less the transport loss. Nobody
/// pays. Relief counts in the food accounts but not in `market_in`, and its
/// flows load the routes but earn no handling. Writes food_in, food_out,
/// flows (relief) and relief_stats.
fn relief(state: &mut State, p: &Params, idx_of: &[u32]) {
    let m = state.settlements.len();
    let held: Vec<f32> = state
        .settlements
        .iter()
        .map(|s| s.food_prod + s.food_in - s.food_out)
        .collect();
    let need: Vec<f32> = state.settlements.iter().map(|s| s.need(p)).collect();
    let surplus: Vec<f32> = (0..m)
        .map(|i| (held[i] - p.food_cap * need[i]).max(0.0))
        .collect();
    // Orders: (receiver, holder, quantity, cost, receiver's partner index).
    let mut orders: Vec<(usize, usize, f32, f32, u32)> = Vec::new();
    let mut ordered = vec![0f32; m];
    let mut deficit = 0f32;
    let mut ws: Vec<(usize, f32, f32, u32)> = Vec::new();
    for k in 0..m {
        let d = (need[k] - held[k]).max(0.0);
        if d <= 0.0 {
            continue;
        }
        deficit += d;
        ws.clear();
        let mut total = 0f32;
        for (pi, q) in state.settlements[k].reach.partners.iter().enumerate() {
            let Some(&j) = idx_of.get(q.id as usize) else {
                continue;
            };
            if j == u32::MAX || surplus[j as usize] <= 0.0 {
                continue;
            }
            let w = surplus[j as usize] * libm::expf(-p.beta * q.cost);
            total += w;
            ws.push((j as usize, w, q.cost, pi as u32));
        }
        if total <= 0.0 {
            continue;
        }
        for &(j, w, c, pi) in &ws {
            let q = d * w / total;
            ordered[j] += q;
            orders.push((k, j, q, c, pi));
        }
    }
    let scale: Vec<f32> = (0..m)
        .map(|j| {
            if ordered[j] > surplus[j] {
                surplus[j] / ordered[j]
            } else {
                1.0
            }
        })
        .collect();
    let (mut sent_all, mut arrived_all) = (0f32, 0f32);
    let mut sent_by = vec![0f32; m];
    for &(k, j, q, c, pi) in &orders {
        let sent = q * scale[j];
        if sent <= 0.0 {
            continue;
        }
        let arrived = sent * (1.0 - p.transport_loss * c).max(0.0);
        sent_by[j] += sent;
        sent_all += sent;
        arrived_all += arrived;
        state.settlements[j].food_out += sent;
        state.settlements[k].food_in += arrived;
        state.flows.push(Flow {
            from: state.settlements[k].id,
            to: state.settlements[j].id,
            amount: sent,
            food: sent,
            partner_idx: pi,
            good: None,
            payment: false,
            import: false,
            relief: true,
        });
    }
    let lost: f32 = (0..m).map(|j| surplus[j] - sent_by[j]).sum();
    state.relief_stats = [deficit, sent_all, arrived_all, lost];
}

/// Value of one unit of a good in grain.
pub fn value(p: &Params, g: Good) -> f32 {
    match g {
        Good::Salt => p.value_salt,
        Good::Iron => p.value_iron,
        Good::Stone => p.value_stone,
    }
}

/// Distance decay of a good: `β / ρ_g` (a good worth more per tonne travels
/// farther for the same cost).
pub fn beta(p: &Params, g: Good) -> f32 {
    let rho = match g {
        Good::Salt => p.rho_salt,
        Good::Iron => p.rho_iron,
        Good::Stone => p.rho_stone,
    };
    p.beta / rho.max(1e-6)
}

/// Yearly demand of a settlement for a good, grain units: salt and iron per
/// person, stone per market person.
pub fn demand(s: &crate::state::Settlement, p: &Params, g: Good) -> f32 {
    match g {
        Good::Salt => p.demand_salt * s.pop,
        Good::Iron => p.demand_iron * s.pop,
        Good::Stone => {
            let w = if p.market_state {
                s.market.max(0.0)
            } else {
                urban_pop(s, p)
            };
            p.demand_stone * w
        }
    }
}

/// Goods by the shopping model (Huff 1964; Wilson 1971; D41). Each consumer
/// buys its demand for a good, its holder's own sites first without travel or
/// payment, the rest split among the working sites in proportion to
/// `capacity × exp(−β_g c)`. Orders beyond a site's capacity are scaled down to
/// fit in one pass; unmet demand stays unmet. A buyer pays `value × quantity` in
/// grain out of its marketed share, capped at that share; the payment reaches the
/// producer less the usual loss and counts in its gross market inflow. Writes
/// each site's `sold`. Returns the grain each settlement paid.
#[allow(clippy::too_many_arguments)]
fn goods(
    state: &mut State,
    p: &Params,
    idx_of: &[u32],
    food_in: &mut [f32],
    food_out: &mut [f32],
    market_in: &mut [f32],
    flows: &mut Vec<Flow>,
) -> Vec<f32> {
    let m = state.settlements.len();
    let mut goods_in = vec![0f32; m];
    let settlement_of = |id: u32| -> Option<usize> {
        idx_of
            .get(id as usize)
            .copied()
            .filter(|&j| j != u32::MAX)
            .map(|j| j as usize)
    };
    // Capacity left at each site after its holder's own use.
    let mut left: Vec<f32> = state.sites.iter().map(|s| s.output.max(0.0)).collect();
    let mut sold = vec![0f32; state.sites.len()];
    // Sites by holder index, for the holder's own use and for lookups by partner.
    let mut by_holder: Vec<Vec<usize>> = vec![Vec::new(); m];
    for (si, site) in state.sites.iter().enumerate() {
        if site.output > 0.0 {
            if let Some(j) = settlement_of(site.holder) {
                by_holder[j].push(si);
            }
        }
    }
    let mut want = vec![[0f32; 3]; m];
    for i in 0..m {
        let s = &state.settlements[i];
        for g in Good::ALL {
            let mut d = demand(s, p, g);
            for &si in &by_holder[i] {
                if state.sites[si].good == g && d > 0.0 {
                    let own = d.min(left[si]);
                    left[si] -= own;
                    sold[si] += own;
                    d -= own;
                }
            }
            want[i][g.index()] = d;
        }
    }
    // Orders: (buyer, site, quantity, cost, buyer's partner index).
    let mut orders: Vec<(usize, usize, f32, f32, u32)> = Vec::new();
    let mut ordered = vec![0f32; state.sites.len()];
    let mut ws: Vec<(usize, f32, f32, u32)> = Vec::new();
    for i in 0..m {
        for g in Good::ALL {
            let d = want[i][g.index()];
            if d <= 0.0 {
                continue;
            }
            let b = beta(p, g);
            ws.clear();
            let mut total = 0.0;
            for (k, q) in state.settlements[i].reach.partners.iter().enumerate() {
                let Some(j) = settlement_of(q.id) else {
                    continue;
                };
                for &si in &by_holder[j] {
                    if state.sites[si].good == g && left[si] > 0.0 {
                        let w = left[si] * libm::expf(-b * q.cost);
                        total += w;
                        ws.push((si, w, q.cost, k as u32));
                    }
                }
            }
            if total <= 0.0 {
                continue;
            }
            for &(si, w, c, k) in &ws {
                let q = d * w / total;
                ordered[si] += q;
                orders.push((i, si, q, c, k));
            }
        }
    }
    // Orders beyond capacity are scaled down to fit.
    let fill: Vec<f32> = (0..state.sites.len())
        .map(|si| {
            if ordered[si] > left[si] {
                left[si] / ordered[si]
            } else {
                1.0
            }
        })
        .collect();
    // What each buyer owes, and the share of it its marketed grain can pay.
    let mut bill = vec![0f32; m];
    for &(i, si, q, _, _) in &orders {
        bill[i] += value(p, state.sites[si].good) * q * fill[si];
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
    for &(i, si, q, c, k) in &orders {
        let q = q * fill[si];
        if q <= 0.0 {
            continue;
        }
        sold[si] += q;
        let good = state.sites[si].good;
        let Some(j) = settlement_of(state.sites[si].holder) else {
            continue;
        };
        let (buyer, producer) = (&state.settlements[i], &state.settlements[j]);
        let worth = value(p, good) * q;
        // Goods travel on the producer's path to the buyer when it has one,
        // else on the buyer's path the other way.
        let (from, to, idx) = match producer
            .reach
            .partners
            .iter()
            .position(|r| r.id == buyer.id)
        {
            Some(fwd) => (producer.id, buyer.id, fwd as u32),
            None => (buyer.id, producer.id, k),
        };
        flows.push(Flow {
            from,
            to,
            amount: worth,
            food: 0.0,
            partner_idx: idx,
            good: Some(good),
            payment: false,
            import: false,
            relief: false,
        });
        let pay = worth * scale[i];
        if pay <= 0.0 {
            continue;
        }
        paid[i] += pay;
        food_out[i] += pay;
        let delivered = pay * (1.0 - p.transport_loss * c).max(0.0);
        food_in[j] += delivered;
        market_in[j] += delivered;
        goods_in[j] += delivered;
        // The payment travels on the buyer's own path to the producer.
        flows.push(Flow {
            from: buyer.id,
            to: producer.id,
            amount: pay,
            food: pay,
            partner_idx: k,
            good: None,
            payment: true,
            import: false,
            relief: false,
        });
    }
    for (site, s) in state.sites.iter_mut().zip(sold) {
        site.sold = s;
    }
    for (s, g) in state.settlements.iter_mut().zip(goods_in) {
        s.goods_in = g;
    }
    paid
}

/// Trade with the outside ports (v5, D64), by the shopping model (D41).
/// Imports: each settlement wants `import_demand` per market person, split
/// among the ports it reaches by `exp(−β/ρ_imp c)`, and pays in grain out of
/// what is left of its marketed share; the grain leaves the map. Grain: each
/// port buys `outside_grain` a year, split among the settlements that reach it
/// by what they have left to sell × `exp(−β c)`, capped at that; the seller is
/// paid in imports, which count in its gross market inflow. Both kinds of
/// trade ride the settlements' stored paths to the port. Returns the grain each
/// settlement paid (goods and imports) and sold to the ports; writes the year's
/// totals to `outside_trade`.
fn outside(
    state: &mut State,
    p: &Params,
    mut paid: Vec<f32>,
    market_in: &mut [f32],
    food_out: &mut [f32],
    flows: &mut Vec<Flow>,
) -> (Vec<f32>, Vec<f32>) {
    let m = state.settlements.len();
    let budget = |i: usize, paid: &[f32]| {
        (state.settlements[i].food_prod * p.market_share - paid[i]).max(0.0)
    };
    let b_imp = p.beta / p.rho_import.max(1e-6);
    let mut imported = 0.0f32;
    for i in 0..m {
        let s = &state.settlements[i];
        let w = if p.market_state {
            s.market.max(0.0)
        } else {
            urban_pop(s, p)
        };
        // With goods by the equilibrium (D71), imports are bought there.
        let want = if p.market_goods {
            0.0
        } else {
            (p.import_demand * w).min(budget(i, &paid))
        };
        if want <= 0.0 {
            continue;
        }
        let ports: Vec<(u32, f32)> = s
            .reach
            .partners
            .iter()
            .enumerate()
            .filter(|(_, q)| port_of(q.id).is_some())
            .map(|(k, q)| (k as u32, libm::expf(-b_imp * q.cost)))
            .collect();
        let total: f32 = ports.iter().map(|x| x.1).sum();
        if total <= 0.0 {
            continue;
        }
        for &(k, wt) in &ports {
            let q = want * wt / total;
            let to = s.reach.partners[k as usize].id;
            // The imports come in, and the grain that pays for them goes out,
            // both on the buyer's path to the port.
            flows.push(Flow {
                from: s.id,
                to,
                amount: q,
                food: 0.0,
                partner_idx: k,
                good: None,
                payment: false,
                import: true,
                relief: false,
            });
            flows.push(Flow {
                from: s.id,
                to,
                amount: q,
                food: q,
                partner_idx: k,
                good: None,
                payment: true,
                import: false,
                relief: false,
            });
        }
        paid[i] += want;
        food_out[i] += want;
        imported += want;
    }
    // Grain bought by the ports.
    let mut sold = vec![0f32; m];
    let mut bought = 0.0f32;
    for (port, _) in state.outside.iter().enumerate() {
        let id = crate::state::OUTSIDE - port as u32;
        let mut offers: Vec<(usize, u32, f32, f32)> = Vec::new();
        for i in 0..m {
            let left = budget(i, &paid) - sold[i];
            if left <= 0.0 {
                continue;
            }
            if let Some(k) = state.settlements[i]
                .reach
                .partners
                .iter()
                .position(|q| q.id == id)
            {
                let c = state.settlements[i].reach.partners[k].cost;
                offers.push((i, k as u32, left * libm::expf(-p.beta * c), left));
            }
        }
        let total: f32 = offers.iter().map(|x| x.2).sum();
        if total <= 0.0 {
            continue;
        }
        for &(i, k, wt, left) in &offers {
            let q = (p.outside_grain * wt / total).min(left);
            if q <= 0.0 {
                continue;
            }
            let s = &state.settlements[i];
            flows.push(Flow {
                from: s.id,
                to: id,
                amount: q,
                food: q,
                partner_idx: k,
                good: None,
                payment: false,
                import: false,
                relief: false,
            });
            // Paid in imports, which come back the same way.
            flows.push(Flow {
                from: s.id,
                to: id,
                amount: q,
                food: 0.0,
                partner_idx: k,
                good: None,
                payment: false,
                import: true,
                relief: false,
            });
            sold[i] += q;
            food_out[i] += q;
            market_in[i] += q;
            bought += q;
        }
    }
    state.outside_trade = if p.market_goods {
        (bought, state.outside_trade.1)
    } else {
        (bought, imported)
    };
    (paid, sold)
}
