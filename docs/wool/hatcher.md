# Population against Hatcher (D112)

Population at each year over 1340's (year 240), Historical calendar at the L2 defaults, 20 seeds a map (median, with the 1450 range):

| | 1350 | 1400 | 1450 | 1500 | 1600 |
| --- | --- | --- | --- | --- | --- |
| Map 1 | 0.66 | 1.00 | **1.18** (1.04–1.34) | 1.40 | 1.64 |
| Map 2 | 0.65 | 1.00 | **1.19** (1.11–1.33) | 1.39 | 1.67 |
| Map 3 | 0.65 | 0.98 | **1.15** (0.97–1.28) | 1.32 | 1.63 |

- **Hatcher (1977):** population fell by between a half and two-thirds in the century after 1348, so 1450 stood at about 0.33–0.50 of 1340.
- **The model** loses a third in 1348–49 but is back to its 1340 level by 1400. It is 15–19% above it by 1450, despite the later outbreaks from 1361 to 1479.
- **Why:** after each outbreak, survivors have abundant land, and the model grows at up to `r_grow` a year on the surplus. The later outbreaks take 6–10% each, about once a decade, which the regrowth outpaces.
- **Historically, the low level persisted** because mortality stayed high between the national outbreaks: endemic plague and other disease, so-called background mortality (Hatcher 1977). Fertility may also have been low. The model has no background mortality and no fertility response.
- **What would close the gap** (the designer's decision):
  - stronger recurrences;
  - a background mortality between outbreaks, 1348–1480;
  - lower regrowth while plague is endemic.

## The endemic era (D114)
- **Built:**
  - In the Historical calendar, from 1349 to 1480, every place loses `endemic_mortality` a year on top of the outbreaks (Hatcher's between-outbreak mortality).
  - The murrain is reshaped after Slavin (2020), approximately: wool ×0.85 in 1315–22 and ×0.9 in 1323–32 (scab).
- **Tuning `endemic_mortality`** (1450 / 1340, two seeds a map):
  - 0.010: 0.69–0.86;
  - 0.013: 0.54–0.72;
  - **0.015: 0.48–0.64 (chosen)**;
  - 0.017: 0.42–0.54;
  - 0.020: 0.32–0.40.
- **Pre-registered, 1450 at 0.40–0.65 of 1340: met in 60/60 runs.** Medians 0.56 / 0.56 / 0.54, range 0.45–0.64; 1500 at 0.69–0.74.
- **v4's recovery check** (≥ 0.9 of the peak by year 500): passes 20/20 on every map, at 1.43–1.46. The plague loss is now regained in 185–198 years (79 before).

**`just validate`, 20 seeds a map: PASSED.**

| | Map 1 | Map 2 | Map 3 |
| --- | --- | --- | --- |
| Over 2,000 | 15/20 (at the threshold) | 19/20 | 16/20 |
| Spacing | 17/20 | 20/20 | 20/20 |
| Roads | 15/20 | 15/20 | 16/20 |
| Outside trade | 0.18, 20/20 | 0.21, 20/20 | 0.23, 20/20 |
| Pasture share, 1340 → 1450 | 0.163 → 0.249 | 0.139 → 0.281 | 0.184 → 0.272 |
| Wool's share of coin, before / after 1348 | 0.60 / 0.92 | 0.56 / 0.92 | 0.64 / 0.93 |
| Carry-over after cheap years (mean / max) | 0.12 / 0.38 | 0.27 / **0.44** | 0.21 / **0.45** |
| Famine dip / weather | 0.19 / 0.87 | 0.19 / 0.88 | 0.19 / 0.88 |
| Runs passing every check | 8/20 | 13/20 | 8/20 |

- **Knock-on effects:**
  - More pasture by 1450, as expected.
  - Towns over 2,000 hold at 15–19/20, so no lever was needed. Map 1 sits at the threshold.
  - Carry-over's maximum rises a little above Clark's 40% on maps 2 and 3. It is reported, not gated.
- **Golden `b4f78a0e4adf502b`;** release wasm gives the same hash at 45.4 ms/year.
