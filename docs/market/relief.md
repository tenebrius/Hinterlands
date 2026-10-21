# v6 S2′: grain relief (report)

After trade, each place's food is `A = food_prod + food_in − food_out`.
- **Surplus:** what it holds above `food_cap × need`, which was lost before.
- **Deficit:** what it lacks below need.
- **Drawing on surplus:** each place in deficit draws its deficit from the surplus holders among its partners, in proportion to `S_j exp(−β c)`. Orders beyond a holder's surplus are scaled down in one pass. Grain arrives less the transport loss, and unmet deficit stays.
- **What it touches:** nobody pays. Relief counts in the food ratio, not in `market_in`. It loads routes (roads) but earns no handling.

The code is `relief` in `crates/hinterlands-core/src/systems/trade.rs`, behind `grain_relief` (off). The golden is unchanged.

## Gate (Historical, 20 seeds a map, `market_goods` on): PASSED, after one tuning change
- **First run, the defaults plus relief:** every check holds except map 1 market spacing, 14/20, with a median of 8.9 km (S1: 17/20). Relief keeps marginal places alive near markets.
- **Tuning, the one failing check only:** `shadow_drain` 0.015 → 0.025 (the urban shadow). 0.02 still gave 14/20; 0.025 gave 18/20.
- **With `shadow_drain` 0.025** (`data/real/experiments/relief.json`), PASSED:
  - spacing 18 / 20 / 19;
  - failure rate 20/20 on every map;
  - runs passing every check: 13 / 17 / 17 (S1: 16 / 18 / 15).
- **Determinism:** ok. Release wasm gives the native hash, `f5b32f88ee3fc22f`, at 30.9 ms/year (S1: 31.9). The viewer's `?selftest=500&relief=1` times it.

## Reported (medians over 20 seeds a map; S1 in brackets)

| | Map 1 | Map 2 | Map 3 |
| --- | --- | --- | --- |
| Relief arrived per year | 2,007 | 1,657 | 1,543 |
| Deficit met, normal years | 0.88 | 0.86 | 0.86 |
| Deficit met, 1315–17 | 0.08 | 0.07 | 0.06 |
| Famine dip | 0.24 (0.24) | 0.24 (0.25) | 0.25 (0.25) |
| Failure rate | 0.471 (0.458) | 0.473 (0.471) | 0.479 (0.475) |
| Surplus still lost above `food_cap`, per year | 5,791 | 5,398 | 5,310 |

- **Normal years.** Relief meets most of the deficit: about 1,500–2,000 grain a year out of 5,300–5,800 that would be lost.
- **The famine.** Relief does almost nothing: 6–8% of the deficit is met. Every cell's yield falls by 40% at once, so almost no place is above `food_cap` to give. The famine dip is unchanged.
- **Failure rate.** It does not fall. It is slightly higher with the stronger shadow drain needed for spacing.
- **Grain lost.** Most of the surplus above `food_cap` is still lost: the holders are towns and rich villages, and few deficits are in their reach. Stores (S3) are what would carry it into the famine.
