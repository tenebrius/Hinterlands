# v6 S2: grain by equilibrium — design note

This note answers the brief in `docs/foundation.md` ("S2: design note first"). It is for the designer's review. No S2 code has been written.

## 1. Supply and demand per settlement
One grain price `p_i` per place, solved each year by the S0/S1 equilibrium. Flows are `x_ij = m_j · exp(θ (p_j − p_i − t_ij))` on the stored paths, plus a zero-cost own arc, with `θ = β / market_tau` as in D69.
- **Supply:** the whole harvest `H_i` (`food_prod`: after famine, mill and, later, weather) is offered. A place's own people are buyers at its own price.
- **Demand:** `need_i · min(p_i^(−ε), c_max)`, with `ε` 0.75 (Clark), within the budget below.
  - `c_max` is the satiation, taking over from `food_cap` (section 4).
  - `need` is `pop × food_per_person`, as now.
- **The food ratio,** which drives growth and shedding as now, becomes grain bought and eaten divided by need. `food_in` and `food_out` are the cleared flows, so roads, hubs, handling and the snapshot keep working.
- **`market_share` stops being a forced shipment.** Grain moves only where the price gap pays for it, so a hungry village buys and grain passes on (P1, P2).

## 2. Unit of value and the yearly budget
- **The unit** is the value of one person-year of grain at the reference price 1, the price at which supply meets need. Grain is no longer the numeraire, since it must be able to grow dear. Goods keep their S1 prices in this unit; their D0 and capacities are unchanged. There are no stocks across years: income is spent within the year, and anything unspent is lost until S5.
- **Every place's budget comes from transfers that already exist in the model, now paid for:**
  - **Its harvest at its own price:** `p_i · H_i`, whether it is sold or eaten at home.
  - **Rents and tithes in, rents out.** `rent_share × p_i H_i` (initially `market_share`) is paid in value, not grain, to markets by today's gravity weights `W_j^α exp(−β c)`, with the home market by `home_weight`. This is today's marketed-share split carried in value. It is the lords and the church, who live in market towns, and it keeps the base of the hierarchy. The payer loses exactly what the receiver gains.
  - **Handling and transshipment fees.** These are no longer created from nothing. Each stored path gets a fee per unit `f_ij = handling_margin × anchors passed + transship_margin × transshipments`, in value units, precomputed at reach refresh. The buyer pays `p_i + t_ij + f_ij`, and the fee is credited to the places where it arises, as `through` and `transship` are now. Because the fee is in the delivered price, the equilibrium routes around it.
  - **Goods sales** (S1, unchanged) and **grain sold** at the seller's price.
  - **Less:** goods and imports bought (S1), rents paid out, grain bought.
- **Demand is capped by the budget:** `p_i D_i ≤ Y_i` net of goods. This binds only for places whose income is not their own harvest, that is, market people.
- **A region-wide famine.** With 40% less grain, `p` roughly doubles (`0.6^(−1/0.75)` ≈ 1.98).
  - A farm's own grain falls by 40% and it eats from it.
  - Rents fall to `rent_share × H` in grain terms.
  - Fees and goods sales are fixed per unit carried in value. They fall with volume, while grain costs twice as much.

  So towns see grain dear against their income: dearer than the villages, which is the historical pattern.
- **Growth of W.** `D_j` for `ε (D_j − K W_j)` becomes real market income, `(rents + fees + goods sales + grain resold) / p_j`. At normal prices this is close to today's `market_in` plus margins in grain units, which is what K was tuned on.

## 3. W as the market premium (decision 4)
As in S0, `m_j = W_j^α` multiplies the buyer side of each grain flow, read as the pull of fairs, credit and many buyers. The price-gap bound then holds up to `(1/θ) ln m_j`: at α 1.09, θ ≈ 70 and W = 1,000, that is about 0.11. A large market pays up to about 11% over transport for its pull. W also keeps steering rents (section 2). Goods keep `m_j` = 1 (S1).

## 4. What replaces `food_cap`
- **Demand saturates at `c_max × need`,** with `c_max` = `food_cap` (1.2). Growth keeps its form: `r_grow` at a food ratio of `c_max`.
- **Surplus sells at a falling price** down to `p_floor`, a new parameter, proposed at 0.25.
  - A supplier whose clearing price would fall below the floor sells at the floor, as much as the flows at that price take.
  - The rest is unsold and lost: spoiled, or fed to animals. It is reported each year as `grain_unsold`.
  - This is S1's `capacity × min(1, p)` turned around.
- **Stores (S3)** will later absorb what is now lost.
- **Gross inflow no longer counts burned grain towards W** (P3), since W grows on paid income only.

## 5. The famine-year convergence fix
S0's grain residual was 0.3–0.5 in 1318, because the Newton steps warm-start from famine prices. Two changes, both deterministic:
1. **Re-level before the first pass.** Scale last year's prices by the closed-form regional level `(Σ supply / Σ need)^(−1/ε)` of this year over last year, then warm-start from that. Relative prices carry over, and the level jumps at once.
2. **Grain uses S1's alternating exact log-space solves** in place of S0's damped Newton. They converged in 8 passes for goods, with no step limit.

**Safeguard.** If the residual after `market_passes` is still above 0.05, run up to 8 more passes. The count depends only on the state, so it is deterministic. The yearly residual and the passes used are reported.

## 6. Cost per year in wasm (limit 45 ms)
- **Measured:**
  - S1 default: 31.9 ms in wasm.
  - S0, which solves all five goods by damped Newton: +18.7 ms in wasm over v5. Natively it adds about 10 ms to 25–27 ms.
- **Estimate for S2,** with `market_shadow` off, since S2 replaces it for grain: **40–45 ms in wasm,** close to the limit.
  - Grain has the most arcs (every partner), so it is most of S0's cost: about 11–13 ms in wasm.
  - The gravity grain loop it replaces saves about 1 ms.
  - Fees per path are computed at reach refresh, every 5 years, about 1 ms averaged.
  - Rents and budgets are linear.
- **If it comes in over the limit, the levers are:**
  - drop arcs whose weight at last year's prices is below 10⁻⁴ of the supplier's total;
  - 6 passes plus the safeguard;
  - store the arcs as flat arrays, not a `Vec` per pass.

  The first build measures the cost before any tuning.

## Found while writing this
- **`market_shadow` is not inert under `market_goods`.** It writes `price`, which D72's founding test reads, so turning on the shadow changes the run (map 1 seed 1: `1ed04bc733038208` against `ec7a24a7caeac414`). The W1 Clark numbers were taken this way.
- **Proposed fix, in S2's first commit:** the shadow writes its own field.

## Decisions asked
1. **Rents in value at `rent_share`.** Is this the right real source for towns? The alternative is the brief's list alone (fees, goods, grain resold). That alone would leave towns far poorer than today and risk the hierarchy.
2. **Fees in the delivered price.** This changes routing: transshipment at 0.24 a unit would push grain off transshipment routes. The alternative is fees paid by the buyer outside the routing price, which is simpler but not a true equilibrium.
3. **`p_floor` 0.25 and `c_max` = `food_cap`.**
