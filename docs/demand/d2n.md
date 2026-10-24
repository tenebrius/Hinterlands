# D158 trace: map 3's market spacing under Clark's wool series (exploration, 20 seeds)

`hinterlands spacetrace --map-seed 3 --seed 1..20`, with and without `wool_series`, in `target/d2/st158`. **An exploration, not a gate.**

| | D144 (off) | Wool series (on) |
| --- | --- | --- |
| runs with spacing over 15 km | **5/20** (seeds 1–20) | **6/20** |
| places over 500 in 1600, median | 24 | 23 |
| median spacing (median of runs) | 14.1 km | 14.1 km |

**Per seed (over 500 / spacing km, off | on):**

| Seed | Off | On |
| --- | --- | --- |
| 1 | 24 / 12.7 | 21 / 16.3 |
| 2 | 24 / 15.6 | 23 / 14.1 |
| 5 | 21 / 16.0 | 27 / 15.8 |
| 7 | 29 / 12.2 | 21 / 15.6 |
| 10 | 18 / 15.5 | 17 / 14.6 |
| 11 | 24 / 14.4 | 23 / 17.1 |
| 14 | 25 / 13.9 | 23 / 15.5 |
| 15 | 32 / 14.1 | 27 / 16.1 |
| 16 | 24 / 17.0 | 27 / 14.1 |
| 19 | 23 / 15.8 | 29 / 11.4 |
| 20 | 21 / 14.9 | 29 / 7.1 |

**The places:**
- **Each pair of runs diverges from the start.** The series differs from `wool_fall` before 1205 too (1.046 in the 1200s), so the same seed grows a different settlement pattern.
- Of the 475 places over 500 in the D144 runs, **251 have no place over 500 within 3 cells under the series**, and 158 have no place of 200 or more there at all. **247 places over 500 are new** under the series.
- **The places lost are not distinct:**

| | Places lost (D144 values) | All D144 places over 500 | New under the series |
| --- | --- | --- | --- |
| people in 1600 | 650 | 727 | 665 |
| field yield / region's | 1.36 | 1.47 | 1.41 |
| pasture share | 0.00 | 0.00 | 0.00 |
| km to the largest place | 56 | 57 | 55 |
| km to the nearest place over 2,000 | 25 | 26 | 26 |
| wool coin, 1450–1600 | 2,278 | 2,590 | 1,489 |

- Where a counterpart exists within 3 cells (93 places), it has 386 people and **0.45 times** the D144 place's wool coin. That is the lower wool price.

**Read:**
- **My reading at D157 is not supported.** The places that fall out are not the outer pastoral districts: they have no pasture, good land and the usual distances.
- **The series does not thin map 3 measurably:** 6 failing runs against 5, and 23 places over 500 against 24.
- **The gate's 25/40 against D144's 30/40** looks like map 3's spacing check resampled. It sits at the 15 km edge (median 14.1 km in both), and 25 against 30 of 40 is about 1.2 standard deviations apart at a pass rate near 0.7.
