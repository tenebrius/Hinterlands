# D165: the growth ceiling (b′). Exploration shows the region never fills; no gate

## Built (behind `growth_ceiling`, off; golden `5de3d525bb23a427` unchanged)
- In growth's natural term, births answer the food ratio averaged over ten years (`ratio_avg`), at most `r_grow_pc` = 1.0% a year, in all eras.
- Migration, founding, losses and `endemic_mortality` are unchanged.

## Exploration: 9 runs (maps 1–3 × seeds 1–3), `landwork --from 0`, `target/d2/gc165` (not a gate)
| | Off (default) | On |
| --- | --- | --- |
| People 1200 / 1250 / 1300 / 1340 (medians) | 6,593 / 21,510 / 54,104 / 62,438 | **1,278 / 2,224 / 3,704 / 4,872** |
| Peak year (pre-1348) | 1340 (1330–1340) | 1340 (all) |
| Hatcher 1450 / 1340 (gated band 0.33–0.50) | 0.55 | **0.22** |
| Trough / peak | 0.53 | 0.16 |
| 1590s / peak | 1.30 | 0.61 |
| Recovery 1480–1590, % a year | 0.77 | 1.21 |

**Against D165's pre-registrations (exploration only):**
- 1600 at most 1.15 × peak: 0.61, met in form, but on a region of 5k;
- recovery 0.5–0.75% a year: 1.21, missed;
- Hatcher in band: 0.22, missed;
- the peak year within 20 years: 1340, met.

**The peak's people fall from 62k to 4.9k.** A gate would fail settlements, over 5,000, spacing and Hatcher on every map, so I did not run it (the owner's rule).

## Why (fact and reading)
- **The model's region starts almost empty.** About 1,700 people in 1150 fill to about 62k by 1340: 37 times in about 200 years, 1.5% a year on average and faster on the frontier.
- That is the model's colonisation, and `r_grow` (2.7%) was set for it. **England's national rates (0.27% in 1270–1300) are those of a settled country,** not of a frontier region filling from hamlets. At 1% the region reaches only about 5k by 1340.
- **After 1348, `endemic_mortality`** (tuned at D111 against 2.7% growth between outbreaks) outruns 1% births. The trough falls to 0.16 and Hatcher to 0.22.

## Options for the designer
1. **A ceiling that depends on the land, not the date:** full `r_grow` where land is free (a frontier fills by colonists' high fertility and in-migration), `r_grow_pc` where a place's land is taken (settled). This is a density switch, which follows the mechanism. Its calibration is what counts as free land.
2. **Start the region nearer its settled density** (Domesday-like, about 40% of the 1300 peak in 1086), so growth to 1300 needs only about 0.5% a year. This is a change to the start and to founding, and it is larger.
3. **Drop (b′),** and take up the trough instead: Hatcher's 1450/1340 at 0.55 is at the top of its band, and it is two thirds of the overshoot.

**My recommendation: 3 now.** The trough is the larger part, and it has a gated, sourced target (Hatcher). Option 1 is the right form for the ceiling if the recovery part is taken up later.
