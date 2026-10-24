# D145: putting-out, a cloth capacity stock. Gate failed on rank-size (map 1); stopped

## Built (D145), behind `putting_out` (off; the default and golden `fc16de69c1855b30` are unchanged)
**Capacity.** Each mill's capacity is its own stock, `cloth_cap` (`systems/pasture.rs`, `wool_market`).
- **Start:** from 1350 (`putting_out_from` 250) it starts at `cloth_lb_per_w` × W.
- **Growth each year:** Δcap = ε × (value added − κ_c × cap) / wool's port price a lb.
  - ε is W's own `epsilon` (0.137), unfitted.
  - Value added = cloth at the port less carriage, less the wool paid.
- **κ_c = 1.64 d a lb** (`cloth_upkeep_d_lb`, calibrated at the start only). It is the median over 9 runs (maps 1–3 × seeds 1–3) of value added a lb of capacity in 1350–59, under today's capacity of 2.5 × W.
  - The per-run values are 1.41–1.76 (`target/d2/cal145`).
- **Implementer's reading:** with value added in the rule, κ_c is the value added at which a network holds. The note's π (net of the making cost) is about 0 in 1350–59, so it cannot fix a starting level.

**The by-employment bound.** s = 25 lb a farm adult a year (`by_employment_lb`, a calibration).
- Farm adults = `adult_share` × (people − W) of each place in a mill's reach, itself included.
- **Implementer's reading:** where several mills reach a place, its adults are split in proportion to capacity × exp(−β × cost). The weight is 1 at the mill itself.
- A mill's capacity is capped at s × its adults.

**Payment.**
- The making cost (2.7 d a lb) × (1 − f) is paid from the mill's market purse to the farm purses of its adults, in the same split.
- f = 0.3 (`finishing_share`, the designer's calibration): fulling and finishing stay in the town.
- **Munro's split was not read for Ypres.** His Medici costing (1556–58, `target/d2/munro_textile_stats.txt`, Table 23) gives these town shares of the making cost:
  - 0.23 without dyeing (fulling, shearing and management against wool preparation, spinning and weaving);
  - 0.35 with dyeing.

  The two bracket 0.3.
- The mill's market keeps cloth less wool less the payout, and that is its `cloth_margin`.

**Kept.**
- `just test` passes, and the golden is unchanged with the switch off.
- `tests/money.rs` was not extended to `putting_out`. The payout moves coin between purses inside the region.

## Gate: 40 seeds, `putting_out` on. **FAILED on rank-size slope, map 1**
`target/d2/v145.txt`. D144 (`v141_late.txt`) is in brackets.

| Check | Map 1 | Map 2 | Map 3 |
| --- | --- | --- | --- |
| **rank-size slope** | **9/40**, median −0.756 (37/40, −0.949) | 32/40, −0.901 (40/40, −0.974) | 32/40, −0.898 (40/40, −1.006) |
| market spacing | 35/40, 11.7 km (34/40, 11.7) | 32/40, 13.6 (37/40, 12.2) | **30/40, 13.5 (30/40, 14.1)** |
| top-10 vs off | 34/40 (39/40) | 33/40 (40/40) | 35/40 (34/40) |
| over 5,000 | 37/40, median 1 (40/40, 3) | 40/40, 2 (40/40, 2) | 38/40, 2 (40/40, 2) |
| largest share | 40/40, 0.088 (39/40, 0.108) | 40/40, 0.096 (40/40, 0.140) | 40/40, 0.114 (38/40, 0.164) |
| recovery by 500 | 40/40, 1.13 (40/40, 1.32) | 36/40, 1.08 (40/40, 1.30) | 39/40, 1.12 (40/40, 1.25) |
| stability | 39/40 (40/40) | 38/40 (40/40) | 38/40 (37/40) |
| pasture outward | 36/40 (40/40) | 40/40 (40/40) | 39/40 (40/40) |
| road length 300–500 | 40/40 (32/40) | 40/40 (38/40) | 40/40 (35/40) |
| exports / output | 40/40, 0.186 (0.084) | 40/40, 0.197 (0.094) | 40/40, 0.197 (0.088) |
| runs passing every check | 6/40 (22/40) | 17/40 (35/40) | 17/40 (20/40) |
| over 2,000 (reported), median | 7 (6) | 7 (6) | 6 (5) |
| urban share (reported), median | 0.335 (0.363) | 0.382 (0.371) | 0.377 (0.391) |

- Every other gated check is at 40/40 on every map.
- **Map 1's rank-size slope is too flat:** −0.756 against the band's −0.8. The largest share falls (0.088), and over 5,000 falls to a median of 1.
- **Reading:** the making cost moves out of the mill towns, which flattens the top of the hierarchy.

## Pre-registered (D145), reported only (medians of 9 runs; `target/d2/rep145`, `pace145`)
| | Result | Met |
| --- | --- | --- |
| Cloth share 0.35–0.65 in 1440–60 | 1.00, 1.00, 1.00 | **no** |
| Cloth share 0.55–0.85 in 1490–1510 | 1.00, 1.00, 1.00 | **no** |
| Cloth share below 0.95 in every decade | 0.24 (1370), 0.42 (1380), 0.89 (1390), then 1.00 from 1400 | **no**, from 1400 |
| Capacity over mill-town W rising over 1420–1500 | 17.4 (1420), 15.4, 16.7, 16.0, 14.7, 17.1, 21.0 (1480), 15.3 (1490), 16.7 (1500) | **no**: flat at about 15–17 |
| Map 3's spacing no worse than D144's 30/40 | 30/40 | **yes** |

**What happened (fact, from the tables):**
- **Capacity grows at 6–9% a year from 1360,** as the mills run full: value added 2.7–3.0 d a lb of capacity against κ_c 1.64.
- **By 1390 the wool supply binds:**
  - use / capacity is 0.83 in 1390, then 0.43–0.70;
  - the mills take every lb that leaves the region;
  - raw wool exports fall to 0 from 1400;
  - home wool use falls below need: 51k lb against 68k in 1400, 31k against 68k in 1500, 54k against 131k in 1600. The mills outbid the households.
- **This is the all-or-nothing share the designer named as a risk in advance.**

## Cloth by decade, 1370–1550 (medians of 9 runs)
| Decade | Capacity lb | Milled lb | Use | Mills bound by s / bidding | Mill towns' W | Capacity / W | W growth / region's | Paid to farm purses (coin a year) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1350 | 24,800 | 16,700 | 0.66 | 0 / 102 | 10,000 | 2.5 | +0.37 / +0.30 | 437 |
| 1370 | 57,600 | 57,600 | 1.00 | 0 / 103 | 13,200 | 4.2 | +0.03 / −0.02 | 1,511 |
| 1380 | 99,700 | 99,100 | 0.99 | 0 / 101 | 13,100 | 8.1 | +0.20 / +0.14 | 2,602 |
| 1390 | 167,300 | 131,400 | 0.83 | 6 / 103 | 15,200 | 12.4 | +0.02 / −0.01 | 3,450 |
| 1400 | 195,200 | 141,300 | 0.70 | 24 / 101 | 15,800 | 14.3 | −0.09 / −0.10 | 3,709 |
| 1420 | 197,100 | 109,800 | 0.58 | 39 / 98 | 11,300 | 17.4 | +0.04 / +0.01 | 2,882 |
| 1450 | 191,900 | 119,200 | 0.61 | 28 / 98 | 10,400 | 16.0 | +0.03 / +0.02 | 3,129 |
| 1470 | 193,000 | 130,000 | 0.64 | 39 / 98 | 9,100 | 17.1 | −0.03 / −0.03 | 3,412 |
| 1490 | 242,000 | 129,200 | 0.52 | 31 / 100 | 13,500 | 15.3 | +0.13 / +0.11 | 3,390 |
| 1500 | 302,600 | 144,700 | 0.48 | 50 / 103 | 15,000 | 16.7 | +0.16 / +0.16 | 3,798 |
| 1520 | 345,100 | 169,800 | 0.45 | 84 / 107 | 22,400 | 15.9 | +0.09 / +0.08 | 4,456 |
| 1540 | 409,800 | 177,500 | 0.43 | 91 / 105 | 25,600 | 15.6 | +0.06 / +0.05 | 4,658 |

**Where and when s binds:**
- never before 1390;
- 24–39 mills of about 100 from 1400 to 1490;
- 50–91 from 1500.

It binds while use is only 0.43–0.70. The capacity stock runs on into unused capacity, to the bound, because κ_c is below the value added a lb of capacity even then (1.6–2.7 d).

## Cloth exports against the customs record (a check, not a target)
**The record:** Carus-Wilson & Coleman, *England's Export Trade* (1963), broadcloths a year, the mean of the decade's quinquennia, as tabulated by Munro (Table 1, `target/d2/munro_textile_stats.txt`).

| Decade | Record (cloths) | Record, against the 1420s | Model D145 milled lb | against the 1420s | D144 milled lb | against the 1420s |
| --- | --- | --- | --- | --- | --- | --- |
| 1360s | 13,100 | 0.33 | 33,200 | 0.30 | 33,400 | 1.49 |
| 1380s | 23,900 | 0.59 | 99,100 | 0.90 | 36,000 | 1.61 |
| 1390s | 39,200 | 0.97 | 131,400 | 1.20 | 36,200 | 1.62 |
| 1400s | 33,200 | 0.82 | 141,300 | 1.29 | 35,900 | 1.60 |
| 1420s | 40,300 | 1.00 | 109,800 | 1.00 | 22,400 | 1.00 |
| 1440s | 51,200 | 1.27 | 116,500 | 1.06 | 21,000 | 0.94 |
| 1450s | 36,600 | 0.91 | 119,200 | 1.09 | 15,100 | 0.67 |
| 1460s | 33,200 | 0.82 | 116,400 | 1.06 | 14,200 | 0.63 |
| 1470s | 43,500 | 1.08 | 130,000 | 1.18 | 13,700 | 0.61 |
| 1480s | 52,100 | 1.30 | 128,800 | 1.17 | 23,100 | 1.03 |
| 1490s | 59,800 | 1.48 | 129,200 | 1.18 | 40,200 | 1.79 |
| 1500s | 81,000 | 2.01 | 144,700 | 1.32 | 49,400 | 2.20 |

- **Model D145:** cloth rises 4 times from the 1360s to the 1400s (the record's rise is 2.5 times), then grows only 1.18 times from 1420 to 1490 (the record's is 1.48), because the clip binds.
- **D144:** close to the record's shape after 1470, but its level falls with W through 1440–70.
- **Not comparable:** the record's ratio of cloth to wool. In the 1420s cloth was about 40k cloths against about 14k sacks of wool, so cloth took about 0.4–0.5 of the wool leaving. The model's share is 1.0.

## The farm purses gaining most (`hinterlands putout`, coin summed 1400–1550, `rep145/u*.txt`)
- **About 430–460 places earn something.** The largest single gainer takes only 3–4% of the total (0.033, 0.032, 0.033, 0.040, 0.027, 0.034, 0.038, 0.042, 0.037 by run).
- **The top five in every run are mill towns themselves** (their own farm adults).
  - They sit on good land: mean field yield 0.9–2.1 times the region's, mostly 1.2–1.7.
  - They have almost no pasture (0.00–0.10).
  - Sizes run from 600 to 7,900 people in 1550.
- **Reading:** the split weights a mill's own farmers at 1 (cost 0), and mills sit on rivers with drainage, in good arable. So the coin goes to the mills' own fields, not to pastoral, poorer-land districts as in Suffolk or the Cotswolds.

## Coin by channel (medians of 9 runs)
| Year | Coin stock | Price level | Grain to ports | Wool in | Cloth in | Imports out | Luxuries out | Wear |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1350 | 62,079 | 0.80 | 574 | 7,829 | 1,023 | 4,171 | 2,194 | 627 |
| 1400 | 103,639 (D144 63,583) | 1.13 | 194 | 1 | 10,828 | 5,290 | 3,177 | 1,047 |
| 1450 | 95,293 (52,483) | 1.19 | 170 | 0 | 9,277 | 4,229 | 3,081 | 963 |
| 1500 | 136,154 (58,343) | 1.27 | 142 | 0 | 14,923 | 5,321 | 5,333 | 1,375 |
| 1550 | 234,415 (96,195) | 1.67 | 64 | 0 | 23,674 | 9,546 | 11,358 | 2,368 |
| 1600 | 301,637 (113,348) | 1.88 | 47 | 0 | 31,525 | 11,734 | 14,953 | 3,047 |

**Population in 1600** is 73,200 against D144's 84,300 (rep137).
