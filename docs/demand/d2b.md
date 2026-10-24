# D2 with mill capacity bound (D130): gate failed; stopped, with the map 3 diagnosis

## Built
- **Mill capacity binds.**
  - Mill capacity is `cloth_lb_per_w` × W.
  - After the entropy solve, a mill's inflow above its capacity is moved, seller by seller, to that seller's port arc at the port's price, after the duties.
  - Mills take about 0.97 of their capacity (`hinterlands cloth`, column `mill_cap_lb`).
- **`cloth_lb_per_w` = 0.1 lb a head of W:** calibration of the start only (no source found).
  - Trials on 9 runs (3 seeds a map), median share of the wool leaving that left as cloth, in 1350–59: 0.1 → 0.099; 0.2 → 0.113; 0.3 → 0.203; 0.5 → 0.233; 1 → 0.463; 5 → 0.646.
- **A fix:** a seller's own-use arc now exists only where it has home demand. This changes nothing when every place has home demand, as in the gate.
- **Rule 5:** that capacity binds in 1350 was confirmed on the probe before the gate (milled/capacity 0.98 in 1350–59).

## Gate: D1b with the capacity-bound D2, 40 seeds. **FAILED**
`target/d2/gate_d2b_40.txt`. Checks under 38/40:

| Check | Map 1 | Map 2 | Map 3 |
| --- | --- | --- | --- |
| over 2,000 | **22/40** | 32/40 | **14/40** (median 4) |
| market spacing | 33/40 | 34/40 | **20/40** (median 15.2 km) |
| top-10 vs off | **26/40** | 39/40 | 36/40 |
| recovery by 500 | 34/40 | ≥ 38 | 31/40 |
| road length | 33/40 | 34/40 | 35/40 |
| every good worked | ≥ 38 | ≥ 38 | 31/40 |
| **runs passing every check** | **13/40** | **24/40** | **6/40** |

- **Map 2 passes every check.**
- **Exports a head c. 1300:** 7.2 / 7.4 / 7.8 d.
- **Stores** (cheapest tenth / all / peak): 0.055–0.071 / 0.017–0.023 / 0.20–0.23.

## Pre-registered (D130), reported only, and why they cannot be read at face value
| | Result (median of 9 runs) | Met as stated |
| --- | --- | --- |
| Cloth share of wool leaving 0.2–0.5 in 1390–1410 | 0.51, 0.50, 0.48 | about |
| 0.35–0.65 in 1440–60 | 0.51, 0.54, 0.54 | yes |
| 0.55–0.85 in 1490–1510 | 0.58, 0.60, 0.59 | yes |
| Raw wool exported in every decade | yes, but about 700–1,300 lb a year after 1400 | yes |
| Cloth-made places over 2,000 rising after 1400 | median 2–3, flat | no |

**The path matches the customs record's shape, but on almost no wool.**
- After 1342, home use is 3–4 times the home need. In 1350 it is 245k lb against a need of 72k; in 1450, 151k against 49k.
- The wool leaving the region falls from about 90k lb in 1300 to about 2,000 lb a year after 1400.
- **The mechanism:**
  - the duties cut the port's price to 0.53 of the world price;
  - as the clip, unable to leave, depresses the sellers' prices, home demand at unit elasticity (D126, for home cloth) takes the wool at prices far below 1;
  - the households absorb the clip the duties push off the export market.
- **The cloth share** is then a ratio of two small numbers (mills about 900–1,700 lb, ports about 700–1,300 lb).

**The coin follows:**
- the region loses its export earnings, and the coin stock falls from 44k (1300) to 16–20k after 1400 (about 60–90k in D1 and D1b);
- the price level falls to about 0.5;
- grain to the ports becomes the main export again.

This is the channel to the over-2,000 failures below.

## Map 3 diagnosis (D130.4), 10 seeds each
`hinterlands diag`, `target/d2/diag/`:

| Build (map 3, 10 seeds) | People 1340 / 1450 / 1600 | Pasture 1450 / 1600 | Over 2,000 (median; runs <5) | Spacing km median (runs wide / tight) | Stability (median; runs >= 5%) | Top 15 in 1600 (median of each rank) |
| --- | --- | --- | --- | --- | --- | --- |
| D1 | 60,400 / 30,016 / 89,874 | 0.28 / 0.19 | 6; 1 | 11.4 (0 / 2) | 0.044; 0 | 9,099, 6,162, 4,624, 3,762, 3,081, 2,025, 1,620, 1,376, 1,175, 1,097, 944, 896, 854, 800, 757 |
| D1b | 62,176 / 32,726 / 88,832 | 0.28 / 0.19 | 6; 3 | 11.8 (1 / 0) | 0.043; 0 | 9,638, 7,196, 4,476, 3,030, 2,291, 1,955, 1,626, 1,346, 1,050, 970, 833, 789, 740, 682, 670 |
| Attribution (wool, no mills) | 62,667 / 31,705 / 86,628 | 0.27 / 0.23 | 5; 3 | 11.9 (1 / 0) | 0.044; 0 | 9,186, 6,776, 4,320, 3,391, 2,149, 1,711, 1,594, 1,392, 1,188, 978, 880, 796, 764, 710, 692 |
| D2, capacity-bound | 59,592 / 25,417 / 61,114 | 0.29 / 0.16 | 4; 9 | 14.6 (4 / 0) | 0.045; 1 | 8,282, 5,985, 4,128, 2,928, 1,442, 1,170, 968, 872, 748, 690, 648, 638, 592, 558, 549 |
|  | stability failures: 0.054 in 1376 (fall) | | | | | |

**Read:**
- **The losses at the top come chiefly from fewer people.**
  - D2 (capacity-bound) has 25k in 1450 and 61k in 1600, against 30–33k and 87–90k in D1, D1b and the attribution run.
  - The top four places are about the same size (8.3k against 9.1–9.6k).
  - **Ranks 5–15 shrink by about 35–45%.** Rank 5 is 1,442 against 2,149–3,081, so over 2,000 drops to a median of 4 (9 of 10 runs below 5).
- **Spacing fails on the wide side** (median 14.6 km; 4 of 10 runs wide, none tight). There are fewer market towns.
- **Stability:** one run failed, a 5.4% fall in 1376 (the endemic era), not a shock year.
- **D1b and the attribution run on map 3 match D1 in people and spacing.** Their over-2,000 losses (D1b 25/40, attribution 21/40 at 40 seeds) come with 3 of 10 runs below 5, against 1 of 10 for D1. That is a spread effect, the inland iron towns gone (D125), not fewer people.
- **D2's further loss is fewer people,** through the coin drain above: home wool absorbing the clip.

## For the designer
- The home-wool demand's elasticity is the lever that turns the duty wedge into the coin drain.
- At unit elasticity, households take 3–4 times their need when wool is cheap.
