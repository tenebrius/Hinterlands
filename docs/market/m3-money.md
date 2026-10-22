# v7 M3: money — design note (D96)

Everybody holds money, in grain-value units (one person-year of grain at price 1). There is no floating currency: coin is a stock that moves only by payment. For review; no code yet.

## 1. Purses, and the order of payment within a year
Each settlement has two purses:
- **`F_i`, the farmers.**
- **`M_i`, the market:** its lords, church and merchants. The works a place holds pay into its `M_i`.

Within the year, after cultivation and production:
1. **Goods (S1).**
   - Buyers pay from the purse of the people who want the good: salt and iron per head, split between `F` and `M` by farmers and market people; stone and imports from `M`. A bill is scaled down to what the purse holds.
   - The producer's receipts go to its holder's `M`. Imports leave the region (coin out).
2. **Grain is cleared** by the marketplace solver (village to market, then market to market, D88–D93), with the budgets of section 4.
3. **Grain settles.** Each unit moved on an arc from seller `i` to buyer `j`:
   - the buyer's purse pays `p_j q`;
   - the seller receives `p_i q (1 − toll)`;
   - the toll `p_i q × toll` goes to the `M` of the market where it is sold;
   - the rest, the price gap `(p_j − p_i) q`, is carriage and handling. It goes to the `M` of the buyer's market (its carters and merchants).

   Nothing is created. Who receives depends on the grain sold:
   - **rent grain:** the lords, in the `M` of the market they live in, as M0 routed it;
   - **farmers' surplus:** `F` of the selling place;
   - **store grain:** `M` of the store's market;
   - **outside ports:** they pay from outside (coin in).
4. **Stores trade inside the solve** (D94, spike rule), paid from or credited to their `M`.
5. **Relief** (S2′) stays unpaid charity, and moves no money.
6. **Year end:** each purse loses its `loss` rate. A purse above its cap spends the excess on things outside the model, so it leaves as loss.

Income received this year is spent next year: budgets are the purses at the start of the year (section 4). This one-year lag is what a stock is.

## 2. Starting stock
- **The ten starting hamlets** each start with `F` = 0.5 × need and `M` = 0.5 × their market people's need.
- **A new hamlet's founders bring** the same share of their donor's `F` as of its people. A planted town brings 0.5 × its need in `M` from outside (coin in, counted).
- **When a place is removed,** its purses pass to its largest partner. If it has none, they are counted as loss.
- **Migrants carry nothing,** and plague deaths leave purses to the survivors.

## 3. Conservation (a unit test)
For every year,

`Σ (F_i + M_i)` after = before + ports' grain payments + planted towns' coin − imports paid − loss − purses of places removed with no partner.

`tests/money.rs` runs 150 years on map 1 and checks this identity each year to 1e-4 of the stock. `validate` reports the stock, purses by type, and the port balance.

## 4. Budgets in the solver
Within the marketplace solve, at buying price `b`:

| Buyer | Demand |
| --- | --- |
| Market people | `min(K W min(b^−ε, food_cap), M_j / b)` (the people eat first, D93) |
| Farmers in dearth | `min(shortfall below need, F_i / b)` (D95) |
| A store buying | `min(gain × gap × T_j, room, (M_j − b × food bought) / b)` |
| Ports | as D78, from outside |

These replace D88's in-year grain budget (rents + toll + fees), which is retired. The rents, tolls and fees arrive as money one year later. `fee_earned` goes from the budget. Handling and transshipment margins still count towards W's growth through `market_in`, as quantities (D27), but they are paid only through the price gap in 3.

## 5. Caps and loss rates

| Purse | Cap | Loss a year | Reading |
| --- | --- | --- | --- |
| `F` (farmers) | 1.0 × the farmers' need | 0.10 | peasant savings: Dyer 1989 |
| `M` (market) | 2.0 × the value of its market people's K-consumption | 0.10 | building, alms and luxuries not modelled: Britnell 1993 |

Both are parameters (`purse_cap`, `purse_loss`, `mpurse_cap`, `mpurse_loss`).

## Gates and reports
- **The new check:** the regional median grain price within 0.4–2.5 at every 50-year mark.
- **D95's pre-registrations stand:**
  - famine dip ≤ 0.16;
  - detrended price autocorrelation ≥ 0.3;
  - famine deficit met ≥ 30%.
- **Reported:** the money stock, `F` and `M` by place type over time, the port balance (coin in and out), and what purses buy.

## The risk
- **The price level is now set by the stock of coin** against the grain sold (Mayhew 1995).
- **If the ports' coin is small against imports and loss,** the stock shrinks, prices fall towards the floor, and towns are starved of budget.
- **The fix:** the loss rates and caps, with the new price check as the guard.
