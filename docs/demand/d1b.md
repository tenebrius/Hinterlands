# D1b: port reach for salt and iron (D124). Gate failed; stopped

## Built
- **`port_reach`** (off by default). A settlement with no port among its partners (within `c_max`) gets `reach.port`, its nearest outside port at any travel cost.
  - It is found in `refresh_one` by an unbounded Dijkstra from the anchor that stops at the first port node (`Dijkstra::run_nearest`), so it is refreshed with the rest of the reach.
- **In `goods_trade`,** salt and iron get an arc from that port at the full travel cost (τ/ρ per unit of cost), with partner index `FAR_PORT`.
  - Flows on it ride the stored path.
  - Every reader of a flow's partner now goes through `Reach::partner(k)`.
- **Grain, founding and every other use of the partner list are unchanged.**
- **Golden unchanged with the flag off** (`4a9737eafc305561`).
- **Rule 5:** the cap on iron's price, a port arc for every buyer that has any port at all, was confirmed in the code before the run.

## Gate: every check on 40 seeds, with `port_reach` on. **FAILED**
`target/d2/d1b_40.txt`.

| Check | Map 1 | Map 2 | Map 3 | D1 default |
| --- | --- | --- | --- | --- |
| outside trade 3–30% | 32/40 (median 0.285) | **14/40** (0.312) | **5/40** (0.339) | 40 / 40 / 35 |
| over 2,000 | 31/40 | 37/40 | **25/40** | 35 / 37 / 31 |
| road length 300–500 | **28/40** | 37/40 | 37/40 | 32 / 36 / 35 |
| top-10 vs off | 33/40 | 36/40 | 35/40 | 40 / 39 / 36 |
| market spacing | 38/40 | 40/40 | 34/40 | 36 / 38 / 32 |
| **runs passing every check** | **15/40** | **10/40** | **2/40** | 27 / 28 / 17 |

**Failing checks** (runs failing / of which the only failure):
- **Map 1:** road length 12/4, over 2,000 9/1, outside trade 8/6, top-10 7/1.
- **Map 2:** outside trade 26/19.
- **Map 3:** outside trade 35/17, over 2,000 15/2, spacing 6/0.

## Pre-registered (reported only): both met
- **Median specialist size under 500:** 79 / 47 / 33 (p90 178 / 201 / 203; D1 had 1,009 / 418 / 407). Map 1's iron towns are gone.
- **Median iron price in formerly portless places below 3:** 1.057 / 1.060 / 1.057 (10 seeds a map, 1300–1600; maximum 1.08). D1 had 7–30.

## Outside trade: broken through imports and wool
Year 500, 10 seeds a map (`hinterlands outside`; parts of the share's denominator):

| | Share | Grain to ports | Imports (luxuries, salt, iron) | Wool |
| --- | --- | --- | --- | --- |
| Map 3, D1 default (40 seeds) | 0.268 | 0.005 | 0.061 | 0.194 |
| Map 3, D1b | 0.340 | 0.010 | 0.098 | 0.238 |
| Map 2, D1b | 0.305 | 0.022 | 0.091 | 0.186 |

- **The ceiling is broken by +0.04 on map 3** (median 0.339 against 0.30), and is just over on map 2 (0.312).
- **Imports are about +0.037:** the formerly portless third of the region now buys its iron and salt at the port instead of from its own mines.
- **Wool is about +0.044:** the coin that leaves for those imports is earned back by more wool exports (the D1 adjustment again). Wool's route is unchanged in D1b.
- **Over 2,000 on map 3 and roads on map 1** fall with it: the inland iron towns that counted among the places over 2,000 are gone.

## Stores, re-read (D124)
- Market stores (of a mean harvest), D1b at 40 seeds, all three maps alike:
  - cheapest tenth of years: 0.066–0.069;
  - all years: 0.023–0.026;
  - peak: 0.31–0.32.
- The same measure on the D1 runs (3 seeds a map; `hinterlands stores`): D1 off 0.119 / 0.045 / 0.47; D1 on 0.089 / 0.037 / 0.39.

## Not run: D2
- D2 is built behind `cloth` (off) and has not been gated, per D124 (stop on a failed D1b gate).
- **What it does:**
  - the fallow flock;
  - wool and export cloth by port reach;
  - mills at mill settlements with capacity `cloth_lb_per_w` × W, bidding in two tiers (home and export) on last year's prices;
  - home cloth as a sixth good in goods trade;
  - unsold and export-tier cloth to the ports at Munro's price less the cloth custom;
  - the wool custom (1275) and subsidy (1342) as coin out.
- **Two calibrations, beyond those accepted in D124:**
  - the cost of making cloth, 3.2 d a lb: the midpoint of the range the record allows, above 2.62 so raw wool, not cloth, is exported before the subsidy, and below 3.77 so cloth overtakes after 1347;
  - unit elasticity for home cloth, since at 0.3 scarce early supply drove the home price to 4–9 times the world price and drained the purses.
- **On map 1 seed 1,** the cloth share of the clip is 0.12–0.46 in 1225–1325 and 0.98–1.00 from 1350. Export cloth counts in outside trade, as wool does.
