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

## D33 — Bridges as input data
`Terrain.bridge` marks river and stream cells with a bridge (empty on generated maps, and left out of the serialised terrain when empty). A crossing costs 0 at a ford or a bridge (`free_crossing`). With `MapParams.road_bridges`, the importer puts a bridge wherever an initial road crosses a river or stream; Kent has 52 (30 on navigable rivers). The pre-registered test (a place of 1,000+ within 3 km of Rochester in at least 5 of 20 seeds) failed at 1 of 20. Golden unchanged.
Kind: input data (owner/designer decision).

## D34 — Underfed places shed people instead of being deleted (switch `shed_unfed`, default off)
The failure rule removed any settlement unfed for `fail_unfed_years`, at any size. In v1.1 it deletes about 170 settlements per generated run that had peaked at 1,000 or more. Under `shed_unfed`, an underfed place loses a third of `(1 − ratio) × P` a year (the leavers are lost), and only the low-population rule removes. Results: every per-seed check improves (runs passing every check 19/20/18 against 7/13/14), but the gated "largest city varies" check fails on maps 2 and 3 (0.20, 0.00). On Kent the Rochester expectation fails (1/20), because the bridge towns still starve, now by shrinking. Golden unchanged. Making it the default is the designer's decision (design authority delegated by the owner).
Kind: model mechanism behind a switch (designer decision); default pending.

## D35 — v1.2: shedding is the default; contingency gated on the middle tier (designer decision)
1. `shed_unfed` (D34) is the default. An underfed place sheds a third of its food gap a year, and only the low-population rule removes. Nothing was re-tuned.
2. Contingency: gated on the ten largest. For each ordered pair of the 20 runs on a map, take the share of one run's ten largest settlements with one of the other run's ten largest within 5 km. The mean must be at most 0.80. The varying largest city (gated at ≥ 0.30 until v1.2), the river-corridor share and the largest city's distance to the mouth are reported. A clearly best site may hold the largest city in every seed; contingency lives in the middle tier.
3. Kent's standard configuration is uniform soil with Roman roads and bridges (`just real`). Kent is a regression report at each milestone, and Rochester is closed as a target.
Golden hash: `2b00434da9781880`, native and in the browser.
Kind: change to the defaults and the acceptance test (designer, with design authority delegated by the owner).

## D36 — Smooth relief drawn by a shader (rendering; R1 from the graphics-overhaul branch)
The base map is drawn by a fragment shader through an `egui_glow` paint callback. Per-cell elevation, signed distance to the coast, yield and hillshade, plus the year's forest cover, go up as float textures and are interpolated bicubically. The result: a noisy coastline from the distance field, shaded shallows, yield- and height-tinted land under a Horn hillshade, and forest as a tint, clumps or tree crowns by zoom. Patterns come from an integer hash of map position. The old cell texture (`TerrainLayer`) stays, off by default, and is the fallback if the shader cannot compile (`?on=terrain&off=relief,rivers`). Details: `docs/rendering.md`.
Kind: rendering.

## D37 — Rivers drawn as lines (rendering; R2 from the graphics-overhaul branch)
Streams and navigable rivers are polylines along `flow_dir`, broken at sources and confluences, smoothed by two rounds of Chaikin corner cutting, and widened with drainage (`0.012 · flux^0.4` km, capped at 0.45 km). Navigable stretches are darker; in spring the seasons layer widens channels by half instead of tinting cells. Details: `docs/rendering.md`.
Kind: rendering.

## D38 — v2 goods: resource sites and production (G1, behind `goods_enabled`)
Resource cells become sites in `State.sites` (ore gives iron, plus salt and stone). A new system, `production`, runs after cultivation. It gives each site a holder: the settlement anchored within `min_spacing_km` at the least travel cost. Output is `min(cap_g, prod_g × (holder's market size W + site_crew))` in grain units a year. Iron is fuel-bound: it asks `fuel_per_iron` of forest per unit, cultivation cuts that from the holder's woods nearest first, and output is capped by the fuel cut. Under `goods_enabled` the flat `resource_out` is retired.
Two values the specification leaves open, chosen here:
- the value of a good `v_g` is 1, because demand and output are both stated in grain units;
- `fuel_per_iron` is 0.1 forest cell-units per unit. At 0.01 a mine's draw was below the regrowth of its woods; at 0.1 every working iron site's woods fall below the land mean by year 200.
With goods off, the golden hash is unchanged.
Kind: model mechanism behind a switch (v2 specification).

## D39 — v2 goods trade with grain payment (G2, behind `goods_enabled`)
Each site's output goes to the settlements within trading reach in proportion to `demand_k × exp(−β_g c)`, with `β_g = β / ρ_g`. The holder's own demand takes its share without travelling or payment. Demand is per person for salt and iron and per market person for stone. A buyer pays `v_g` grain per unit out of its marketed share, capped at that share. The payment reaches the producer less the usual loss per unit cost, counts in the producer's gross market inflow, and travels as an ordinary grain flow on the buyer's path. The rest of the marketed share goes to markets as before. Goods and payments load the routes, so handling and transshipment income (D27) apply to them at their value. Flows carry `good` and `payment` markers, left out of the snapshot bytes when absent, so the golden hash is unchanged with goods off.
Check (3 maps × 6 seeds, years 200 and 500):
- payments balance: goods worth 53,226 received, 52,405 paid (the difference is buyers capped at their marketed share), 48,253 arrived after loss;
- amount-weighted median route cost: iron 19.7, salt 14.5, grain 13.3, stone 9.0; the full order holds in 22 of 24 runs that trade all four.
Kind: model mechanism behind a switch (v2 specification).

## D40 — v2 specialist settlements (G3, behind `goods_enabled`)
At each founding step, for each good, the free resource site (no settlement within `min_spacing_km`) with the most reachable demand gets a hamlet of `found_pop` on the cell, if that demand would feed the founders. Migrants come from the nearest settlement that can spare them. Its kind is `Mine`, `Saltern` or `Quarry`, and the founding is logged as `Works`. Ordinary failure rules apply. `validate` gates "sites producing ≥ 2/3" and "largest not specialist" (trivially true with goods off). The gate failed with the starting values: production-constrained sales let salterns earn at the cap regardless of demand and grow into the largest places. The resolution is pending with the designer.
Kind: model mechanism behind a switch (v2 specification).

## D41 — Goods trade by the shopping model (designer decision; replaces D39's producer-origin flows)
Buyers are the origin (Huff 1964; Wilson 1971). A holder's own demand is met first from its own sites, without travel or payment. The rest of each buyer's demand is split among working sites in proportion to `capacity × exp(−β_g c)`. Orders beyond a site's capacity are scaled down to fit in one pass, and unmet demand stays unmet. A site's output is what it sells (`Site.sold`, written by trade). Grain payment and route loading are as in D39. The v2 gate re-run still fails ("largest not specialist" 1/20 and 12/20 on maps 1 and 2): the largest specialists are salterns founded on the best coastal city sites, which grow into the city that grows there without goods and keep their kind. See `docs/real-terrain-experiments.md`.
Kind: model mechanism behind a switch (designer decision).

## D42 — Fields, pasture, hedges and seasons drawn in the relief shader (rendering; R3 from the graphics-overhaul branch)
The 1.x countryside layers drew one square per cell. The relief shader now draws them per pixel from two per-year textures, land use and plots, with the same rules for which cell is what (strip direction is one shared function, `countryside::strip_orientation`):
- fields get one smooth, roughened outline per block, with furlongs and reversed-S strips close in;
- hedges follow the outlines and the boundaries between holdings;
- pasture sits under fields and fields under forest;
- season tints are per pixel.
Each part draws only while its own layer is on. The per-cell layers stand down only while the shader runs. Sheep, coppice and stumps stay in egui. Details: `docs/rendering.md`.
Kind: rendering.

## D43 — Roads, routes and trade drawn as smoothed lines (rendering; R4 from the graphics-overhaul branch)
Route segments are joined into chains by kind (water, land trade off roads, roads). Chains break only at junctions, ends, changes of kind and settlement centres, and are smoothed with the same Chaikin corner cutting as rivers (shared in `lines.rs`). Wide roads are drawn cased, land trade off roads as dashed tracks, water trade as a pale line. Trade dots follow the smoothed lines, and their positions are cached in map space. The 2% share rule and the dot trip, cargo and speed rules are unchanged. Details: `docs/rendering.md`.
Kind: rendering.

## D44 — A specialist's kind follows its economy (designer decision; the designer numbered it D43, which the R4 merge had taken)
Trade records each settlement's goods sales (`goods_in`, payment grain received). Growth keeps ten-year averages of goods sales and gross inflow. A specialist at least 10 years old whose average sales are below `specialist_share` (0.25) of its average inflow becomes an ordinary settlement for good. It keeps holding and working its site, and the chronicle records it ("1340. Mereport has outgrown its salt works."). The gate "largest is not a specialist" now tests the economy through this rule.
Gate re-run (goods on, starting values): "largest not specialist" 20/20 on all maps. Specialists per run 1.4 / 3.7 / 0.4, size median 70 / 85 / 207, max 471. Payment share of market inflow 0.03. Every v1.2 per-seed check passes. Failing: the over-5,000 median on map 3 (1). That is a v1.2 check, which the specification lets the implementer re-tune with goods on.
Kind: model mechanism behind a switch (designer decision).

## D45 — Settlement footprints, ruins and city walls drawn in the relief shader (rendering; R5 from the graphics-overhaul branch)
Built-up areas, ruins and walls come from a per-year town texture: built up, ruin, inside walls, and the ruin's age. Each built-up area has one smooth outline. Close in it fills with street blocks, house plots and gardens. Ruins fade grey, and walls with towers follow the smoothed outline. Roads and rivers are drawn over towns. Markers and other building glyphs stay in egui. One known difference from D22: where a road runs along a river through a town, the road now covers the river line there. Details: `docs/rendering.md`.
Kind: rendering.

## D46 — Footpaths, parish boundaries and quays drawn as smoothed lines (rendering; R6 from the graphics-overhaul branch)
`lines.rs` gains `chains`, now shared with `routes.rs` with unchanged output. Footpaths and parish boundaries are smoothed dashed lines. Sea quays are drawn in the relief shader on the smoothed shore: the town texture's fourth value carries a "town or larger" flag on built cells. Rivers are embanked through towns, and the cell-edge quays stand down while the shader runs. Details: `docs/rendering.md`.
Kind: rendering.

## D47 — Lettering and panel theme (rendering; R7 from the graphics-overhaul branch)
Labels and the legend are set in EB Garamond Medium and Medium Italic: Latin subsets of about 50 KB each in `crates/hinterlands-web/assets/fonts/`, under the SIL Open Font License 1.1, with `OFL.txt` beside them. Panels get a parchment theme in light and dark mode (`src/style.rs`). The wasm grows by about 130 KB.
Kind: rendering.

## D48 — The over-5,000 median is reported, not gated (designer decision)
From v2, the median count of settlements over 5,000 across the 20 seeds is reported, not gated (it was gated at ≥ 2 from D20). The per-seed check (1–5 over 5,000) stays gated. Reasons: v1.2 passed the median on map 3 by one seed; map 3 has one dominant site; and the per-seed hierarchy checks carry what the median was for. `validate` prints it as reported, and the search no longer scores it (tuning towards it was declined).
Kind: change to the acceptance test (designer).

## D49 — v2: goods on by default, re-tuned values, and the goods view
- `goods_enabled` is on by default, with the values re-tuned under goods (on the generated maps, 4 seeds a map): α 1.0883, β 0.1390, ε 0.1373, K 3.3905, market share 0.4866, stone demand 0.0160. The other goods values are the specification's starting values, plus v_g 1 and fuel_per_iron 0.1 (D38) and specialist_share 0.25 (D44). Files that reproduce earlier versions: `data/real/experiments/v1.json` gives v1's golden `afd6c175b193443b`, and `v12.json` gives v1.2's `2b00434da9781880`.
- The snapshot gains each settlement's `kind` and `goods_in`, and the year's `sites` (good, holder, capacity, sold). All are left out of the bytes at their defaults, so goods-off runs keep their old hashes.
- The view gains a new layer, `layers/goods.rs`: goods trade as smoothed lines coloured by good (salt white, iron rust, stone grey), with a dark casing; site symbols (square salt works, triangle mine, diamond quarry), filled while selling; and a ring in the good's colour round each specialist. Legend entries come from the layer.
- The view gains a new panel, `panels/goods.rs`: totals by good, and the site or specialist at the selected cell.
- The chronicle names new works and specialists that outgrow their works.
Golden hash: `b0de71500774acf8`, native and in the browser.
Kind: change to the defaults (designer), and rendering.

## D50 — v2.1: Kent's resource sites as data
- `MapParams.resources_file` names a CSV of sites (`name,x_km,y_km,good`). When it is set, the DEM importer clears the random sites and places these. A site that falls in the sea moves to the nearest land cell within two cells; a site with none is dropped.
- `data/real/kent/resources.csv` lists 17 sites with their source: 6 Wealden bloomeries (Bardown, Beauport Park, Ashdown, Lamberhurst, Mayfield, Burwash), 5 ragstone and sandstone quarries (Boughton Monchelsea, Allington, Hythe, Merstham, Bethersden) and 6 salt-making sites (Seasalter, Graveney, Milton Regis, New Romney, Pevensey, Tollesbury). Cowden is left out because no coordinates were found for it. `tools/real/resources_km.py` converts them to grid km in `resources_km.csv`. `data/real/kent/map-resources.json` is `map-bridges.json` plus this file.
- The generated maps and the golden hash are unchanged.
- Before this, Kent ran with the importer's 9 random sites (4 salt and 5 stone, no iron). The v2 milestone note that Kent had no resource sites was wrong and has been corrected.
Kind: data and an importer option (reported, not gated, per the foundation's plan after v2).

## D51 — v2.1: coppice for iron fuel (designer decision)
- With `coppice` (on by default), an iron site's fuel cut leaves `coppice_floor` (0.3) of forest standing on each cell. Household fuel and field clearing are unchanged.
- Iron output is also capped by the regrowth above the floor: the sum of `regrow_rate × (forest0 − coppice_floor)` over the uncultivated cells of the holder's fuel catchment, divided by `fuel_per_iron`. The site asks only that much fuel. A mine therefore settles at a steady output, and a mine with a small catchment stays small.
- `data/real/experiments/v2.json` (`coppice` off) reproduces v2's golden `b0de71500774acf8`.
- Gate re-run, 20 seeds a map: validation passed, with every check at 19/20 or better. Iron output per held site at year 500, median (4 seeds a map): 25 / 18 / 28, against 47 / 38 / 43 without coppice. Forest within 5 km of iron sites: 0.19 / 0.22 / 0.17 against a land mean of 0.46 / 0.45 / 0.44; it was 0.00 without coppice.
Golden hash: `f67bf8565c500fda`, native and in the browser.
Kind: model mechanism behind a switch (designer decision).

## D52 — v3(a): the shadow in km, behind a switch (off until validation passes)
- With `shadow_km`, the founding shadow and the growth drain use the straight-line distance between anchors, against `shadow_radius_km`. The access term stays in travel cost and counts only settlements outside the shadow. The drain still comes only from trade partners.
- The switch is off by default because validation has not yet passed with it (see `docs/real-terrain-experiments.md`, "v3(a)"). The golden hash is unchanged at `f67bf8565c500fda`.
- `data/real/experiments/v3a-candidate.json` holds the best values found so far. The older experiment files pin `shadow_km` off.
Kind: model mechanism behind a switch (designer's specification); the default awaits the designer.

## D53 — v3(a) dropped: the shadow stays in travel cost (designer decision on the outcome)
- The designer's option 1 was tried: the drain gets its own radius, `drain_radius_km` (about 8 km), and the founding shadow keeps `shadow_radius_km` (15–18 km). The floor on the founding shadow was declined.
- **Re-tune.** A search ran over both radii, shadow weight, drain, threshold, noise, ε and K: 60 samples × 3 maps × 3 seeds. Its best point failed full validation (20 seeds a map):
  - market spacing passes in 3 / 18 / 9 of 20 seeds;
  - rank-size slope in 9 / 9 / 20;
  - failure rate in 12 / 19 / 20.
- **Controlled check.** The best single-radius candidate (`v3a-candidate.json`, radius 15.2 km) was re-run with only `drain_radius_km` changed, at 8, 10 and 12 km, on 3 seeds a map. Map 1 spacing is 6.3–8.5 km in every case, and map 3 is 5.0–10.6 km. A shorter drain lets more mid-sized places survive close together, so spacing falls. In the km form, the drain is what spaces the markets, and a short drain cannot do it.
- **Outcome, by the designer's rule.** v3(a) is dropped. `shadow_km` stays in the code, off by default, with `drain_radius_km` beside it; the golden is unchanged (`f67bf8565c500fda`). v3(b) goes ahead with the cost shadow.
Kind: designer decision (scope); no change to the defaults.

## D54 — v3(b): roads follow traffic (designer's specification)
- Each road cell has a condition `q` (`road_q`). Every `road_interval` the top `road_pairs` trading pairs' paths are raised to `q` = 1, as before.
- Every other road cell whose trade load that step is below `road_keep_load` loses `road_decay` (0.05). The load is the yearly trade value of all flows whose path crosses the cell. A cell below 0.1 stops being road, and the road edges that touch it go with it.
- Initial roads start at `q` = 1 and follow the same rule.
- Moving between two road cells costs `land + q × (cost_road − land)` per km, with `q` the lower of the two. At `q` = 1 the cost is `cost_road` exactly, so older versions keep their hashes. The cost grid is rebuilt at road steps only.
- **Tuning.** `road_keep_load` is the only value tuned: 800, chosen so that the network settles (10 gave +33% road length between years 300 and 500, 200 gave +16%, 800 gave +2.5%). No model value was re-tuned.
- **Validation** (20 seeds a map): PASSED. Every v2.1 check holds. Road length changes 300→500 by a median of 2.5% / 3.6% / 2.3%, gated under 10% per run (20/20 on every map). The share of road cells abandoned during the run is 0.41 / 0.40 / 0.40.
- **Validation tooling.** `validate` gains the road-length check, which passes trivially with `traffic_roads` off, and the abandoned share as a reported line. `hinterlands roads` reports road cells, condition, initial roads kept, and cells abandoned.
- **Viewer.** The snapshot does not carry `road_q`. The viewer's cost overlay treats every road cell as full condition.
- **Reproducibility.** The earlier experiment files pin `traffic_roads` off. `data/real/experiments/v21.json` reproduces v2.1's `f67bf8565c500fda`.
Golden hash: `0b590b3b017b63d0`, native and in the browser.
Kind: model mechanism (designer's specification), on by default.

## D55 — v3(c): bridges at busy crossings (designer's specification)
- With `traffic_bridges` (on), at each road step a road cell on a river or stream, not already a ford or an input bridge, counts the road steps in a row on which its trade load reached `bridge_load` (1,600 = 2 × `road_keep_load`). The load is the same per-cell trade value as D54's. At `bridge_steps` (3) it becomes a bridge.
- A bridge is crossed at no cost, through the free-crossing rule of D33. It is never removed, even if its road fades.
- Model bridges are state (`bridge`, `bridge_streak`) and go to the cost grid with the road condition. The snapshot lists them (`bridges`).
- **Viewer.** It draws a road crossing as a bridge where the model has built one or the map has one (D33), and as a ford at a ford. The view's own traffic rule is gone.
- **Validation,** with mills (D56), 20 seeds a map: PASSED (see the v3 milestone). Bridges per run: median 14 / 17 / 15. Places of 1,000 or more within 2 km of a bridge: 0.11 / 0.25 / 0.44, against 0.01 / 0.02 / 0.02 of all settlements.
Kind: model mechanism (designer's specification), on by default.

## D56 — v3(d): mills (designer's specification)
- With `mills` (on), cultivation gives a settlement a mill when one of its fields is a stream or river cell with drainage of at least `mill_area` (20 km²). Drainage is `flux × cell_km²`.
- A mill raises the settlement's food produce by `mill_bonus` (3%). The cells it claims are chosen on the produce without the bonus, as before.
- The snapshot carries each settlement's `mill`, left out of the bytes when false. The viewer draws the model's mill, on the nearest stream or river beside the settlement.
- **Note.** On the generated maps (1 km cells), a stream needs a flux of at least `stream_area` (60 cells, 60 km²), so `mill_area` at 20 km² never binds: every stream field gives a mill. It would bind on finer grids or with a higher threshold.
- Share of settlements with a mill: 0.41 / 0.41 / 0.38.
Kind: model mechanism (designer's specification), on by default.

## D57 — v4 E1: the event queue, the historical calendar, famine and plague
- A new `events` system runs first in each year. It queues the events that fall due, and `Event::apply` makes every change.
- **`events_mode`:** 0 Off, 1 Historical, 2 Random. Off draws nothing and reproduces `171ec4461a912a1d`.
  - In Historical mode (year 0 = 1100), the famine starts at `famine_year` 215 for `famine_years` 3, and plague strikes at `plague_year` 248, with recurrences at 261 and 269 at `plague_recur_share` (1/3) of its mortality.
  - In Random mode each kind is drawn yearly from the run's `Rng`, at `famine_interval` 120 and `plague_interval` 150 years.
- **Famine.** Cultivation cuts each settlement's produce by `famine_loss` (0.4) after its fields are chosen: a failed harvest cannot be made up by ploughing more in the same year.
- **Plague.** Each settlement's share `m` is drawn when the plague strikes: uniform within `plague_mortality` ± `plague_spread` (0.4 ± 0.1), scaled down for a recurrence, and clamped to [0, 0.95]. It is taken in equal yearly parts over `plague_years` (2): `1 − (1 − m)^(1/2)` a year, of both people and market size W. The event queues its own second year.
- **Log.** It gains `Famine` and `Plague { lost }` entries, one per settlement struck. The log is now cleared before the events run, so their entries are kept for the year.
- For now `events_mode` defaults to Off, so the golden stays fixed through E1–E3; the gate runs in Historical mode.
Kind: model mechanism (designer's specification), behind a switch.

## D58 — v4 gate measures (implementer's reading)
- **Stability with events.** The v1 stability check (under 5% a year) leaves out the years of each event and the one after: famine years 215–219, and plague years 248–251, 261–264 and 269–272. A plague is meant to break steady state.
- **Desertion.** "Places in the lowest-yield quarter" is read as the *local* yield: the mean yield of land within 3 km of the anchor, among the settlements alive at the plague year. The quarters are within each run. A place's own fields are its best cells and hide how poor its surroundings are: by field yield the check held in 5 of 12 trial runs, by local yield in 10 of 12.
- **Top-10 against Off.** The share of a run's top ten within 5 km of the same seed's top ten with events off.
Kind: reading of the acceptance test (implementer); the designer to confirm.

## D59 — Desertion measured beyond the urban drain (designer decision)
The desertion check counts only settlements more than 8 km from any place over 2,000 at the plague year. Quarters of local yield (D58) are taken among those settlements.
- **Re-run of the E1 gate** (Historical, 20 seeds a map): poor land deserted in 16/20, 20/20 and **11/20**. Map 3 fails.
- **Failure rates 248–300,** lowest against highest quarter: 0.22 / 0.10, 0.21 / 0.08 and 0.18 / 0.15.
- **Every other check passes on every map.**
- **Reported:**
  - plague drop 0.37 / 0.37 / 0.36;
  - the pre-plague total regained in 48 / 49 / 50 years;
  - total at year 500 against the same seed with events off: 1.00 / 0.98 / 0.98.
- **Outcome.** Stopped for the designer, by the rule. Option 2 (Dyer's moves to vacant good land) is held in reserve for this case.
Kind: acceptance test (designer).

## D60 — v4 E2: harbour silting (written; not yet gated)
- **Effect.** A silted harbour adds `silt_cost` (3) to every land–water step at the anchor and its eight neighbours: coast to sea, sea to coast, boarding and leaving a river boat, and changing mode at a navigable cell. The costs are kept in `State.silt`, so `Terrain` stays fixed.
- **Which port.** The settlement with the most transshipped trade whose anchor, or a cell next to it, borders sea no deeper than `silt_depth_m` (10 m) or a navigable river cell that flows into the sea. A port already silted is skipped.
- **When.** In historical mode the year is drawn when the window opens (`silt_from` 250, plus 0–`silt_window` 150), so the run's `Rng` stream is unchanged before year 250. In random mode it is drawn at `silt_interval` (200 years).
- **Snapshot.** The silt costs are not in the snapshot's `Transport`, so the hash with events off is unchanged.
Kind: model mechanism (designer's specification).

## D61 — Resettlement after a plague (designer decision; Dyer)
- **Rule.** For `resettle_years` (30) after a plague strikes, a settlement whose local yield is below its partners' median sends `resettle_rate` (2%) of its people a year to the partner with the most free food per head. Local yield is the land within 3 km of the anchor (D58). Free food is the unowned yield in the partner's cultivation reach.
- **Where.** It runs in growth, only with events on and after a plague. The plague's `apply` records the year in `State.last_plague`.
- **Tuning.** Both values stay at their starting values.
- **E1 gate** (Historical, silting held off, 20 seeds a map): **PASSED**.
  - Poor land deserted in 20/20 on every map: failure 248–300 of 0.23 / 0.23 / 0.17 in the lowest quarter of local yield, against 0.00 in the highest.
  - Plague drop 0.37 / 0.37 / 0.36, 20/20.
  - Recovery by 500: 1.84 / 1.78 / 1.91 of the pre-plague peak.
  - Top-10 against Off 20/20, and every v3 check passes. Map 3's tightest checks are largest share (16/20) and settlement count (17/20).
  - Runs passing every check: 15 / 19 / 11.
- **Reported:**
  - the pre-plague total is regained in 45 / 44 / 45 years;
  - the total at year 500 against the same seed Off is 1.05 / 1.04 / 1.05;
  - the famine dip is 0.23–0.24, regained in 15 years;
  - the slowest run took 27.3 ms a year natively.
Kind: model mechanism (designer decision).

## D60, gated — harbour silting: `silt_cost` 1.0
- **At the specified 3.0:** E2 failed on map 1 market spacing (14/20). The silted port is the region's largest place in every run. 100 years later it had fallen to a median rank of 9 / 33 / 4.
- **At 1.5:** map 1 spacing was still 14/20.
- **At 1.0:** the gate **PASSED**. Every check holds. Map 1 spacing is 16/20, and runs passing every check are 14 / 15 / 17.
- **Reported:** the port silts in 20/20 runs on every map. Its rank 100 years later is a median 4 / 5 / 1 (from 1), and it loses rank in 13 / 15 / **0** of 20 runs. On map 3 the largest city keeps its place: its land trade carries it.
Kind: event value tuned (implementer, as the brief allows).

## D62 — Silting leaves out the largest settlement (designer decision)
- **Rule.** The silting target is the busiest qualifying port other than the largest settlement: the Sandwich and Winchelsea pattern, since the capitals did not silt. `silt_cost` is back to 3.
- **Result.** Gated with E3 in one run (see D63): PASSED at `silt_cost` 3.
- **Reported:**
  - the port silts in 20/20 runs on every map;
  - its median rank goes from 5 to 12 / 11 / 10 a century later;
  - it loses rank in 18 / 18 / 19 of 20 runs.
Kind: model mechanism (designer decision).

## D63 — v4 E3: planted towns
- **Rule.** With events on, every `plant_interval` (50) years from `plant_from` a lord plants a town of `plant_pop` (200) people, with market size `plant_market` × `market_w0` (5×). The site is a founding candidate on or next to a road or navigable river, at least `plant_spacing_km` (10.7) from any place over 500, with the highest market access: Σ pop/1000 × exp(−`access_decay` × cost) over settlements within `c_max`.
- **Mechanics.** The planted town counts as founded and carries a `planted` flag. Its people come from outside the region.
- **Stability.** The planting year is left out of the stability check like the other events (D58). Early in a run, 200 people is a 6% jump in the total.
- **`plant_from` = 150 (1250), tuned.** Planting from year 50 pulled the top-10 overlap with Off down to 0.3–0.4 (gate 0.5). This happened whatever the size of the planted town (100 people at 1× W₀ gave the same), because a town planted while the region is filling redirects all later growth. From year 150 the overlap is 0.70; from 200, 0.80. 1250 still falls in the main period of English plantation (Beresford: c. 1150–1300).
- **Combined E2 + E3 gate** (Historical, 20 seeds a map): **PASSED**.
  - Every check holds. Top-10 against Off is 17 / 18 / 16 of 20, poor land deserted 20/20, and market spacing 18 / 18 / 20.
  - Runs passing every check: 15 / 14 / 16.
- **Reported:**
  - **Planted towns alive 100 years after planting: 0.82 / 0.70 / 0.57**, 100 judged a map. Two maps are above the brief's expected 30–70%. Planted at the best-access cells, most are supported by trade.
  - Plague drop 0.37; the pre-plague total regained in 44–47 years; total at 500 against Off 1.03–1.04.
Kind: model mechanism (designer's specification); `plant_from` tuned by the implementer.

## D64 — v5 O1: outside ports and imports (stopped: map 1 market spacing)
- **Ports.** One per open-sea map edge (a side with at least three sea cells), at the edge sea cell nearest the busiest coast. At the start of a run there is no traffic, so the busiest coast is proxied by the harbour cell with the most yield within 10 km. Imported maps read the ports from a file instead: Kent's are the Continent off Dover and the North Sea off the Thames mouth (`data/real/kent/outside_km.csv`, `MapParams.outside_file`). Ports are partners with id `OUTSIDE − k`, and every outside flow rides the settlement's stored path, so handling and transship income come with it.
- **Trade order in the trade system:** goods (D41), then imports, then grain to the ports, then the domestic grain market with what is left.
  - **Imports:** `import_demand` × market size, split among the ports a place reaches by `exp(−β/ρ_imp c)` (ρ 10), and paid in grain that leaves the map.
  - **Grain:** each port buys `outside_grain`, split among the sellers that reach it by remaining marketed grain × `exp(−β c)` and capped at that. The seller is paid in imports, which count in its market inflow.
  - **Import flows** are marked with `Flow.import`.
- **D25's outside world is retired.** Its parameters (`outside_enabled`, `outside_size`, `outside_beta`, `outside_out`) are removed; old files that name them still load, because unknown keys are ignored.
- **Default.** `outside_ports` is off until the gate passes, so the golden is unchanged (`2c5e9c5c90ab1d43`).
- **Tuning (Historical, map 1 unless noted):**

| `outside_grain`, `import_demand` | Seeds | Outside trade (median, passing) | Map 1 market spacing passing |
| --- | --- | --- | --- |
| 300, 0.05 | 2 a map | 0.025–0.030, 0/2 | — |
| 1,000, 0.1 | 2 a map | 0.07–0.08, all | 0/2 |
| 700, 0.07 | 20, all maps | 0.05, 20/20 every map | **13/20** (maps 2 and 3 pass) |
| 200, 0.1 | 20 | 0.031, 12/20 | 13/20 |
| 1,200, 0.02 | 20 | 0.063, 20/20 | 14/20 |
| 500, 0.03 | 20 | 0.031, 13/20 | 16/20 |
| 650, 0.04 | 20 | 0.041, 20/20 | 13/20 |

- **Reading.** Map 1 spacing is a marginal check: 18/20 at v4, and 15/20 or 16/20 at other versions. It moves between 13 and 16 with no trend in the trade values, which is seed noise around a pass rate near 70%. No outside-trade setting gives both checks the 75% margin on map 1. Every other check passes.
Kind: model mechanism (designer's specification); stopped for the designer.

## D65 — v5 O2: better ships (written; not yet gated)
- **Rule.** In historical mode, at each road step from `ship_from` (200, 1300) to `ship_to` (400, 1500), sea cost per km moves along a straight line to `(1 − ship_gain)` of `cost_sea`. It is written into the state's transport ratios through an event, and the cost grid is rebuilt then, so only at road steps. 1450 is announced in the chronicle.
- **Default.** `ship_gain` is 0 until O1 is settled. The brief's 0.3 goes in at the O2 gate.
Kind: model mechanism (designer's specification).

## D66 — Re-tune with outside ports on (designer decision)
- **Settings.** Outside ports on at `outside_grain` 700 and `import_demand` 0.07. One tuning pass over `shadow_weight`, `found_threshold` and `noise`, the spacing measure unchanged.
- **Screen.** A search of 24 samples, 3 maps × 4 seeds each, ranked these; the best point went to full validation.
- **New defaults:** `shadow_weight` 4.5327 (was 4.3476), `found_threshold` 0.4493 (0.3270), `noise` 0.3244 (0.3083).
- **Full gate** (Historical, 20 seeds a map): **PASSED**.
  - Every check holds.
  - Map 1 market spacing: 15/20, exactly at the 75% line; median 8.9 km. Maps 2 and 3: 20/20 and 18/20.
  - Outside trade: 0.051 / 0.052 / 0.055 of market inflow, 20/20 on every map.
  - Runs passing every check: 15 / 18 / 16.
- **Reproducibility.** `data/real/experiments/v4.json` pins the old founding values with ports off and reproduces v4's `2c5e9c5c90ab1d43`. The earlier version files pin them too.
- **Golden with outside ports, before O2:** `8ee85c10266b1dd2`.
Kind: change to the defaults (designer decision).

## D65, gated — better ships on
- **Setting.** `ship_gain` is 0.3, on by default.
- **Full gate** (Historical, 20 seeds a map, outside ports on): **PASSED**. Every check holds. Outside trade is 0.052 / 0.053 / 0.052. Runs passing every check: 14 / 17 / 17.
- **Coastal share of the people in places over 2,000, 1300 → 1600:**
  - with ships: 0.46 → 0.39, 0.20 → 0.22, 0.52 → 0.47;
  - without (`ship_gain` 0): 0.46 → 0.40, 0.20 → 0.22, 0.52 → 0.41.

  Ships raise the coastal share at 1600 only on map 3, by 6 points; maps 1 and 2 are unchanged. The fall from 1300 happens with or without ships.
- **Top ten of 1400 by 1600:** mean rank change 4.0 / 4.2 / 4.3, with 8 still in the top ten.
- **Golden** `4ce58fae34550933`. `data/real/experiments/v5-o1.json` (ships off) gives O1's `8ee85c10266b1dd2`.
Kind: model mechanism (designer's specification).

## D67 — v5 O3: the outside world in the view
- **New layer, `layers/outside.rs`.**
  - Imports are drawn as a sixth route colour, wine red. Only the 25 largest import flows are drawn, thin and partly transparent: every buyer has its own import path, and drawing all of them floods the sea.
  - Each outside port is a ship at the map edge.
- **Chronicle.** "1450. Larger ships now sail the coast." comes from the ships event at the midpoint of the drift.
- **Kent.** `data/real/kent/map-outside.json` is `map-resources.json` plus `outside_file`.
- **Watling Street mask.** `tools/real/watling.py` writes `watling.asc`: the Roman road cells within 2 km of the line London – Rochester – Canterbury – Dover, 157 cells. `hinterlands roads --mask` reports the share still road.
Kind: rendering and tooling.

## D68 — Rendering R8 merged (hover card, fuller chronicle)
- **What.** The graphics session's R8 (branch `claude/elegant-ride-g0sr1h`) is merged into v5 under the owner's standing permission for frontend-only merges: a hover card at the cursor, and a chronicle with filters, growth notes and fly-to.
- **Conflicts.** R8 was branched before v2, so its conflicts were with the v4/v5 view work:
  - the chronicle keeps R8's structure, and adds an "Events" filter and the lines for famine, plague, silting, planted towns and ships, in calendar years;
  - `view.rs` keeps both the event marks and R8's notes;
  - the panel list keeps the events and goods panels, with the chronicle second as R8 has it.
- **Check.** The model is unchanged: `?selftest=500` gives `4ce58fae34550933` (27.4 ms/year).
Kind: rendering (frontend merge).

## D69 — v6 S0: shadow prices (designer's brief, D70 plan)
- **Switch.** `market_shadow`, off by default. It runs after trade, writes only settlements' `price` and the market records in `State`, and nothing reads them back. The snapshot carries the prices only when the switch is on.
- **Form.** An entropy-regularised spatial price equilibrium (decision 3), with one price per place and good:
  - flows `x_ij = m_j · exp(θ (p_j − p_i − t_ij))`, with `t_ij = τ_g c_ij`, `τ_g = market_tau / ρ_g` (`market_tau` 0.002, Clark's carriage);
  - `θ = β / market_tau`, so that at equal prices the flows fall off with cost as the gravity trade does;
  - `m_j = W_j^α` for grain (decision 4, the market premium), and 1 for goods.
- **Supply and demand:**
  - **Grain:** each place's whole harvest; demand is need × `p^(−ε)`.
  - **Salt, iron and stone:** supply is what the sites sold this year; demand is the v2 demand.
  - **Imports:** sold by the ports without limit at a price of 1.
  - **Ports' grain:** they buy their fixed `outside_grain`.
  - **Elasticities:** grain 0.75, salt 0.2, iron 0.5, stone 1.0 (the brief), imports 1.0 (mine; the brief gives none).
- **Solver.** `market_passes` (8) Furness passes, warm-started from last year. Each pass:
  1. sets the price level so that the region's demand equals its supply, by bisection;
  2. shares each supplier's supply among its outlets by a max-subtracted softmax;
  3. moves each buyer's price by a damped Newton step (`market_damping` 0.7, at most `market_step` 1.0).

  The level step is needed because a buyer's own step only moves it against its rivals. Without it, a region-wide famine did not converge.
- **Result** (report in `docs/market/s0.md`):
  - 44.9 ms/year in wasm, with the same hash native and in the browser (`7f0884378d30344a`);
  - `b_P` 0.98–1.15 and R² 0.66–0.98, inside the ranges in 9 of 9 runs;
  - `b_Y` within ±0.03 of zero, and slightly positive in 5 of 9, because the model has no local yield variation;
  - price gaps about a quarter of transport on the trading arcs;
  - 1316 prices about double 1310's, region-wide.
- **Tooling.** The viewer's `?selftest=N&shadow=1` times it in the browser.
Kind: model mechanism behind a switch (designer's brief), off.

## D71 — v6 S1: goods by the price equilibrium (stopped: map 2 spacing)
- **Switch.** `market_goods`, off. When on, salt, iron, stone and imports trade by the S0 equilibrium in place of D41, and the outside ports' import sale moves into it. Grain is the numeraire at 1 and its trade is unchanged.
- **Supply.** At holders, `capacity × min(1, p_i)`: a site sells all its capacity at a supply price of 1, the old value, or more, and holds back below. This is my reading of "supply up to capacity, with the supply price clearing it". The ports sell imports without limit at 1.
- **Demand.** `D0 · p^(−ε)` (salt 0.2, iron 0.5, stone 1.0, imports 1.0).
- **Flows and solver.** Flows are `exp(θ (p_j − p_i − t))` on the stored paths, with a zero-cost arc for a holder's own use. Prices are solved by alternating exact log-space solves, suppliers then buyers, for `market_passes` passes, warm-started each year.
- **Payment.** The buyer pays `p_j q`, capped at its marketed share as in D41. The producer receives `p_i q` as a grain flow. Carriage and the entropy margin are lost. Own use moves no money.
- **Gate** (Historical, 20 seeds a map): **FAILED on map 2 market spacing (14/20).**
  - Every other check holds. Largest not a specialist: 20 / 19 / 20. Outside trade: 4.5–5.4%.
  - Release wasm 31.2 ms/year; the hash `3e18908a758b4937` is the same natively and in the browser.
- **Cause.**
  - **Iron:** at the v2 demand against coppice-limited output (D51), it prices at 22–80, with about 2,000 a year of demand unmet.
  - **Mines:** the rent makes them large, with specialist medians of 1,500–2,800 people against v5's 40–105. On map 2, which has the most mines, that crowds its spacing.
  - **Price gaps:** on goods arcs the median gap is 0.85–1.06 of transport.
- **Report:** `docs/market/s1.md`, with the options. `market_goods` stays off, and the golden is unchanged.
- **Tooling.** The viewer's `?selftest=N&goods=1` times S1 in the browser.
Kind: model mechanism behind a switch (designer's brief); stopped for the designer.

## D71, re-run — iron re-scaled to 0.0013 (designer decision, option 1). Stopped: sites producing
- **The value.** `demand_iron` 0.0013 puts the pooled median town price of iron at about 1.2: 1.0–1.4 on map 1, 5.6–6.9 on map 2, 0.93–0.96 on map 3.
- **Gate** (Historical, 20 seeds a map, `market_goods` on): **FAILED on "sites producing": 14 / 17 / 14 of 20.**
  - Market spacing now passes (17 / 19 / 20). Specialists are near v5 sizes (medians 106–334).
- **Cause.** With iron demand at a fifteenth, the specialist founding rule (D40) finds too little demand in quantity in reach of remote iron sites, and they stay unheld: 7 against 1 on map 1, 3 seeds.
- **Options** (`docs/market/s1.md`):
  - value the founding test at the delivered price under `market_goods`;
  - scale the works threshold;
  - re-tune founding.
- **State.** `market_goods` stays off, and the golden is unchanged.
- **Reporting fix.** A good no working supply reaches is recorded as unreached (price 0), not at the solver's floor.
Kind: stopped for the designer.

## D72 — The specialist founding test counts demand by value under `market_goods` (designer decision). Gate still short on map 1
- **Rule.** Under `market_goods` the works test (D40) sums `p_k · q_k · exp(−β_g c)` over buyers, at each buyer's latest price for the good (this year's, set by the trade that runs before founding), or 1 where no working site reaches the buyer.
- **Gate** (Historical, 20 seeds a map, `demand_iron` 0.0013): **FAILED, sites producing 14/20 on map 1.** Maps 2 and 3 now pass, at 20/20 and 17/20.
  - **Every other check holds:** spacing 17 / 20 / 18, largest not a specialist 20/20 everywhere, outside trade 4.7–5.4%.
  - **Specialists:** medians 132 / 44 / 91, maximum 545.
  - **Runs passing every check:** 10 / 19 / 14.
- **Iron at year 450,** median town price, 3 seeds a map: **map 1 1.15 / 1.19 / 1.35, map 2 2.71 / 3.26 / 9.61, map 3 0.90 / 1.16 / 1.87.**
- **Why map 1 is still short.** Its iron is only mildly dear, so valuing demand barely raises it. The same remote eastern sites stay unheld in most seeds: iron at (102,49) and (86,25), stone at (90,54) and (79,46). Under the v2 demand they were reached.
- **State.** `market_goods` stays off; the golden is unchanged. Stopped for the designer.
Kind: model mechanism (designer decision); stopped.

## D73 — Under `market_goods`, idle marginal sites are the equilibrium (designer decision); `market_goods` on by default
- **Rule.** Under `market_goods` the v2 gate's sites-producing quota is reported, not gated. The gate is instead that every good with a site on the map has at least one working site at the final year, in 75% of seeds a map (`every good worked` in `validate`).
- **Why** (the designer, `docs/foundation.md`, "S1 third gate"): when the working sites meet demand at about cost, the marginal ones correctly stay idle. That is the extensive margin, not a fault.
- **Gate** (Historical, 20 seeds a map, the defaults below): **PASSED.**
  - every good worked: 20 / 19 / 18 of 20;
  - sites producing, now reported: medians 0.73 / 0.80 / 0.73;
  - spacing 17 / 20 / 18; outside trade 4.7–5.4%;
  - runs passing every check: 16 / 18 / 15.
- **Defaults.** `market_goods` true, `demand_iron` 0.0013 (D71). Golden `ec7a24a7caeac414`, native and release wasm (`?selftest=500`, 31.9 ms/year).
- **Reproducibility.** `data/real/experiments/v5.json` pins `market_goods` off and `demand_iron` 0.02, and gives v5's `4ce58fae34550933`. Every earlier version file pins the same two values. `v4.json` also pins `ship_gain` 0, which it had lacked since D65, and gives v4's `2c5e9c5c90ab1d43` again.
Kind: validation (designer decision); defaults.

## D74 — v6 W1: local harvests (stopped: map 1 top-10 vs off)
- **Mechanism, as briefed:** `systems/harvest.rs`, a fresh smooth yield field each year, applied in cultivation after the fields are chosen. It is behind `local_harvests` (off), with `harvest_sd` 0.15, `harvest_km` 15, and the multiplier clamped to 0.5–1.5. Node values are drawn from the run's `Rng`, as u32 uniforms through Box–Muller in `libm`, one field a year.
- **Gate (Historical, 20 seeds a map): FAILED, top-10 vs off 12/20 on map 1.** Every other check passes on every map. Wasm runs at 30.9 ms/year, and the hash matches native.
- **Reported:** `b_Y` is identified but tiny (about −0.007). The multiplier's own CV is 0.095. Details are in `docs/market/w1.md`.
- **State.** `local_harvests` stays off, and the golden is unchanged at `ec7a24a7caeac414`. Stopped for the designer.
Kind: model mechanism; stopped.

## D75 — W1: weather fixed per seed, sd 0.15 at the cells (designer decision). Gate failed; stopped
- **Rule.** With `local_harvests` on, the node field for `harvest_years` (500) years is drawn at initialisation, before anything else draws from the run's `Rng`. Later years are drawn when reached. The interpolated field is divided by its own sd at each cell, so the multiplier has sd 0.15 at every cell.
- **Gate: FAILED.**
  - top-10 vs off: 13 / 10 / 13;
  - market spacing: 14 / 13 / 16;
  - failure rate: 8 / 7 / 19, medians about 0.40.
- **Why.** Noise with mean zero slows early growth, because losses are asymmetric. On map 1 seed 1, population at year 200 is 47.8k with sd 0, 27.6k at 0.095 and 14.8k at 0.15. The shocks then strike a smaller region. Details are in `docs/market/w1.md`.
- **State.** `local_harvests` stays off, and the golden is unchanged at `ec7a24a7caeac414`. Stopped.
Kind: model mechanism (designer decision); stopped.

## D77 — S2 design decisions (designer); the shadow market decoupled
- **The designer's decisions on `docs/market/s2-design.md`:**
  - rents are paid in value at `rent_share`;
  - fees are paid by the buyer outside the routing price, which is a deliberate departure from a pure equilibrium, so that break of bulk is not counted twice;
  - `p_floor` is 0.25 and `c_max` is `food_cap`.
- **The shadow is decoupled (this commit).** `market_shadow` now writes its own fields: `shadow_price` per settlement and `shadow_arcs`, beside `port_price` and `market_residual`. The model reads none of them, and `price` belongs to the trading equilibria (S1 goods, S2 grain).
  - Before this, the shadow overwrote S1's goods prices. D72's founding test and S1's warm start then read them, so a shadow-on run differed (`1ed04bc733038208` against `ec7a24a7caeac414`).
  - `tests/shadow.rs` checks that a run with the shadow on matches one with it off, prices aside.
  - The snapshot carries `shadow_price` when the shadow is on. The probe prints S1's goods prices and the shadow's grain price.
Kind: designer decision; fix.

## D78 — v6 S2 first build: grain by equilibrium (stopped: no towns form)
- **Built as in D77,** behind `market_grain` (off). The golden is unchanged.
- **Changes from the note, made in building:**
  - rent is taken in kind, and farmers eat their own grain first;
  - the outside ports buy up to `outside_grain` at a world price, `port_world_price` 0.6 (elasticity `port_elast` 4 above it);
  - the solver gets a level step, a common price shift that leaves the flows unchanged, with supply prices stopping at the floor;
  - goods are paid in value from last year's income, with no grain payment flows;
  - each flow's buyer pays the handling and transshipment fees its path earns, using the same counts as `through` and `transship`, so fees paid equal fees earned.
- **Solver:** grain residual about 0.0002 in 8–11 passes, every year. Native cost +3 ms a year.
- **Result: FAILED before the gate.**
  - Every place still cultivates for its own need plus rent, so rent grain has no buyers.
  - About half of each harvest is unsold at the floor, W's real inflow is a fraction of v5's, and no place passes about 600 people (map 1 seed 1).
  - v5's K (3.39) was tuned on gross inflow that included burned grain (P3).
- **Options for the designer** are in `docs/market/s2.md`. Recommended: market establishments demand `K × W` grain, paid from rents and fees.
- **Experiment switch:** `grain_market_people` (off) makes fields feed only pop − W.
Kind: model mechanism behind a switch; stopped.

## D79 — S2 second build: the establishment demand (designer decision). Gate failed; stopped
- **Rule.** A market's establishment buys `K × W` grain with elasticity ε and no satiation, paid from rents, goods sales and fees. Fields feed pop − W, and the farmers buy beyond their own grain from their sales. W follows `ε (D − K W)` with `D` = real income / p.
- **Tuning round** (`rent_share`, K, ε, α; table in `docs/market/s2.md`): no setting forms a town over 2,000. The best is K 10, ε 0.04, rent 0.3, in `data/real/experiments/s2-best.json`.
- **Gate on the best: FAILED** on every map: places over 2,000 and 5,000, rank-size slope, spacing and siting, all 0/20.
- **Clark:** R² 0.25–0.37 (fails); `b_P` 1.10–1.18; `b_Y` mostly positive.
- **State.** `market_grain` stays off, and the golden is unchanged. Stopped.
Kind: model mechanism (designer decision); stopped.

## D80 — S2 stopped (designer); S2′ grain relief built and gated
- **The designer's decision.** S2 is stopped by the proposal's stop rule. `market_grain` stays off, with its code kept as the record. S1 stays the default, and Clark is reported on shadow prices only.
- **S2′, grain relief** (`grain_relief`, off). After trade, places in deficit draw their deficit, unpaid and shopping-model style, on what partners hold above `food_cap × need`. Relief counts as food but not as market inflow, and loads routes without handling.
- **Gate: PASSED** with `shadow_drain` 0.025 (from 0.015). With the defaults, map 1 spacing was 14/20.
  - Wasm 30.9 ms/year, with the same hash natively and in the browser (`f5b32f88ee3fc22f`).
- **Reported:**
  - relief of 1,500–2,000 a year;
  - 86–88% of deficit met in normal years, but 6–8% in 1315–17, so the famine dip is unchanged at 0.24–0.25;
  - failure rate 0.47–0.48 (S1 0.46–0.48);
  - 5,300–5,800 a year still lost above `food_cap`.
- **Details:** `docs/market/relief.md`.
Kind: model mechanism behind a switch (designer decision).
