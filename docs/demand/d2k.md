# D150: wool grades, then grades with putting-out. Gate failed on rank-size (maps 1 and 2); stopped

## Built (D150), behind `wool_grades` (off)
**The grades.**
- Each clip is sold as three lots: good, middle and refuse.
- The port pays each lot its own price. Munro's Leominster 1499 ratios are 100 : 70 : 49. They are scaled so the clip's mean stays at Lloyd's price, which gives 1.370 / 0.959 / 0.671 of the old price.
- Home use and the mills pay one price for all grades.
- In the entropy split each lot is a seller. Its grade's price enters as a lower transport on its port arc.

**The grade shares are equal thirds, a named prior.**
- No sorting proportions were found.
- Slavin (2020, *EcHR* 73, p. 903) says the advance contracts "distinguished between the three qualities of wool (good, middling, and locks), and specified the share of each type", but he gives no shares.
- Bell et al., *Advance Contracts*, Munro 1978 and Lloyd 1977 were not reachable.

**The default and golden are unchanged** (`fc16de69c1855b30`) with `wool_grades` off.

## 1. Grades alone (`putting_out` off): hash `2484e8b2439e29b4`
9 runs (maps 1–3 × seeds 1–3), `target/d2/rep150`, `pace150`.
- **The cloth share by decade:**
  - 0.18 (1300), 0.12 (1350);
  - 0.18–0.28 in 1370–1410;
  - 0.16–0.21 in 1410–80;
  - 0.32–0.40 from 1490.

  Capacity still binds at 2.5 × W, so this is close to D144.
- **Raw wool to the ports, by grade, lb a year (medians):**

| Decade | Good | Middle | Refuse |
| --- | --- | --- | --- |
| 1350 | 85,900 | 69,900 | 600 |
| 1400 | 64,300 | 34,900 | 1,100 |
| 1450 | 55,800 | 26,800 | 700 |
| 1480 | 53,100 | 20,700 | 600 |
| 1500 | 57,100 | 1,200 | 300 |
| 1540 | 79,000 | 1,000 | 0 |

- **Good wool leaves raw in every decade.** Middle wool leaves raw until 1480 and goes to the mills when the later price lifts cloth's netback. Refuse goes to home use, the cheapest wool.

## 2. Grades with putting-out: the gate, 40 seeds. **FAILED**
`target/d2/v150.txt`, params `{"wool_grades": true, "putting_out": true}`.

| Check | Map 1 | Map 2 | Map 3 | D144 |
| --- | --- | --- | --- | --- |
| **rank-size slope** | **19/40**, −0.789 | **23/40**, −0.839 | 30/40, −0.881 | 37 / 40 / 40 |
| over 5,000 | 35/40 | 37/40 | 35/40 | 40 / 40 / 40 |
| market spacing | 35/40 | 32/40 | 33/40 | 34 / 37 / 30 |
| top-10 vs off | ≥ 36 | 33/40 | 31/40 | 39 / 40 / 34 |
| exports / output | 40/40, 0.181 | 40/40, 0.193 | 40/40, 0.195 | 0.084 / 0.094 / 0.088 |
| runs passing every check | 14/40 | 14/40 | 17/40 | 22 / 35 / 20 |
| old outside share (reported) | 0.709, 0/40 inside 3–50% | 0.701, 0/40 | 0.699, 0/40 | 0.50–0.53 |
| over 2,000 (reported), median | 7 | 7 | 6 | 6 / 6 / 5 |
| urban share (reported), median | 0.350 | 0.352 | 0.356 | 0.363 / 0.371 / 0.391 |

Every other gated check is 35/40 or better.

## Pre-registered (D150), reported only (9 runs; `rep150b`, `pace150b`)
| | Result | Met |
| --- | --- | --- |
| Cloth share below 0.95 every decade | 0.53–0.60 in 1380–1480, 0.66 in 1490, 1.00 from 1500 | **no**, from 1500 |
| 0.35–0.65 in 1440–60 | 0.56, 0.56, 0.55 | **yes** |
| 0.55–0.85 in 1490–1510 | 0.66, 1.00, 1.00 | **no** (1490 yes) |
| Map 1's rank-size 30/40 or better | 19/40 | **no** |
| Coin in 1600 at most 1.5 × D144's | 293k against 113k (2.6×) | **no** |
| Raw wool leaving in every decade, at no less than the good grade's share (1/3) | raw share 0.40–0.47 in 1380–1480, 0.34 in 1490, 0 from 1500 | **no**, from 1500 |

## What happened (fact, from the tables)
- **To 1480 the grades work as intended.** Middle and refuse wool go to cloth from the 1370s, while good wool (55–86k lb a year) keeps leaving raw. The share holds at 0.53–0.60, in the record's band for 1440–60.
- **From 1490 the later cloth price lifts the single cloth netback above good wool's port price.**
  - The median cloth netback rises from 3.0 d to about 6 d a lb by 1500; good wool fetches about 3.3 d.
  - So good wool goes to cloth too, and the share is 1.0 from 1500.
- **Reading:** with one cloth netback for every grade (D150 question 2), Munro's later cloth price outbids every grade. A netback rising with grade, the record's fine wool into dearer cloth, would keep good wool's order. But that needs a grade-to-cloth-value source.
- **Capacity under putting-out:**
  - capacity / W is 3 in 1350, 9–10 in 1380–1410, 12–19 in 1420–80, and 16–18 from 1500;
  - use / capacity falls to 0.42–0.65 once the clip binds;
  - the by-employment bound holds 3–10 mills in 1400–80, and 37–90 from 1500.
- **Home wool falls below need from 1400:** 52k against 70k lb in 1400, 41k against 80k in 1500.
- **The top farm-purse gainers** are again mill towns' own farmers: 2.6–3.9% each, field yield 1.06–1.80 × the region's, no pasture.

## Tables
**Cloth by decade (medians):**

| Decade | Capacity lb | Milled lb | Use | Bound / bidding | Mill towns' W | Raw good lb | Paid to farms (coin) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1350 | 29,500 | 27,900 | 0.98 | 0 / 101 | 9,400 | 85,900 | 733 |
| 1380 | 108,400 | 79,000 | 0.72 | 0 / 106 | 12,400 | 71,300 | 2,073 |
| 1420 | 126,100 | 72,500 | 0.54 | 7 / 99 | 9,700 | 57,200 | 1,902 |
| 1450 | 117,100 | 72,000 | 0.60 | 5 / 101 | 9,400 | 56,400 | 1,889 |
| 1480 | 142,300 | 84,900 | 0.60 | 10 / 103 | 9,000 | 51,900 | 2,229 |
| 1490 | 188,500 | 118,700 | 0.63 | 17 / 104 | 15,600 | 12,200 | 3,115 |
| 1500 | 285,300 | 151,500 | 0.54 | 37 / 110 | 18,000 | 100 | 3,976 |
| 1540 | 478,100 | 201,000 | 0.42 | 90 / 115 | 29,800 | 0 | 5,276 |

**Coin by channel:**

| Year | Coin stock | Price level | Wool in | Cloth in | Imports out | Luxuries out |
| --- | --- | --- | --- | --- | --- | --- |
| 1350 | 75,227 | 0.86 | 8,220 | 1,781 | 4,397 | 3,006 |
| 1400 | 116,399 | 1.11 | 3,075 | 6,399 | 4,867 | 4,073 |
| 1450 | 87,687 | 1.15 | 2,583 | 5,439 | 4,271 | 3,226 |
| 1500 | 119,925 | 1.20 | 12 | 15,047 | 6,018 | 5,466 |
| 1550 | 256,667 | 1.72 | 0 | 26,929 | 11,452 | 11,514 |
| 1600 | 293,243 | 1.86 | 0 | 30,086 | 12,930 | 14,964 |

**Population in 1600** is 73,800, against D144's 84,300.
