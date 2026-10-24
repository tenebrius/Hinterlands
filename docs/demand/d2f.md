# D137: mills pay the clearing price; Munro's later cloth price. Gate failed, stopped

## Built (D137), on the branch at 0eb62f1, not on main
1. **Mills pay the seller's clearing price out of the cloth proceeds** (`systems/pasture.rs`, `wool_market`).
   - A mill pays the seller its price (`sup × q`), and the carriage stays with the mill's market.
   - It is settled when the cloth is sold. The mill's market purse takes `gross − wool paid`, and that is also its `cloth_margin`.
   - Home wool is paid as before.
2. **Cloth price from 1450 (`cloth_price_late`, on)** follows Munro's all-ports export value a cloth (`EngclothPrices2.xls`, AllPorts6M) over its 1371–1450 mean (£1.879). It is divided by Clark's wheat price (2004, appendix table 1, annual, from `Agprice.pdf`) over its 1371–1450 mean (0.651 s a bushel), taken over the same years.

| Years | Munro £ a cloth | / 1371–1450 | Clark wheat s/bu | / 1371–1450 | Deflated |
| --- | --- | --- | --- | --- | --- |
| 1451–60 | 1.86 | 0.991 | 0.639 | 0.982 | 1.009 |
| 1461–70 | 1.86 | 0.990 | 0.654 | 1.005 | 0.985 |
| 1471–80 | 2.13 | 1.134 | 0.667 | 1.025 | 1.106 |
| 1481–90 | 2.35 | 1.251 | 0.770 | 1.183 | 1.057 |
| 1491–1500 | 2.55 | 1.355 | 0.644 | 0.990 | 1.369 |
| 1501–10 | 3.24 | 1.723 | 0.731 | 1.123 | 1.534 |
| 1511–15 | 3.61 | 1.919 | 0.752 | 1.155 | 1.661 |

- **Implementer's readings:**
  - points sit at the middle of their years, linear between them;
  - held at 1.009 from 1450 to 1455, and at 1.661 after 1513;
  - 1.0 before 1450, as D137 says.
- **Before 1450 the deflated series would swing** 0.71–1.41 by decade (1371–1450). It is not used there.

**Nothing else changed.** `cloth_lb_per_w` is 2.5, and cloth carriage is still not taken from the mill's gross (as before). `tests/money.rs` (coin conservation) passes; only the golden changes.

## Gate: 40 seeds. **FAILED on outside trade, on every map**
`target/d2/v137.txt`. Every other gated check is 30/40 or better.

| Check | Map 1 | Map 2 | Map 3 | D136 |
| --- | --- | --- | --- | --- |
| **outside trade (3–50%)** | **19/40** (median 0.504) | **5/40** (0.526) | **18/40** (0.502) | 40 / 40 / 40 (0.404 / 0.422 / 0.411) |
| market spacing | 34/40 | 37/40 | 30/40 | 37 / 37 / 34 |
| road length 300–500 | 32/40 | 38/40 | 35/40 | 36 / 40 / 36 |
| pasture 1450>1340 | 34/40 | 40/40 | 39/40 | 35 / 40 / 40 |
| top-10 vs off | 39/40 | 40/40 | 34/40 | 37 / 40 / 36 |
| runs passing every check | 9/40 | 5/40 | 6/40 | 25 / 36 / 28 |

| Reported | Map 1 | Map 2 | Map 3 |
| --- | --- | --- | --- |
| over 2,000 in 1600, median (range) | 6 (3–9) | 6 (4–8) | 5 (3–8) |
| urban share in 1600, median | 0.363 | 0.371 | 0.391 |

**Attribution** (`target/d2/v137_nolate10.txt`): the payment fix alone, with `cloth_price_late` off, 10 seeds.
- **Outside trade passes 10/10 on every map,** with medians 0.425 / 0.449 / 0.420. That is up 0.02–0.03 from D136.
- **So the later cloth price takes outside trade over the ceiling.**
- **Mechanism, as read from the formula in `validate`:** outside share = (grain + iron + wool in + cloth in) / (market inflow + wool in + cloth in).
  - Cloth in doubles by 1600 (5.5k → 10.7k) at 1.66 times the price.
  - The mill towns' inflow is now only the margin.

## Pre-registered (D137), reported only
| | Result (9 runs) | Met |
| --- | --- | --- |
| Median price mills pay within 10% of the sellers' clearing price, every year | Mill-paid over the clip-weighted median realised price, yearly, range over runs per decade: 0.996–1.05 to 1480. It reaches 1.076–1.142 in some years of 1490–1530 (max 1.142 in the 1520s) | **no**, in some years after 1490 |
| Growers' coin rises against D136 | Farmers' purses (money probe F): 1400 32.8k against 34.4k; 1450 26.0k against 25.4k; 1500 28.5k against 24.5k; 1550 41.1k against 30.6k | **no before 1450, yes after** |
| Mill towns' W falls against D136 | 1350–1440 higher (e.g. 1400: 15.5k against 14.9k); 1450–1480 lower (5.6–5.9k against 7.5–8.7k); 1490 on higher (1500: 18.2k against 12.3k) | **only 1450–1480** |
| Cloth share around 1500 stays below the record's 0.55–0.85 | 1490 0.33, 1500 0.35, 1510 0.40 | **yes** |

**Readings:**
- **The price ratio is not 1 by construction.** Each mill pays its sellers their own clearing price exactly (`r = sup × q`). The gap is composition: the sellers near mills clear above the region's median seller.
- **Growers' receipts by year are now in the trace (`grower_coin`):** 10.8k in 1350, 4.3–4.7k in 1450–1500, 5.4–5.5k from 1520. D136's trace did not record them, so the comparison uses the farmers' purses.

## Report tables (medians of 9 runs; `target/d2/rep137`, `pace137`; D136 from `rep4`, `pace136`)

**Cloth share of wool leaving, by decade, against the record:**

| Decade | 1390 | 1400 | 1410 | 1420 | 1440 | 1450 | 1460 | 1470 | 1480 | 1490 | 1500 | 1510 | 1520 | 1530 | 1540 | 1550 | 1600 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| D137 | 0.24 | 0.26 | 0.22 | 0.17 | 0.16 | 0.15 | 0.18 | 0.15 | 0.17 | 0.33 | 0.35 | 0.40 | 0.48 | 0.51 | 0.58 | 0.59 | 0.53 |
| D136 | 0.26 | 0.28 | 0.25 | 0.21 | 0.19 | 0.20 | 0.22 | 0.21 | 0.20 | 0.34 | 0.35 | 0.33 | 0.34 | 0.33 | 0.33 | 0.34 | 0.39 |
| Record | 0.2–0.5 (1390–1410) | | | | 0.35–0.65 (1440–60) | | | | | 0.55–0.85 (1490–1510) | | | | | | | |

**Mill capacity and use:**
- From the 1370s every mill bidding is at capacity, use / capacity 1.00, as in D136.

| Decade | Capacity lb, D137 (D136) | Mill towns' W, D137 (D136) | W growth / region's | Margin, d a head of W a year, D137 (D136) | Cloth price d a lb |
| --- | --- | --- | --- | --- | --- |
| 1350 | 29,500 (28,000) | 10,000 (9,550) | +0.39 / +0.31 | 4.8 (7.4) | 5.70 |
| 1400 | 35,900 (34,600) | 15,500 (14,900) | −0.09 / −0.09 | 7.7 (10.6) | 5.70 |
| 1450 | 15,100 (20,400) | 5,920 (8,660) | +0.06 / +0.00 | 8.7 (11.9) | 5.75 |
| 1470 | 13,700 (19,200) | 5,620 (7,860) | −0.02 / −0.02 | 9.2 (11.3) | 6.27 |
| 1490 | 40,200 (28,700) | 12,100 (10,800) | +0.12 / +0.09 | 13.6 (13.6) | 7.71 |
| 1500 | 49,400 (33,000) | 18,200 (12,300) | +0.15 / +0.14 | 16.6 (13.8) | 8.70 |
| 1520 | 63,600 (42,700) | 24,400 (16,200) | +0.10 / +0.10 | 18.3 (13.8) | 9.47 |
| 1540 | 73,400 (50,400) | 28,700 (19,600) | +0.06 / +0.05 | 18.0 (13.6) | 9.47 |

**Coin by channel** (D136 → D137, medians of 9 runs, coin a year or stock):

| Year | Farmers' purses | Market purses | Coin stock | Wool in | Cloth in | Imports out | Luxuries out | Wear |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1350 | 30,323 → 28,735 | 33,158 → 33,344 | 63,481 → 62,079 | 7,875 → 7,829 | 924 → 1,023 | 4,157 → 4,171 | 2,201 → 2,194 | 641 → 627 |
| 1400 | 34,426 → 32,774 | 38,723 → 32,001 | 74,092 → 63,583 | 3,275 → 2,810 | 2,905 → 3,033 | 3,836 → 3,548 | 1,624 → 1,573 | 748 → 642 |
| 1450 | 25,389 → 26,016 | 28,373 → 29,236 | 51,755 → 52,483 | 2,588 → 2,414 | 1,696 → 1,161 | 2,833 → 2,831 | 1,135 → 1,143 | 523 → 530 |
| 1500 | 24,511 → 28,539 | 25,969 → 31,719 | 51,018 → 58,343 | 1,783 → 1,760 | 2,423 → 5,034 | 2,971 → 3,925 | 832 → 1,218 | 515 → 589 |
| 1550 | 30,583 → 41,054 | 36,552 → 51,573 | 67,135 → 96,195 | 2,706 → 1,408 | 4,099 → 9,834 | 4,694 → 6,974 | 1,419 → 2,355 | 678 → 972 |
| 1600 | 32,004 → 44,656 | 53,235 → 68,477 | 84,914 → 113,348 | 3,414 → 2,706 | 5,505 → 10,748 | 5,278 → 7,887 | 2,452 → 3,128 | 858 → 1,145 |

**Readings:**
- **Mill towns' W still grows at about the region's rate,** 0.00–0.08 a decade faster.
- **The rise after 1490 is the price acting through the margin:**
  - the margin rises to 16.6–18.3 d a head of W;
  - mill towns' W is 1.5 times D136's by 1500–1540;
  - capacity grows with it;
  - the share climbs to 0.58 by 1540.
- **The record's 0.55–0.85 at 1490–1510 is reached about 40 years late,** and the dip of 1420–80 is deeper than D136's.

**No new golden:** the gate failed. Main stays at D136 (`5e773d6f60251a23`).
