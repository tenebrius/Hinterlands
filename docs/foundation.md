# Hinterlands — Foundation Document

Foundation document for the implementer. Owner: Divish Shamloll. Last revised 21 October 2026: v1.2 (D34–D35) and the v2 specification.

## Purpose and scope

Hinterlands is a self-running regional simulator: given a map with terrain, water and resources, plus a few starting villages, it shows where settlements grow, shrink, appear and die over several centuries, and where fields are cultivated. It runs in the browser, written in Rust compiled to WebAssembly.

The unit of simulation is the settlement, with a single population number, not households or people. Land is a grid of cells, each with a yield, a resource and a travel cost. Trade is modelled as flows between settlements, not as individual merchants.

It is better if every mechanism must come from a model that has been researched and recognized. The audience for this document is the implementer; it fixes what the simulation must do before it fixes how.

The simulation is pre-industrial throughout: no railways, no steam, no fossil fuel. Transport by foot, cart, river boat and sailing ship sets the economics.

## What we expect to see

A run should look like a medieval region filling in over 300–500 years: villages spread across farmland first, then a few of them become market towns, then one or two of those become cities on the best river and coast sites. Each behaviour below is a requirement, with the model it comes from. The basis column says how strong that footing is: **Proven** = the model reproduced this against historical or archaeological data; **Recognised** = a standard published model predicts it, not calibrated for our setting; **Heuristic** = our own rule, grounded in qualitative history.

| Behaviour | What the viewer sees | Model / evidence | Basis | Version |
| --- | --- | --- | --- | --- |
| Villages follow food and water | Early settlements sit on good soil next to rivers, springs or lakes; dry or poor land stays empty | Artificial Anasazi (Dean et al. 2002), Village Ecodynamics Project (Kohler et al.) | Proven | v1 |
| Fields form rings around settlements | Cultivated cells cluster nearest each settlement and extend along cheap routes (rivers); fuel wood is kept close | von Thünen (1826); VEP calorie-cost placement | Proven | v1 |
| Towns grow where transport is cheap | The largest settlements sit on navigable rivers, sheltered coasts and route crossings, not on the best soil | Bosker & Buringh 2017: road hub +6.9 pp, river +1.2 pp, sea +0.7 pp per century | Proven | v1 |
| Break-of-bulk points attract towns | Settlements appear at river mouths, heads of navigation, fords and the foot of mountain passes | Bleakley & Lin 2012 (portage sites); Fujita & Mori 1996 | Proven | v1 (falls out of the cost grid, no special rule) |
| Towns keep their distance | Market villages are about a half-day walk apart (roughly 10 km); no new market survives close to a bigger one | Bracton’s 6⅔-mile rule; Bosker & Buringh negative effect within 20 km | Proven | v1 |
| A few places pull ahead | From many similar villages, a handful become dominant towns; the rest stay small or feed them | Rihll & Wilson 1987 on 109 Greek sites; Evans & Rivers 2017 | Proven | v1 |
| Trade follows gravity | Flow between two towns rises with both sizes and falls with travel cost; water links carry most volume | Barjamovic et al. 2019; ORBIS cost ratios | Proven | v1, one abstract trade good |
| Many foundings, many failures | Most new markets fade within a century or two; survivors are the well-placed ones | English market charters: 39% of 1300-era markets survived to c. 1600 | Proven | v1 |
| Towns outlive their reason | A town founded at a portage or ford persists after a bridge or new route removes the advantage | Bleakley & Lin 2012; Dalgaard et al. 2022 | Proven | v1 (population inertia only) |
| City sizes follow a rank-size curve | Sorted by size, settlements roughly halve from rank to rank (Zipf); a capital may sit above the curve | Zipf / rank-size literature; Barjamovic et al. 2019 | Recognised | v1, as a check, not a rule |
| Outcomes are partly contingent | A different random seed keeps the big rivers and coasts busy but may swap which town becomes the regional centre | Evans & Rivers 2017 (“Was Thebes necessary?”) | Proven | v1, as a check |
| Roads create towns, towns create roads | Once a road joins two towns, settlements appear at its crossings; traffic strengthens the road | Dalgaard et al. 2022; Azgaar generator loop | Recognised | v1 simplified: roads are least-cost paths between the top trading pairs, recomputed every 10 years; traffic-based upgrading is Later |
| Resource sites spawn specialist settlements | Mines, salt pans and quarries get small settlements that trade with the nearest town | Weber 1909 | Recognised | Later |
| Goods and production chains | Ore smelted at the mine, finished at the town; towns between ore and people grow | Weber 1909; Barjamovic et al. 2019 on complementarity | Recognised | Later |
| Lords plant towns | A town is founded at a chosen site regardless of score; it lasts only if the market logic supports it | English market charters; German Bannmeile | Heuristic | Later |
| Way-stations on long routes | Small stops about a day’s travel apart on routes through sparse land | Christaller transport principle; caravanserai spacing | Heuristic | Later |
| Shocks reshape the map | Drought, plague and war change yields, population and the defence weight | Anasazi climate reconstruction; medieval demography | Heuristic | Later |

v1 is four systems over a grid: travel cost, cultivation, gravity trade with growth, and founding/failure. Everything marked Later is excluded from v1 scope and must not be built before v1 passes its validation targets. The rule for each v1 system is that it is built and checked alone before the next is switched on.

## On screen

The map updates once per simulated year at the chosen speed. Everything drawn is derived from simulation state (population, cultivated cells, trade flows, the cost grid), so view layers can be added without touching the model. The rule: if it can be recomputed from the state at any year, it is a view layer and cannot break the simulation.

**v1 view**

- Terrain base: elevation shading, rivers with width by flux and the navigable stretch in a stronger blue, coast and sea, forest, grassland and poor land by colour. Static.
- Fields: cultivated cells tinted; the ring grows around a settlement as it grows and stretches along rivers. Forest cleared for fields is visibly lost around larger towns. Abandoned fields revert to scrub, then forest.
- Settlements: a built-up footprint of cells, not a circle. Each settlement has one canonical anchor cell used for all distances, scoring and trade. The footprint is `population / density_per_cell` cells filled outward from the anchor by a fixed attractiveness (flat, next to water, along a road); built cells stay built, so the town grows by accretion, and shrinks from its least attractive edge, leaving ruins for a while. Footprint cells are never cultivated. The footprint is rendering only and feeds nothing back.
- Routes: roads brown, water routes blue, thickness proportional to the yearly trade flow on the segment. Trade is animated as moving dots: yearly flow divided by cargo per trip (about 1 t per cart, 20–30 t per barge, more per ship) gives trips per year, and the renderer spawns dots at that rate along the stored path at the mode’s speed. Dots are capped per route with cargo per dot scaled up when flows are large. The simulation never sees the dots.
- Names: every settlement gets a generated name at founding, kept for its life and reused in the log and inspector. Map generation names the rivers, mountain ranges, seas, bays and larger forests. Labels are a view layer with a toggle per class (settlements, water, relief), sized by rank so hamlets only show when zoomed in. Plain generated names are enough for v1; site-based suffixes (-ford, -mouth, -minster) are the 1.x refinement.
- Legend: a panel listing every colour and symbol currently on screen (terrain classes, water, fields and their states, settlement ranks, route types, overlays), built from the active layers so it stays correct as layers are toggled.
- Controls: play, pause, speed, year counter, seed. Panels: a live rank-size chart and a log of foundings and failures with year and cause.
- Overlays: site-score heat map (where the next settlement would appear), travel cost from a clicked cell, cultivation catchments.
- Sliders for the parameters that matter: transport cost ratios, growth exponent, trade distance decay, market spacing.
- Hover: settlement (name, population, rank, year founded, food produced vs needed, main trading partners, 50-year trend); cell (terrain, yield, owner settlement, travel cost to nearest town, site score and its reasons); route (endpoints, mode, flow).

**1.x derived layers**, in rough order of effort, none required for v1:

| Layer | Drawn from | Note |
| --- | --- | --- |
| Field strips with rotating fallow | cultivated cells, year mod 3 | strips oriented away from the nearest road or river |
| Pasture and managed-woodland rings | von Thünen rings per settlement | animal dots, coppice texture |
| Clearing frontier | cells turned forest to field in the last few years | stumps before crops |
| Persistent hedgerows and old field boundaries | cultivation history | dead villages leave a ghost pattern |
| Site-earned buildings | rank, flow, adjacency | mill on a fast river, quay and warehouses at a water route, market square, church then cathedral, walls at city rank |
| Fords becoming bridges | road–river crossings, flow threshold |  |
| Smoke | population, fuel use, season | charcoal burners in the woodland ring |
| Procedural names from the site | site features | -ford, -bridge, -mouth, -port, -mill, -by, -minster |
| Seasons within the year | month | fields green, gold, brown, bare; rivers high in spring; sea dots stop in winter (as ORBIS) |
| Chronicle panel | founding and failure log | plain prose: “1132. The market at Ashford failed; its trade went to Wyeford.” |
| Parish-like boundaries | cultivation catchments | faint lines |
| Footpaths | least-cost paths village to fields and to neighbours | desire lines on the existing grid |

Three of these may feed back into the simulation at low cost, each a one-line change and a real historical mechanism: a bridge makes its crossing edge cheap; walls add a small defence term to the site score; a mill adds a few percent to that settlement’s yield. Everything else stays derived.

## Research basis

v1 rests on six models, each decades old, published, and reproduced by others. Everything in the specification maps back to one of them; the implementer should not need to invent a mechanism.

| Model | What it gives us | Where it is used | Standing |
| --- | --- | --- | --- |
| Least-cost paths on a cost surface (Tobler 1993; ORBIS, Scheidel & Meeks 2012) | Travel cost between any two cells over terrain, rivers and sea, with mode cost ratios (sea 1 : downriver 5 : upriver 10 : cart \~50 in the Roman Price Edict; land : river : sea roughly 8 : 4 : 1 in medieval England) | The cost grid, roads, catchments, trade decay, break-of-bulk sites | Standard in archaeology and GIS; weak at predicting roads on flat open land (31% within 250 m in lowland Britain, 65% in highland) |
| von Thünen land use (1826) | Where fields, woodland and pasture sit around a settlement: nearest and cheapest to reach first, stretched along cheap transport; a town’s size bounded by what it can reach | Cultivation and fuel, the food ceiling on growth | Textbook model; VEP and Artificial Anasazi place farm plots by the same cost logic and reproduce real settlement histories |
| Gravity model of spatial interaction (Wilson 1970; Huff 1964) | Flow between places rises with both sizes and falls with travel cost | Yearly trade flows, food delivered to markets, route thickness | Estimated on Bronze Age merchant records with a distance elasticity close to modern trade (Barjamovic et al. 2019) |
| Rihll–Wilson urban dynamics (Rihll & Wilson 1987; Harris & Wilson 1978) | Settlement size grows with inflow and decays with maintenance cost; with increasing returns a few sites dominate | The growth rule and the emergence of hierarchy | Regenerated Athens, Corinth and Argos from 109 equal sites; results sensitive to the decay function (Evans & Rivers 2017) |
| Central place spacing (Christaller 1933) and the New Economic Geography urban shadow (Krugman 1991; Bosker & Buringh 2017) | New settlements are suppressed near an existing larger one and favoured at a middle distance; markets about a half-day walk apart | The founding score: shadow and access terms, minimum spacing | Spacing documented in English market law (Bracton, 6⅔ miles) and measured for 259,776 European locations 800–1800 |
| First-nature site factors (Bosker & Buringh 2017; Bleakley & Lin 2012) | Relative weights for water, river, coast and route crossings in founding; persistence of towns after their advantage is gone | Which villages become towns, through growth on market inflow and handling income; population inertia. Not founding: the Kent test showed these weights are city-scale, and hamlets are scored on food and water | Regression on all 1,588 European cities over 1,000 years; portage-site persistence in the US |

Two results shape the design rather than any single mechanism. First, founding is common and failure is the norm: only 39% of English places with market rights by 1300 were still market towns around 1600, so the model must found often and let most new places fail. Those figures count markets that lapsed, not places that vanished: a failed market falls back to the village its fields feed, and only places that dwindle to a handful of people are removed. Second, outcomes are partly contingent, mostly in the middle tier: the same geography can raise different market towns on different seeds, and that is correct behaviour, not a bug. The regional centre may be the same on every seed when one site is clearly the best; the largest cities are the most persistent places in the record.

## Model specification

v1 is a grid of cells, a list of settlements, one cost grid, and a yearly tick of six steps. All numbers below are defaults for a 128 × 128 map of 1 km cells; every one of them is a parameter. This specification was corrected after the Kent real-terrain test (D23–D29): market size is now its own state, as in the published Rihll–Wilson model; places earn income from trade they handle; hamlet founding no longer scores transport; and water supply is brook-scale. The earlier forms are kept in the decisions log, not here.

**World (static after generation)**

| Per cell | Type | Meaning |
| --- | --- | --- |
| elevation, slope | f32 | from the heightmap |
| water | enum | None, Stream, NavigableRiver, Sea; Navigable = flux above a threshold and slope below one |
| yield | f32 0–1 | food output of a cultivated cell as a share of prime land |
| forest | f32 0–1 | wood cover; cleared when cultivated, regrows slowly when released |
| resource | enum | None or one point resource (v1 treats them all as the one abstract trade good) |
| harbour | bool | coastal cell touching at most two sea cells |

Edge costs, in km-equivalents, computed once and recomputed only when a road or bridge changes:

| Move | Cost per km | Note |
| --- | --- | --- |
| Land, flat | 1.0 | multiplied by Tobler’s slope factor, `exp(3.5 * abs(slope + 0.05)) / exp(0.175)` |
| Along a road | 0.5 | roads marked by the road step |
| Along a navigable river, downstream | 0.2 | upstream 0.4 |
| Sea, cell to cell | 0.08 |  |
| Crossing a river (perpendicular) | +5 fixed (+1 for a stream) | 0 at a ford; fords are cells where slope is low on both banks. Bridges are not yet applied (a v3 feedback); a road crossing a river still pays the fixed cost |
| Stepping between land and water | +3 fixed | transshipment; this is what makes ports |

**Settlement state (dynamic)**

| Field | Meaning |
| --- | --- |
| anchor | the canonical cell; all distances use it |
| P | population, f32: farmers plus market people |
| W | market size, f32: the Rihll–Wilson attractiveness, a state of its own, never derived from P or from land shortage; starts at a common small value `W_0` and has a floor so it cannot reach zero |
| founded, last\_fed | years; `last_fed` is the last year food covered need |
| fields | the set of cultivated cells it owns |
| cost\_to | travel cost to every other settlement, from a bounded Dijkstra, refreshed every 5 years |

Rank is derived from P: hamlet < 100, village < 500, town < 2,000, city above.

**Yearly tick**

Values given here are the tuned defaults for the generated maps, rounded; `docs/parameters.md` is authoritative.

1. **Cultivation.** Need is `P × 1` food units. Starting from the anchor, claim free cells in order of travel cost until yield covers need or the cost radius `R_cult` is reached (1.3 on the generated maps); one prime cell feeds 150 people. Release the farthest cells when need falls. Claimed cells lose forest at 20% a year; released cells regain it at 2% a year. Produced food `F_i` is the sum of yield over owned cells × 150.
2. **Trade flows.** For every pair within cost `C_max` (30 on the generated maps):

```latex
T_{ij} = O_i \, \frac{W_j^{\alpha} e^{-\beta c_{ij}}}{\sum_k W_k^{\alpha} e^{-\beta c_{ik}}}
```

`O_i` is what i sends to market: a fixed share `market_share` (about 0.45) of its produce plus the output of any resource cells it owns, sent whether or not any market needs it. The attraction is the market size `W_j`, not population; `α` about 1.1, `β` about 0.14 on the generated maps. The sum includes j = i, so a place's own market takes its share. Food arrives less a loss of `transport_loss × c_ij` and is consumed at the market it reaches; it is not passed back to senders in deficit. Each flow is assigned to the stored least-cost path between the pair; summing per edge gives route load. Gross market inflow `D_j` is everything delivered to j, including its own share.

3. **Growth.** Two equations, one for the market and one for the people. The market size follows the published Rihll–Wilson dynamic:

```latex
W_j \leftarrow \max\big(W_{\min},\; W_j + \varepsilon\,(D_j + H_j - K\,W_j)\big)
```

`D_j` is gross market inflow from the trade step. `H_j` is handling income: a margin `handling_margin` on goods that pass j's anchor on a stored path or are transshipped there (land–water or water–land at j), counted as food-equivalent. `ε` is about 0.15 and `K` is tuned per map set (3.5 on the generated maps) so an isolated fed village's market neither grows nor shrinks; `W_min` is a small floor. All places start at the same `W_0`, so hierarchy comes from competition, not from the starting state. Then population: farmers are fed by the fields; market people are fed by `D_j + H_j`; `P_j` grows at `r = 0.02` a year toward what both can feed, capped at 5% growth a year, and when underfed loses a third of the people its food does not feed, `(1 − ratio) × P / 3`, each year (v1.2, D34). Newcomers to a market join the market people rather than becoming farmers, so an inland market no longer erases its own W by claiming more fields.

4. **Founding.** Every 5 years, score every free land cell that has a brook of at least 5 km² drainage or the coast within 3 km and no settlement within 3 km. At 1 km cells this admits nearly all land; dry ridge tops are kept out by the food term, not by the water filter. Hamlets are scored on food and water only; transport decides which villages become towns, through growth, not founding:

```latex
S(x) = w_{f}\,\text{food} + w_{w1}\,\text{water}_{0} + w_{w2}\,\text{water}_{1} - \text{shadow} + \text{access} + \eta
```

`food` is free yield reachable within `R_cult`, weighted by `w_food`; `water_0` = 1 if a stream, river or coast is in the cell, `water_1` = 1 if within 1 km, with weights `w_w1`, `w_w2` per the params file; `shadow = shadow_weight × Σ (P_k / 1000)` over settlements within travel cost 10 (`shadow_weight` about 4.35); `access = 0.3 × Σ (P_k / 1000) e^{−0.05 c}` over settlements from cost 10 to `C_max`; `η` is uniform noise up to about 0.3. There are no transport flags (hub, river, coast, road) in founding: Bosker & Buringh's weights are for cities, and villages followed farmland and water. Found hamlets of 30 people at the best-scoring cells above the founding threshold, at most `max_found_per_step` (6) per 5-year step. Migrants come from the nearest settlement with a food ratio above 1.1.

5. **Failure.** A settlement with P below 20 for `fail_low_years` (12) consecutive years is removed; its fields are released and its people move to the nearest fed settlement. Hunger alone never removes a place: an underfed town sheds people (step 3) until its food supports it, so a market town that loses its trade becomes a village, not a gap (v1.2, D34).
6. **Roads.** Every 10 years, take the 20 pairs with the largest `T_ij` whose path is not already a road and mark that path as road. Roads never decay in v1. Recompute edge costs and `cost_to`.

**Initial conditions.** Eight to twelve hamlets of 50 people, placed at the highest `food_4km` cells that pass the water rule, at least 8 km apart. Run length 500 years.

**Determinism.** One seed drives map generation and the noise term `η`. Two runs with the same seed and parameters must produce identical state; this is the basis of every test.

## Achievability review

Verdict: v1 as specified is achievable by one developer in roughly 6–8 weeks of part-time work, about half of it tuning. The code is small; the risk is in the interaction of the feedback loops and in losing discipline about scope. The table lists what could sink it and the guard against each.

| Risk | Why it happens | Guard |
| --- | --- | --- |
| One town swallows the map | α too high or β too low in the growth loop; trade inflow unbounded | Cap inflow growth per year (say 5%); α slider defaults to 1.15 and the headless run reports the rank-size slope every 50 years |
| Nothing happens | Shadow too strong, founding threshold too high, or K too large | Site-score overlay shows the ceiling at a glance; founding logs the best score each step even when nothing is founded |
| Oscillation | Cultivation claims and releases cells every year; trade flips between partners | 3-year smoothing on population loss; hysteresis on cultivation (release only when need drops 10%) |
| Slow ticks | Dijkstra from every settlement every year; all-pairs flows | Bounded Dijkstra (cost cap `C_max`) every 5 years; pairs filtered by cost; 200 settlements × 128² cells is well under 100 ms in Rust |
| Roads look wrong on plains | Known weakness of least-cost paths on flat land | Accept it in v1; roads come from trade pairs, not terrain prediction; traffic-based upgrading is Later |
| Scope creep | Goods, politics, agents, pretty rendering before the model behaves | Milestones gate on validation targets; Later items are not opened until v1 passes; derived view layers never touch state |
| Untestable “looks wrong” | No definition of correct | Validation targets are numeric and checked headless; determinism makes regressions reproducible by seed |

Two things in this specification are genuine unknowns and may need a different form once tried: the exact growth equation (the food cap and inflow term may need rebalancing) and the founding threshold. Both are single numbers behind sliders. Nothing else is expected to change shape.

What makes this different from a project that stalls: there is no agent layer, no goods layer and no political layer, each of which multiplies state and tuning; all distances come from one cost grid; and each system has a pass/fail check before the next is switched on.

## Validation targets

v1 is done when the checks below pass on three generated maps, judged by pass rate across seeds, not by a single run: each check must pass in at least 75% of 20 seeds on every map, and the seed-aggregate checks (contingency, median counts) are computed over the same 20. A single seed lands on a range edge by chance; pass rates measure the model. The default run (map 1, seed 1) is still the golden snapshot for the determinism test. The checks run as assertions in the CLI, so a parameter change that breaks one is caught by seed. Ranges are scaled from medieval England to a 16,000 km² map and are deliberately wide; the point is to catch the two failure modes (one city eats the map, or nothing happens), not to match a number.

| Check | Target | Catches |
| --- | --- | --- |
| Determinism | Same seed and params give an identical final snapshot hash | Hidden nondeterminism |
| Settlement count at year 500 | 150–350 | Founding too weak or too strong |
| Settlements over 2,000 | 5–12 | Hierarchy absent or runaway |
| Settlements over 5,000 | at least 1 and at most 5 per run; the median over 20 seeds is reported (gated at ≥ 2 until v2, D48) | Hierarchy absent or runaway; a region with one city over 5,000 is historically normal |
| Largest settlement’s share of total population | under 25% | One city eats the map |
| Rank-size slope (log size vs log rank, top 30) | between −0.8 and −1.3 | Flat scatter or primacy |
| Market-town spacing (nearest neighbour among settlements over 500) | median 8–15 km | Spacing rule wrong |
| Siting of large towns | at least 70% of settlements over 2,000 touch a navigable river, coast or road crossing | Transport not driving location |
| Failure rate | 40–70% of settlements founded during the run are gone by year 500 | Culling too soft or too hard |
| Forest near cities | under 30% cover within 5 km of any settlement over 5,000 | Fuel ring not clearing |
| Stability | total population changes less than 5% year on year after year 100 | Oscillation |
| Contingency | over 20 seeds on the same terrain, the ten largest places differ between runs: for each pair of runs, the share of one run’s ten largest with one of the other run’s ten largest within 5 km; the mean over all pairs is at most 0.80 (gated, v1.2). Reported, not gated: the share of runs whose largest city lies outside the modal cluster, the share of seeds with a settlement over 5,000 on the main river corridor or the coast within 15 km of it, and the distance from the largest city to the main river mouth. Contingency lives in the middle tier; a clearly best site may hold the largest city in every seed, and the per-seed siting check carries the transport claim | Too deterministic: geography alone decides the hierarchy |
| Performance | one year under 50 ms in release WASM | Budget |

**Real-terrain tests** (a DEM of a documented region, scored against its medieval towns) are not a v1 gate, but the CLI accepts the ground-truth file from the start. The standing rule, from the Kent test (D23–D31): judge placement only where the inputs carry signal, and judge count and spacing where they do not. On Kent, terrain and soil hold no information about where the small inland towns sat (feature scores 0.20–0.48 against real sites, 0.5 being none); their sites came from roads, market spacing and lordship. So for towns under about 1,000 the yardstick is count and nearest-neighbour spacing (real Kent: 29 inland places over 500, 10.6 km apart), and placement is judged for ports, river towns and towns over 1,000, using AUC against lowland cells, which cannot be fooled by “towns sit low”. Second-nature inputs such as an initial road network are data in `Terrain`, not model changes, and are the next source of placement signal.

## Architecture and technical constraints

The Cargo workspace is \`hinterlands\`, with crates \`hinterlands-core\`, \`hinterlands-mapgen\`, \`hinterlands-web\` and \`hinterlands-cli\`, referred to below by their short names. The design has one load-bearing decision: the simulation is a pure Rust library with no I/O, and everything else reads from a read-only snapshot of it. That boundary is what keeps v1 small and lets 1.x add goods, events and view layers without touching the core.

```text
Params + seed ──▶ core crate (pure Rust, no I/O) ──▶ Snapshot (read-only, serde, per year)
   (JSON; mapgen        World = Terrain (static) + State (dynamic)           │
    or DEM)             systems, each step(&mut State, &Terrain, &Params):   │
       │                cultivation · trade flows · growth ·                  ▼
       │                founding · failure · roads + costs           web crate (wasm, egui)
       │                              │                              reads Snapshot only:
       └──────────────────────────────▼                              terrain, water, forest ·
                            cli crate (headless):                    fields and catchments ·
                            run N years, metrics,                    settlement footprints ·
                            determinism tests                        routes and trade dots ·
                                                                     overlays, hover, panels
                                                                     (layers never write back)
```

*Architecture: three crates, one snapshot boundary.*

Params and a seed go into the core; the core produces a Snapshot each year; the viewer and the CLI only ever read snapshots.

**Crates**

| Crate | Depends on | Contains |
| --- | --- | --- |
| `core` | `rand`, `serde` only; no wasm, no I/O | `Terrain`, `State`, `Params`, `Snapshot`, the six systems, `fn step_year(&mut State, &Terrain, &Params, &mut Rng)` |
| `mapgen` | `core` | `fn generate(seed, &MapParams) -> Terrain` as a pipeline of passes, and `fn import_dem(path, cell_km) -> Terrain` |
| `web` | `core`, `mapgen`, `eframe`/`egui` | the app: map view, panels, layers, parameter UI; built with `trunk` |
| `cli` | `core`, `mapgen` | `run --seed N --years 500 --params p.json`, metrics per 50 years, golden-file determinism test, validation checks, scoring against a ground-truth file |

**Data layout.** The world is two structs. `Terrain` is what map generation produces and the tick never changes: elevation, slope, water class, baseline yield, resources, harbour flags, plus `cell_km` and the grid dimensions. `State` is what the tick changes: forest, owner, built, road flags, the cost grid, the settlements, the flow matrix, the year, and the current transport cost ratios. Both are struct-of-arrays, one `Vec` per property indexed by cell id. The cost grid is a `Vec<[f32; 8]>` of edge costs per cell. No entity-component framework, no trait objects in the hot path, no game engine. Dijkstra is bounded by a cost cap and writes into a reusable buffer. Nothing in `core` assumes a cell size or a map size: every cost constant is per km and scaled by `cell_km`.

**Systems.** Each system is a free function in its own module with the same signature and an `enabled` flag in `Params`. `step_year` calls them in the fixed order from the specification. A system may read any state and write only its own outputs (cultivation writes owners and forest; trade writes the flow matrix and route loads; growth writes population; founding and failure add or remove settlements; roads write road flags and trigger a cost rebuild). This is the rule that makes each system testable alone.

**Snapshot.** A `#[derive(Serialize)]` struct copied from `State` once per year: dynamic cell arrays, settlements with population and anchor, flows above a threshold with their stored paths, and the event log for the year. `Terrain` is sent once at load, keyed by its hash, and snapshots reference it. The viewer receives snapshots over the wasm boundary as bytes (`bincode`) and keeps the last one. The same bytes are what the CLI writes to disk for replay and what tests compare. Footprints, trade dots, seasons and every 1.x layer are computed from the snapshot inside the viewer.

**Determinism and tests.** One `Rng` (`rand_chacha`) seeded from the seed; no `HashMap` iteration in the core (use `BTreeMap` or sorted `Vec`); no floats from wall time. The CLI test runs a fixed seed for 500 years and compares the final snapshot hash to a stored value. Validation targets run as assertions on the headless run.

**Params.** One `Params` struct with `serde` and defaults, loaded from JSON. The viewer builds its sliders from the same struct by listing the fields with a small macro, so a new parameter is one line in `core` and appears in the UI with no viewer change.

**Performance budget.** 128 × 128 cells, up to 300 settlements, one year in under 50 ms in release WASM, so 20 years per second at normal speed. Cost refresh every 5 years is the only step that approaches that; everything else is linear in cells or quadratic in settlements within the cost cap.

**Extension points for 1.x and later**, each designed in now and left empty:

- Goods: `O_i`, flows and stocks become arrays indexed by a `GoodId`; v1 has one good, so the arrays have length 1.
- Events: a `Vec<Event>` applied at the start of `step_year` by `Event::apply(&mut World)`; v1 has no events, so the loop is empty. Shocks, lords planting towns and bridges all arrive this way.
- Specialist settlements: a `kind` field on `Settlement`, v1 always `Agrarian`.
- Politics: a `domain` id per cell, unused in v1, so borders can be added as a system that reads and writes only that array.
- View layers: a `Layer` trait with `fn draw(&self, snap: &Snapshot, ctx: &Canvas)`; the renderer iterates a list, so a layer is a new file and one line of registration.
- Traffic-based roads: the edge load array already exists from the trade step; upgrading roads from it is a change inside the roads module only.

**Decisions taken now for the long-term plan**, each cheap in v1 and costly to retrofit:

1. Terrain and State are separate structs, so a DEM importer and a generator produce the same `Terrain`, and snapshots carry terrain once.
2. Cell size and map dimensions are parameters; no system assumes 1 km or 128 cells.
3. Transport cost ratios are copied from `Params` into `State` at start and the cost-grid rebuild reads them from `State`, so an event (v4) or a technology curve (v5) can change them mid-run.
4. Map generation is a pipeline of passes, each `fn(&mut Terrain, &MapParams, &mut Rng)`: v1 has heightmap, thermal erosion, hydrology and yield; later versions add hydraulic erosion, floodplains, climate, biome, geology and tectonics as further passes.
5. The CLI accepts a ground-truth file (settlement name, coordinates, estimated population, date) and scores a run against it, so real-terrain validation needs no new tooling later.

Not designed for now: maps above 512 × 512 or several joined regions. If v5 needs them, a coarse routing layer can be added above the cost grid without changing the systems.

**UI architecture**

The UI will grow faster than the simulation, so it gets the same discipline: one state, one direction of data flow, and parts that are added by registration rather than by editing a central file.

- **Framework: `egui` via `eframe`.** All Rust, immediate mode, with sliders, panels, tables and plots built in, and it runs in the browser through `trunk`. It looks like a tool, not a game, which is right for v1 to v3. A polished presentation front end, if ever wanted, is a separate crate reading the same `Snapshot`; nothing in `core` or `web` has to change for it.
- **One `ViewState` struct** holds everything the UI shows: the current snapshot, the terrain, camera (pan, zoom), selected cell and settlement, which layers are on, playback speed, and panel visibility. Every frame draws from it. Nothing else holds UI state.
- **One `Msg` enum** is the only way `ViewState` changes: `Step(n)`, `SetSpeed`, `Select(cell)`, `ToggleLayer(id)`, `SetParam(path, value)`, `Reset(seed)`, `LoadTerrain`, `Save`, `Load`. Widgets emit messages; one `update(&mut ViewState, Msg)` function applies them. This is the Elm pattern and it is what keeps a growing UI legible: any behaviour can be traced from its message.
- **`SimHandle`** is the only door to the simulation. It owns `State`, `Terrain`, `Params` and the `Rng`, and exposes `step(n)`, `snapshot()`, `set_params()`, `reset(seed)`. In v1 it runs on the main thread (a year is under 50 ms). The interface is message-shaped so it can move to a web worker later with no UI change.
- **Map rendering** is a `Vec<Box<dyn Layer>>` drawn in order into one texture per frame: terrain, water, fields, roads, footprints, routes and dots, overlays, selection. A `Layer` has `fn draw(&mut self, snap, terrain, view, painter)` and an optional cache keyed by snapshot year, so a derived layer recomputes once per year, not per frame. Adding a 1.x layer is a new file and one line of registration.
- **Panels** follow the same pattern: a `Panel` trait with `fn ui(&mut self, ui, &ViewState) -> Vec<Msg>`, registered in a list. v1 panels: time controls, layer toggles, inspector (hover and selection), parameters, charts (rank-size, population over time), chronicle. A panel never touches `SimHandle` or another panel; it reads `ViewState` and emits messages.
- **Parameters UI is generated**, not hand-built: `Params` derives a small trait that lists its fields with name, range and group, and one panel renders every field. A new parameter in `core` appears in the UI with no `web` change.
- **Inspector lookups** (what is under the cursor, which settlement owns this cell, what is the travel cost here) are functions in `core` over `Snapshot` and `Terrain`, not in the UI, so the CLI can print the same answers.
- **Camera and picking** are one module: screen to cell and cell to screen in one place. Every layer and panel uses it.

Rules for every later UI change: a new feature is a new layer or a new panel; a new interaction is a new `Msg`; `ViewState` gains a field only when something must persist across frames; no panel reaches into another; no UI code calls `step_year`.

## Out of scope for v1

- Individual people, households, merchants or vehicles as simulated entities, in any version.
- A price economy: markets clearing, money, wages. Transport cost is the only price.
- More than one trade good; production chains; specialist settlements.
- Politics: lords, borders, taxes, war. The `domain` field exists but nothing writes it.
- Events and shocks. The event queue exists but is empty.
- Roads that upgrade or decay from traffic; bridges, mills, walls with effects.
- Technology or any change of transport ratios during a run.
- Map generation beyond heightmap, thermal erosion, hydrology and yield.
- Maps above 256 × 256 or several joined regions.
- A polished presentation UI, mobile layout, saving to the cloud, multiplayer.
- Every 1.x view layer (field strips, seasons, buildings, names, chronicle prose).

These are not rejected ideas; they are the roadmap in the next sections. The rule is that none of them is started until v1 passes its validation targets.

## Milestones

Seven milestones, each with a check that must pass before the next starts. Durations assume one developer part-time; the tuning milestone is the one that slips.

1. **M0 – Skeleton (1 week).** Workspace with the four crates, `Terrain` and `State` structs, `Params` with serde, `Snapshot`, an empty `step_year`, the CLI running 500 empty years, egui app showing a blank map. Check: determinism test passes on the empty run; wasm build loads in the browser.
2. **M1 – Map and cost grid (1 week).** Heightmap, erosion, rivers, yield, harbours; edge costs with river, sea, crossing and transshipment rules; bounded Dijkstra. Viewer draws terrain and a travel-cost overlay from a clicked cell. Check: a path from an inland cell to the coast prefers the river and pays transshipment once; ports and heads of navigation are visible as cost minima.
3. **M2 – Villages and cultivation (1 week).** Initial hamlets, cultivation step, food-only growth, forest clearing and regrowth. Check: an isolated village on prime land settles at about 150 people per claimed cell and stops; on poor land it stays small; fields are visible as rings stretched along rivers.
4. **M3 – Trade and hierarchy (1–2 weeks).** Gravity flows, food redistribution, Rihll–Wilson growth term, route rendering with thickness. Check: with trade on, a few river and coast settlements pull ahead; the largest settlement’s share stays under 25%; rank-size slope in range.
5. **M4 – Founding, failure and roads (1 week).** Site score with shadow and access, founding and failure, roads between top trading pairs, site-score overlay, chronicle log. Check: settlement count, spacing, failure rate and siting targets pass.
6. **M5 – Tuning (2–3 weeks).** Sliders for every parameter, charts panel, 20-seed batch runs in the CLI, all validation targets passing on three different generated maps. Check: the full validation table passes; two people looking at a run agree it looks like a region.
7. **M6 – Release v1 (1 week).** Footprints, trade dots, inspector, hover, save and load, a README with the model summary and the parameter table. Check: a stranger can open the page, press play, and understand what they are watching from the panels alone.

## Working agreement

The implementer works autonomously from M0 to M6 without approval between milestones. This document is the brief; the validation targets are the acceptance test; the owner is consulted only at the points listed below.

- **No hosted CI.** No GitHub Actions or any other CI service. Validation is local: `cargo test` runs the determinism and unit tests, and `cargo run -p cli -- validate --seeds 20` runs the full validation table. Both must pass before a milestone is called done. A `justfile` or `Makefile` with `test`, `validate`, `web` (trunk build) and `run` targets is the whole build system.
- **Trunk-based, direct to main.** All work is committed to `main`. No pull requests, no review gates, no feature branches that live longer than a day. Small commits with messages that name the milestone and the system touched.
- **Milestone record, not approval.** At each milestone the implementer commits a short note in `docs/milestones.md`: what passed, the numbers from the validation run, the seed and params used, and anything deferred. The golden snapshot hash for the determinism test is committed with it.
- **Parameters and tuning are the implementer’s call.** Any default in the specification may be changed to make the validation targets pass; the change and the reason go in the milestone note. The shape of the equations may be changed only if a target cannot be met by tuning, and that is one of the points to raise.
- **Points to raise with the owner**, and the only ones: a validation target that cannot be met after a tuning pass; a change to the core boundary (`Terrain`, `State`, `Snapshot`, the system signature); a change of UI framework; anything that would add a Later item into v1. Everything else proceeds.
- **Done means v1 released.** M6 complete, all validation targets passing on three generated maps, the web build served from a static folder, a README that explains the model in one page. At that point the owner reviews, and the roadmap after v1 is planned together.

## v2 specification: goods and specialists

Written by the designer on 21 October 2026, after v1.2 passed validation (D35). v2 adds the second half of the brief's picture: resource sites with their own small settlements, goods moving along the rivers and roads, and towns that grow by handling them. It stays inside the two standing limits: there is no price economy, because goods trade at fixed exchange ratios against grain, and there are no agents.

**What the viewer should see.** Salt works on the coast, quarries in the hills and iron mines in the uplands, each a small settlement that buys its bread. Woods thin around the iron mines. Grain moves up the valleys to the mines and iron comes down; stone comes out of the quarries to the larger towns. Ports and river towns on those routes grow on the trade they handle.

**Goods.** Four goods: grain (the existing good), salt, iron and stone. Each non-grain good `g` has:
- quantities stated in **grain units** (output, demand and payment alike), so a unit of good exchanges for one unit of grain: a fixed ratio, not a price (`v_g` = 1, D38);
- a **value density** `ρ_g` relative to grain, from which its distance decay follows: `β_g = β / ρ_g`. A good worth more per tonne travels farther for the same cost. This applies ORBIS's per-weight costs to goods of different value, and adds no free parameter for decay. Starting values: iron `ρ` 8, salt 3, stone 0.4;
- a **demand**: salt and iron per person, stone per market person (towns build; villages do not), in grain units a year. Starting values: salt 0.02, iron 0.02, stone 0.03.

**Sites and production** (a new system, `production`, after cultivation; Weber 1909).
- Resource cells come from map generation as now: ore, salt and stone. The **holder** of a site is the settlement whose anchor is within `min_spacing_km` of it at the least travel cost.
- Output, in grain units a year: `min(cap_g, prod_g × (market size W of the holder + site_crew))`. Starting values: `prod_g` 2 for every good, `site_crew` 10, `cap_g` 1,000.
- **Iron is fuel-bound.** Smelting is the weight-losing step Weber places at the fuel source. Each unit of iron draws `fuel_per_iron` (0.1, D38) of forest, which cultivation (the owner of forest) adds to the holder's fuel demand and cuts nearest first, as for household fuel. Iron output is capped by the fuel actually obtained the year before. **Coppice (v2.1):** the iron draw may cut a cell only down to `coppice_floor` (0.3, standing coppice), so iron output is capped by the regrowth above that floor across the holder's fuel catchment, `regrow_rate × (forest0 − floor)` per cell. A mine settles at a steady output with managed woods around it, and a mine with a small catchment stays small: the Wealden pattern.
- Under v2 the old `resource_out` (a flat 50 per owned resource cell) is retired.

**Goods trade** (in the trade system). The buyer is the origin: this is Wilson's shopping model (Huff 1964; Wilson 1971), in which each zone spends a fixed amount, shared among the places that sell. It is the right form for goods because demand is fixed and supply is not.
- Each consumer `k` buys its demand for good `g`, split among the working sites in proportion to `S_j × exp(−β_g c_jk)` over the pairs within `C_max`. `S_j` is the site's capacity this year, `min(cap_g, prod_g × (W + site_crew))`, and iron's fuel cap applies.
- If the orders reaching a site exceed its capacity, every order to that site is scaled down to fit. The unmet part stays unmet, and there is no second pass. A site's output is what it sells.
- Competing sites therefore share the market (Weber's market areas), and supply beyond demand earns nothing. The holder's own demand is served first, without travel or payment.
- **Payment is grain.** A consumer pays one grain unit per unit received out of its marketed share `O_i`, capped at `O_i`. That grain travels to the producer as an ordinary food flow, on the buyer's path, with the usual loss per unit cost. What is left of `O_i` goes to markets as now.
- So no food is created: a specialist place is fed by real grain deliveries, which count in its gross inflow `D_j`, and it grows by the same Rihll–Wilson rule as any market.
- Goods and the grain paying for them load the routes. Handling and transshipment income (D27) apply to them as to grain, at the goods' value. This is how towns between ore and people grow.
- Complementarity (Barjamovic et al. 2019) needs no extra term: goods flow only from the places that have them.

(Until D41 the producer was the origin and sold its whole output whatever the demand. That let a saltern earn at its cap and grow into the regional market town: a specification error, corrected here.)

**Specialist settlements** (in founding and failure).
- At each founding step, a resource cell with no settlement within `min_spacing_km` gets a hamlet of `found_pop` on the cell if the demand it could reach, `Σ_k demand_k × exp(−β_g c)`, would feed the founders. At most one per resource type per step, with migrants found as usual.
- Its `kind` is `Mine`, `Saltern` or `Quarry`. It cultivates like any settlement, and its newcomers join the market people, who work the site.
- It fails by the ordinary rules (it sheds when underfed and is removed below `fail_pop`). There are no special cases.
- **The kind follows the economy** (D43). A specialist stays one while its goods sales are at least `specialist_share` (0.25) of its gross inflow, averaged over 10 years. Below that it becomes an ordinary settlement for good: it still holds and works its site, and the chronicle records it ("1340. Mereport has outgrown its salt works."). This is the history of the wich towns and the mining boroughs, places that began on a resource and became market towns.

**Switch.** All of v2 sits behind `goods_enabled`, off until the v2 milestone passes. With it off, the golden hash stays `2b00434da9781880`.

**Stages.** Each stage is built and checked alone, then reported over the channel. The next stage starts without waiting for approval unless a check fails or a spec question comes up.
1. **G1 Production.** Sites, holders, output and iron's fuel draw; no goods trade yet. Check: held sites produce; forest within 5 km of a working iron site falls below the land mean by year 200.
2. **G2 Goods trade.** Production-constrained flows, grain payment, handling income on goods. Check: payments balance (grain paid equals goods value received, less loss); median flow cost orders iron > salt > grain > stone.
3. **G3 Specialists.** Founding, `kind`, failure. Check: the v2 gate below.
4. **G4 View and milestone.** Goods colours on routes (salt white, iron rust, stone grey), specialist symbols, goods lines in the inspector and legend, and the chronicle naming new works ("1214. Salt works founded at …"). Add these as new layers and panel lines, not edits to shared files, because a graphics session is also working on `hinterlands-web`. Then tuning and the milestone record.

**v2 gate.** Judged as in v1: pass rates over 20 seeds on each of the three maps, 75% per check.
- **Gated:**
  - all v1.2 checks still pass;
  - at least two thirds of resource sites have output at year 500;
  - the largest place is not a specialist, meaning a place whose goods sales are at least a quarter of its inflow (D43);
  - one year under 50 ms in release wasm;
  - determinism, natively and in the browser.
- **Reported:**
  - specialist sizes (expected tens to several hundred);
  - the share of all market inflow that is payment for goods (expected 5–25%);
  - forest within 5 km of iron sites against the land mean;
  - the share of settlements over 2,000 on a route carrying goods.

**Tuning.** The goods parameters above are starting values and are the implementer's to tune. The v1.2 parameters are re-tuned only if a v1.2 check fails with goods on, as the over-5,000 median on map 3 does now (goods payments take about 3% of marketed grain away from the market towns). Every change goes in the milestone note. Kent has no resource layer, so its regression should not change. Placing Kent's Wealden iron, ragstone and estuary salt as data is a later option, not part of v2.

## v4 specification: shocks

Written by the designer on 21 October 2026, after v3 passed validation (D55–D56, golden `171ec4461a912a1d`). v4 puts the event queue to use. A region that has filled in is struck by famine and plague and has harbours silt up. Lords plant towns in it. Each shock is answered by the existing rules: shedding, founding, failure, trade and roads. Nothing here changes `Terrain`.

**Calendar.** `events_mode` is `Off`, `Historical` or `Random`. `Historical` sets year 0 = 1100 and is the default in the viewer and in validation:
- the Great Famine in years 215–217 (1315–17);
- the Black Death in year 248 (1348);
- two lesser recurrences, in 261 and 269, at a third of the first mortality (1361, 1369).

`Random` draws the same kinds of event from the run's one `Rng`, at mean intervals set in `Params`, for open-ended play. `Off` reproduces v3 exactly, with golden `171ec4461a912a1d`.

**Events** (a new `events` system at the start of `step_year`, writing only through `Event::apply`):
1. **Famine.** Every cell's yield is multiplied by `1 − famine_loss` (0.4) for the event's years. The existing rules then do the work: hungry places shed people and marginal hamlets fail. (Jordan 1996: yields fell by a third to a half in 1315–17.)
2. **Plague.** Each settlement loses a share `m` of its people, with the same share taken from market size W, spread over two years. `m` is drawn per settlement around `plague_mortality` (0.4), uniform within ±0.1. Land is left over afterwards, so the survivors are better fed and recover by the ordinary growth rules. (Benedictow 2004: 35–50% mortality in England.)
2a. **Resettlement after plague** (D61; Dyer 2002, Beresford 1954). After a plague the deaths leave good holdings vacant, and survivors leave poor land to take them. For `resettle_years` (30) after a plague strikes, each settlement whose local yield (3 km, D58) is below its trading partners' median sends `resettle_rate` (2% a year) of its people to the partner with the most free food per head within `r_cult` reach. The rule sits in growth, so shedding and failure finish the job. It acts only after a plague. Both values are the implementer's to tune.
3. **Harbour silting.** At each event, the busiest port whose harbour faces a river mouth or shallow coast silts up, leaving out the largest settlement (D62). The ports that silted were secondary harbours on shingle and marsh coasts, while London and Bristol did not. Silting the capital in every run reshuffled the whole hierarchy. Its land–water stepping cost rises permanently by `silt_cost` (3; 1.0 was tried while the target was the largest city, D60), so its trade moves elsewhere. This is the history of Sandwich, New Romney and Winchelsea. In `Historical` mode there is one silting, at a random port between years 250 and 400; in `Random` mode it is drawn at a rate. Cost changes are in `State`, so `Terrain` stays fixed.
4. **Planted towns.** Every `plant_interval` years (50), from `plant_from` (year 150, 1250; D63), a lord plants a town: a cell on a road or navigable river with the highest market access, at least 10.7 km (Bracton's 6⅔ miles) from any place over 500. It starts with `plant_pop` (200) people and market size at five times `W_0`. It survives only if trade supports it. (Beresford 1967: many planted towns failed.)

**Gate.** Pass rates over 20 seeds on each of the three maps, 75% per check, with `Historical` events.
- **Gated:**
  - every v3 check at year 500, meaning the region recovers within 250 years of the plague. The stability check leaves out event years and the year after each (D58);
  - determinism with events, natively and in the browser;
  - total population falls by at least 25% within five years of year 248;
  - total population at year 500 is at least 90% of its peak before year 248;
  - places in the lowest-yield quarter fail between years 248 and 300 at a higher rate than places in the highest quarter (the deserted villages; Beresford 1954, Dyer). Yield is local, meaning the land within 3 km of the anchor; places within 8 km of a place over 2,000 are left out, because the urban drain there is a separate effect that falls mostly on good land (D58, D59);
  - the top-10 overlap between a run with events and the same seed without them is at least 0.5, so the old hierarchy comes back, mostly;
  - one year under 50 ms in release wasm.
- **Reported:**
  - the silted port's rank 100 years after silting against its rank before;
  - the share of planted towns alive 100 years after planting (expected 30–70%; 57–82% at E3, because planting at the best-access cells makes them likely to succeed. Accepted: many of Beresford's survivors stayed small);
  - the depth and length of the famine dip, and the years taken after 248 to regain the pre-plague total. At E1 the population at year 500 was 1.7 times the pre-plague peak, so recovery is fast, and the trough is the number to watch;
  - the Kent regression.

**Stages.** Each stage is built and checked alone, then reported. The next stage starts without waiting unless a check fails or the spec is unclear.
1. **E1** The event queue, calendar, famine and plague. Check: the contraction, recovery and desertion checks.
2. **E2** Harbour silting. Check: the silted port loses rank (reported); the gate still holds.
3. **E3** Planted towns. Check: the survival share (reported); the gate still holds.
4. **E4** View and milestone. The chronicle gets plain lines ("1348. The pestilence came; Harthorpe lost 4,100 of its people.", "1290. The harbour at Wenenford silted.", "1250. Lord's town planted at Newport."). The map gets a brief plague tint, a silted-harbour mark and a planted-town symbol. A timeline strip of events goes under the population chart. All of it goes in new layer and panel files, as in v2.

## Future versions

Each version adds one layer, on the extension point already left for it, and only after the gate before it. The research footing weakens with each step: v2 and v3 still rest on recognised models, v4 is mostly heuristic, v5 is design. That is acceptable because every later layer sits on a validated v1.

```text
Simulation lane (each gate must pass before the next version starts)

  v1 Region        v2 Goods         v3 Landscape      v4 Shocks         v5 World
  four systems  ◆  3–5 goods,    ◆  roads from     ◆  events, plague ◆  joined regions
  one good      │  Weber,        │  traffic,       │  lords and      │  sea trade,
                │  specialist    │  bridges,       │  borders        │  technology
                │  sites         │  1.x layers     │                 │
          validation       goods tuned       roads stable    events deterministic
            passes

Map generation track (any pass can land early; real DEM import at any point)

  Noise            Water and soil   Climate           Tectonics         Real terrain
  heightmap,       hydraulic        rain shadow,      plates, ranges,   SRTM import,
  rivers, yield    erosion, flood-  biome, geology,   aligned valleys   history
  from slope       plains, lakes    resources                           validation
```

*Roadmap: five versions and the map generation track.*

The simulation lane runs left to right through its gates; the map generation track runs in parallel and any of its passes can land early, since generation is a pure function the systems never see.

| Version | What is added | What the viewer sees | Research footing |
| --- | --- | --- | --- |
| v2 Goods and specialists | 3 to 5 goods with point sources; Weber’s rule for where processing sits; complementarity in the gravity flow; `kind` on settlements for mines, salt pans, quarries | Resource towns; river corridors carrying ore toward population; ports specialising | Weber 1909; Barjamovic et al. 2019 (trade lower between places with similar baskets) |
| v3 Landscape | Roads upgraded and decaying from edge load; fords becoming bridges that cut crossing cost; mills and walls with their small effects; most 1.x view layers | Field strips and seasons, site-earned buildings, names, the chronicle. The version that looks like a place | Dalgaard et al. 2022 on road persistence; medieval market and bridge history |
| v4 Shocks and politics-lite | The event queue in use: drought, plague, a river shifting, a route cut. First use of `domain`: lords holding territory, planting towns, taking tolls; borders from catchments; a defence weight, no warfare | The map reorganising after a plague; planted towns competing with organic ones; a capital above the rank-size curve | Anasazi climate work; English charters and the Bannmeile; mostly heuristic |
| v5 World | Larger or joined regions with a coarse routing layer; way-stations on sparse stretches; sea trade between coasts; regions with different parameters; transport ratios drifting with technology | Long-distance corridors; a highland of pastoralists and a coast of traders; late in a run a better ship moving the whole hierarchy | Christaller’s transport principle; caravanserai spacing; design |

**Plan after v2** (designer, 21 October 2026). This replaces the table above where they differ.
1. **v2.1 Kent resources and coppice.** The coppice rule for iron fuel (see Sites and production) comes first, with one re-run of the v2 gate. Then come the Kent resources (data only). Wealden iron, Maidstone ragstone and the estuary salt pans are added to Kent's `Terrain` through the importer, as a showcase of v2 on real ground. It is reported, not gated, and no expectation is set for any single site.
2. **v3 Landscape.** The work comes in this order, with the steps in one version because each depends on the last:
   - (a) *(Tried and dropped, D52–D53.)* **The shadow is measured in km, not travel cost.** Bosker & Buringh's bands and Bracton's spacing are distances. With the shadow in travel cost, every road deepens the shadow along itself, which works against the next step.
     - **Change:** both uses of the shadow, the founding penalty and the growth drain (`shadow_drain`, with its linear fall-off), use the straight-line distance between anchors in km, against `shadow_radius_km`, which starts at 10. The access term stays in travel cost, because market access is about the cost of getting there.
     - **Validation:** a re-tune and full validation follow. A new golden hash is recorded.
     - **Expectation on Kent, pre-registered:** the share of settlements on or next to a Roman road reaches at least the land share (32%), against 21% now. Kent's inland spacing (6.0 km against 10.6) is reported.
     - **Stop:** if validation cannot be passed by tuning, stop and report.
     - **After the first attempt (D52):** with one radius, spacing and failure rate pulled against each other (map 1 spacing 8/20). Decided (D53): the two uses get two distances, each with its own footing. The founding shadow uses `shadow_radius_km`, about 15–18, matching Bosker & Buringh's bands around existing cities. The drain uses `drain_radius_km`, about 8, the distance people walked to a weekly market, within which a small place loses people to a larger one. The floor on the founding shadow (option 2) is declined: within a 15 km radius it would stop founding near any hamlet. The spacing check (8–15 km) stays as it is. If this fails validation after tuning, v3(a) is dropped (`shadow_km` stays off) and v3 continues with (b).
   - (b) **Roads follow traffic.** They upgrade with edge load and decay slowly when it falls. Dalgaard et al. (2022) found Roman roads persisted where traffic kept them in use and faded where it did not.
     - **Condition.** Each road cell carries a condition `q` in [0, 1]. Every `road_interval`, the paths of the top `road_pairs` trading pairs are raised to `q` = 1, as now. Every other road cell whose edge load is below `road_keep_load` loses `road_decay` of its condition (starting value 0.05 per step, about 200 years from full to gone). A cell below 0.1 stops being road. Initial roads (Roman, D32) start at `q` = 1 and follow the same rule.
     - **Cost.** Moving along a road costs `land + q × (cost_road − land)` per km, so a fading road is slower before it disappears. The cost grid is rebuilt only at road steps, as now.
     - **Gate.** All v2.1 checks pass. Total road length changes by less than 10% between years 300 and 500, a road network that settles rather than churns.
     - **Reported.** The share of road cells abandoned during the run. On Kent, the share of Roman road cells still road at year 500, and the on-road share of settlements.
     - **Stop.** If validation fails after tuning, stop and report.
     - **Shadow note.** v3(a) was dropped (D53): in km form the drain is what spaces markets, so splitting it off did not help. The shadow stays in travel cost. Since the cost shadow pushes places off roads, the on-road share is the number to watch in (b).

   - (c) **Bridges at busy crossings,** using the free-crossing mechanism of D33.
     - **Rule.** At each road step, a road cell that crosses a river or stream away from a ford becomes a bridge if its load has been at least `bridge_load` for `bridge_steps` road steps in a row. Starting values are 2 × `road_keep_load` and 3 steps.
     - **Effect.** The bridge's crossing cost is 0. A bridge never decays, even if its road fades: towns outlive their reason, and bridges outlived their roads.
     - **Viewer.** The viewer's bridges come from the model's bridges from now on, instead of its own traffic rule.
     - **Gate.** The v3(b) gate holds.
     - **Reported.** Bridges per run, and the share of places of 1,000 or more within 2 km of a bridge, against the share of all settlements.
   - (d) **Mills.** A settlement whose fields include a stream or river cell with drainage of at least `mill_area` (starting value 20 km²) has a mill. Its yield is raised by `mill_bonus` (3%). The viewer's mill comes from the model's from now on. Gate: the v3(b) gate holds. Reported: the share of settlements with a mill.
   - **v3 milestone** after (d), recorded as for v2.
   - **Kent note at (b)** (a51cd28). Spacing of inland places over 500 now matches the real figure (10.8 km against 10.6), but there are only 12 of them against 29 real, and there are 445 settlements against 238 before. 19% of Roman road cells survive, and Watling Street, which in reality persisted, mostly fades: the model has no Continental traffic through Dover. That belongs to v5's outside world. It is reported, not chased.


   Walls have no effect until something in the model uses defence. The 1.x view layers already exist, and the graphics work is separate.
3. **v4 Shocks.** Specified below, in "v4 specification: shocks".
4. **v5 World** will be decided after v4. The outside world as a seller, tried in D25 without effect, is folded into v5's sea trade and not done before it.

**Map generation track.** v1 ships noise, thermal erosion, hydrology and yield. Later passes, in order of effect on realism: hydraulic erosion with floodplains and deltas as the best land; lakes and marshes; coastline detail; a rain-shadow climate with biomes from temperature and moisture; resources placed by geology; tectonic plates giving ranges a direction. Each is a new pass in the pipeline and changes nothing else.

**Real terrain.** Importing a free 90 m DEM (SRTM) of a documented region, resampled to the chosen cell size, is the single most valuable step after v1. Run on medieval Kent or the Rhône valley and compare the settlements the model produces with the ones that existed. If the model puts a city where Canterbury or Lyon stands, the model is right rather than merely plausible. Generated maps stay the tool for development and tuning; real terrain is for validation and for the demonstration that convinces people.

**Never planned, in any version:** individual agents and a full price economy. Nothing on the behaviours list needs them, and each is the usual way a project of this kind sinks.

## References

**Empirical studies**

- Bosker, M. & Buringh, E. (2017). City seeds: Geography and the origins of the European city system. *Journal of Urban Economics* 98, 139–157. [PDF](https://research-portal.uu.nl/ws/files/44916121/1_s2.0_S0094119015000613_main.pdf). Source of the founding-score weights and the urban-shadow bands.
- Bleakley, H. & Lin, J. (2012). Portage and path dependence. *Quarterly Journal of Economics* 127(2), 587–644. [PDF](http://www.ralfmeisenzahl.com/uploads/7/6/8/1/76818505/bleakley_lin.pdf). Break-of-bulk founding and persistence.
- Dalgaard, C.-J., Kaarsen, N., Olsson, O. & Selaya, P. (2022). Roman roads to prosperity: Persistence and non-persistence of public infrastructure. *Journal of Comparative Economics* 50(4), 896–916. [Abstract](https://www.sciencedirect.com/science/article/abs/pii/S0147596722000269).
- Barjamovic, G., Chaney, T., Coşar, K. & Hortaçsu, A. (2019). Trade, merchants, and the lost cities of the Bronze Age. *Quarterly Journal of Economics* 134(3). [SSRN](https://papers.ssrn.com/sol3/papers.cfm?abstract_id=3065804). Gravity estimated on ancient trade.
- Letters, S. et al. *Gazetteer of Markets and Fairs in England and Wales to 1516*. Institute of Historical Research. [Introduction](https://archives.history.ac.uk/gazetteer/fullintro.html); [ESRC report on market survival](https://archives.history.ac.uk/ihrcms/cmh/assets/sites/history.ac.uk/files/EoA_Report_Results.pdf).
- Bracton, *De Legibus*, vol. 3 p. 198 (Thorne ed.), on the day’s journey and neighbouring markets. [Text](https://amesfoundation.law.harvard.edu/Bracton/Unframed/English/v3/198.htm).
- Masschaele, J. (1993). Transport costs in medieval England. *Economic History Review* 46(2). Summarised in Langdon & Claridge, [Transport in Medieval England](https://static1.squarespace.com/static/5a4d52ff7131a5845cdd5162/t/5a503b02652dea46dba01c55/1515207426683/Claridge+Transport+Medieval+England.pdf).
- Clark, G. (1999). Markets and economic growth: the grain market of medieval England. [PDF](https://faculty.econ.ucdavis.edu/faculty/gclark/papers/market99.pdf).

**Models**

- Rihll, T. E. & Wilson, A. G. (1987). Spatial interaction and structural models in historical analysis. *Histoire & Mesure* 2(1). Re-analysed in Evans, T. & Rivers, R. (2017), Was Thebes necessary? Contingency in spatial modelling. *Frontiers in Digital Humanities* 4:8. [Article](https://www.frontiersin.org/journals/digital-humanities/articles/10.3389/fdigh.2017.00008/full); [arXiv](https://arxiv.org/pdf/1611.07839); [site dataset](https://figshare.com/articles/dataset/Locations_of_108_Archaeic_Greek_settlements_used_in_Rihll_and_Wilson_1987_/868961).
- Scheidel, W. & Meeks, E. ORBIS: The Stanford Geospatial Network Model of the Roman World. [Site](https://orbis.stanford.edu); [paper](https://orbis.stanford.edu/assets/Scheidel_59.pdf). Transport mode cost ratios and speeds.
- Dean, J. S., Gumerman, G. J., Epstein, J. M., Axtell, R. L. et al. (2002). Population growth and collapse in a multiagent model of the Kayenta Anasazi in Long House Valley. *PNAS* 99. [Article](https://www.pnas.org/doi/10.1073/pnas.092080799). Replication: Janssen (2009), [JASSS 12(4)13](https://jasss.soc.surrey.ac.uk/12/4/13.html).
- Kohler, T. A. et al. Village Ecodynamics Project. [Project](http://village.anth.wsu.edu/about); [model archive on tDAR](https://core.tdar.org/dataset/425610/village-ecodynamics-project-settlement-model-version-54-vep-i).
- Knappett, C., Evans, T. & Rivers, R. (2008). Modelling maritime interaction in the Aegean Bronze Age. *Antiquity* 82. [Summary](https://www.researchgate.net/publication/259230635_Modelling_Maritime_Interaction_In_The_Aegean_Bronze_Age).
- Lewis, J. (2018). The suitability of using least cost path analysis in the prediction of Roman roads. [Slides](https://josephlewis.github.io/GISRUK2018_presentation.pdf). Accuracy of LCPs in highland vs lowland.
- Christaller, W. (1933). *Die zentralen Orte in Süddeutschland*. Overview: [CSISS Classics](https://escholarship.org/content/qt6188p69v/qt6188p69v.pdf).
- von Thünen, J. H. (1826). *Der isolierte Staat*. Weber, A. (1909). *Über den Standort der Industrien*. Krugman, P. (1991). Increasing returns and economic geography. *Journal of Political Economy* 99(3). Fujita, M. & Mori, T. (1996). The role of ports in the making of major cities. *Journal of Development Economics* 49.

**Generators and code**

- O’Leary, M. Generating fantasy maps. [Notes and code](https://github.com/mewo2); C++ port [rlguy/FantasyMapGenerator](https://github.com/rlguy/FantasyMapGenerator).
- Azgaar. Fantasy Map Generator. [Settlements and routes design notes](https://azgaar.wordpress.com/2017/11/21/settlements/); [wiki](https://github.com/Azgaar/Fantasy-Map-Generator/wiki/Knowledge-Base).
- Dwarf Fortress world generation. [Wiki](https://dwarffortresswiki.org/index.php/World_generation).
- SRTM 90 m elevation data (CGIAR-CSI) for real-terrain import.

Links were checked during the research pass of 2026-10-21; the medieval transport ratios and caravanserai spacing come from secondary sources and should be treated as approximate.
