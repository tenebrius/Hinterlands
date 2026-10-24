# D156: Clark's wool series alone. Gate failed on map 3's market spacing; stopped

## Built
`wool_series` (D152; off): the port's wool price follows Clark's wool against wheat by decade (1290–1309 = 1), in place of `wool_fall`. Grades and putting-out are off. Details of the series: `docs/demand/d2l.md`.

## Exploration (9 runs; `target/d2/rep156`, `pace156`; not a gate)
**The cloth share by decade** (series / D144):

| Decade | Series | D144 |
| --- | --- | --- |
| 1420 | 0.19 | 0.17 |
| 1450 | 0.20 | 0.15 |
| 1480 | 0.20 | 0.17 |
| 1500 | 0.45 | 0.35 |
| 1510 | 0.47 | 0.40 |
| 1520 | 0.49 | 0.48 |
| 1530 | 0.48 | 0.51 |
| 1540 | 0.48 | 0.58 |
| 1550 | 0.50 | 0.59 |
| 1560 | 0.52 | 0.58 |
| 1570 | 0.53 | 0.56 |
| 1580 | 0.55 | 0.55 |
| 1600 | 0.55 | 0.53 |

**Also:**
- raw wool to the ports 47–98k lb a year, in every decade;
- home wool at need;
- coin in 1600: 101k (D144 113k);
- people in 1600: 79.7k (D144 84.3k).

## Gate: 40 seeds, `{"wool_series": true}`. **FAILED on one check**
`target/d2/v156.txt`.

| Check | Map 1 | Map 2 | Map 3 | D144 |
| --- | --- | --- | --- | --- |
| **market spacing** | 33/40, 11.5 km | 38/40, 12.4 | **25/40, 14.1** | 34 / 37 / **30**, 11.7 / 12.2 / 14.1 |
| rank-size slope | 40/40, −0.954 | 40/40, −0.999 | 40/40, −1.005 | 37 / 40 / 40 |
| road length 300–500 | 32/40 | 37/40 | 30/40 | 32 / 38 / 35 |
| top-10 vs off | 37/40 | 39/40 | 36/40 | 39 / 40 / 34 |
| exports / output | 0.077 | 0.085 | 0.074 | 0.084 / 0.094 / 0.088 |
| runs passing every check | 24/40 | 33/40 | 15/40 | 22 / 35 / 20 |
| over 2,000 (reported), median | 6 | 6 | 4 | 6 / 6 / 5 |
| urban share (reported), median | 0.376 | 0.392 | 0.395 | 0.363 / 0.371 / 0.391 |

- Every other gated check is 30/40 or better.
- **Map 3's spacing fails on the wide side.** Its median is unchanged (14.1 km), but more runs pass 15 km: 15 failures, 10 of them the run's only failure.
- **D144 had it exactly at the floor (30/40),** the watch item since D144. Over the last builds it has moved 34 → 30 → 25.

## Pre-registered (D156), reported only
| | Result | Met |
| --- | --- | --- |
| Cloth share after 1500 at or above D144's − 0.05 | Above or within 0.05 in 1500–30 and 1570–1600; 0.48–0.52 against 0.58–0.59 in 1540–60 | **no** in 1540–60 (exploration) |
| Map 1's rank-size 30/40 or better | 40/40 | **yes** |
| Raw wool leaving in every decade | 47–98k lb | **yes** |

## Reading (not traced)
- The cheaper wool after 1450 lowers wool income in the pastoral, outer districts. Fewer of their places reach 500 people, so the nearest-neighbour distance among places over 500 widens on map 3, whose geography already puts it near 15 km.
- **This is not checked.** A trace of map 3's failing runs (which places drop below 500, and where) would show it.
