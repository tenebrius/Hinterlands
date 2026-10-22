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
