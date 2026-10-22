# v7 M1: marketplaces — design note

This note answers the brief in `docs/foundation.md` ("M0 result and the M1 brief", D87), for review. No code has been written. M0's numbers are in `docs/market/m0.md`. Prices are in grain value units, as in S1 and S2. Everything below sits behind `marketplaces`.

## 1. Village to market
Each year, after cultivation, each place `i` with harvest `H_i`, need `N_i` and rent share `r` (`market_share`):
- **Rent goes to market in kind:** `r H_i`, the lords' grain.
- **The farmers eat first** from the rest, up to their growth target: `own_i = min((1 − r) H_i, food_cap × N_farm,i)`. `N_farm` is the need of the people not counted in W.
- **Their surplus is sold:** `(1 − r) H_i − own_i`.
- **Routing.** Grain for sale is sent to markets `j` by today's weights `W_j^α exp(−β c_ij)`, with its own market by `home_weight`, and arrives less `transport_loss` per unit of cost, as now. Turnover `T_j` is the grain arriving at `j`. This is today's gravity loop with the marketed share replaced by rent plus surplus. It is cheap and keeps the routing feedback that makes towns (M0: Spearman ≥ 0.955 with `market_in`).
- **Deficit places.** A place whose own food falls below need first takes S2′ relief and S3 stores, as now. Beyond that it buys, within its budget (its farmers' sales), at the markets it would sell to, in the same proportions.

## 2. Market to market
- **Who takes part.** Only markets with `W_j ≥ market_min` (a new parameter, about 50). That is the top 30–60 places on a generated map; M0's top ten are all far above it.
- **Pricing.** Each takes part with its net position at its price: `T_j` against its buyers' demand (section 3). Prices clear by the entropy equilibrium (S1/S2's alternating log-space solves, with S2's common-shift level step and floor) over market pairs within `C_max`, at `θ = β / market_tau`. Flows ride the stored paths, load roads and earn fees as now.
- **Small markets** below the threshold clear locally. Their excess goes to the nearest large market in reach at that market's price less transport, and their deficit is bought there. That is one arc each.

## 3. Buyers at a market
- **Market people (W).** They buy their food, `W_j × food_per_person × min(p^(−ε), food_cap)`, from their income within budget:
  - rents received: `p_j × the rent grain arriving at j`, since the lords live in the market where their rent is sold;
  - the toll on turnover;
  - fees;
  - goods sales.

  There is no establishment purchase (D79). What is left after food buys goods and imports as now (S1), and anything still left is not carried over.
- **Deficit villages,** within their budgets (section 1).
- **The outside ports:** up to `outside_grain` at up to the world price, at the large markets on their paths (D78).
- **Sellers** receive `p_j (1 − toll)`. The toll is paid to the market's people.

## 4. Growth
- **W follows `ε (T_j − K W_j)`,** with `T_j` the grain sold at `j` in grain units, using W2's ten-year average (D85). Since `T_j` tracks `market_in` (M0), K (3.39) should carry over. This is checked first, in M2's no-tuning run.
- **Market people who cannot pay for their food go short.** Their place's food ratio falls, and D34's shedding removes them as now. No new rule is needed.

## 5. Unsold grain before M3
- Grain that does not clear at the floor (`p_floor` 0.25, D77) is **lost**, and it is reported.
- **It does not count toward W:** `T_j` is grain sold, not grain arrived.
- With stores on (S3), unsold grain fills the market's own store up to `store_cap`. That is M3's starting point, kept off in M2 so that M2 is judged alone.
- M0 suggests the amount is small: in towns over 2,000 the food bill is 84% of turnover, and deficit villages and ports take more.

## 6. Toll
- **Starting value: `toll` 0.03.** At 2% M0 has 83–91% of towns over 2,000 breaking even; at 5%, 88–93%. Three percent covers most of the shortfall without moving much value off farmers.
- The towns that need 11–24% are places with little through-trade, which should shrink under this design. That is the intended consumer-city dynamic, not a failure.
- **Range for tuning in M2:** 0.01–0.10.

## 7. Cost
- **Level 1** is today's gravity loop: about 1 ms.
- **Level 2:** 30–60 markets with at most about 1,800 pairs, against S2's roughly 30,000 arcs. At 8–16 passes with the level step, about 1–2 ms in wasm.
- **Budgets and the small-market step** are linear in places.
- **Estimate:** about 33–35 ms a year in wasm against today's 30.5, well inside 50. It is measured in M2's first build.

## 8. The M2 gate
- **Every v6 check**, with W2 on.
- **Clark, gated per seed** (75% of seeds a map), on the market prices:
  - `b_P` 0.8–1.2;
  - R² ≥ 0.6;
  - `b_Y` between −0.15 and 0.

  A place's price is the price at its markets, weighted as it sells.
- **Determinism**, and at most 50 ms a year in wasm.
- **Reported:**
  - unsold grain;
  - towns short of food and the people shed;
  - the toll's share of income;
  - price gaps against transport on market pairs;
  - the famine dip against v6 W2;
  - Kent.

## Questions for the designer
1. **`market_min` near 50:** the threshold for level 2.
2. **Deficit villages buy only with their own sales income.** Relief and stores stay free, as charity and the manor.
3. **Unsold grain fills market stores only in M3,** and is lost in M2.
