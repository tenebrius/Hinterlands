# Validation definitions

Exact definitions of the checks in the foundation's validation table, as computed by `hinterlands_core::metrics` and `hinterlands validate`. They are fixed before tuning so the targets cannot drift.

All checks are taken on the final snapshot of a 500-year run (after year 499's tick), on generated 128 × 128 maps of 1 km cells.

| Check | Definition | Target |
| --- | --- | --- |
| Determinism | FNV-1a hash of the bincode snapshot after 500 years; the canonical run (map 1, seed 1) is run twice and must match. The browser build must match the native hash (`?selftest=N`). | identical |
| Settlement count | Settlements alive at the end. | 150–350 |
| Over 2,000 / over 5,000 | Settlements with population strictly above 2,000 / 5,000. Over 5,000 is also gated per map: the median over the 20 seeds must be at least 2 (D20). | 5–12 / 1–5, median ≥ 2 |
| Largest share | Largest population divided by total population. | < 0.25 |
| Rank-size slope | Ordinary least squares slope of ln(population) on ln(rank) over the 30 largest settlements (all, if fewer than 30; undefined below 3). | −1.3 to −0.8 |
| Market-town spacing | For each settlement over 500, straight-line distance between anchor cell centres to the nearest other settlement over 500; the median of these. | 8–15 km |
| Siting | Share of settlements over 2,000 whose anchor, or a cell in its 8-neighbourhood, is a navigable river, the sea, a route hub (a cell where routes carrying at least `hub_min_flow` meet from three or more directions), or a road crossing (a road cell with road in three or more of its eight neighbours). | ≥ 0.70 |
| Failure rate | Of the settlements founded during the run (the starting hamlets excluded), the share that failed before the end. | 0.40–0.70 |
| Forest near cities | For each settlement over 5,000, the mean forest cover over land cells within 5 km (straight line) of its anchor; the largest of these. | < 0.30 |
| Stability | The largest year-on-year change in total population, as a share of the earlier year, over years 101–500. | < 0.05 |
| Contingency | Over 20 seeds on the same map (v1.2, D35): (a) for each pair of runs, the share of run A's ten largest settlements with one of run B's ten largest within 5 km (anchor to anchor); the mean over all pairs. Reported, not gated: (b) the share of runs whose largest settlement lies outside the modal cluster (the anchor with the most other runs' largest within 5 km); (c) the share of runs with a settlement over 5,000 on or next to the **main river corridor** or coastal within 15 km of it (D20); (d) the median distance of the largest settlement from the main mouth. Until v1.2, (b) was gated at ≥ 0.30. | (a) ≤ 0.80; (b)–(d) reported |
| Performance | Mean time of one simulated year in the release wasm build in a browser, measured over a 500-year run. Native time is reported by `hinterlands bench` as a proxy. | < 50 ms |

## Pass rule

`hinterlands validate --maps 3 --seeds 20` runs maps 1–3, 20 seeds each. v1 passes when (D21):

- every per-run check holds in at least 75% of the 20 seeds (15 of 20) on every map;
- the aggregate checks (contingency, and the over-5,000 median), computed over the same 20 seeds, pass on every map;
- determinism passes. Map 1 seed 1 is the determinism golden; it is not otherwise special.

The "seed 1" column is shown for reference only.

## Real-terrain tests

Not a release gate. The standing rule (foundation, after D23–D31): judge placement only where the inputs carry signal, and judge count and spacing where they do not. On Kent, terrain and soil hold no signal for towns under about 1,000 (feature AUCs 0.20–0.48 against lowland cells), so they are judged by count and nearest-neighbour spacing of inland places over 500 (real: 29 at 10.6 km). Ports, river towns and towns of 1,000 or more are judged by AUC against lowland cells. `just real` produces the table.
