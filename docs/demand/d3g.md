# D176: the paired re-read of the default, and the sourced calendar's gate. FAILED; stopped

## Validate (D176)
- **Road persistence** is measured on routes: a year-300 road cell is kept if road lies within 2 cells of it in 1600. Gated at ≥ 0.90 a run. The strict cell measure is reported.
- **Market spacing** (market places, W ≥ 100) and road persistence are **paired no-worse checks:** on seeds 1–40 a map passes if its count is at least the default's count minus 2. The default's counts (D161, read now) are in `PAIRED`:
  - spacing 25 / 35 / 25;
  - roads 23 / 35 / 17.

## The default (D161), 40 seeds (`v176_def.txt`): the reference
| | Map 1 | Map 2 | Map 3 |
| --- | --- | --- | --- |
| market spacing, market places | 25/40, median 13.6 km | 35/40, 13.0 | 25/40, 14.4 |
| road persistence ±2 | 23/40, median 0.903 | 35/40, 0.920 | 17/40, 0.894 |
| road persistence, cells (reported) | 0.551 | 0.568 | 0.536 |
| spacing, over 500 (reported) | 33/40, 11.5 | 38/40, 12.4 | 25/40, 14.1 |
| road length 300–500 (reported) | 32/40, 0.058 | 37/40, 0.059 | 30/40, 0.074 |
| recovery by 500 (0.75–1.15) | **5/40**, 1.25 | **4/40**, 1.29 | **15/40**, 1.21 |
| stability | 40/40, 0.026 | 40/40 | 40/40 |

**With `PAIRED` filled,** the default passes spacing and roads by definition. **It fails recovery on every map,** as D176 accepted: validate stays red on the default.

## The gate: `plague_sourced`, endemic 0.015, the same seeds (`v176_ps.txt`). **FAILED**
| Check | Map 1 | Map 2 | Map 3 | Rule |
| --- | --- | --- | --- | --- |
| market spacing (paired) | 26/40 (ref 25) ✓, median 13.3 | 33/40 (35) ✓, 13.0 | **20/40 (25) ✗**, 15.0 | ≥ ref − 2 |
| **road persistence ±2 (paired)** | **8/40 (23) ✗**, median 0.869 | **13/40 (35) ✗**, 0.890 | **5/40 (17) ✗**, 0.855 | ≥ ref − 2 |
| **recovery by 500** | **28/40** ✗, 1.09 | 33/40, 1.08 | 36/40, 0.99 | 30/40 |
| stability | 40/40 | 40/40 | 40/40 | |
| spacing, over 500 (reported) | 26/40, 14.3 | 34/40, 12.8 | 17/40, 15.3 | |
| road length 300–500 (reported) | 27/40, 0.083 | 37/40, 0.072 | 27/40, 0.081 | |
| road persistence, cells (reported) | 0.524 | 0.546 | 0.525 | |

Every other gated check is 33/40 or better. Runs passing every check: 1 / 9 / 2 (the default's 0 / 3 / 1, from recovery).

## Read
- **Roads.**
  - The calendar lowers route persistence by about 0.03 (medians 0.869 / 0.890 / 0.855 against 0.903 / 0.920 / 0.894).
  - The 0.90 threshold sits at the default's median, so the counts collapse: 8 against 23, 13 against 35, 5 against 17.
  - **Reading:** the deeper trough (1400 is near it) and the recovery move traffic, and with it the routes.
  - **The paired count is very sensitive here** because the threshold sits on the default's own median. A paired comparison of medians, or of the distribution, would read the same 0.03 as a small change.
- **Map 3's spacing** is 20 against 25: three short of the rule.
- **Map 1's recovery** is the known miss (median 1.09, 12 runs above 1.15).

## Stopped
- `plague_sourced` stays off.
- `PAIRED` is recorded with the default's counts.
- Validate on the default fails only recovery (D176).
- **D176 says the band is revisited first if the gate fails.**
