# D162: the labour-led land rule (exploration, 9 runs)

## Built (behind `labour_land`, off; golden `5de3d525bb23a427` unchanged)
**The rule** (`systems/cultivation.rs`):
- fields are claimed to need × `cult_margin` as before;
- but released only when they exceed what the farmers can work: `acres_per_farmer` × (people − W);
- so survivors keep land the need rule would drop.

**`acres_per_farmer` = 6.18** = 247.1 / `arable_labour` (40 people a km² of arable, L0's 39–45). The virgate household gives about 6.7 (30 acres, 4.5 people). **No new calibration:** the value is the one the model already uses for labour on arable.
- **Implementer's reading:** D162 asked for a start calibration (1300 arable per head unchanged). Under a release-only rule 1300 is unchanged by construction only if no place reaches the cap while growing, so that calibration cannot pin the value. The sourced labour figure is used instead.

**D3's income now counts own food uncapped** (D162; `wealth_stocks` stays off).

## Exploration: 9 runs (maps 1–3 × seeds 1–3), `landwork` probe, `target/d2/ll162` (not a gate)
| | Rule off (default) | Rule on |
| --- | --- | --- |
| Arable acres a head in 1300 | 3.71 | 3.53 |
| Arable a head, 1450 / 1300 | 1.00 | **0.89** |
| Arable a head, 1500 / 1300 | 0.93 | 0.84 |
| Arable a head, 1590 / 1300 | 0.93 | 0.86 |
| Output (own harvest) a head, 1300 / 1450 / 1500 / 1590 | 1.10 / 1.17 / 1.10 / 0.97 | 1.11 / 1.16 / 1.06 / 0.99 |
| Hatcher, people 1450 / 1340 | 0.55 | 0.52 |
| People in 1590s / pre-1348 peak | 1.30 | **1.07** |
| Trough / peak (1350–1490 low against 1300–40 high) | 0.53 | 0.50 |
| People, 1300 / 1340 / 1450 / 1500 | 54.1k / 62.4k / 34.0k / 44.6k | 46.1k / 52.5k / 27.6k / 34.3k |
| Total arable, 1300 / 1450 (k acres) | 199 / 127 | 164 / 88 |
| Places (people) at full growth, 1300 | 0.24 (0.13) | 0.17 (0.14) |
| 1360 | 0.43 (0.54) | 0.36 (0.48) |
| 1400 | 0.36 (0.44) | 0.23 (0.43) |
| 1450 | 0.15 (0.21) | 0.07 (0.09) |
| 1500 | 0.04 (0.03) | 0.01 (0.01) |
| Pasture share of used land, 1340 / 1450 | 0.210 / 0.275 | 0.292 / 0.277 |

**The test (arable a head in 1450 at × 1.5–1.65): missed.** It is 0.89 with the rule (1.00 without).

**Why (fact, from the totals):**
- **The model's farmers already work about 5 acres each in 1300** (3.7 acres a head with W about a quarter to a third). That is near the cap of 6.18.
- **After the plague the cap binds at once,** so land falls with the farmers, as it fell with need before.
- **During growth the cap also trims the fields of the larger places,** so the whole region is smaller: 46k against 54k people in 1300.

**Reading:**
- The model's land per farming person in 1300 is already about the record's post-plague level. Broadberry's arable a head was 2.44 acres in 1300 and 4.02 in 1450, with non-farmers counted. The model's lower yield per acre makes up the difference.
- **So the rule can only raise land per head with `acres_per_farmer` well above the virgate,** about 9–10 for × 1.5. That would be fitting it to the test.
- The rule does reduce the 1600 overshoot (1.07 against 1.30), through a smaller region that grows more slowly, not through land per worker.

**The real question:**
- In the model, yield per acre and people per prime cell set land per head at 1300 near the record's post-plague level.
- Raising land per worker after 1348 means pre-plague land per head must be lower, as in the record: higher yields, or smaller holdings before the plague.
- That touches `people_per_prime_cell` and the yield raster, calibrated since v1. **I would not do it without a note of its own.**
