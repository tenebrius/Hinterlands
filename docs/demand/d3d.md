# D168: the sourced plague calendar, gated. FAILED; stopped

## Validate change (D168)
`recovery by 500` is gated at 0.75–1.15 × the pre-plague peak, replacing ≥ 0.90.

## Gate: 40 seeds, `{"plague_sourced": true}`, endemic 0.015. **FAILED**
`target/d2/v168.txt`. In brackets, the default (D161; `v156.txt`, seeds 1–40).

| Check | Map 1 | Map 2 | Map 3 |
| --- | --- | --- | --- |
| **stability (< 5%/yr)** | **0/40**, max 0.093 (40/40) | **0/40**, 0.091 (39/40) | **0/40**, 0.092 (37/40) |
| **market spacing** | **26/40**, 14.3 km (33/40) | 34/40, 12.8 (38/40) | **17/40, 15.3** (25/40; reference 56/80) |
| **road length 300–500** | **27/40** (32/40) | 37/40 (37/40) | **27/40** (30/40) |
| **recovery by 500 (0.75–1.15)** | **28/40**, median 1.09 | 33/40, 1.08 | 36/40, 0.99 |
| top-10 vs off | 34/40 | 38/40 | 34/40 |
| largest share | ≥ 39 | ≥ 39 | 36/40 |
| runs passing every check | 0/40 | 0/40 | 0/40 |

- **v4 (reported):** plague drop 0.38; recovery 1.09 / 1.07 / 0.99; total at end against shocks off 0.77 / 0.75 / 0.67.
- Every other gated check is 34/40 or better.

## 1. Stability: a measurement gap, not the model
- **Stability (D58) exempts** the event years and the year after: the famine, 1348, 1361 and 1369 (`validate.rs`, `shocked`). **It does not exempt the later outbreaks** (D108's `LATER_PLAGUES`, nor `SOURCED_PLAGUES`).
- **Under D108 this went unseen.** 10% falling to 6%, over `plague_years` 2, gives about 5% a year or less, and the default's maximum is 0.044–0.049.
- **Under the sourced calendar,** 1457 (0.189) and 1471 (0.139) give about 9.5% and 7% in a year, so every run fails.
- **Proposed fix, consistent with D58:** exempt every calendar outbreak and the year after. Not applied: it is a change to a gated check, so it is yours to decide.

## 2. The real failures
- **Map 3's spacing** is 17/40 (median 15.3 km, too wide), against the reference of 56/80 (about 28/40).
- **Map 1's spacing** is 26/40.
- **Road length** is 27/40 on maps 1 and 3.
- **Recovery by 500 on map 1** is 28/40: its median is 1.09, so 12 runs are above 1.15.

**Reading (not traced):**
- A deeper trough (0.38 of the peak) and 13 outbreaks thin the places over 500 through the 1400s. Spacing widens, and roads lose load and fade (300–500 change).
- The fast recovery (0.91%) then overshoots 1.15 on map 1.
- **Both point to the recovery rate:** the trough is now right and the recovery is too fast, which is the next note (option 1).

## Pre-registered (D168), reported only (from the D167 exploration, 9 runs; the gate does not report them)
| | Result | Met |
| --- | --- | --- |
| Trough 0.33–0.48 | 0.38 | yes |
| 1450 / 1340 within 0.40–0.65 | 0.49 | yes |
| Recovery 1480–1590 against 0.59% a year | 0.91% | above the record |

## Proposal
1. **Fix stability's exemptions** (all calendar outbreaks).
2. **Take the recovery-rate rule** (`docs/proposals/recovery-rate.md`) together with the sourced calendar, since the calendar alone over-recovers and thins the middle.
3. **Re-gate them together.**
