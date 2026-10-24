# D2 corrected (D128): attribution run and gate. Gate failed; stopped

## Built (switches off; golden unchanged, `4a9737eafc305561`)
- **Home cloth is made where it is worn.**
  - Each place needs 1.5 yd × 2 lb × adults (0.6 of people) of wool a year. It buys it in the wool market, its own clip first, with unit elasticity.
  - The spinning and weaving is household labour; no coin goes to the mills for it.
  - The home-cloth good in goods trade is removed.
- **Export cloth is the only chain.**
  - Mills (mill settlements with W, capacity 30 lb × W) buy wool and sell broadcloth at the nearest port. The price is Munro's 5.7 d a lb, less the custom and carriage.
  - A mill bids that less the making cost of 3.2 d. Its demand is its capacity at or below the bid and falls steeply above it (`elast_mill` 20, a calibration).
  - Its margin over the wool is its market's inflow.
- **Wool's three outlets split by the S1 entropy form.** The outlets are home use, the ports at the world price less the duties, and the mills. The equations are the goods' (θ = β/τ, transport τ/ρ_wool × cost); the sellers sell their whole clip.
  - Coin: buyers pay their price, sellers get theirs, and the margin goes to the buyer's market (to the seller's for the ports). Duties and the cloth custom leave the region.
  - Coin is conserved (`tests/money.rs`, with both switches on).
- **Kept:** the fallow flock, port reach, the duties, 3.2 d, and the switches.

## 1. Attribution run: D1b, the fallow flock and wool by port reach; no mills, no home cloth (40 seeds)
`target/d2/attrib_40.txt`, against D1b alone (`target/d2/d1b_40.txt`). Outside trade is now at the 50% ceiling.

| Check | Map 1 | Map 2 | Map 3 |
| --- | --- | --- | --- |
| over 2,000 | **28/40** (D1b 31) | 37 (37) | **21/40** (25) |
| outside trade (median) | 36 (0.479) | **19/40** (0.501) | 33 (0.480) |
| recovery by 500 | 40 (40) | 40 (40) | 39 (40) |
| market spacing | 39 (38) | 38 (40) | 37 (34) |
| rank-size slope | 37 (40) | 40 (40) | 40 (40) |
| pasture 1450>1340 | 37 (39) | 40 (40) | 39 (40) |
| road length | 35 (28) | 37 (37) | 36 (37) |
| runs passing every check | 17 (15) | 14 (10) | 9 (2) |

**Exports a head c. 1300:** 12.5 / 13.2 / 13.6 d.

**Read:**
- **The wool side does not cost map 2's recovery** (40/40), so that pre-registration stands.
- **It does cost over 2,000 on maps 1 and 3.** All the wool now leaves the region.
- **On map 2 it pushes outside trade to the 50% ceiling.**

## 2. The corrected D2, gated with D1b (40 seeds). **FAILED**
`target/d2/gate_d2c_40.txt`. Checks below 30/40 in bold.

| Check | Map 1 | Map 2 | Map 3 |
| --- | --- | --- | --- |
| outside trade (median) | **29/40** (0.470) | 34 (0.470) | 38 (0.437) |
| over 2,000 | 40 | 35 | **23/40** |
| market spacing (median) | 33 (13.2 km) | 34 (12.0) | **25/40** (13.6) |
| stability | 38 | 37 | **28/40** |
| rank-size slope | 34 | 39 | 40 |
| recovery by 500 | 39 | 40 | 39 |
| top-10 vs off | 35 | 39 | 33 |
| road length | 36 | 34 | 30 |
| pasture 1450>1340 | 35 | 40 | 39 |
| **runs passing every check** | **10/40** | **20/40** | **5/40** |

**Map 2 passes every check.** Map 1 fails only outside trade, by one run. Map 3 fails over 2,000, spacing and stability.

**Exports a head c. 1300:** 7.3 / 8.0 / 7.5 d. Home use now takes about half the clip.

## Pre-registered (D128), reported only
| | Result | Met |
| --- | --- | --- |
| Market spacing and rank-size back to D1b's levels, 30/40 or better | spacing 33 / 34 / **25**; rank-size 34 / 39 / 40 | spacing on map 3 **no**; the rest yes |
| Cloth towns grow only after 1347; cloth-making places over 2,000 at 0–1 before 1340 | median 2–4, max 3–5 in 1290–1340 (9 runs) | **no** |
| Cloth share of exported wool rises after 1347 but stays below 0.95 in every decade | 0.17–0.18 in 1300–1340, **1.00** in every decade from 1350 | rises, yes; below 0.95, **no** |
| Map 2's recovery back to 30/40 | 40/40 | **yes** |

**Mechanisms (rule 5), confirmed in the code before the run:**
- Clothing bills stay at home: home wool only, no coin to the mills.
- Export cloth is the mills' only trade.
- The entropy split of the three outlets.

**Why the split is all-or-nothing:**
- θ = β / τ ≈ 70. After 1342 the port takes wool at 0.53 of the world price (the subsidy and custom are 1.54 d of 3.3 d a lb). The mills bid about 0.71.
- A gap of 0.18 gives the ports e^−12.5 of a share, so 1.00 is what the specified form gives at the S1 θ.
- Mill capacity (30 lb × W) is far from binding.
- **The cloth-making places over 2,000 before 1340** are market towns with mills that make some cloth for want of a nearer port. They are not towns grown by cloth: export cloth before the subsidy is 0.17 of the wool leaving.

## Tables (corrected D2; medians of 9 runs, 3 seeds a map)
| Year | People | Clip lb | Home use lb (need) | To mills lb | To ports lb | Cloth share of wool leaving | Cloth made at W 100-2,000 | Cloth towns | Over 2,000 (median / max of runs) | Median / largest cloth town |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1150 | 1,740 | 4,690 | 3,159 (3,049) | 24 | 1,506 | 0.02 | -0.00 | 24 | 0 / 0 | 47 / 66 |
| 1200 | 5,784 | 37,174 | 10,851 (10,185) | 291 | 25,997 | 0.01 | -0.00 | 53 | 0 / 0 | 55 / 153 |
| 1250 | 18,650 | 117,392 | 34,782 (32,794) | 2,047 | 81,069 | 0.02 | 0.68 | 85 | 0 / 0 | 72 / 1,029 |
| 1300 | 49,957 | 190,414 | 99,187 (88,943) | 15,591 | 72,206 | 0.17 | 0.74 | 96 | 2 / 3 | 133 / 3,333 |
| 1350 | 41,602 | 256,017 | 109,643 (73,199) | 149,347 | 0 | 1.00 | 0.69 | 101 | 2 / 4 | 121 / 3,986 |
| 1400 | 36,671 | 175,937 | 66,951 (64,526) | 111,880 | 0 | 1.00 | 0.58 | 100 | 3 / 6 | 48 / 5,043 |
| 1450 | 26,947 | 114,480 | 48,616 (47,855) | 62,245 | 0 | 1.00 | 0.63 | 90 | 1 / 3 | 38 / 4,487 |
| 1500 | 31,166 | 115,838 | 54,059 (55,348) | 61,778 | 0 | 1.00 | 0.38 | 94 | 3 / 4 | 56 / 6,001 |
| 1550 | 55,480 | 190,664 | 97,945 (99,151) | 86,701 | 0 | 1.00 | 0.34 | 104 | 5 / 8 | 133 / 8,884 |
| 1600 | 76,233 | 282,865 | 135,819 (136,966) | 134,167 | 0 | 1.00 | 0.39 | 111 | 5 / 8 | 227 / 9,285 |

Cloth share of wool leaving, by decade 1300-1600 (median of runs): 1300 0.17, 1310 0.18, 1320 0.17, 1330 0.18, 1340 0.18, 1350 1.00, 1360 1.00, 1370 1.00, 1380 1.00, 1390 1.00, 1400 1.00, 1410 1.00, 1420 1.00, 1430 1.00, 1440 1.00, 1450 1.00, 1460 1.00, 1470 1.00, 1480 1.00, 1490 1.00, 1500 1.00, 1510 1.00, 1520 1.00, 1530 1.00, 1540 1.00, 1550 1.00, 1560 1.00, 1570 1.00, 1580 1.00, 1590 1.00, 1600 1.00

Cloth-making places over 2,000, by decade 1250-1340 (median / max of runs): 1250 0/0, 1260 0/0, 1270 0/1, 1280 1/2, 1290 2/3, 1300 2/3, 1310 3/4, 1320 2/3, 1330 3/5, 1340 4/5

| Year | Coin stock | Price level | Grain to ports | Wool in | Cloth in | Duties out | Imports out | Luxuries out | Wear |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1150 | 10,613 | nan | 1,331 | 69 | 2 | 0 | 275 | 921 | 107 |
| 1200 | 24,172 | 0.52 | 2,106 | 1,190 | 23 | 0 | 897 | 1,657 | 244 |
| 1250 | 45,433 | 0.52 | 1,658 | 3,712 | 162 | 0 | 2,606 | 2,209 | 459 |
| 1300 | 45,716 | 0.89 | 426 | 3,306 | 1,234 | 220 | 3,532 | 793 | 462 |
| 1350 | 59,396 | 0.82 | 508 | 0 | 11,823 | 346 | 4,708 | 2,025 | 600 |
| 1400 | 90,165 | 1.16 | 198 | 0 | 8,857 | 259 | 4,957 | 3,242 | 911 |
| 1450 | 55,633 | 1.17 | 180 | 0 | 4,928 | 144 | 3,406 | 1,485 | 562 |
| 1500 | 49,669 | 1.18 | 195 | 0 | 4,891 | 143 | 3,300 | 1,027 | 502 |
| 1550 | 61,377 | 1.37 | 117 | 0 | 6,864 | 201 | 4,518 | 1,224 | 620 |
| 1600 | 86,234 | 1.63 | 69 | 0 | 10,622 | 311 | 6,496 | 2,251 | 871 |

## Stores (D124 measure: cheapest tenth / all years / peak, of a mean harvest)
- **Corrected D2:** 0.059–0.079 / 0.020–0.031 / 0.36–0.43.
- **Attribution run:** 0.114–0.128 / 0.047–0.051 / 0.40–0.48.
- **D1b alone:** 0.066–0.069 / 0.023–0.026 / 0.31–0.32.

## Outside trade
- **Corrected D2:** medians 0.470 / 0.470 / 0.437. Map 1 is 29/40 at the 50% ceiling.
- **Attribution run:** 0.479 / 0.501 / 0.480.
