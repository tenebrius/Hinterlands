# Decisions log

Every departure from, or addition to, `docs/foundation.md`. Newest last. Each entry: what, why, and whether it is a parameter (implementer's call) or a shape change (owner is consulted).

## D1 — Two seeds: `map_seed` and `sim_seed`
The foundation says one seed drives map generation and the noise term, but the contingency check runs 20 seeds on the *same* terrain. Map generation takes `map_seed`; the simulation `Rng` takes `sim_seed`. A run is identified by `(map_seed, sim_seed, params)`.
Kind: interface detail, approved by owner.

## D2 — Cultivation targets a surplus margin
As written, cultivation claims cells until yield covers need, so `food/need ≈ 1` and the growth term `r·min(ratio, 1.2) − r` is ≈ 0: villages could never grow on food. Cultivation now claims until yield covers `need × cult_margin` (default 1.2), still bounded by `R_cult`. Release hysteresis (10%) applies to the same target.
Kind: parameter (`cult_margin`), no change to the growth equation.

## D3 — Cross-platform float determinism via `libm`
`exp`, `ln`, `powf` in `core` go through the `libm` crate so native (CLI) and wasm builds produce bit-identical snapshots and the golden hash holds on both.
Kind: implementation.

## D4 — Package names
Packages are `hinterlands-core`, `hinterlands-mapgen`, `hinterlands-web`, `hinterlands-cli` (a crate named `core` collides with Rust's `core`). Commands use the full names, e.g. `cargo run -p hinterlands-cli -- validate --seeds 20`.
Kind: implementation.

## D5 — Small gaps filled
- Diagonal edges cost `√2 ×` the per-km rate.
- `hub` = the cell's route load carries flow from at least two distinct trading pairs.
- Gravity flows `T_ij` are computed over all pairs within `C_max` (route loads, inflow); physical food transfer goes only to settlements in deficit.
- Map generation records the main river mouth cell in `Terrain` for the contingency check.
Kind: interpretation of the spec.

## D6 — Random draws never use `usize`
`rand`'s `gen_range` over `usize` samples differently on 32-bit wasm and 64-bit native, which broke cross-platform determinism in the first browser self-test. All draws use `u32` ranges. Checked by `?selftest=N` in the browser against `hinterlands hash --years N`.
Kind: implementation.

## D7 — Stream-power incision lands early on the map track
On a tilted-plane heightmap, D8 drainage gives parallel streams to the sea and no dominant river, which defeats the river and break-of-bulk behaviours. A simple stream-power incision pass (`fluvial_incision`, between thermal erosion and hydrology) produces dendritic networks with a few main rivers. The foundation lists hydraulic erosion as a later map pass and allows map passes to land early, since the systems never see how terrain was made.
Kind: map generation pass, allowed by the map track rule.

## D8 — Saves are replays
A save file is `(map_seed, sim_seed, params, year)`. Loading regenerates the terrain and replays to the year; determinism makes the state identical, and the file stays tiny and readable.
Kind: implementation.

## D9 — The viewer reads snapshots in-process
The viewer and the core are both Rust in one wasm module, so the snapshot is passed as a value through `SimHandle` rather than as bincode bytes over a JS boundary. The bincode form remains what the CLI writes and what tests hash.
Kind: implementation.

## D10 — Boarding and alighting from the bank
Boats are boarded from a bank cell next to a navigable river (transshipment plus one land step) and left the same way, rather than by first crossing onto the river cell. Without this, every boarding also paid the river-crossing cost and inland settlements could not use rivers.
Kind: interpretation of the cost table.

## D11 — Longer navigable rivers
Navigability now needs 200 km² of drainage and a gradient under 1% (was 300 km² and 0.4%), giving main rivers navigable 40–60 km inland on a 128 km map, as on medieval lowland rivers. Map parameters, not model parameters.
Kind: parameter.

## D12 — Natural growth reaches `r` at a full surplus; K = 1 food unit per person
As written, `P(1 + r·min(ratio, 1.2) − r)` gives at most 0.2·r = 0.4%/yr even with a full surplus; from ten hamlets of 50 that cannot fill a region in 500 years. Natural growth is now `r_grow · (min(ratio, cap) − 1) / (cap − 1)`: zero when exactly fed, `r_grow` (2%/yr) at the cap, and a shortfall shrinks at `r_shrink · (1 − ratio)`, smoothed over 3 years. This is the stated intent ("r = 0.02, natural growth when fed") with the cap as a normaliser, not a new mechanism. `K` (`k_maint`) is 1 food unit per person, so an isolated village that is exactly fed is held by the inflow term, as the specification asks; the equilibrium is then 150 people per prime cell.
Kind: parameter reading of the growth equation (no change to its shape).

## D13 — Trade in the Rihll–Wilson form (shape change, flagged for owner review)
As specified, trade flows come only from food surplus (`max(F − need, 0)`). Once settlements reach their land limit the surplus, and all trade, goes to zero, and no hierarchy can form (M3 runs: ten or sixty equal settlements stayed equal for 400 years at every α and β tried). Changes:
1. Each settlement sends a fixed **marketed share** of its harvest (`market_share`, 0.2: rents, tithes and sales) plus its resource output to market. It keeps the rest.
2. Flows go to partners in proportion to `W_j^α · e^(−β c_ij)`, where `W_j` is the **non-farming population**: population minus those fed by the settlement's own kept harvest, plus a small seed (`urban_seed`, 10). This is Rihll and Wilson's W (the size of central functions), separate from the rural base that generates the flow. With the whole population as attractor, the land-fed majority damps the feedback and the system cannot leave the equal state.
3. Marketed food is delivered to every destination, less `transport_loss` per unit cost, not only to settlements in deficit.
4. There is no home-market term by default (`home_weight` 0); a settlement with no partners keeps its grain.
5. Growth inflow `I_j` is the food the settlement has (kept plus received) plus `through_weight` × (non-food trade received + trade passing through). `through_weight` defaults to 0 for now.
6. A settlement counts as fed at a food ratio of 0.9 or more (`fed_ratio`). Hubs run slightly below 1.
7. The cultivation target includes the marketed share, so villages keep their land.
Result on map 1 with 60 equal starting villages and founding off: a coastal river port reaches 11k, a ring of inland market centres reaches about 3k, and the remaining villages stay at 1.5–2.5k.
Kind: **change to the shape of the trade and growth equations**. This is one of the points to raise with the owner; done under the owner's instruction to make the decisions needed for success, and listed for review.

## D14 — β = 0.25 and r_cult = 2.5
With β = 0.08, sea transport at 0.08/km puts the whole coast in one market and one port takes everything. At β = 0.25 several centres survive (Rihll and Wilson's results depend on this decay). r_cult = 2.5 keeps village field systems within about 2.5 km, as in Chisholm's studies of village field distances, and keeps villages in the hundreds to low thousands. Both are tuning values and may move in M5.
Kind: parameter.
