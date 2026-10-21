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

## D15 — Growth split into rural demography and Rihll–Wilson town growth fed by migration (shape change, flagged)
With one growth law driving every settlement toward its food, every village grows to fill its land at the growth cap, all places end up of similar size, and nothing fails. Changes:
1. The natural term (D12) is the rural rate: at most `r_grow` a year at a full surplus.
2. The Rihll–Wilson term `ε(D − K·W)` acts on `W`, the population beyond what the settlement's own harvest feeds, with `D` the **net** food imports (plus `through_weight` × non-food trade and trade passing through). A village that trades evenly has W ≈ 0 and no term; a town that imports grows. `W + urban_seed` is also the market pull in trade (replacing D13's kept-harvest measure, which counted peer exchange as urban).
3. A positive term is met by **migrants** drawn from the town's partners in proportion to `P_i · e^(−β c)`, each source giving at most `migration_cap` of its people a year. Population is conserved by the term; towns grow from their countryside, as medieval towns did.
Kind: change to the growth equation's shape; flagged for owner review.

## D16 — Urban shadow acts on growth as well as founding
"No new market survives close to a bigger one" (Bracton; Bosker & Buringh's negative effect within 20 km) is a listed v1 behaviour, but as specified the shadow only scores founding sites, so nothing near a town ever declines and the failure rate stays near zero. A settlement within `shadow_radius` of one at least `shadow_size_ratio` (3×) its size now loses `shadow_drain × P_k/1000 × (1 − c/R)` of its people a year to it (capped). Small foundations next to towns dwindle and fail; established places far from towns do not.
Kind: new term, grounded in a listed behaviour; flagged for owner review.

## D17 — Performance
Native time per year fell from 69 ms to about 17 ms: a compact edge list (CSR) built with the cost grid; Dial's bucket queue (bucket width = cheapest edge, exact; heap fallback if a parameter allows zero-cost edges); reach refreshed for a fifth of the settlements each year instead of all every fifth year; `c_max` 30 (at β ≈ 0.3, trade beyond cost 30 is below e⁻⁹).
Kind: implementation and parameter.

## D18 — Break of bulk at river mouths
Moving cargo between river barge and sea ship at a mouth costs `cost_river_sea_transship`. River-to-sea trade now funnels through the mouth cell as a transshipment point (Fujita & Mori 1996), within the cost grid as the foundation requires.
Kind: cost-grid rule.

## D19 — Parameters are tuned by search
`hinterlands search` samples parameter ranges and scores each sample by its distance from the validation targets over several maps and seeds; `hinterlands sweep` runs grids. Defaults come from these runs and are recorded with their numbers in `docs/milestones.md`.
Kind: process.

## D20 — Revised checks: river corridor instead of the mouth; at least one city over 5,000 (owner/designer decision)
On the simulation designer's advice, accepted by the owner:
1. Contingency (b) no longer asks for a city over 5,000 within 10 km of the main mouth. Big pre-industrial cities sat at the lowest bridging point or the head of sea navigation (London, Rouen and Paris, Hamburg, Bordeaux, Seville, Antwerp), not at marshy, exposed mouths, and the research cited separates coast access and navigable rivers as positives without a "mouth" term. The check now asks for a settlement over 5,000 on the main river system's navigable stretch, or on the coast within 15 km of it, in at least 80% of seeds. The largest city's distance from the main mouth is reported.
2. Over 5,000 per run is 1–5, with the median over 20 seeds per map at least 2. A 16,000 km² region with one city over 5,000 is historically normal; 2–5 was scaled from all of England. The second city is not to be bought by pushing the failure rate over its ceiling.
3. **Not built in v1:** a long-distance trade tier (an "outside world" reached by sea). This is the known limit of single-tier trade: one β cannot give both regional markets and long-distance transfer traffic. It is "sea trade between coasts", a Later (v5) item. After v1 passes, an optional experiment is allowed: one virtual settlement on a sea-edge cell with a fixed size and its own low β, behind a toggle that defaults off, to inform whether v2 or v5 should bring it forward.
Kind: change to the acceptance test, by the owner.

## D21 — v1 is judged on pass rates; names and a legend join the v1 view (owner/designer decision)
1. A check passes when it holds in at least 75% of 20 seeds on every map; aggregate checks are computed over the same 20 seeds. The default run (map 1, seed 1) is kept only as the determinism golden. A single seed at 15.8 km against a 15 km spacing ceiling is noise, and loosening the shadow to rescue it made the other maps worse.
2. The v1 view adds names: settlements are named at founding; rivers, ranges, seas, bays and large forests are named at generation. Labels can be toggled per class and are sized by rank, and a legend is built from the active layers. Names are derived from a hash of the site and its features, not from the run's random generator, so naming does not change the simulation.
Kind: change to the acceptance test and the v1 view, by the owner.

## D22 — Town footprints stay off navigable rivers
Cells are 1 km and a river cell is land with a river through it, so settlements may sit on one, as towns sit astride rivers. But the drawn footprint grew over navigable-river cells and painted the river out, so towns looked built in the water. Footprints now grow along the banks and never onto a navigable-river cell (a settlement centred on one keeps that cell), and the viewer draws the river or stream channel across any built-up cell it passes through. Footprints are rendering-only; the simulation is unchanged (the golden hash changes only because footprints are in the snapshot).
Kind: rendering.

## D23 — Real-terrain import: open sea, edge inflows, uniform soil
For the real-terrain test (`docs/real-terrain.md`), the DEM importer changes in three ways:
1. Only cells at or below zero that join the open sea at the map edge are sea. A 1 km average can put a tidal river or a marsh at 0 m, and a one-cell inland sea there stopped the Thames at Woolwich. Such pockets are now land at 0.5 m.
2. `MapParams.inflows` adds drainage that enters across the map edge, so a river whose catchment lies mostly off the map (the Thames: 9,950 km² above Teddington) is as large as it really is.
3. `MapParams.soil_noise` and `forest_noise` set the amplitude of the noise in soil yield and natural forest. On an imported map with no soil map they are 0 (uniform), so no invented fertile patch biases the test.
The defaults reproduce the generated maps exactly; the golden hash is unchanged.
Kind: map import.

## D24 — Experiment: markets grow on gross inflow (switch, default off)
`gross_inflow` makes the Rihll–Wilson growth term use the gross market inflow (food delivered to a market plus the share of its own harvest sold there) instead of net food imports, as in the published model. On Kent it changed nothing (lowland AUC 0.46 / 0.53 / 0.42, against 0.48 / 0.54 / 0.44 for v1): inland places receive almost no market inflow because the attraction is the derived non-farming population, which stays near zero wherever there is free land. Kept as a switch; the proposed follow-up (market size as its own state) is in `docs/real-terrain-experiments.md`.
Kind: experiment behind a switch; flagged shape change if adopted.

## D25 — Experiment: the outside world (switch, default off)
As allowed by D20: `outside_enabled` adds one virtual partner on the map-edge sea cell nearest the main river mouth, with fixed size `outside_size`, its own decay `outside_beta` and the usual trade range. It buys marketed produce, which leaves the map, and sends `outside_out` of goods into the region by pull and closeness; both load the routes. On Kent no variant formed London: as a strong buyer it halves the region's population, and as a seller its goods count only at `through_weight` in growth. Numbers in `docs/real-terrain-experiments.md`.
Kind: experiment behind a switch (a Later item, v5, tried early with approval).

## D26 — Market size as its own state (switch, default off; v1.1 candidate, designer-approved)
`market_state` makes Rihll–Wilson's W_j a state of each settlement, in people. It starts at `market_w0` with a floor of `market_floor`, moves by ε(D_j − K·W_j) on gross market inflow (the home self-term included), and changes only with the migrants the market actually gains or loses. Attraction uses W_j. This corrects the specification, whose W, the non-farming population, let a place become a market only after it ran out of land. Results and validation status are in `docs/real-terrain-experiments.md`; not yet the default, because validation on the generated maps does not pass.
Kind: correction of the growth equation and settlement state (owner/designer decision); default pending validation.

## D27 — Income from handling trade (switch, default off; v1.1 candidate, designer-approved)
`handling_income` adds to a place's inflow a margin on the trade passing its anchor (`handling_margin`) and on the trade changing mode within a cell of it (`transship_margin`: boarding or leaving a boat, river barge to sea ship). This is break of bulk as an income (Bleakley & Lin 2012; Fujita & Mori 1996), and it replaces the through-traffic weight for that trade. The yearly growth cap still applies.
Kind: model mechanism (owner/designer decision); default pending validation.

## D28 — Hamlet founding without transport weights (v1.1 candidate, designer-approved)
The founding score used Bosker & Buringh's weights, which were estimated for cities over 5,000. Village-scale models (Artificial Anasazi, VEP) place settlements by farmland and water alone. Under v1.1 the transport weights in founding (`w_hub`, `w_river`, `w_coast`, `w_road`) are 0, and the market and handling terms (D26, D27) decide which villages become towns. The change is to parameter values only; v1 defaults are unchanged until v1.1 passes validation. First result: the water filter for founding sites (`water_radius_km` with `stream_area`) still confines hamlets to river and coast sites; see `docs/real-terrain-experiments.md`.
Kind: correction of the founding score (owner/designer decision); default pending validation.

## D29 — Water supply at brook scale (switch `spring_water`, default off; v1.1 candidate, designer-approved)
The v1 water rule ("water within 1 km", with streams from 60 km²) is a river definition, not a water supply; it left about a quarter of the land eligible for founding. Under `spring_water`, a site needs a brook (at least `brook_area` = 5 km² of drainage) or the sea within `supply_radius_km` = 3 km, and water enters the founding score as adjacency: `w_water_cell` for a brook or river in the cell, `w_water_near` for one, or the sea, within 1 km. This applies to the starting hamlets too. No wells layer. On 1 km cells the filter admits 100% of land, so water acts only through the score.
Kind: correction of the water-supply rule (owner/designer decision); default pending validation.

## D30 — v1.1: the corrected model becomes the default; the river-corridor share is reported (owner/designer decision)
1. The defaults are now market size as its own state (D26), handling income (D27), founding on food and water only (D28) and water supply at brook scale (D29), at the values from the last re-tune on the generated maps (`docs/parameters.md`). The foundation document is corrected to match: settlement state has W, trade attracts on W, growth is the two-equation form with handling income, founding scores food and water with the brook-scale rule, and site factors act through growth, not founding.
2. Contingency: only (a), the largest city's anchor varying in at least 30% of seeds, is gated. The river-corridor share (b) and the distance to the main mouth are reported. With a populated middle tier, which river gets the big city is partly contingent, and the transport claim is carried by the per-seed siting check.
3. Founding's candidate scan was made cheaper with identical results: free food near each cell is cached and refreshed only near changed owners or roads, settlement spacing is stamped once, and shadow and access are summed with settlements outermost in the same order. Founding fell from 6.5 to 2.4 ms per year.
Golden hash: `68beae826cf800cd` (map 1, seed 1, 500 years), native and in the browser.
Kind: change to the defaults and the acceptance test (owner/designer decision).

## D31 — Soil for imported maps; DEM cell count rounding
`MapParams` gains `soil_file` (an ESRI ASCII soil-quality grid on the map's grid, read by the DEM importer) and `soil_tpi`, `tpi_radius_km` and `tpi_scale_m` (a valley-position factor on soil). Both are rescaled to the uniform mean yield, and both are off by default, so generated maps and the golden hash are unchanged. `tools/real/fetch_soil.py` builds a soil grid from ISRIC SoilGrids 2.0, filling built-up holes from the surrounding cells. The ESDB needs a request form, so it was not fetched. The importer now rounds a target cell count within 0.05 of a whole number (a 999.8 m source cell is 1 km); Kent imports at 128 × 128, where it had been 127 × 127. Results: `docs/real-terrain-experiments.md`.
Kind: map import.

## D32 — Initial roads as input data
`Terrain.initial_road` holds roads present at the start (empty on generated maps, and left out of the serialised terrain when empty, so their hash is unchanged). `State` starts its road set from it. `MapParams.roads_file` names an ESRI ASCII 0/1 grid read by the DEM importer; `tools/real/fetch_roads.py` rasterises Itiner-e Roman roads (CC BY 4.0). The model is unchanged. The pre-registered Kent test (Rochester and Canterbury from Watling Street's river crossings) failed. Diagnosis and options are in `docs/real-terrain-experiments.md`.
Kind: input data (owner/designer decision).
