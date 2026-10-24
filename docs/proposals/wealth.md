# Note: D3, wealth raises stocks (D154)

From the implementer, 24 October 2026, for the designer to decide. **No code.** Each figure says where it comes from; *to check* marks what I have not read in the original.

## What D3 is for
- **The demand for stocks is fixed per head today:**
  - salt 15 lb a person;
  - household iron 1 d;
  - home cloth 1.5 yards an adult (`cloth_yd_adult`, held constant since D123);
  - stone per W.
- **The record says it rose with income after 1348.** Wages and living standards rose (Dyer 1989; Hatcher 1977), and people bought more and better cloth, iron goods and housing.
- **D3 lets each stock per head rise with income per head,** so that the rise comes from the model's own income, not from a path in time (D123: "a path in time is a script").

## 1. The income measure
**Proposed: real income per head = the coin a place's people earn in a year, over their number, in grain units, as a ten-year average.**

**The coin earned is already in the model:**
- farmers' income from grain sales and rents (`income`, written by grain: `farm_inc + estab_inc`);
- wool receipts (`wool_market`'s receipts, farmers' and lords' shares);
- for W, the market's inflow (`market_in`), which already includes rents, goods sales, fees and the cloth margin.

**Why grain units:** the model's coin is tied to grain (`to_coin`). Income in grain units is a real income, deflated by the bread price, as the wage series are.

**Why a ten-year average:** purses smooth spending (D95, D97). W's pull already uses a ten-year average inflow (W2, D85), and stocks are bought over years.

**Why not purse stocks:** a purse is wealth, not income, and it is capped (`purse_cap`, `mpurse_cap`). It would saturate at the cap in good years and say little.

**Reference:** each place's income per head relative to the region's mean in 1300–1339 (y₀), before the plague. Stocks are at today's values at y₀, so nothing calibrated in D1 or D2 moves at the start.

## 2. The elasticity and its source
**The rule:** stock per head = today's value × (y / y₀)^e_g for each good g. Demand stays replacement plus growth (D118).

**Engel's law is the footing:** the budget share of necessities falls as income rises, and that of clothing, housing and durables holds or rises.

| Good | Proposed e | Source | Status |
| --- | --- | --- | --- |
| Clothing (home cloth) | 1.0 | Houthakker (1957, *Econometrica*, "An international comparison of household expenditure patterns"): about 1.0–1.3 across countries, the poorer at the low end | *to check: the figure itself* |
| Household iron (durables) | 1.0 | Houthakker (1957), durables near or above 1 | *to check* |
| Building stone (housing) | 0.8 | Houthakker (1957), housing about 0.8 | *to check* |
| Salt | 0.2 | Engel's law: a necessity that saturates, like food at 0.5–0.7 | a reading, not sourced |

- **A medieval check on the order of magnitude:** Dyer (1989) on the rise in peasant consumption after 1348. Better clothing, more meat and ale, larger houses. *To check: whether he gives a ratio.*
- **The elasticities must not be fitted to Dyer's figure.** The figure is the check below. If the sourced elasticities miss it, that is the result.

## 3. The check: Dyer's three yards in 1500
**Dyer's figure is per adult, not per person.** The project has it as about three yards of cloth a year per adult c. 1500: 1,250,000 adults buying three yards each would take 160,000 cloths (Dyer 1989 as cited; `docs/demand/cloth-note.md`, `d0.md`). D154 says "a person". **Which do you mean?** Per adult is how it is sourced, and how `cloth_yd_adult` is counted.

**Today:**
- 1.5 yards an adult, constant;
- at e = 1 the check needs real income per head in 1500 about twice that of 1300–1339;
- with e = 1.3 it needs about 1.7 times.

**The record's rise in real income (*to check*, from the series already used):**
- Clark's farm real wage roughly doubles from the 1300s to the mid-1400s and stays near that to 1500;
- Phelps Brown–Hopkins' builders' real wage shows the same.

So e of 1.0–1.3 with a real-income rise near 2 would put the model near three yards. **Whether the model's income rises that much is the test.** In the model, per-head income after the plague rises through cheap grain and fewer people on the same land.

## What it needs calibrated
- **Nothing new if Houthakker's elasticities are taken as sourced.** The reference y₀ is the pre-plague mean, which leaves the start unchanged.
- **Salt's 0.2 is a reading.** It could be set to 0 (salt fixed) to keep D3 to cloth, iron and stone.

## Risks
- **Home cloth rising doubles the wool demanded at home by 1500,** at the same clip. This competes with the mills and the ports, so the cloth share of wool leaving (D2's check) will move. Reported.
- **Coin.** More spending on iron and salt goes to the ports as coin out (`imports_out`), so the coin stock falls. Coin is already above D1b's from 1550, so this pulls the other way.
- **The purses' spending rule (`purse_loss` 5%: "dues, cloth, ale, the church") already spends a fixed share on the market.** D3 adds goods bought from income on top. **Implementer's reading:** they do not overlap, since `purse_loss` buys the market's services and D3 buys goods. But the designer should confirm.

## Questions for the designer
1. Per adult or per person for Dyer's three yards?
2. Houthakker's elasticities as sourced (pending a check of the figures), or a single e for all goods?
3. Salt included (e 0.2) or left fixed?
4. The pre-registrations, for example:
   - cloth per adult in 1500 at 2.5–3.5 yards;
   - goods spending per head after 1348 above before (the D118 table's check);
   - every gated check.
