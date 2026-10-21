# Real-terrain experiments: market inflow and the outside world

The two experiments the designer asked for after the Kent test ([`real-terrain.md`](real-terrain.md)), each behind a switch that defaults off, scored on Kent with the same 20 seeds, and the lowland AUC as the standard measure.

**Both are negative as specified. The reasons are structural, and they point to two specific changes. Those are design decisions, set out at the end.**

| Run | AUC vs lowland: all / ≥1,000 / <1,000 | Inland towns found | Model places on rivers | Spearman | London: top-3 place within 10 km | Region pop. (median) | Largest (median) |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| v1 (baseline) | 0.48 / 0.54 / 0.44 | 29% | 49% | 0.08 | 5 of 20 | 95,478 | 19,442 |
| Exp 2: gross inflow | 0.46 / 0.53 / 0.42 | 28% | 46% | 0.05 | 5 of 20 | 101,811 | 16,776 |
| Exp 2 diagnostic: gross, β 0.35 | 0.46 / 0.52 / 0.42 | 29% | 46% | 0.04 | 6 of 20 | 105,691 | 8,262 |
| Exp 2 diagnostic: gross, β 0.35, seed pull 100 | 0.44 / 0.60 / 0.34 | 17% | 62% | 0.23 | 3 of 20 | 38,234 | 3,039 |
| Exp 1: outside, size 5,000 | 0.49 / 0.61 / 0.41 | 23% | 47% | 0.12 | 3 of 20 | 48,992 | 3,900 |
| Exp 1: outside, size 1,000 | 0.47 / 0.56 / 0.41 | 28% | 48% | 0.13 | 8 of 20 | 81,712 | 16,866 |
| Exp 1: outside sells only (20,000/yr) | 0.47 / 0.55 / 0.42 | 29% | 49% | 0.10 | 2 of 20 | 98,679 | 17,362 |

Full scores for each run are in [`real/kent/experiments/`](real/kent/experiments/); the parameter files are in `data/real/experiments/`.

## Experiment 2: markets grow on gross inflow (D24)

**What the code already did.** Since D13, every settlement sends a fixed share of its harvest (`market_share`, 0.49) to markets by gravity, whether or not they need it. So that half of the published Rihll–Wilson model was in place. What differed:

- growth counted *net* food imports (`food_in − food_out`), not gross market inflow;
- the share sold at a place's own market counted for nothing.

`gross_inflow` restores the published D_j: everything delivered to a market, plus the home share.

**Result: no change** (lowland AUC 0.46 / 0.53 / 0.42; inland towns found 28%).

**Why.** The settlement accounts show it directly. An inland village of about 500 people sends about 500–750 units of food a year to market and receives 5–8. Gross or net, its inflow is about zero. The cause is the attraction, `W_j^α` with α = 1.1. W_j is the non-farming population, derived as "people beyond what the place's own fields feed", plus a seed of 10:

1. A city with W = 5,000 pulls about 900 times as hard as a village. That outweighs the distance decay across the whole trade range: at cost 25 a city still outbids a village at cost 3. So every village ships to the river cities, and no inland market can start.
2. W measures land shortage, not market activity. When people arrive at an inland place, cultivation claims more fields to feed them, so its W stays at zero. Only towns hemmed in by water, neighbours or the cultivation radius build up W, which is why the model's towns are water towns.

**Diagnostic, not tuning.**
- Raising the seed pull to 100–300 at β 0.2 collapses most runs: villages trade among themselves and migration empties them.
- Raising β to 0.35 makes the river bias *worse*: 62% of places on rivers, 17% of inland towns found. Water travel costs 0.08–0.2 per km, so a short decay length on cost is a long one in km for river and sea places.

No parameter setting rescues it, because the attractor is the wrong quantity.

## Experiment 1: the outside world (D25)

As approved: one virtual partner on the map-edge sea cell nearest the main river mouth (for Kent, the North Sea east of the estuary). It has a fixed size (`outside_size`), its own decay (`outside_beta` 0.05), and the usual trade range. It buys marketed produce, which leaves the map, and sends `outside_out` of goods to the settlements by pull and closeness. Both directions load the routes.

**Result: London does not form in any variant.**

- **Size 5,000:** the outside world outbids every local market for the ports' and river towns' food. Regional population halves (95,000 to 49,000), the coastal cities shrink, and the largest place (median 3,900) ends up *inland*: Cuckfield, Hoddesdon, East Grinstead.
- **Size 1,000:** close to v1, with a top-3 place near London in 8 seeds against 5. The largest city stays at the estuary mouths.
- **Seller only:** the largest city moves toward the lower Thames and Medway in 9 of 20 seeds (Gravesend 4, Dartford 3, Rochester 2, against 3 in v1). That is the direction the designer accepted ("Gravesend rather than Southwark"), but weakly. Goods count only at `through_weight` (2.5%) in growth, so a gateway gains almost nothing from handling them.

**Why.** Food is the model's only currency of growth. The outside world as a buyer is a drain, and as a seller its goods barely register. London grew on the margin it earned handling trade in and out of the region. The model has no income from handling trade, only the through-traffic term at 2.5%.

## What I propose (for the designer)

Both results point at the same place: how a place's pull and growth are measured. Two changes, each a departure in the model's equations, so I would not make them without approval:

**A. Market size as its own state: the published model's W_j.** Every settlement gets a market size W_j:
- it starts at a common small value;
- it changes by ε(D_j − K·W_j), with D_j the gross market inflow;
- it is used for attraction instead of the derived non-farming population.

Population is then farmers plus the market's people, so newcomers to a market no longer turn into farmers and erase its W. This is the literal Rihll–Wilson state variable, the retail "floorspace". Experiment 2 restored only the D side; this restores the W side. With all places starting equal, the published model's competition among neighbours should give inland markets at a spacing set by β, with no separate spacing term, as the designer expects.

**B. Income from handling trade.** A place earns a margin on the goods that pass its anchor or are transshipped there. The margin counts as food it can buy, so it feeds people like food imports. This lets an entrepôt grow on through-trade. Combined with the outside world as a seller, it is the mechanism by which London should form at the tidal head. Today this is the `through_weight` term at 2.5%, which is too small to matter. B would replace it with an accounted margin.

Order: A first, as the v1.1 candidate, scored on Kent with this table and also run through the full validation on the three generated maps. A v1.1 fix must keep v1 passing. Then B with the outside world. About a day each.

Both switches added here stay in the code, default off; v1 results and the golden hash are unchanged.

## v1.1 candidates: A (market size as its own state) and B (income from handling trade)

Approved by the designer. Both are in the code behind switches that still default off, because neither setting yet passes the full validation on the generated maps.

**A, `market_state` (D26).**
- Each settlement has a market size W_j, in people. It starts at `market_w0` with a floor of `market_floor`, and grows or shrinks by ε(D_j − K·W_j) on gross market inflow.
- W_j moves with the migrants the market actually gains or loses.
- Attraction uses W_j in place of the derived non-farming population.
- The home self-term is on (`home_weight` 1), as in the published model.

**B, `handling_income` (D27).** Inflow gains a margin on trade passing the anchor (`handling_margin`) and on trade changing mode within a cell of it (`transship_margin`: boat to land, or river barge to sea ship). This replaces the 2.5% through-traffic weight; the yearly growth cap still applies.

### Kent (20 seeds; v1 defaults otherwise unless stated)

| Run | AUC vs lowland: all / ≥1,000 / <1,000 | Inland towns found | Model places on rivers | Spearman | Inland places >500: spacing, count per seed | Largest city near (seeds) | Largest share |
| --- | --- | ---: | ---: | ---: | --- | --- | ---: |
| v1 | 0.48 / 0.54 / 0.44 | 29% | 49% | 0.08 | 10.1 km, 16 | Maldon 7, Rochford 3, Dover 2, Colchester 2, … | 0.21 |
| A, α 1.05, K 2 | 0.49 / 0.64 / 0.39 | 26% | 41% | 0.17 | 11.6 km, 10 | | |
| A, α 1.1, K 2 | 0.49 / 0.61 / 0.41 | 27% | 31% | 0.14 | 13.7 km, 10 | | |
| A + B, tuned on generated maps | 0.49 / 0.61 / 0.42 | 27% | 54% | 0.15 | 10.3 km, 11 | **Gravesend 15**, Dartford 1, Barking 1, … | 0.28 |
| A + B + outside world (seller) | 0.48 / 0.61 / 0.39 | 26% | 54% | 0.16 | 10.3 km, 10 | **Gravesend 16**, Dartford 1, Barking 1, … | 0.29 |

Real: inland towns over 500, nearest-neighbour spacing 10.6 km, 29 towns. The full scores are in `docs/real/kent/experiments/` and the parameter files in `data/real/experiments/market-*.json`.

What A and B do on Kent:
- **The larger towns improve.** Lowland AUC rises from 0.54 to 0.61–0.64, and rank agreement doubles.
- **B puts the largest city on the tidal Thames.** It sits at Gravesend in 15–16 of 20 seeds, where v1 put it at the estuary mouths. This is the outcome the designer accepted for now: the model has no notion of a first bridge, so the city stops at the head of easy sea navigation, not at Southwark. Its share of the region's people is 0.28, against 0.21 in v1; it does not capture everything. (The real London's share of this region was roughly 0.3.)
- **The small inland towns do not return** (inland towns found 26–27%, small-town AUC about 0.40). A consolidates the middle of the hierarchy: places of 500–2,000 fall from 37 to 18 per seed.

**Why the small inland towns stay missing: founding.** Even hamlets of under 100 people are placed mostly by water, 72% of them on river or coast sites against 28% of the land. The founding score rewards transport (navigable river, coast, hub, road) for every new settlement. Markets can only grow where settlements exist, and few exist inland. This is the next spec question for the designer. Villages were placed by farmland and water supply; transport decided which villages became towns. In the model, transport decides both.

### Validation on the generated maps (A + B, the best of two searches; 20 seeds per map)

| Check | Map 1 | Map 2 | Map 3 |
| --- | --- | --- | --- |
| Settlements, over 2,000, over 5,000, largest share, spacing, siting, failure rate, forest, stability | pass | pass | pass |
| Rank-size slope (≥ 15/20) | **12/20** | 15/20 | 20/20 |
| River-corridor city over 5,000 (≥ 0.80 of seeds) | **0.60** | **0.70** | **0.65** |
| Median count over 5,000 (≥ 2) | 2 | 2 | **1** |
| Runs passing every check | 10/20 | 10/20 | 16/20 |

K per map: the searches tuned one value for all three maps; the best is K = 3.54 (α 1.081, β 0.124, ε 0.152, market share 0.448, handling margin 0.029, transship margin 0.26). The earlier A-only setting on Kent used K = 2. Validation fails on the river-corridor city: with A the hierarchy is flatter, with one or two cities over 5,000, and one of them is on the main river corridor in about two thirds of seeds, against the 0.80 required. v1 had 0.95 / 0.85 / 1.00.

## Founding correction, one re-tune of A + B (designer-approved round)

**Change (D28).** Hamlet founding no longer scores transport. `w_hub`, `w_river`, `w_coast` and `w_road` are 0, so a site is scored on reachable food (`w_food` 1.5), the shadow and the access terms, plus noise. It still needs water within 1 km. A and B decide which villages become towns. One re-tune on the generated maps (56 settings × 3 maps × 3 seeds) kept its starting point as best: K 3.54, α 1.081, β 0.124, ε 0.152, handling margin 0.029, transship margin 0.26, w_food 1.5, noise 0.9, threshold 0.69, shadow weight 3.85. Parameters: `data/real/experiments/market-handling-founding.json`.

### Validation on the generated maps (20 seeds per map): FAILED

| Check | Map 1 | Map 2 | Map 3 |
| --- | --- | --- | --- |
| Rank-size slope (≥ 15/20) | 16/20 | 15/20 | 19/20 |
| Median count over 5,000 (≥ 2) | 2 | 2 | 2 |
| Market spacing 8–15 km (≥ 15/20) | **12/20** | 16/20 | 19/20 |
| River-corridor city over 5,000 (≥ 0.80; fallback 0.60) | **0.50** | **0.45** | 0.65 |
| All other checks | pass | pass | pass |
| Runs passing every check | 8/20 | 12/20 | 14/20 |

As the designer expected, the rank-size and median-count misses resolve. The corridor check gets worse (it was 0.60 / 0.70 / 0.65 before the founding change) and is below the 0.6 fallback on two maps; market spacing on map 1 drops to 12/20.

### Kent (20 seeds)

| Run | AUC vs lowland: all / ≥1,000 / <1,000 | Inland towns found | Model places on rivers | Spearman | Inland places >500: spacing, count | Largest city near (seeds) | Largest share |
| --- | --- | ---: | ---: | ---: | --- | --- | ---: |
| v1 | 0.48 / 0.54 / 0.44 | 29% | 49% | 0.08 | 10.1 km, 16 | estuary mouths | 0.21 |
| A + B | 0.49 / 0.61 / 0.42 | 27% | 54% | 0.15 | 10.3 km, 11 | Gravesend 15 | 0.28 |
| A + B + founding correction | 0.47 / 0.56 / 0.42 | 28% | 42% | 0.15 | 10.4 km, 12 | Gravesend 12 | 0.26 |

Real: inland towns over 500 at 10.6 km, 29 of them.

Settlements by size band, share inland (land is 72% inland):

| Run | Under 100 | 100–500 | 500–2,000 | 2,000+ |
| --- | --- | --- | --- | --- |
| v1 | 58 per seed, 28% inland | 72, 34% | 37, 38% | 5, 6% |
| A + B | 87, 35% | 50, 29% | 36, 26% | 7, 22% |
| A + B + founding correction | 87, 33% | 44, 37% | 30, 36% | 7, 23% |

**The founding correction did not spread villages inland on Kent.** New hamlets are still two-thirds on river or coast sites.

**Why: the water filter, not the score.** A founding site must have a stream, a river or the coast within 1 km (`water_radius_km`). Streams begin at 60 km² of drainage (`stream_area`), so on a 1 km grid only about 760 of Kent's 11,500 land cells carry a stream or river. Most of the Weald and the Downs is never a candidate, whatever its score. Real villages used springs, wells and small brooks well below 60 km² of drainage. On Kent's uniform soil, the food term also has little to work with, as the designer anticipated.

## Water supply at brook scale, one re-tune of A + B (designer-approved round)

**Change (D29, switch `spring_water`).** The water filter for founding (and for the starting hamlets) becomes a looser filter plus a score term. A site is allowed if a brook, meaning a cell with at least `brook_area` = 5 km² of drainage, or the sea lies within `supply_radius_km` = 3 km. Water then enters the score as `w_water_cell` for a brook or river in the cell, and `w_water_near` for one, or the sea, within 1 km. No wells layer. Transport stays out of founding (D28). One re-tune on the generated maps (56 settings × 3 maps × 3 seeds); best: w_water_cell 1.14, w_water_near 0.43, w_food 1.10, noise 0.31, threshold 0.33, shadow weight 4.35, K 3.50, α 1.095, β 0.142, ε 0.145, transship margin 0.24, handling margin 0.046. Parameters: `data/real/experiments/market-handling-founding-brooks.json`.

**Land eligible for founding:**

| Map | v1 rule (any stream ≥ 60 km², river or sea within 1 km) | Brook rule (brook ≥ 5 km² or sea within 3 km) |
| --- | ---: | ---: |
| Kent | 22% | 100% |
| Generated map 1 / 2 / 3 | 26% / 27% / 26% | 100% / 100% / 100% |

On 1 km cells, almost every cell has a cell with 5 km² of drainage within 3 km, so the brook rule excludes nothing: not ridge tops, and not plateaus. In practice the filter is gone and water acts only through the score term. If the designer wants dry ground kept out, the radius or threshold has to tighten. With 1 km cells the 1 km adjacency bonus is the effective rule.

### Validation on the generated maps (20 seeds per map): FAILED, on the river-corridor check only

| Check | Map 1 | Map 2 | Map 3 |
| --- | --- | --- | --- |
| Settlements | 20/20 | 20/20 | 20/20 |
| Over 2,000 | 16/20 | 18/20 | 19/20 |
| Over 5,000 (median) | 19/20 (2) | 20/20 (2) | 20/20 (2) |
| Largest share | 20/20 | 20/20 | 20/20 |
| Rank-size slope | 19/20 | 15/20 | 20/20 |
| Market spacing | 15/20 | 18/20 | 17/20 |
| Siting, failure, forest, stability | pass | pass | pass |
| River-corridor city over 5,000 (≥ 0.80; fallback 0.60) | **0.35** | **0.40** | **0.45** |
| Runs passing every check | 7/20 | 13/20 | 14/20 |

Every per-seed check passes on every map. The middle tier is back: 198 / 217 / 197 settlements, about 10 over 2,000 and 2 over 5,000. The river-corridor aggregate falls further, to 0.35–0.45, below the 0.6 fallback too. Villages and towns now fill the land between the rivers, and one of the two cities over 5,000 lands off the main river corridor in more than half the seeds.

Performance: founding now scores every land cell, so a year takes 30 ms native, against 18 in v1. This needs optimising before it becomes the default.

### Kent (20 seeds)

| Run | AUC vs lowland: all / ≥1,000 / <1,000 | Inland towns found | Model places: river / coast / inland | Spearman | Inland places >500: spacing, count | Largest city near (seeds) | Largest share |
| --- | --- | ---: | --- | ---: | --- | --- | ---: |
| v1 | 0.48 / 0.54 / 0.44 | 29% | 49% / 13% / 38% | 0.08 | 10.1 km, 16 | estuary mouths | 0.21 |
| A + B | 0.49 / 0.61 / 0.42 | 27% | 54% / – / – | 0.15 | 10.3 km, 11 | Gravesend 15 | 0.28 |
| A + B + founding | 0.47 / 0.56 / 0.42 | 28% | 42% / 22% / 36% | 0.15 | 10.4 km, 12 | Gravesend 12 | 0.26 |
| A + B + founding + brooks | 0.43 / 0.55 / 0.36 | 29% | 31% / 14% / 55% | 0.16 | 9.3 km, 20 | Faversham 7, Gravesend 7 | 0.24 |
| Real | | | 26% / 9% / 66% | | 10.6 km, 29 | London | ≈0.3 |

Settlements by size band, share inland (land is 72% inland):

| Run | Under 100 | 100–500 | 500–2,000 | 2,000+ |
| --- | --- | --- | --- | --- |
| v1 | 58 per seed, 28% inland | 72, 34% | 37, 38% | 5, 6% |
| A + B + founding | 87, 33% | 44, 37% | 30, 36% | 7, 23% |
| A + B + founding + brooks | 118, 70% | 77, 61% | 32, 52% | 9, 52% |

**What this round shows.** The brook rule does what it was meant to do for the pattern:
- villages now cover the land in proportion to it;
- the site mix of the model's top places (55% inland) is close to the real one (66%);
- inland places over 500 rise from 12 to 20 per seed, at 9.3 km spacing against the real 10.6.

The model still does not find the *right* inland places: inland towns are found 29% of the time, and the small-town AUC against lowland falls to 0.36. Its inland towns sit on higher ground than the real ones, which lie in the valleys. On Kent the founding score has nothing that prefers a valley floor: soil is uniform, and yield varies only with slope. The real Kent's valley loams, Weald clay and chalk downland differ a great deal. The soil map the designer deferred is now the input this test needs.

## Soil: SoilGrids and a valley-floor proxy (v1.1 defaults, mapgen only)

**Correction first.** The Kent DEM header gives a cell size of 999.8 m, and the importer floored 127.97 cells to 127, so every Kent run above used a 127 × 127 grid about 0.8% smaller than the real coordinates (up to about 1 km at the far edge). The importer now rounds near-whole cell counts. Kent imports at 128 × 128, and v1 and v1.1 were re-run on it below; v1's parameters are in `data/real/experiments/v1.json` and reproduce the v1 golden hash. The overall picture holds. One claim changes: on the true grid, **v1 already put its largest city at Gravesend in 6 of 20 seeds** (not never). v1.1 raises that to 15.

**Soil data.** The European Soil Database cannot be downloaded directly: ESDAC hands it out through a request form that asks for the requester's name, organisation and email. In its place: ISRIC SoilGrids 2.0 (250 m, CC-BY 4.0), fetched by `tools/real/fetch_soil.py`, which uses topsoil (5–15 cm) clay, sand and coarse fragments. It scores medieval arable quality from 0.1 to 1, penalising:
- heavy clay over 30%;
- sand over 55%;
- coarse fragments over 10% (thin chalk and flint soils).

SoilGrids leaves built-up areas blank (London and every modern town). Those holes are filled from the surrounding cells, because blank town sites would leak the answer. The importer reads any soil grid (`soil_file`), so the ESDB can replace SoilGrids if it is requested. The valley-floor proxy is a topographic position index, height above the mean within 3 km (`soil_tpi`, fixed at 0.5 beforehand and not tuned). Both are rescaled to the mean yield of uniform soil, so they move yield around without adding any.

### Kent, 128 × 128, 20 seeds

| Run | AUC vs lowland: all / ≥1,000 / <1,000 | Inland towns found | Model places on rivers | Spearman | Inland places >500: spacing, count | Largest city near (seeds) | Largest share |
| --- | --- | ---: | ---: | ---: | --- | --- | ---: |
| v1 | 0.48 / 0.55 / 0.43 | 30% | 48% | 0.12 | 9.3 km, 14 | Gravesend 6, Faversham 3, Maldon 3 | 0.22 |
| v1.1 (uniform soil) | 0.44 / 0.54 / 0.37 | 31% | 32% | 0.11 | 9.1 km, 20 | Gravesend 15, Faversham 3 | 0.30 |
| v1.1 + SoilGrids | 0.46 / 0.51 / 0.43 | 30% | 34% | 0.07 | 9.0 km, 21 | Gravesend 15 | 0.30 |
| v1.1 + valley floor | 0.41 / 0.51 / 0.35 | 27% | 30% | 0.17 | 8.8 km, 22 | Gravesend 6, Faversham 5 | 0.26 |
| v1.1 + both | 0.47 / 0.51 / 0.44 | 28% | 31% | 0.11 | 9.0 km, 18 | Gravesend 9 | 0.27 |
| Real | | | 26% | | 10.6 km, 29 | London | ≈0.3 |

**Neither soil layer improves placement.** The lowland AUC stays between 0.41 and 0.47 for all towns, and the inland find rate between 27% and 31%.

### Do the layers carry information about where towns were?

Each static feature, scored directly against the real towns: the AUC of the real towns' cells against lowland cells (0.5 = no information).

| Feature | All | ≥1,000 | <1,000 |
| --- | ---: | ---: | ---: |
| SoilGrids quality | 0.41 | 0.40 | 0.42 |
| Valley floor (−TPI) | 0.53 | 0.62 | 0.48 |
| Low elevation | 0.29 | 0.44 | 0.20 |
| Near a navigable river | 0.42 | 0.45 | 0.40 |
| Near the coast | 0.37 | 0.50 | 0.29 |

None points toward the small towns. Within the lowlands they sit *higher* than average (on dry ground above the marshes and estuaries), and not on better soil. SoilGrids' quality is also low-contrast here: on lowland cells, p10 0.83, median 0.92, p90 0.98. Only the valley-floor term carries some signal, and only for towns of 1,000 or more.

**What this means.** With the first-nature data available, the lowland AUC for small towns has a ceiling near 0.5, whatever the model does. Their siting at the scale of a few km is second-nature: roads, the spacing of markets, lordship and charters. The model's 0.37–0.44 for small towns is therefore partly a property of the data. The number to judge small towns by is their count and spacing, where v1.1 has moved toward the real pattern: 20 inland places over 500 at 9.1 km, against 14 in v1 and 29 at 10.6 km in reality. The location AUC remains the right test for towns of 1,000 or more, and there the model stays at 0.51–0.55, about where the valley-floor feature alone gets (0.62).

**Recommendation.** Keep uniform soil as the Kent default for now, and keep the soil and valley inputs as options (`data/real/kent/map-soil*.json`, `map-tpi.json`). The next gains for Kent are not soil. They are second-nature inputs: Roman roads as a given network at the start (Watling Street and Stane Street, which several missed towns sit on), or the outside world as a seller, now that B makes the tidal Thames the gateway.

## Roman roads as an initial road network (pre-registered, before running)

Written and committed before any run with roads.

**Change.** Roman roads become input data: an initial road set in `Terrain`, rasterised from published road lines, the same kind of input as the DEM. The model is unchanged. Roads present at the start cost `cost_road` per km like any other road, and the road step adds to them as before. Research footing: Dalgaard et al. 2022 on the persistence of Roman roads.

**Expectation (the designer's).** Towns form at the crossings of Watling Street with the Medway and the Stour, that is, at Rochester and at Canterbury. If they appear, the model has reproduced the two best-documented town sites in the region from roads plus rivers alone.

**How it will be scored.**
1. Primary: the share of the 20 seeds with a model place of 1,000 or more within 3 km of Rochester, and the same for Canterbury, against v1.1 without roads, run on the same seeds.
2. The usual table: lowland AUC for towns of 1,000 or more; count and spacing of inland places over 500; the largest city; and the share of settlements on or next to a Roman road, against the share of land that is.

No parameter is re-tuned for this run.
