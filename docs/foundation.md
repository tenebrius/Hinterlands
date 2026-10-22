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

## v5 specification: the outside world

Written by the designer on 21 October 2026, after v4 passed validation (golden `2c5e9c5c90ab1d43`). It is the last version of the gravity-only model; a market layer (v6, D70) follows the v5 release. It adds the two things a region in isolation cannot have: trade with places beyond the map, and ships getting better over the centuries. After v5 the work is a release pass, with no further mechanisms.

**Outside ports.**
- Map generation marks one outside port on each open-sea map edge, at the sea cell nearest the busiest coastline. Busiest is read as the harbour with the most yield within 10 km, since there is no traffic at year 0 (D64). On imported maps the importer reads them from a file: for Kent, the Continent (off Dover) and the North Sea (off the Thames mouth).
- Each outside port is a fixed partner in the existing machinery, with two roles:
  1. **A buyer of grain** with a fixed demand, `outside_grain` in grain units a year, through the shopping model (D41), so it buys a fixed amount rather than everything on offer. Paid in imports (below).
  2. **The only seller of a fifth good, imports** (wine, cloth, spices), demanded by market people only, `import_demand` per market person, with a high value density (`ρ` 10). Paid in grain like any good.
- The trade rides the stored paths, so the ports and towns on those routes earn handling and transshipment income on it. This is what Watling Street lacked: traffic through Dover to the Continent.
- D25's outside world, which bought everything by gravity and halved the population, is replaced by this. Its parameters are retired.

**Tuning with ports on (D66).** At O1, map 1's market spacing fell to 13/20; it had always been marginal there (15–18). The implementer may re-tune the founding and shadow values (`shadow_weight`, `found_threshold`, `noise`) with ports on, judged on all three maps at 20 seeds. The way spacing is measured does not change. If one tuning pass cannot restore it, accept the best point and record map 1's spacing as an exception in the milestone, not as a new rule.

**Better ships.** Transport cost ratios drift during the run, as decision 3 of the architecture allowed for (ratios live in `State`). In the Historical calendar, sea cost per km falls by `ship_gain` (30%) in a straight line between 1300 and 1500, for the cog, the hulk and then the carrack. River, road and land costs stay fixed. The cost grid is rebuilt at road steps, as now. Expected: coastal ports take a larger share late in the run, and some inland towns lose rank.

**Gate.** As in v4, 20 seeds a map, 75% per check, Historical mode.
- **Gated:**
  - every v4 check;
  - determinism;
  - under 50 ms a year in wasm;
  - outside trade (grain bought plus imports sold) between 3% and 20% of all market inflow.
- **Reported:**
  - the coastal share of population in places over 2,000, at 1300 and at 1600;
  - rank change of the top 10 between 1400 and 1600;
  - on Kent, the share of Watling Street cells still road at 1600 and the largest city's site.

**Stages.**
1. **O1** Outside ports and imports.
2. **O2** Ship drift.
3. **O3** View and milestone: imports shown as a sixth colour of route, outside ports drawn as ships at the map edge, and a chronicle line when sea costs fall ("1450. Larger ships now sail the coast."); then the milestone.

As before, report after each stage and carry on unless a check fails or the spec is unclear.

**After v5: release** (approved 21 October 2026, after the v5 milestone, golden `4ce58fae34550933`).
1. **README.** Rewrite it as one page covering the whole model as it now stands: the six systems, goods, roads with bridges and mills, shocks and the outside world. Give each mechanism its source in a line.
2. **`docs/parameters.md`.** Regenerate it from `Params`, so it matches the defaults exactly.
3. **"What you are watching" panel.** A short panel open on first load, a few plain sentences per phase of a run: villages fill the farmland (1100–1200); markets and towns pull ahead; works and goods; the famine and the pestilence (1315–1349); recovery and the planted towns; ships and the coast (1450 onward). It closes and is reopened from the panel list.
4. **View tidy, two items from the v5 screenshot.**
   - The sea lanes along the coast draw as a wall of parallel lines. Bundle sea-going flows that share a corridor into one lane, its width set by total flow, with goods colours as thin stripes or on hover.
   - Some labels collide ("The Gile Sea" with "Ewamouth", "Everelford" with "Dew Forest"). Settlement names take priority, and feature names move or hide.

   If these touch files the graphics session is working in, leave a note for it in `docs/rendering.md` instead of editing them.
5. **Final screenshots.** At 1200, 1349, 1450 and 1600 on map 1, seed 1, and one of Kent at 1600.
6. **Freeze.** Record the release golden hash in `docs/milestones.md` as "v5 release". Run `just test`, `just validate`, the wasm timing and the Kent regression one last time.

No new mechanisms in the release. After it, v6 S0 begins as briefed in "v6 plan" below.

## v6 plan: markets, consumers and stores (D70)

Decided by the designer on 21 October 2026, on the owner's proposal `docs/proposals/markets.md`. That document holds the case, the evidence and the design; this section holds the decisions.

**Why.** The proposal is right on its main charge. In the model, grain is pushed to large markets by gravity on W and never bought. A hungry place cannot buy, received grain is not passed on, food above `food_cap` is lost, there are no stores, and delivered goods do nothing for the buyer. Clark (1999), already in the references, finds the opposite for medieval England: local wheat prices followed the national price (elasticity 0.997), a manor's own yield barely moved its price, and up to 40% of a harvest was carried over after cheap years. These are faults against the record. The fix is published, and it is the dual of the gravity model the project already uses (Samuelson 1952; Takayama & Judge 1971; Wilson 1970; Erlander & Stewart 1990).

**Decisions** (numbered as in the proposal's section 8):
1. **Yes, beyond v5,** as v6, starting after the v5 release. The v5 work and the release go ahead unchanged, so a finished, frozen v5 exists before any of this starts.
2. **Market-clearing prices are admitted,** in grain units, as an aggregate equilibrium between settlements. The never-planned list above now excludes wages, a labour market, floating money and agents instead of "a full price economy". The reason for the old rule was scope, and the staging below answers it.
3. **Form: entropy-regularised spatial price equilibrium,** solved by a fixed count of damped Furness passes, warm-started each year. The lagged price index is declined, because it does not get grain to the hungry, which is the point.
4. **W: option 2, as a market premium, is the first form.** `W_j^α` multiplies the demand-side factor, read as the convenience of a large market (fairs, credit, many buyers). Every check from v1 to v5 rests on the hierarchy that W produces, and option 2 keeps it. Option 1 (W as demand only) is run as a reported diagnostic at S2, not as the default. Option 3 is declined, because it fixes none of the grain faults.
5. **Stages approved: S0 only, then stop and report.** S1 to S3 are planned, and each is approved only after the one before reports. S4 (goods in use) and S5 (coin, projects, mayors) are not scheduled; they will be decided after S3, on what S0 to S3 show.
6. **The Clark checks are reported at S0 and gated at S2,** at the proposed ranges:
   - local price response to the regional price within 0.8–1.2;
   - response to local yield, with the regional price controlled for, within −0.15 to 0;
   - the regional price explaining at least 60% of local price variation.
7. **P7 (transport loss against Clark's carriage cost) is reported only.** It is looked at in S2, where grain carriage gets its transport price `τ` from Clark (about 0.2% of value per km by land, a tenth of that by water).

**S0 brief (shadow prices).**
- **Scope.** Solve the equilibrium each year beside the existing trade, for grain and the four goods, and record prices and would-be flows. Nothing feeds back.
- **Switch.** `market_shadow` off by default, golden unchanged.
- **Demand elasticities, starting values:** grain 0.75 (Clark), salt 0.2, iron 0.5, stone 1.0.
- **Report:**
  - time per year natively and in wasm, which must leave the 50 ms budget;
  - determinism, natively and with `?selftest`;
  - the three Clark regressions on the shadow grain prices, per map;
  - the price gap on trading pairs against transport cost;
  - one map of shadow prices in a normal year and in 1316.
- **Stop** and report. If S0 alone breaks the time budget, v6 waits for a cheaper solver, and nothing else is changed to make room.

**S0 result (D69, `docs/market/s0.md`) and the designer's reading.**
- **The solver works.** 8 warm-started Furness passes with a price-level step, deterministic natively and in wasm.
- **The cost is real:** 15 ms a year in wasm (29.9 → 44.9 ms). Most of that is grain, which every settlement both sells and buys.
- **Clark:** `b_P` 0.98–1.15 and R² 0.66–0.98 are in range. `b_Y` is not identified, because the model has no local weather: a place's yield per cell barely moves except in the regional famine. Prices are nearly uniform across the region (1310 median 0.69, 1316 median 1.40). That fits Clark's integration finding, but the test is empty until local harvests vary.
- **The year after the famine** converges poorly (residual 0.3–0.5). This must be fixed before grain trades by price (S2).

**Decisions after S0.**
1. **S1 approved** (goods by equilibrium, below).
2. **W1, local harvests, is added before S2.** A yearly yield multiplier with spatial correlation, as a smooth noise field at about 15 km with a standard deviation of about 15%, drawn from the run's `Rng` and behind a switch. Medieval manor yields varied from year to year and place to place (Clark's 65 manors). Without that variation there is no local dearth for S2 to relieve and no `b_Y` to test. It also makes runs livelier: a bad harvest in one valley, a good one in the next. W1 is briefed after S1 reports.
3. **The S2 gates stand,** with `b_Y` measured with W1 on. If `b_Y` is still unidentified then, the range is revisited.
4. **The import elasticity of 1.0** (implementer's choice) is accepted.

**S1 brief (goods by equilibrium).**
- **Switch.** `market_goods`, off until S1 passes, then on by default with a new golden hash.
- **Scope.** Salt, iron, stone and imports trade by the entropy equilibrium (the S0 form), replacing the shopping model of D41. Grain is the numeraire at price 1 everywhere in S1, and grain trade is unchanged. So the grain solve is not run in S1, and `market_shadow` stays available for reports.
- **Demand** at price 1 is the current demand per head (or per market person for stone and imports). It responds to price with elasticities: salt 0.2, iron 0.5, stone 1.0, imports 1.0. **Supply** is up to the site's capacity, as now, with the supply price clearing it.
- **Payment.** The buyer pays the delivered price `p_j × q` out of its marketed share `O_i`, capped at `O_i`. The producer receives the supply price `p_i × q` as an ordinary grain flow. The transport part `t × q` is spent on carriage and lost, as transport loss is now. Goods and payments load routes and earn handling income as now.
- **Gate:**
  - every v5 check;
  - determinism;
  - at most 45 ms a year in wasm, so that S2 keeps some room;
  - outside trade still within 3–20%;
  - no specialist is the largest place.
- **Reported:**
  - goods prices at sites and in towns;
  - unmet demand by good;
  - price gaps against transport on trading arcs;
  - specialist sizes against v5;
  - one goods-price map.
- **Stop.** Report and stop.

**S1 first gate (D71, `docs/market/s1.md`) and the designer's decision.** The gate failed on map 2 spacing (14/20). The cause is iron: v2's demand (0.02 per head, about 2,300 a year) was set before coppice (D51) cut iron output to about 150 a year. Under the shopping model the shortfall simply went unmet. Under prices, iron reaches 22–80 in towns, and the mines grow to 1,500–2,800 people.
- **Decided: option 1.** `demand_iron` is re-scaled to the coppiced supply, so that the median town price of iron sits between 1 and 3. It is a value set before a later change, not a mechanism. Options 2 and 3 are declined (3 would be a new rule). Option 4 (re-tuning founding) is used only if spacing still fails with iron re-scaled.
- **Readings confirmed.** Supply is `capacity × min(1, p_i)`. A good with no working site in reach is reported as unreached, not priced at the floor.
- **Added to the report:** the median town price per good, which should be within 0.5–3 for each.
- **Re-run the S1 gate once.** If it passes, make `market_goods` the default and report. If not, stop and report.

**S1 second gate (cabdac6) and the designer's decision.** With `demand_iron` at 0.0013 (pooled median town price about 1.2), spacing and every other check pass. Map 2 has less iron and prices it at 5.6–6.9; no single value puts every map within 1–3, and 0.0013 is accepted. The new failure is sites producing, 14/17/14 of 20: at a fifteenth of the old iron demand, remote iron sites no longer pass the specialist founding test (D40), which counts demand in quantity.
- **Decided (D72): option 1.** Under `market_goods`, the founding test counts the value of demand in reach: `Σ_k p_k q_k exp(−β_g c)`, using each buyer's price for the good from the year before. Where no working site reaches a buyer, its price counts as 1. This is the consistent reading of D40 once goods have prices. It is also the market signal the equilibrium exists to give: dear iron draws new mines. Options 2 and 3 are declined.
- **Re-run the S1 gate once,** with the per-map iron price reported. If it passes, make `market_goods` the default and report. If not, stop and report.

**S1 third gate (d31e04a) and the designer's decision (D73).** Every check passes on maps 2 and 3. On map 1, sites producing is 14/20: four remote eastern sites stay unworked while iron sells at 1.19 in towns. That is a clearing market, not a fault. When the working sites meet demand at about cost, the marginal ones stay idle: this is the extensive margin, Ricardo's no-rent land. The two-thirds quota measured v2's rule that every site with demand in reach gets a works, and the market rightly overrides it. The check is changed after seeing it fail, so the reason is given in full:
- **Under `market_goods`, the gate becomes:** every good that has a site on the map has at least one working site at year 500 (75% of seeds per map). The share of sites producing is reported, not gated. The viewer already shows idle sites as unfilled symbols, which tells that story on the map.
- **No other change.** `demand_iron` stays 0.0013 and D72 stays.
- **If the gate passes,** make `market_goods` the default, record the golden hash, report, and go straight on to W1.

**W1 brief (local harvests).**
- **Switch.** `local_harvests`, on by default once it passes.
- **Mechanism.** Every cell's yield each year is multiplied by `m = clamp(1 + σ z, 0.5, 1.5)`, with `σ` = 0.15. `z` is a smooth field: standard normal values are drawn from the run's `Rng` (Box–Muller with `libm`) on a coarse grid of nodes spaced `harvest_km` (15 km) apart, and interpolated bilinearly to cells. Each year is drawn fresh, with no carry-over (Clark's yield autocorrelation is 0.16). The multiplier sits on top of the famine factor.
- **Gate:** every v5 check with `market_goods` and W1 on; determinism; at most 45 ms a year in wasm.
- **Reported:**
  - the coefficient of variation of each settlement's harvest;
  - the three Clark regressions on shadow prices (`market_shadow` on, for the report), so that `b_Y` is now identified;
  - the depth of local dearths: the worst place's yield multiplier per year.
- **Stop** and report before S2.

**W1 first gate (D74, `docs/market/w1.md`) and the designer's decision (D75).** `market_goods` is the default (28ddc53, golden `ec7a24a7caeac414`). With W1 on, every check passes except map 1's top-10 overlap against the same seed run Off (12/20, median 0.60). The comparison stopped being controlled: the Off run draws a different random stream, so it also gets 500 different years of weather. The fix is to make the comparison controlled again, not to change the threshold:
1. **Weather is drawn up front.** With `local_harvests` on, the whole node field for the run's configured length is drawn from the run's one `Rng` at initialisation, before anything else draws. Events on or off, the same seed then has the same weather, and the hard rule of one `Rng` per run holds. A viewer run that goes past that length draws further years when it reaches them.
2. **The standard deviation is set at the cells.** Scale the node values so that the interpolated multiplier has a standard deviation of `σ` = 0.15 at the cells, not 0.095.
3. **Re-run the W1 gate once.** If it passes, make `local_harvests` the default, record the golden hash and report, and S2 will be briefed. If not, stop and report.

Noted from the report: `b_Y` is now identified (−0.003 to −0.011, inside the S2 range), because the equilibrium spreads a local shortfall across the region at `market_tau` 0.002. Map 2 seed 2 is an outlier (`b_P` 1.35, R² 0.32), to watch at S2.

**W1 second gate (2c066b6, D75) and the designer's decision (D76).** With the weather controlled and the standard deviation at 0.15, the gate fails more widely: top-10 overlap on all maps, spacing and failure rate on maps 1 and 2. The implementer's diagnosis is right, and the error is the designer's. Weather with a mean of zero is not neutral when losses are asymmetric. A bad year sheds people and kills young places; a good year gives only capped growth. Map 1 seed 1 has 14.8k people at year 200 against 47.8k without weather. Real villages carried bad years with stores and markets. Those buffers are exactly S2 (grain to the hungry) and S3 (stores), and they are not built yet. I put the shock before the mechanisms that absorb it.
- **W1 is parked,** behind its switch, until S2 and S3 are in. It is then re-gated with the same σ.
- **The order is now S2 → S3 → W1.** At S2 the Clark checks `b_P` and R² are gated and `b_Y` is reported. `b_Y` is gated at W1.
- **The post-famine convergence fix belongs to S2.**

**S2: design note first (approved).** Grain by equilibrium touches the core of the model: who pays for grain, with what, and how market people are fed. Before building, the implementer writes `docs/market/s2-design.md`, at most two pages, proposing:
1. **Supply and demand of grain per settlement,** from S0's form: a place's harvest, its need, and the demand elasticity of 0.75.
2. **The unit of value and the budget.** What a buyer pays with, in value units within the year, with no stocks across years until S5. Market people's income has to come from somewhere real: the handling and transshipment margin, goods sales, and payment for grain they sell on. No food or value is created. A region-wide famine must show as grain dear against that income.
3. **How W enters** as the market premium (decision 4).
4. **What replaces `food_cap`:** surplus is sold at a falling price, or lost if unsold; stores come in S3.
5. **The famine-year convergence fix.**
6. **An estimate of the cost per year in wasm,** against the 45 ms limit.

Then stop. The designer reviews the note before any S2 code is written.

**S2 design note reviewed (`docs/market/s2-design.md`, d1faf0c); decisions (D77).** The note is approved as written, except for decision 2.
1. **Rents in value: approved.** `rent_share × p_i H_i`, paid by today's gravity weights on W, is the real income of market towns: lords, the church and their households lived in them on rents and tithes. It keeps the base of the hierarchy, and the payer loses exactly what the receiver gains.
2. **Fees: paid by the buyer, outside the routing price.** Handling (0.046) and transshipment (0.24) were tuned as income terms (D27). The physical cost of breaking bulk is already in the cost grid (+3 per transshipment). Putting both into the routing price would count break of bulk twice and push grain off the water routes that make ports. So each flow's buyer pays `f_ij × q`, which is credited where it arises, and routing uses `p_i + t_ij`. This is a deliberate departure from a pure equilibrium, recorded as such.
3. **`p_floor` 0.25 and `c_max` = `food_cap` (1.2): approved.** Unsold grain is lost and reported until S3.
4. **The shadow fix first:** in S2's first commit, `market_shadow` writes its own field, so turning it on cannot change a run. The W1 Clark numbers were taken with the old coupling and are re-measured at W1.
5. **Convergence** (re-level, then exact log-space solves, then the safeguard): approved. **Cost:** S2's limit is the hard budget, 50 ms a year in wasm, with 45 as the target; use the note's levers in order if over.

**S2 build (approved).** Switch `market_grain`, off until S2 passes.
- **Gate,** 20 seeds a map, Historical, with `market_goods` on and W1 off:
  - every v5 check;
  - Clark `b_P` within 0.8–1.2 and R² at least 0.6 (75% of seeds per map);
  - determinism;
  - at most 50 ms a year in wasm.
- **Reported:**
  - `b_Y`;
  - grain unsold per year;
  - the price gap against transport on grain arcs;
  - the share of places whose demand is capped by their budget;
  - the 1316 grain price in towns against villages;
  - the solver's residual and passes per year, and the worst year;
  - the W-as-demand-only diagnostic (option 1) on 3 seeds a map;
  - the Kent regression.
- **Tuning.** `rent_share`, `ε`, `p_floor` and the W premium's α are the implementer's to tune, with every change recorded.
- **Stop** and report, whether it passes or fails.

**S2 first build (7b00639, D78) and the designer's decision (D79).** The solver works: residual about 0.0002 in 8–11 passes, at +3 ms a year natively. But no towns form (map 1 seed 1: largest under 600 at every year shown, 51–55% of each harvest unsold). Every place grows its own food plus rent, so only the ports buy; the rent grain has no buyers; and K was tuned on gross inflow that counted burned grain. The D78 changes made while building are accepted: rent in kind with farmers eating first, ports buying up to `outside_grain` at a world price of 0.6, the level step, and goods paid in value.
- **Decided: option C.** K becomes a real purchase. This is the published reading of K: in Wilson's retail model, `K W_j` is the cost of running facility j, paid from its revenue `D_j`. It also has historical footing. A medieval town's grain demand went well beyond bread, to malting for ale, fodder for horses and the households of lords and religious houses (Campbell, Galloway, Keene & Murphy 1993, *A Medieval Capital and its Grain Supply*).
  - Each market buys grain for its establishment, so that its total yearly purchase for W is `K × W`, its market people's own food included. Do not add both, or the old K is counted twice.
  - The purchase is priced like any demand (ε 0.75) and paid from rents, fees and goods sales. The satiation applies to people, not to the establishment.
  - W then follows `ε(D_j − K W_j)` with `D_j` as real income divided by `p_j`, as in the note. A market whose income cannot buy its establishment shrinks.
- **Market people do not farm** (`grain_market_people` on). The fields feed `pop − W`, as the v1.1 rule that newcomers join the market people always meant.
- **Options A and B are declined.** A brings back burned grain growing W (P3), and B re-calibrates the whole hierarchy.
- **Tuning.** One round over `rent_share`, `K`, `ε` (growth) and α is allowed, with every change recorded. Then run the full S2 gate as briefed, and stop and report either way.
- **Added to the report:** the establishment's share of grain demand in towns over 2,000, against the people's own food.

**S2 second build (bf77fcd) and the designer's decision (D80): S2 is stopped, by the proposal's own stop rule.** With K as a real purchase and one tuning round over 20 settings, no town over 2,000 forms on any map. The hierarchy checks fail 0/20, the Clark R² is 0.25–0.37, and `b_Y` has the wrong sign. The implementer's reading is right. v5's hierarchy is a positive feedback on inflow that cost the market nothing to receive. Once W must be bought from income, every village keeps a small market on its own rents, and nothing concentrates. Making that work means redesigning the growth form on which every check from v1 to v5 rests. That is the over-ambition this project exists to avoid. So:
- **`market_grain` stays off.** Its code stays behind the switch as the record. S1 (goods by price) stays the default. Shadow grain prices remain for reports, and the Clark checks are reported in shadow mode only.
- **The proposal's visible aims are kept by a smaller route:** grain reaching the hungry, nothing burned, and stores that carry good years into bad ones. The hierarchy feedback is untouched.

**S2′ grain relief (approved; switch `grain_relief`).** This is the foundation's original intent, which the code never carried out: food reaches places in deficit.
- **After the trade step,** each place has `A_i = food_prod + food_in − food_out` against need `N_i`. Its surplus is `S_i = max(0, A_i − c_max N_i)`, the grain now lost above `food_cap`. Its deficit is `d_i = max(0, N_i − A_i)`.
- **Each place in deficit draws its deficit** from the surplus holders in proportion to `S_j exp(−β c_jk)` within `C_max`, as in the shopping model (D41). Orders above a holder's surplus are scaled down in one pass. Relief arrives less the usual transport loss, and unmet deficit stays.
- **The surplus would otherwise be lost, so nobody pays and nothing is created.** Relief counts in the food ratio but not in `market_in`, so W and the hierarchy feedback are unchanged. It loads the routes, so grain is seen moving to a dearth, but earns no handling income.
- **Gate:** every v5 check with `market_goods` on; determinism; under 50 ms a year in wasm.
- **Reported:**
  - relief volume per year;
  - the share of deficit met, in normal years and in 1315–17;
  - the famine dip against v5 with S1;
  - the failure rate against v5 with S1;
  - grain still lost above `c_max`.
- **Tuning.** None beyond what a failing v5 check needs, recorded.
- **Next:** S3 stores, briefed after S2′, then W1.

**S2′ result (8875174) and S3 brief (D81).** Relief passes the gate with `shadow_drain` 0.015 → 0.025, for map 1 spacing. It meets 86–88% of deficit in normal years, but only 6–8% in 1315–17, because a region-wide shortfall leaves nobody above `food_cap`. The famine dip is unchanged (0.24). About 5,300–5,800 grain a year is still lost above `food_cap`. As expected, moving grain across a famine needs stores.
- **`grain_relief` becomes the default** with the tuned drain (`data/real/experiments/relief.json`). Record the new golden hash.

**S3 stores** (switch `grain_stores`):
- **State.** Each settlement has a grain store `G_i`, carried from year to year.
- **Order each year, after trade:**
  1. a place in deficit draws first on its own store;
  2. relief (S2′) then draws on neighbours' surplus above `c_max N` and on the stores of neighbours not themselves in deficit, in the same shopping-model allocation;
  3. what is still above `c_max N` goes into the place's own store, up to `store_cap × N` (0.5, half a year's need), and the rest is lost as now;
  4. stores lose `store_loss` (0.08) a year to spoilage, vermin and holding.
- **Footing.** Clark (1999): carry-over reached about 40% of a harvest after cheap years, with storage costing up to 5% a year plus interest.
- **Accounting.** Drawn store counts in the food ratio, not in `market_in`. It loads routes when it moves as relief.
- **Expectation (pre-registered).** The 1315–17 famine dip falls from 0.24 to 0.18 or less. Regional stores stand at 20–50% of a year's need in the decade before 1315 and are mostly drawn down by 1317.
- **View,** in new files:
  - a small granary fill mark on settlements over 500;
  - a regional store line under the population chart;
  - the chronicle line "1316. The granaries of Harthorpe are empty."
- **Gate:** every v5 check; determinism; under 50 ms a year in wasm.
- **Reported:**
  - regional stores as a share of need over time;
  - the 1315–17 draw-down;
  - the famine dip against S2′;
  - grain still lost;
  - the failure rate.
- **Tuning.** `store_cap` and `store_loss` are the implementer's, within 0.25–1.0 and 0.03–0.15.
- **Stop** and report. W1 comes next if S3 passes.

**S3 result (b88f75f) and decisions (D82).** The gate passes at every setting. At the chosen `store_cap` 1.0 and `store_loss` 0.05 (Clark's figure for storage):
- the famine dip is 0.17–0.18, so the pre-registered target (≤ 0.18) is met on every map;
- stores before 1315 are 0.49–0.52 of need, with map 1 at 0.52, just over the expected 20–50% band. That is a near miss and is accepted;
- the famine deficit met rises from 6–8% to 55–59%.
- **`grain_stores` becomes the default** at 1.0 / 0.05. Record the golden hash.
- **Finding:** stores fill early (0.94 of need at year 100), empty by 1317, and barely refill (0.01–0.02 by year 400). They are being filled by the slack in cultivation, not by good harvests, and without weather there are no good harvests. That is what W1 supplies, so W1 is the next step.

**W1 re-gate (approved),** with relief and stores on, at `σ` 0.15 as D75 built it:
- **Gate:** every v5 check; determinism; under 50 ms a year in wasm.
- **Pre-registered:**
  - total population at year 200 within 20% of the same seed without weather (14.8k against 47.8k before the buffers);
  - stores refill between shocks: regional stores at least 0.2 of need at year 400.
- **Reported:**
  - Clark's three regressions in shadow mode, `b_Y` included;
  - harvest coefficient of variation;
  - the share of local deficit met by own store and by relief;
  - the famine dip.
- **Tuning.** If a v5 check fails, the implementer may tune `σ` down to 0.10 and the store values within their ranges, recording each change.
- **Stop** and report. If W1 passes, it becomes the default, and v6 closes with a milestone. S4 and S5 are decided then.

**W1 third gate (1928344) and the designer's decision (D83).** With relief and stores on, W1 still fails the top-10 check on every map. Population at year 200 is 0.30–0.34 of the run without weather (pre-registered: within 20%), and stores at year 400 are 0.06–0.08 (pre-registered: ≥ 0.2). σ 0.10 fails too. Clark in shadow mode is in range (`b_Y` −0.011 to −0.015). The implementer's diagnosis is right. Stores act only below need, but growth runs up to a food ratio of 1.2. A bad year slows growth without ever drawing on the store, and a good year fills a store that is never used. It is the same asymmetry as before, one level up.
- **Decided: option A, as the last attempt for W1.** A place tops up its food from its own store to `food_cap × need`, its growth target, not just to need. Households drew on stores to keep their consumption level through poor years, not only to avert hunger, so this is consumption smoothing. It is the direct use of the store that D81 meant. The store then smooths growth as well as famine, and good years pay for bad ones. Relief still covers deficits below need only.
- **Re-gate once at σ 0.15,** with the same gate and the same pre-registered expectations. The fallback to σ 0.10 is allowed as before.
- **If W1 fails again, it is dropped.** `local_harvests` stays off for good, v6 closes without weather, and the weather code stays behind its switch as the record. Option B (a smoothed food ratio, changing D15) and option C (σ 0.05) are declined: B reworks growth, and C is too small to matter.
- **Either way, report, then record the v6 milestone:** defaults, golden hash, validation, wasm timing, Kent, and screenshots in 1316 and 1450.

**v6 closed (f12337e, golden `0e65d09ec147b5c7`, 30.3 ms a year in wasm).** The defaults are goods by price (S1), grain relief (S2′) and stores (S3). The famine dip is 0.17–0.18 against v5's 0.24–0.25. Kept off as the record: the S0 shadow (used for reports), S2 priced grain, W1 weather and store smoothing.

**S4 and S5 (D84).**
- **S5 (coin, projects, mayors) is dropped.** It needs settlements to hold and spend money from budgets. S2 showed that paying for what towns now receive free breaks the hierarchy the whole model rests on.
- **S4 (goods in use) is not scheduled.** Iron raising yields and stone feeding bridges each add a positive feedback, and the W1 finding shows how the growth rule turns uneven effects into drift. It would be reconsidered only after the growth rule's asymmetry is dealt with.
- **Weather, one attempt (approved by the owner, D85):** a single attempt at weather with the market pull on the 10-year average inflow (`avg_inflow`), which addresses the asymmetry W1 found. It would be validated without weather first, then gated with weather at σ 0.15, and dropped if it fails.

**W2 brief (D85).**
- **Change.** Switch `pull_smoothed`: the market pull uses the 10-year moving average of inflow (`avg_inflow`, already kept), not the year's inflow. A market's traders and lords do not leave over one bad harvest. Nothing else changes.
- **Step 1, without weather.** Run full validation with `pull_smoothed` on. Report every check against v6 and the golden hash. Without weather the inflow is steady, so the result should be close to v6. If a check fails, stop and report.
- **Step 2, with weather.** `local_harvests` at σ 0.15, with relief and stores on.
  - Gate: every v6 check; determinism; under 50 ms a year in wasm.
  - Pre-registered, as before: total population at year 200 within 20% of the same seed with smoothing on and no weather, and regional stores at least 0.2 of need at year 400.
  - Reported: the top-10 overlap against the Off run, Clark in shadow mode, and the famine dip.
- **Result.**
  - If step 2 passes, `pull_smoothed` and `local_harvests` become the defaults, with a new golden hash.
  - If it fails, weather is dropped for good, and `pull_smoothed` is kept only if step 1 showed it harmless.

**W2 result (7d7173e) and decision (D86).**
- **Step 1 passed** with no check worse than v6. Runs passing every check went from 14/17/15 to 17/19/19.
- **Step 2's gate passed** on every map. Top-10 against Off is 20/20, and runs passing every check are 18/20/15.
- **The main pre-registered condition was met:** population at year 200 is 0.95–0.96 of the run without weather, against 0.30–0.37 under W1. Smoothing the pull removed the drain.
- **The second was missed:** stores at year 400 are 0.02–0.03, against an expected ≥ 0.2. Stores fill before 1315 (0.34–0.38) and never refill after the plague, because once the land is full little rises above `food_cap`, and relief takes it first. This is a limit of the store rule, not harm from weather, and it is recorded as a miss.
- **Decided: `pull_smoothed` and `local_harvests` (σ 0.15) become the defaults,** with a new golden hash. The famine dip is 0.18–0.20, slightly up from v6's 0.17–0.18, because stores are lower. Clark in shadow mode is in range.
- **A possible follow-up, not scheduled:** fill a place's own store before its surplus goes out as relief, so that stores refill between shocks.


## v7 plan: marketplaces (owner's decision, 22 October 2026)

**Why.** Divish has decided that the project does not stop at v6. Knowing that grain is pushed to towns for free, and is not bought and sold, breaks the fun of watching. The model has to be a trade economy underneath, not only on the surface. The design authority stays with the designer. This section is the plan.

**What failed before and why** (D78–D80). Under S2, a market's size grew on its net income. Once towns had to pay for grain, the Rihll–Wilson feedback that concentrates people into a few towns disappeared: every village kept a small market on its own rents. The mistake was to tie market size to profit. In Rihll & Wilson, a market's size responds to the flow through it, its revenue, and that is the variable v5 used and the checks were built on.

**The design: markets are the places where trade happens.** Medieval grain was sold at chartered markets, which took tolls (Britnell 1993; the Gazetteer of Markets and Fairs).
1. **Sellers.** Each settlement's grain for sale is its rent (paid by selling grain at market) plus what its people do not eat. It is taken to markets chosen as now, by `W_j^α exp(−β c)`. That keeps the feedback that makes towns: big markets draw more sellers.
2. **Buyers** buy at markets by price, using the S2 equilibrium machinery. The buyers are:
   - the market's own people and establishment;
   - places in deficit;
   - other towns;
   - the outside ports.

   So grain physically goes where it is wanted, and nothing is pushed, burned or free.
3. **Market size grows on turnover,** the value of trade passing through the market, as Rihll–Wilson's `D_j`. A market town's traders, brokers, inns and toll-takers scale with what passes through it.
4. **Market people live on real income:** tolls on turnover, the rents of the lords who live there, handling fees and goods sales. They buy their food at market prices. A town whose income cannot feed its people sheds them.

**Stages, each with a stop rule:**
- **M0, the diagnostic. It changes nothing, and is approved now.** In ordinary default runs (3 seeds a map), compute what each market's turnover would be under seller routing (rent plus surplus, valued at the shadow grain price), and compare it with today's `market_in`: rank-size slope, top-10 overlap and Spearman rank correlation. Then compute, for each place over 500:
  - its income under marketplace accounting (tolls at 2%, 5% and 10% of turnover; rents at `market_share`, received by `W^α` weights; fees; goods);
  - its food bill at shadow prices;
  - the toll rate at which it breaks even.

  **Expectation (pre-registered):** turnover concentrates like `market_in` (Spearman ≥ 0.8), and most towns over 2,000 break even at a toll of 10% or less, rents included. If towns cannot feed themselves at any plausible toll, the design is wrong, and we learn that before writing it.
- **M1, the design note,** at most two pages, written after M0. It covers flows and accounting, how K relates to turnover, tolls, the cost estimate, and how stores trade (buying cheap and selling dear, Clark's 12.5% a year).
- **M2, the build,** behind `marketplaces`. Gate: every v6 check; Clark's `b_P`, `b_Y` and R² gated per seed; under 50 ms a year in wasm. One tuning round. If it fails, stop and report; the designer then decides.
- **M3, stores in the market:** granaries buy low and sell high, which may fix the refill miss of D86.

**M0 result (8c6aa27, `docs/market/m0.md`) and the M1 brief (D87).** Both pre-registered expectations are met:
- turnover under seller routing ranks like `market_in` (Spearman 0.955–0.982; the rank-size slope agrees within 0.03; the top ten are the same in 15 of 18 run-years);
- 91–93% of towns over 2,000 break even at a toll of 10% or less, and most need none.

**The main finding is that rents carry the towns.** In towns over 2,000 the means are turnover 4,414, rents 4,058, fees 356, goods 51 and a food bill of 3,712. This is the medieval consumer city of Sombart and Weber: towns lived largely on the rents of the landlords and churchmen who lived in them (Britnell 1993, Dyer). Tolls are a small top-up. Fees matter at ports and transshipment towns.

**M1, the design note (approved),** `docs/market/m1-design.md`, at most two pages. It should propose a two-level market, which follows from M0 and is cheaper than S2:
1. **Village to market.** Each place takes its grain for sale, rent plus surplus, to the markets it chooses by `W^α exp(−β c)` as now. That fixes each market's turnover. A place in deficit buys at the market it would choose the same way.
2. **Market to market.** Prices clear between markets only, by the entropy equilibrium (the S0, S1 and S2 solver) over market pairs within `C_max`. Grain moves between markets where the price gap pays for it. Only markets above a size threshold take part, which should cut the arcs well below S2's.
3. **Who buys at a market:**
   - its market people's food, from their income (rents received, tolls, fees, goods sales), within their budget;
   - deficit villages;
   - the ports.

   No establishment purchase, because that is what collapsed S2 (D79). Rent income above the food bill buys goods and imports as now.
4. **Growth.** W follows `ε(T_j − K W_j)`, with `T_j` the turnover in grain units. M0 shows `T_j` tracks `market_in`, so K should carry over. Market people who cannot pay for their food are shed.
5. **Grain unsold at the floor:** where it goes before M3. It must not count toward W.
6. **The toll rate:** a starting value, with M0's distribution as the guide.
7. **A cost estimate** against 50 ms in wasm.
8. **The Clark checks** as the M2 gate.

Then stop for review.

**M1 note reviewed (5420934, `docs/market/m1-design.md`); decisions (D88).** The note is approved as written. It is coherent end to end. The rent grain arriving at a market is the lords' income. The lords and their households, the market people, buy their food from it at the market price. Any excess clears to deficit villages, ports or other markets, so the v5 loop (grain reaches towns and feeds market people) now runs through prices and nothing is burned.
1. **`market_min` 50: approved,** as a tuning parameter.
2. **Deficit villages pay only from their own sales: approved.** Relief and the villages' own stores stay free, as the manor and charity. A region-wide famine will still bite the poorest villages hardest, which is the historical pattern.
3. **Unsold grain lost in M2, market stores in M3: approved, with one clarification.** S3's village stores and S2′ relief stay on in M2, as in v6 W2. Only the new market-level stores wait for M3. Otherwise the famine dip would not be comparable.

**M2 build (approved),** behind `marketplaces`:
- **First an untuned run,** to check that K carries over (pre-registered: the hierarchy checks pass on at least two maps without tuning).
- **Then one tuning round** over `toll` (0.01–0.10), `market_min`, `market_share` and K, with every change recorded.
- **Gate,** as in the note:
  - every v6 check with W2 on;
  - Clark per seed (`b_P` 0.8–1.2, R² ≥ 0.6, `b_Y` −0.15 to 0);
  - determinism;
  - at most 50 ms a year in wasm.
- **Reported:** as in the note, plus the share of towns over 2,000 whose food is fully paid.
- **Stop** and report either way. If it passes, `marketplaces` becomes the default and the gravity grain push is retired.

**M2 first build (20db863, D89) and the designer's decision (D90).** No towns form, untuned or tuned (largest place 1,371–2,475 against 14,537). The implementer's diagnosis is right. With W growing on grain sold, and grain at a market bought almost only by its own market people, turnover scales with W itself and the feedback dies. The error is the designer's. M0 measured grain arriving, and the brief then said only grain sold may count. I treated arriving grain as P3 in disguise without asking what it means economically.

**What arriving grain means.** Grain brought to a market and not yet eaten is a supply of food there. When a market has more grain than its people eat, food is cheap and people come: that is the Malthusian draw of a town with plenty. The newcomers then eat the grain. This is how a market town grows on its trade in a real economy, and it is what the Rihll–Wilson `D_j` stands for. P3 was something else. v6 wasted about 70% of what arrived (K 3.39 per market person against 1–1.2 eaten) and still counted all of it, so the waste was permanent.

**Decided: option A, with K re-read as consumption and waste made a gated check.**
1. **W follows `ε(A_j − K W_j)`,** with `A_j` the grain arriving at market j (rent plus surplus, on W2's ten-year average). That is M0's quantity, which ranks like `market_in`.
2. **K becomes the grain a market person and their share of the town's establishment consume,** within 1.2–2.0: food, ale, fodder and the lords' households. It is tuned together with `market_share` (the rent share), so that the equilibrium market size, `A_j / K`, stays close to v6. M0 says rents carry the towns, so a lower rent share with a lower K should keep the hierarchy.
3. **Market people buy and eat what arrives,** at the market price, within their income, as built. Grain still unsold at the floor is lost this year and reported. It is the transient while W catches up, not a permanent sink.
4. **A new gated check separates this from P3:** after year 200, grain unsold at markets is at most 10% of what arrives (v6 burned about 70%). If the hierarchy holds only with large waste, this design has failed, and we stop.

**Re-run:** first untuned at K 1.5 with `market_share` scaled by 1.5/3.39. Pre-registered: the hierarchy checks pass on at least two maps, and unsold is at most 10%. Then one tuning round over K (1.2–2.0), `market_share`, toll and `market_min`, then the full M2 gate plus the waste check. Stop and report either way. Option B (stores first) is declined: it delays the same problem. Option C is declined: the owner has ruled that v6 is not the end.

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
4. **v5 The outside world.** Specified below, in "v5 specification: the outside world". It is followed by the release, then v6 (markets, D70). Joined regions, coarse routing, way-stations, pastoral highlands and per-region parameters are dropped, since they are the scale-up that sinks projects of this kind.

**Map generation track.** v1 ships noise, thermal erosion, hydrology and yield. Later passes, in order of effect on realism: hydraulic erosion with floodplains and deltas as the best land; lakes and marshes; coastline detail; a rain-shadow climate with biomes from temperature and moisture; resources placed by geology; tectonic plates giving ranges a direction. Each is a new pass in the pipeline and changes nothing else.

**Real terrain.** Importing a free 90 m DEM (SRTM) of a documented region, resampled to the chosen cell size, is the single most valuable step after v1. Run on medieval Kent or the Rhône valley and compare the settlements the model produces with the ones that existed. If the model puts a city where Canterbury or Lyon stands, the model is right rather than merely plausible. Generated maps stay the tool for development and tuning; real terrain is for validation and for the demonstration that convinces people.

**Never planned, in any version:** individual agents, wages and a labour market, and money as a floating currency. Nothing on the behaviours list needs them, and each is the usual way a project of this kind sinks. Market-clearing prices in grain units, as an aggregate spatial equilibrium between settlements, were admitted by D70 (see v6 below). The settlement stays the unit.

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
