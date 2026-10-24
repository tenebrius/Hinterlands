# D142: graphics merge checked; exports over output by value added; the later cloth price the default

## 0. The graphics merge (D142, D143)
- **R9–R32 are merged into `main` (bcf947f), PR #1.** Frontend only.
- **Golden `2e6856056e73a1c6` unchanged,** natively and in `?selftest=500`.
- **`just test` passes.**
- **`just validate` on 40 seeds is identical** to D140's run, line for line apart from timings (`target/d2/v142_merge.txt` against `v139_step1.txt`).

## 1. Exports over output, cloth by value added (D141 step 1)
- **The change, in the output denominator:** cloth counts as its value less the wool it uses, at the port wool price (`cloth in − milled lb × wool's port price`). The whole clip is still counted at the port price, so milled wool is counted once.
- **Exports are unchanged:** what the ports paid for grain, wool and cloth.
- **The model is unchanged** (validate only).

| Medians, 1600 | Map 1 | Map 2 | Map 3 |
| --- | --- | --- | --- |
| before (D140) | 0.066 | 0.078 | 0.073 |
| **by value added** (`target/d2/v141_default.txt`) | **0.067** | **0.079** | **0.074** |

- **Every run is inside 1–35%** (40/40 on each map), and every other line is unchanged.
- **Reading:** the correction is small, because milled wool is about 0.2–0.4 of the clip and cloth's value added is about half its value.

## 2. The later cloth price (`cloth_price_late`), gated on 40 seeds: **PASSED**
`target/d2/v141_late.txt`.

| | Map 1 | Map 2 | Map 3 | Default before (v141_default) |
| --- | --- | --- | --- | --- |
| exports / output, median (1–35%) | 0.084, 40/40 | 0.094, 40/40 | 0.088, 40/40 | 0.067 / 0.079 / 0.074 |
| old outside share (reported), median; inside 3–50% | 0.504; 19/40 | 0.526; 5/40 | 0.502; 18/40 | 0.425 / 0.457 / 0.432; 39 / 39 / 40 |
| lowest gated checks | road length 32/40, spacing 34/40, pasture 1450>1340 34/40 | settlements 39/40, spacing 37/40 | **spacing 30/40**, top-10 vs off 34/40, road length 35/40 | spacing 33 / 39 / 34 |
| runs passing every check | 22/40 | 35/40 | 20/40 | 23 / 37 / 25 |
| over 2,000 in 1600, median (range) | 6 (3–9) | 6 (4–8) | 5 (3–8) | 6 / 7 / 5 |
| urban share in 1600, median | 0.363 | 0.371 | 0.391 | 0.339 / 0.354 / 0.360 |

- **Map 3's market spacing is at the floor, 30/40.** Of its 10 failures, 8 are the run's only failure.
- **It is now the default.** Golden `fc16de69c1855b30`, the same in the browser (`?selftest=500`, 100.9 ms a year).

## Pre-registered (D139), reported only
| | Result | Met |
| --- | --- | --- |
| Exports over 1600 output between 5% and 25% on every map | Medians 0.084 / 0.094 / 0.088 with the later price; 0.067 / 0.079 / 0.074 without it. The per-run range is not printed. | **yes**, in the medians |
| The cloth share reaches the record's level about 40 years late, as in D138 | The same model as D138's run (`target/d2/rep137`, 9 runs): 1490 0.33, 1500 0.35, 1510 0.40, 1520 0.48, 1530 0.51, 1540 0.58, 1550 0.59. The record is 0.55–0.85 in 1490–1510. | **yes**: reached in the 1540s |

The decade tables (capacity, mill towns' W, coin by channel) are those of `docs/demand/d2f.md`, the same model.
