# D152: Clark's wool series built; the gate held back on a 9-run exploration

## Built (D152), behind `wool_series` (off; golden `fc16de69c1855b30` unchanged)
**The series.** The port's wool price follows Clark's wool against wheat for the whole run (`WOOL_SERIES`, `wool_price_factor` in `systems/pasture.rs`), in place of `wool_fall`.
- **Data:** Clark 2004, *Agprice*, appendix tables 1 and 2. Wool is in d a lb and wheat in s a bushel, both annual.
- **The ratio:** the decade's mean wool over its mean wheat, taken over the years with both.
- **Scaled** to its 1290–1309 mean, the decades of Lloyd's 1294 price.
- **In time:** linear between decade midpoints, held at the 1200s value before 1205 and at the 1590s value after.
- **Grades** scale with it, since they are multiples of the port price.

**Both price series are in the code.** Munro's cloth price (`CLOTH_LATE`, `cloth_price_late`, on by default) and Clark's wool (`WOOL_SERIES`, `wool_series`).

**Implementer's readings:**
- the ratio of decade means, not the mean of annual ratios;
- the 1290–1309 reference.
- The series is noisy where wheat swings (for example 1.187 in the 1300s, 0.763 in the 1310s).

**Clark's wool against wheat, by decade (1290–1309 = 1):**
- 1300 1.19, 1310 0.76, 1320 1.05, 1330 0.93, 1340 0.87, 1350 0.57, 1360 0.70, 1370 0.88, 1380 0.94, 1390 0.83;
- 1400 0.87, 1410 0.83, 1420 0.86, 1430 0.71, 1440 0.86, 1450 0.54, 1460 0.70, 1470 0.54, 1480 0.62, 1490 0.59;
- 1500 0.49, 1510 0.66, 1520 0.47, 1530 0.46, 1540 0.50, 1550 0.42, 1560 0.45, 1570 0.47, 1580 0.39, 1590 0.35.

(The model held 0.75 from 1400.) **Clark's wool keeps falling against wheat after 1450,** while Munro's cloth, deflated by the same wheat, rises 1.37–1.66 times from 1490.

## Why the gate was not run
- The series widens the gap D152 meant to close.
- The gate run was stopped after a few minutes. A 9-run exploration was run instead: maps 1–3 × seeds 1–3, `{"wool_grades", "putting_out", "wool_series"}` on, `target/d2/rep152`, `pace152`.
- **These are exploration results, not a gate.**

| Decade | Wool's port price, clip mean (d a lb) | Good | Middle | Refuse | Cloth value | Cloth netback (median mill bid) | Raw good wool (lb) | Cloth share |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1350 | 2.06 | 2.82 | 1.98 | 1.38 | 5.70 | 3.00 | 85,600 | 0.15 |
| 1370 | 2.82 | 3.86 | 2.70 | 1.89 | 5.70 | 3.00 | 79,000 | 0.56 |
| 1400 | 2.84 | 3.89 | 2.72 | 1.91 | 5.70 | 3.00 | 66,200 | 0.51 |
| 1430 | 2.46 | 3.37 | 2.36 | 1.65 | 5.70 | 3.00 | 62,700 | 0.52 |
| 1450 | 2.00 | 2.74 | 1.92 | 1.34 | 5.75 | 3.04 | 64,600 | 0.57 |
| 1470 | 1.89 | 2.59 | 1.81 | 1.27 | 6.27 | 3.45 | 55,200 | 0.60 |
| 1480 | 1.99 | 2.73 | 1.91 | 1.34 | 6.04 | 3.51 | 7,100 | 0.90 |
| 1490 | 1.92 | 2.63 | 1.84 | 1.29 | 7.71 | 4.85 | 90 | 0.99 |
| 1500 | 1.72 | 2.36 | 1.65 | 1.15 | 8.70 | 5.95 | 0 | 1.00 |
| 1540 | 1.59 | 2.18 | 1.52 | 1.07 | 9.47 | 6.76 | 0 | 1.00 |

**Home wool against need:**

| Year | Use | Need |
| --- | --- | --- |
| 1400 | 65.5k | 69.3k |
| 1450 | 42.1k | 63.8k |
| 1500 | 36.0k | 82.9k |
| 1600 | 49.0k | 127.2k |

**Coin in 1600:** 278k (D144 113k, 2.5 ×).

**Read:**
- **To the 1440s,** good wool's port price (3.4–4.2 d) is above cloth's netback (3.0 d). Good wool leaves raw and the share holds at 0.49–0.60. The 1440–60 band is met in this exploration.
- **From 1470,** Clark's falling wool and Munro's rising cloth cross. Every grade goes to cloth by 1490, and the share is 1.0 from 1500.
- **Against D150's pre-registrations, read from exploration only (not a gate):**
  - below 0.95 every decade: no, from 1490;
  - 0.55–0.85 in 1490–1510: no;
  - coin at most 1.5 × D144's: no;
  - raw wool at least the good grade's share: no, from 1480.

  A 40-seed gate would be expected to fail at least the rank-size check of D151.
