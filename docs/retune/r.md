# R: the robustness re-tune (D119)

## Diagnosis at the current setting (seeds 1–20)
- **Map 1:** 8 runs pass every check. Failing checks (runs failing / of which the only failure): over 2,000 5/3; road length 300-500 5/3; market spacing 3/2; every good worked 1/0; pasture outward 1/0; stability 1/1; top-10 vs off 1/0.
- **Map 2:** 13 runs pass every check. Failing checks (runs failing / of which the only failure): road length 300-500 5/3; top-10 vs off 2/2; every good worked 1/0; over 2,000 1/0.
- **Map 3:** 8 runs pass every check. Failing checks (runs failing / of which the only failure): top-10 vs off 5/2; over 2,000 4/2; road length 300-500 4/2; stability 2/1; failure rate 1/0; pasture outward 1/0; settlements 1/0.

## Screen, one lever at a time (seeds 1–20)

| Setting | Map 1 | Map 2 | Map 3 | Min | Total | Gated checks < 15/20 | Most-failed check (map 1 / 2 / 3) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| road_keep_load=700 | 12 | 11 | 10 | 10 | 33 | 1 | over 2,000 3/2 / road length 300-500 3/3 / over 2,000 7/1 |
| k_maint=1.2 | 13 | 14 | 9 | 9 | 36 | 0 | over 2,000 4/1 / road length 300-500 5/3 / market spacing 4/2 |
| k_maint=1.4 | 9 | 12 | 10 | 9 | 31 | 1 | road length 300-500 6/5 / road length 300-500 5/4 / over 2,000 5/0 |
| shadow_drain=0.055 | 10 | 10 | 9 | 9 | 29 | 3 | road length 300-500 6/2 / over 2,000 6/4 / road length 300-500 6/2 |
| k_maint=1.25 | 9 | 13 | 8 | 8 | 30 | 2 | road length 300-500 8/4 / over 2,000 3/2 / over 2,000 5/4 |
| current | 8 | 13 | 8 | 8 | 29 | 0 | over 2,000 5/3 / road length 300-500 5/3 / top-10 vs off 5/2 |
| shadow_drain=0.04 | 10 | 11 | 8 | 8 | 29 | 3 | over 2,000 6/2 / road length 300-500 5/4 / over 2,000 6/3 |
| market_share=0.30 | 9 | 10 | 8 | 8 | 27 | 2 | over 2,000 7/4 / road length 300-500 6/4 / outside trade 5/3 |
| road_keep_load=450 | 11 | 15 | 7 | 7 | 33 | 1 | over 2,000 5/2 / market spacing 2/2 / over 2,000 6/3 |
| road_keep_load=525 | 10 | 14 | 7 | 7 | 31 | 0 | over 2,000 4/2 / road length 300-500 4/3 / market spacing 5/3 |
| k_maint=1.35 | 10 | 13 | 7 | 7 | 30 | 3 | road length 300-500 7/3 / road length 300-500 5/5 / over 2,000 6/4 |
| market_share=0.40 | 9 | 14 | 7 | 7 | 30 | 2 | over 2,000 9/1 / road length 300-500 4/3 / over 2,000 5/3 |
| shadow_drain=0.035 | 12 | 11 | 7 | 7 | 30 | 2 | road length 300-500 5/3 / settlements 6/2 / settlements 6/1 |
| market_share=0.375 | 7 | 11 | 10 | 7 | 28 | 2 | over 2,000 7/2 / road length 300-500 5/4 / road length 300-500 5/4 |
| toll=0.075 | 9 | 12 | 7 | 7 | 28 | 2 | over 2,000 8/6 / market spacing 3/1 / top-10 vs off 8/4 |
| market_min=60 | 8 | 11 | 7 | 7 | 26 | 4 | over 2,000 7/3 / road length 300-500 6/5 / over 2,000 9/3 |
| toll=0.05 | 11 | 8 | 7 | 7 | 26 | 1 | over 2,000 4/2 / road length 300-500 8/6 / over 2,000 5/2 |
| road_keep_load=800 | 7 | 11 | 7 | 7 | 25 | 4 | road length 300-500 12/8 / road length 300-500 6/5 / road length 300-500 9/2 |
| market_share=0.325 | 9 | 9 | 6 | 6 | 24 | 1 | market spacing 4/3 / road length 300-500 4/3 / over 2,000 6/0 |
| toll=0.15 | 6 | 9 | 6 | 6 | 21 | 4 | over 2,000 8/3 / road length 300-500 7/6 / over 2,000 6/4 |
| market_min=80 | 9 | 13 | 5 | 5 | 27 | 2 | market spacing 5/2 / over 2,000 3/2 / over 2,000 7/3 |
| market_min=150 | 8 | 16 | 4 | 4 | 28 | 3 | road length 300-500 6/4 / road length 300-500 2/1 / top-10 vs off 8/3 |
| shadow_drain=0.05 | 8 | 12 | 4 | 4 | 24 | 2 | over 2,000 5/2 / over 2,000 4/2 / over 2,000 8/5 |
| market_min=120 | 8 | 11 | 4 | 4 | 23 | 3 | over 2,000 6/0 / road length 300-500 6/6 / over 2,000 7/4 |
| toll=0.125 | 9 | 16 | 3 | 3 | 28 | 2 | over 2,000 7/6 / failure rate 1/0 / over 2,000 8/5 |

## Joint grid: `k_maint` × `road_keep_load` (seeds 1–20)

`k_maint` 1.2, the screen's best feasible value, sat at the edge of its range, so the grid extends it down to 1.1. `road_keep_load` 700 had the screen's highest minimum but failed a gated check.

| Setting | Map 1 | Map 2 | Map 3 | Min | Total | Gated checks < 15/20 | Most-failed check (map 1 / 2 / 3) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| road_keep_load=700 | 12 | 11 | 10 | 10 | 33 | 1 | over 2,000 3/2 / road length 300-500 3/3 / over 2,000 7/1 |
| k_maint=1.2 | 13 | 14 | 9 | 9 | 36 | 0 | over 2,000 4/1 / road length 300-500 5/3 / market spacing 4/2 |
| current | 8 | 13 | 8 | 8 | 29 | 0 | over 2,000 5/3 / road length 300-500 5/3 / top-10 vs off 5/2 |
| k_maint=1.1_road_keep_load=700 | 8 | 10 | 10 | 8 | 28 | 3 | road length 300-500 9/4 / road length 300-500 8/5 / market spacing 5/3 |
| k_maint=1.15_road_keep_load=700 | 9 | 8 | 8 | 8 | 25 | 3 | road length 300-500 10/5 / over 2,000 6/4 / over 2,000 7/0 |
| k_maint=1.2_road_keep_load=700 | 11 | 7 | 8 | 7 | 26 | 3 | road length 300-500 6/3 / road length 300-500 11/9 / road length 300-500 6/4 |
| k_maint=1.1_road_keep_load=650 | 10 | 11 | 6 | 6 | 27 | 4 | road length 300-500 7/3 / road length 300-500 4/2 / road length 300-500 7/2 |
| k_maint=1.2_road_keep_load=650 | 9 | 13 | 5 | 5 | 27 | 2 | road length 300-500 6/2 / road length 300-500 4/3 / market spacing 8/4 |
| k_maint=1.15 | 8 | 11 | 5 | 5 | 24 | 3 | road length 300-500 6/2 / over 2,000 5/3 / over 2,000 6/0 |
| k_maint=1.1 | 5 | 8 | 6 | 5 | 19 | 4 | road length 300-500 11/4 / road length 300-500 8/4 / over 2,000 6/3 |
| k_maint=1.15_road_keep_load=650 | 4 | 8 | 4 | 4 | 16 | 4 | road length 300-500 10/3 / road length 300-500 5/4 / over 2,000 9/2 |

- **The grid found nothing better than `k_maint` 1.2 alone.**
  - Every combination with `road_keep_load` 650 or 700 puts at least one gated check below 15/20, mostly road length.
  - `k_maint` below 1.2 is worse on every map.
- **Chosen for confirmation:** `k_maint` 1.2. Minimum 9, total 36, no gated check below 15/20. The current setting has minimum 8, total 29.

## Confirmation on fresh seeds (21–40)

| Setting | Map 1 | Map 2 | Map 3 | Min | Total | Gated checks < 15/20 |
| --- | --- | --- | --- | --- | --- | --- |
| current | 11 | 14 | 8 | **8** | **33** | 2: road length on map 1 (14/20), over 2,000 on map 3 (14/20) |
| `k_maint` 1.2 | 7 | 13 | 9 | 7 | 29 | 1: road length on map 1 (12/20) |

- **`k_maint` 1.2 does not beat the current setting on the fresh seeds** (minimum 7 against 8, total 29 against 33). By the pre-registered rule, **the current setting is kept.**
- **Pre-registered** (12 or more on every map on seeds 1–20, and fewer on seeds 21–40): **not met.** No setting reached 12 on every map; the best minimum on seeds 1–20 was 10 (`road_keep_load` 700, which fails a gated check). The second half holds: the chosen setting's minimum fell from 9 to 7 on fresh seeds.
- **Finding 1: run-to-run noise dominates.** At 20 seeds a map, neighbouring settings differ by 2–5 runs passing every check, with no consistent direction. For example, `shadow_drain` gives minimums of 7, 8, 8, 4 and 9 from 0.035 to 0.055. The six tunables have no setting that is robustly better than the current one.
- **Finding 2: the gate's margin is thin.** On seeds 21–40 the current setting has two gated checks at 14/20, one short of 15:
  - road length on map 1;
  - over 2,000 on map 3.

  The gate passes on seeds 1–20, but its margin is about one seed in twenty for those two checks.
- **The binding checks** throughout are road length 300–500, over 2,000, top-10 vs off and market spacing. A real gain in robustness would need changes to the mechanisms behind them (road persistence, the hierarchy's middle), which is outside R's levers.

## At the kept (current) setting, reported (seeds 1–20)
- **Famine dip:** 0.19 on every map.
- **Store carry-over** after cheap years (mean / max): 0.12 / 0.38, 0.27 / 0.44 and 0.21 / 0.45.
- **Clark** (map 1 / 2 / 3): `b_P` 1.03 / 1.04 / 1.04; `b_Y` −0.03 / −0.03 / −0.03; R² 0.91 / 0.92 / 0.95.
- **Hatcher, 1450 / 1340:** 0.54–0.56 (as at D114; the setting is unchanged).
- **Kent:** unchanged from the close (`docs/real/kent-geology.md`).
- **Golden:** unchanged, `65aee272c30de1f9`.

## Tools
- `validate --first-seed N` runs seeds N to N+19; the determinism check reruns the first seed run.
- `validate` now prints, per map, the failing checks with how many runs fail each and in how many it is the only failure.
- `tools/retune/screen.sh` and `tools/retune/summary.py`.
