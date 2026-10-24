# D170: the free-land growth rule. Exploration shows it would fail; no gate

## Built (all behind switches, off; golden `5de3d525bb23a427` unchanged)
- **Stability's exemptions (validate, D170):** every calendar outbreak and the year after, D108's list or the sourced one in use, beside 1348, 1361, 1369 and the famine. **This is live in validate now.**
- **`ever_owned`:** cells ever claimed as fields, written by cultivation under `free_land_growth`.
- **`free_land_growth`:** natural growth at most r_grow_pc (1%) + (r_grow 2.7% − 1%) × the free share of the place's field order (farmable cells never cleared). This is D170's spec.
- **`free_land_region`** (an exploration variant, off): the free share is the region's farmable land never cleared, not each place's reach.

## Exploration: 9 runs (maps 1–3 × seeds 1–3), `landwork --from 0`, `target/d2/fl170` (not a gate)
All with `plague_sourced`.

| Setting | People 1300 | Peak (1340) | 1450 / 1340 (range) | Trough / peak | 1590s / peak (range) | Recovery 1480–1590 |
| --- | --- | --- | --- | --- | --- | --- |
| Default (D108, endemic 0.015, no rule) | 54.1k | 62.4k | 0.55 (0.48–0.65) | 0.53 | 1.30 (1.16–1.40) | 0.77% |
| Sourced calendar, endemic 0.015 (D167) | — | — | 0.49 (0.42–0.58) | 0.38 | 1.04 | 0.91% |
| **D170 spec:** rule (reach), endemic 0.007 | **32.3k** | **40.1k** | 0.41 (0.36–0.46) | **0.29** | 0.76 (0.54–1.00) | 0.87% |
| Rule (region), endemic 0.007 | 53.7k | 61.4k | 0.77 (0.71–0.81) | 0.63 | 1.29 (1.15–1.47) | 0.64% |
| Rule (region), endemic 0.015 | 53.7k | 61.3k | 0.45 (0.40–0.50) | 0.34 | 1.03 (0.89–1.23) | 0.99% |

**D170's spec: validate at 10 seeds a map (exploration, `a170_10.txt`).**
- **Market spacing:** 0/10, 1/10 and 1/10 (medians 21.6 / 22.4 / 34.6 km).
- **Recovery by 500:** 9/10, 2/10 and 2/10.
- **Also:** largest share 4/10 (map 2) and 5/10 (map 3); failure rate 3/10 and 5/10; top-10 vs off 6, 6 and 4/10; road length 5/10 (map 1).
- **Runs passing every check:** 0 / 0 / 1.
- **A 40-seed gate would fail on every map,** so it was not run.

## Why (fact from the tables; the reading is mine)
**The reach version:**
- A place's field order overlaps its neighbours' land. Its free share falls while the region is still filling, so villages slow to 1% long before 1348.
- Colonisation loses about 40% (1300: 32k against 54k), and too few places reach 500: spacing 22–35 km.
- With the settled rate after 1348 and endemic at 0.007, the trough (0.29) is too deep for the smaller region.

**The region version:**
- The region never clears all its farmable land: marginal cells stay free. The free share stays well above 0 after 1348, and births run at about 1.5–2%, not 1%.
- **With endemic at 0.007,** the trough is far too shallow (0.63).
- **With 0.015,** it is about the sourced calendar alone. The rule barely binds, and the recovery (0.99%) is as fast as before.

**So in this model, land alone cannot separate the frontier's growth from the recovery's.** The pre-plague colonisation and the post-plague regrowth come from the same `r_grow`:
- a local measure of free land slows colonisation;
- a regional one does not bind after the plague.

## What is left, for the designer
1. **The sourced calendar with endemic 0.015** (D167) gets the trough right: 0.34–0.38, with 1450/1340 0.45–0.49. Its recovery is too fast (0.91–0.99%), but 1600 lands at about 1.03, inside the gated 0.75–1.15.
   - **Its gate (D168) failed on stability, now fixed, and on spacing, roads and map 1's recovery.**
   - **A re-gate with the stability fix** would show which of those remain. Map 3's spacing is the likeliest.
2. **The recovery rate:** the declined date switch (a settled rate from the first great mortality) is the one form the model can express. D165's ceiling and D170's free-land rule both failed for want of it. **I'd ask you to reconsider it,** on this evidence. The reason is the model's own: its frontier growth and its recovery share one parameter.
3. **Or accept the recovery rate as a known miss** (0.9–1.0% against 0.59%), since 1600 sits inside the gated band.

**My recommendation:** re-gate the sourced calendar (endemic 0.015) with the stability fix now. It is the sourced change that the trough analysis supports. If it fails only on map 3's spacing, report that paired against 56/80 as D170 says. The recovery rate waits for your decision on 2 or 3.
