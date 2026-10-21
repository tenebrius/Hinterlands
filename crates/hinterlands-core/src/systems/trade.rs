//! Gravity trade (Wilson 1970; Huff 1964; Barjamovic et al. 2019), in the
//! Rihll–Wilson form: each settlement sends a fixed marketed share of its harvest,
//! plus its resource output, to markets in proportion to
//! `W_j^alpha * exp(-beta * c_ij)`, where `W_j` is the non-farming population, its own market included with weight `home_weight` (D13).
//! With `outside_enabled` (D25) the outside world is one more partner, of fixed
//! size `outside_size` and its own decay `outside_beta`: it buys marketed produce,
//! which leaves the map, and sends `outside_out` of goods to the settlements in
//! proportion to their pull, along the same paths.
//! Food arrives less a loss per unit cost. Flows are assigned to the stored
//! least-cost paths, giving the load on each cell and the route junctions (hubs).
//! Writes: flows, cell_load, hub, trade_in, food_in, food_out, through, market_in,
//! transship, sites' sold.

use crate::cost::{back_dir, step_dir};
use crate::params::Params;
use crate::state::{Flow, Good, State, OUTSIDE};
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
    // (destination index, partner index, weight, cost); usize::MAX is the outside.
    let mut weights: Vec<(usize, u32, f32, f32)> = Vec::new();
    let outside_pull = libm::powf(p.outside_size, p.alpha);
    // Imports from the outside: (settlement index, partner index, weight).
    let mut imports: Vec<(usize, u32, f32)> = Vec::new();

    // Goods (v2, D39): each site's output goes to consumers in proportion to
    // demand × exp(−β_g c), and they pay for it in grain out of their marketed
    // share, which travels to the producer as an ordinary food flow.
    let paid = if p.goods_enabled {
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

    for i in 0..m {
        let s = &state.settlements[i];
        if p.outside_enabled {
            if let Some(k) = s.reach.partners.iter().position(|q| q.id == OUTSIDE) {
                let c = s.reach.partners[k].cost;
                imports.push((i, k as u32, attract[i] * libm::expf(-p.outside_beta * c)));
            }
        }
        // What is left of the marketed share after paying for goods.
        let food = (s.food_prod * p.market_share - paid[i]).max(0.0);
        let out = food + s.resource_out;
        if out <= 0.0 {
            continue;
        }
        weights.clear();
        // A home-market weight keeps grain at home when markets are far or small.
        let mut total = attract[i] * p.home_weight;
        for (k, partner) in s.reach.partners.iter().enumerate() {
            if partner.id == OUTSIDE {
                if p.outside_enabled {
                    let w = outside_pull * libm::expf(-p.outside_beta * partner.cost);
                    total += w;
                    weights.push((usize::MAX, k as u32, w, partner.cost));
                }
                continue;
            }
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
            if j == usize::MAX {
                // Sold off the map.
                food_out[i] += sent;
                flows.push(Flow {
                    from: s.id,
                    to: OUTSIDE,
                    amount,
                    food: sent,
                    partner_idx: k,
                    good: None,
                    payment: false,
                });
                continue;
            }
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
            });
        }
    }

    // Goods from the outside, shared by pull and closeness; recorded on the
    // importer's own path to the sea edge.
    let total_import: f32 = imports.iter().map(|x| x.2).sum();
    if total_import > 0.0 {
        for &(i, k, w) in &imports {
            let amount = p.outside_out * w / total_import;
            trade_in[i] += amount;
            flows.push(Flow {
                from: state.settlements[i].id,
                to: OUTSIDE,
                amount,
                food: 0.0,
                partner_idx: k,
                good: None,
                payment: false,
            });
        }
    }

    // Assign flows to paths: load per cell, and the directions routes use there.
    let grid = t.grid;
    let mut dirs = vec![0u16; t.n()];
    // Where cargo changes mode: boarding or leaving a boat, or barge to sea ship.
    let mut transship = vec![0f32; t.n()];
    for f in &flows {
        let s = &state.settlements[idx_of[f.from as usize] as usize];
        let path = &s.reach.partners[f.partner_idx as usize].path;
        for w in path.windows(2).filter(|_| p.handling_income) {
            let (a, b) = (w[0], w[1]);
            let (wa, wb) = (state.cost.is_water_node(a), state.cost.is_water_node(b));
            let (ca, cb) = (state.cost.node_cell(a), state.cost.node_cell(b));
            if wa != wb {
                transship[if wa { cb } else { ca }] += f.amount;
            } else if wa && t.is_nav(ca) != t.is_nav(cb) {
                transship[if t.is_nav(ca) { ca } else { cb }] += f.amount;
            }
        }
        let mark = f.amount >= p.hub_min_flow;
        for q in 1..path.len().saturating_sub(1) {
            let cell = state.cost.node_cell(path[q]);
            state.cell_load[cell] += f.amount;
            if mark {
                let din = back_dir(step_dir(&grid, &state.cost, path[q - 1], path[q]));
                let dout = step_dir(&grid, &state.cost, path[q], path[q + 1]);
                dirs[cell] |= (1 << din) | (1 << dout);
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
    }
    state.flows = flows;
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
        // The payment travels on the buyer's own path to the producer.
        flows.push(Flow {
            from: buyer.id,
            to: producer.id,
            amount: pay,
            food: pay,
            partner_idx: k,
            good: None,
            payment: true,
        });
    }
    for (site, s) in state.sites.iter_mut().zip(sold) {
        site.sold = s;
    }
    paid
}
