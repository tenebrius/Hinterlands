# Validation definitions

Exact definitions of the checks in the foundation's validation table, as computed by `hinterlands_core::metrics` and `hinterlands validate`. They are fixed before tuning so the targets cannot drift.

All checks are taken on the final snapshot of a 500-year run (after year 499's tick), on generated 128 × 128 maps of 1 km cells.

| Check | Definition | Target |
| --- | --- | --- |
| Determinism | FNV-1a hash of the bincode snapshot after 500 years; the canonical run (map 1, seed 1) is run twice and must match. The browser build must match the native hash (`?selftest=N`). | identical |
| Settlement count | Settlements alive at the end. | 150–350 |
| Over 2,000 / over 5,000 | Settlements with population strictly above 2,000 / 5,000. The median of over 5,000 across the 20 seeds is reported (gated at ≥ 2 from D20 until v2, D48). | 5–12 / 1–5; median reported |
| Largest share | Largest population divided by total population. | < 0.25 |
| Rank-size slope | Ordinary least squares slope of ln(population) on ln(rank) over the 30 largest settlements (all, if fewer than 30; undefined below 3). | −1.3 to −0.8 |
| Market-town spacing | For each settlement over 500, straight-line distance between anchor cell centres to the nearest other settlement over 500; the median of these. | 8–15 km |
| Siting | Share of settlements over 2,000 whose anchor, or a cell in its 8-neighbourhood, is a navigable river, the sea, a route hub (a cell where routes carrying at least `hub_min_flow` meet from three or more directions), or a road crossing (a road cell with road in three or more of its eight neighbours). | ≥ 0.70 |
| Failure rate | Of the settlements founded during the run (the starting hamlets excluded), the share that failed before the end. | 0.40–0.70 |
| Forest near cities | For each settlement over 5,000, the mean forest cover over land cells within 5 km (straight line) of its anchor; the largest of these. | < 0.30 |
| Stability | The largest year-on-year change in total population, as a share of the earlier year, over years 101–500. | < 0.05 |
| Contingency | Over 20 seeds on the same map (v1.2, D35): (a) for each pair of runs, the share of run A's ten largest settlements with one of run B's ten largest within 5 km (anchor to anchor); the mean over all pairs. Reported, not gated: (b) the share of runs whose largest settlement lies outside the modal cluster (the anchor with the most other runs' largest within 5 km); (c) the share of runs with a settlement over 5,000 on or next to the **main river corridor** or coastal within 15 km of it (D20); (d) the median distance of the largest settlement from the main mouth. Until v1.2, (b) was gated at ≥ 0.30. | (a) ≤ 0.80; (b)–(d) reported |
| Performance | Mean time of one simulated year in the release wasm build in a browser, measured over a 500-year run. Native time is reported by `hinterlands bench` as a proxy. Gated at < 50 ms until v7; reported only since D91 (owner). | reported |

## Pass rule

**Outside trade (D126):** the band is 3–50% of market inflow. The ceiling guards against an export colony, a region whose market is mostly outside trade; it was 30% from L1 (D110) and 20% from v5. Exports per head against England's (about 14 d c. 1300) are reported.

**From D120 (22 October 2026) the gate runs 40 seeds a map:** `hinterlands validate --maps 3 --seeds 40`. Every per-run check must hold in 75% of the runs (30 of 40) on every map, and the contingency check is computed over the 40. Where the accepted model itself is below 30/40 (at D120: road length 300–500 on map 1, 29/40), a stage passes that check if it is no worse than the accepted model's count. Runs passing every check are reported, not gated. The rule as it stood before D120 follows.

`hinterlands validate --maps 3 --seeds 20` runs maps 1–3, 20 seeds each. v1 passes when (D21):

- every per-run check holds in at least 75% of the 20 seeds (15 of 20) on every map;
- the aggregate check (contingency), computed over the same 20 seeds, passes on every map;
- determinism passes. Map 1 seed 1 is the determinism golden; it is not otherwise special.

The "seed 1" column is shown for reference only.

## Real-terrain tests

Not a release gate. The standing rule (foundation, after D23–D31): judge placement only where the inputs carry signal, and judge count and spacing where they do not. On Kent, terrain and soil hold no signal for towns under about 1,000 (feature AUCs 0.20–0.48 against lowland cells), so they are judged by count and nearest-neighbour spacing of inland places over 500 (real: 29 at 10.6 km). Ports, river towns and towns of 1,000 or more are judged by AUC against lowland cells. `just real` produces the table.
