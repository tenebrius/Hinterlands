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

### Result: the pre-registered expectation fails

**Data.** Itiner-e static version 2024 (de Soto, Pažout, Brughmans & Bjerregaard Vahlstrup 2025, *Scientific Data*; Zenodo 10.5281/zenodo.17122148; CC BY 4.0). It is rasterised by `tools/real/fetch_roads.py` into `data/real/kent/roads.asc`:
- 58 road segments in the box, including Watling Street (Londinium–Durobrivae–Durovernum–Portus Dubris), the Richborough, Lympne and Reculver branches, London–Colchester, London–Lewes and the Weald roads;
- all are classed Conjectured in the dataset, and Hypothetical segments are dropped;
- 1,337 road cells, 8% of the grid; 32% of land lies on or next to a Roman road.

The run uses v1.1 defaults with uniform soil (`map-roads.json`), 20 seeds, no re-tuning. Full scores: `docs/real/kent/experiments/roads.md`.

| Kent, 20 seeds | Rochester: place ≥1,000 within 3 km | Canterbury | Settlements on or next to a Roman road (all / ≥1,000) | Lowland AUC: all / ≥1,000 / <1,000 | Inland >500: spacing, count | Largest city near |
| --- | ---: | ---: | --- | --- | --- | --- |
| v1.1 | 1/20 | 1/20 | 26% / 26% | 0.44 / 0.54 / 0.37 | 9.1 km, 20 | Gravesend 15 |
| v1.1 + Roman roads | 1/20 | 0/20 | 20% / 24% | 0.43 / 0.48 / 0.40 | 8.1 km, 20 | Gravesend 6, Rayleigh 4 |
| Diagnostic: no shadow, no roads | 0/20 | 0/20 | 24% / 5% | | | |
| Diagnostic: no shadow, + roads | 0/20 | 0/20 | 29% / 24% | | | |
| Real | Rochester | Canterbury | 47% of real towns; 32% of land | | 10.6 km, 29 | London |

Rochester and Canterbury do not appear. Settlements become *less* likely to sit on a Roman road (20%, against 32% of land and 47% of real towns), and the largest city leaves the tidal Thames more often.

**Why, from the model.**
1. **The shadow is measured in travel cost.** A road makes neighbours along it cheaper to reach, so they fall inside each other's shadow and suppress one another. With the shadow off (diagnostic only), roads raise settlements on roads from 24% to 29% and places of 1,000+ from 5% to 24%, but still not above the land share.
2. **Nothing rewards a site on a road.** Founding ignores transport by design (D28). A place on a road gains only the handling margin on traffic passing its anchor (0.046 of value), which is small next to market inflow.
3. **A Roman road crossing a river is not a bridge.** A crossing away from a ford pays +5 even on a road, because bridges are a 1.x feedback that is not applied. In the model, Watling Street's crossings of the Medway at Rochester and of the Stour at Canterbury are no cheaper than anywhere else on those rivers. They are not hubs and not transshipment points.

**For the designer.** Three ways the road effect could enter, each a spec decision:
- (a) **Roman bridges as input data**, with the crossing cost zero where a Roman road crosses a river. This is the cheapest, and it is what the expectation actually rests on: the crossing becomes a junction of land and water routes, so through-traffic and transshipment income land there.
- (b) **Shadow in km rather than travel cost.** Bosker & Buringh's bands and Christaller's spacing are distances, so a road would no longer intensify competition along itself.
- (c) **A larger margin on through-traffic.**

(a) and (b) are input or measurement corrections; (c) is tuning. Not tried: the test was run once, as pre-registered.

### Designer's reading of the roads result

The test was run once and reported as pre-registered, with a land-share baseline and a diagnostic separating the causes. It failed, and the failure is informative.

1. **The expectation was a specification error, and it was the designer's.** It rested on Watling Street's river crossings being special. The brief said a crossing costs 0 "at a ford or bridge", but the model has fords only, and that was not checked before the expectation was written. Rule from now on: a pre-registration names the mechanism its expectation depends on, and the implementer confirms the code has it before the run.
2. **Roman roads are the first Kent input with signal for small towns.** 47% of real towns sit on or next to one, against 32% of land; soil and terrain carried none. The model moves settlements *off* roads (20%), the opposite of the record. That is a model fault, not noise, and the shadow diagnostic points to its cause.
3. **Options.** (a) is approved below, alone. (b), the shadow in km, has research footing (Bosker & Buringh's bands and Bracton's spacing are distances) and is the likely fix for point 2, but it changes the generated maps and needs a re-tune and full validation; it is the next candidate, decided after (a) reports. (c) is declined: raising a margin until the expected towns appear is tuning toward the answer.

## Roman bridges as input data (pre-registered, before running)

Written by the designer and committed before any run with bridges. This is a second test, not a rescue of the first: the roads test above stays recorded as failed.

**Change.** Every cell where an Itiner-e road crosses a river or stream is a bridge: the fixed crossing cost there is 0, as at a ford. Bridges are input data in `Terrain`, derived by the importer from the road grid and the drainage, not placed by hand. A Roman road has to cross a river by bridge or ford, and Rochester's Roman bridge is attested. The model is unchanged; generated maps have no initial roads, so the golden hash must stay `68beae826cf800cd`. No parameter is re-tuned. This is input data, not the v3 mechanism of fords becoming bridges from traffic.

**Check the input before running.** Report the number of bridge cells, and whether the Medway at Rochester and the Stour at Canterbury are navigable in the model at the bridge cell. If either is not, say so before the run: the transshipment term cannot act there, and the expectation for that site is then weak.

**Expectation (the designer's).** The bridged crossings become junctions of land and water routes, so through-traffic and transshipment income settle there. Primary: a place of 1,000 or more within 3 km of Rochester in at least 5 of 20 seeds. Canterbury is reported against the same bar, but the expectation for it is weaker, because the navigable Stour historically ended at Fordwich, Canterbury's port 3 km downstream. The cost-based shadow still suppresses places along a road, so the share of settlements on roads is not expected to reach the land share under (a) alone.

**How it will be scored.** The same tables as the roads test, against both v1.1 and v1.1 + roads on the same 20 seeds: Rochester and Canterbury counts; settlements on or next to a Roman road (all / 1,000+) against 32% of land and 47% of real towns; lowland AUC; count and spacing of inland places over 500; the largest city's site, with Gravesend's count stated. A fall in Gravesend's count below the 15 of v1.1 is a cost to report, not a reason to tune.

**Stop.** Run once, then stop and report. If the expectation fails, nothing is tuned; the next decision is (b).

### Input check (implementer, before the run)

The mechanism is now in the code. `Terrain.bridge` is input data, and `free_crossing` makes a crossing cost 0 at a ford or a bridge. A unit test (`crossing_is_free_at_a_bridge`) covers it. The importer marks a bridge on every river or stream cell an Itiner-e road passes through, and on both cells where the road slips diagonally between two linked river cells. The golden hash is unchanged: `68beae826cf800cd`.

- **Bridge cells:** 52; 30 on navigable rivers, 22 on streams.
- **Rochester:** two bridge cells on the **navigable** Medway, at (54, 57) and (55, 57), 2.2 km from Rochester. The mechanism can act there.
- **Canterbury:** three bridge cells within 2 km, all on a **stream**. In the model the Stour is not navigable at Canterbury; the nearest navigable cell is 8.2 km away. The transshipment term cannot act at Canterbury, so the expectation there is weak, as the designer anticipated.
- **Also bridged on a navigable river:** the Medway at Maidstone, 0.6 km away, on the road towards Lympne.

### Result: the expectation fails

Bridges are opt-in (`road_bridges` in `data/real/kent/map-bridges.json`), so the roads-only test still reproduces exactly (seed 1 checked against the D32 run). The run was done once, on v1.1 defaults with uniform soil, 20 seeds, no re-tuning. Full scores: `docs/real/kent/experiments/bridges.md`.

| Kent, 20 seeds | Rochester: place ≥1,000 within 3 km | Canterbury | Maidstone | On or next to a Roman road (all / ≥1,000) | Lowland AUC: all / ≥1,000 / <1,000 | Inland >500: spacing, count | Largest city near (seeds) | Largest share |
| --- | ---: | ---: | ---: | --- | --- | --- | --- | ---: |
| v1.1 | 1/20 | 1/20 | 3/20 | 26% / 26% | 0.44 / 0.54 / 0.37 | 9.1 km, 20 | Gravesend 15 | 0.30 |
| v1.1 + roads | 1/20 | 0/20 | 2/20 | 20% / 24% | 0.43 / 0.48 / 0.40 | 8.1 km, 20 | Gravesend 6, Rayleigh 4 | 0.25 |
| v1.1 + roads + bridges | **1/20** | 1/20 | 1/20 | 20% / 22% | 0.46 / 0.41 / 0.49 | 8.6 km, 20 | Gravesend 8, Rayleigh 5, Billericay 2 | 0.27 |
| Real | | | | 47% of towns; 32% of land | | 10.6 km, 29 | London | ≈0.3 |

**The expectation fails.** Rochester reaches 1 of 20 seeds against a bar of 5, and in most seeds there is no settlement within 3 km at all (median 0). Canterbury is at 1 of 20, as weakly expected. The on-road share stays at 20%, as the designer expected under (a) alone. Larger-town placement is worse (AUC 0.41), and Gravesend falls from 15 to 8, a cost of roads, not of bridges, since it was already 6 with roads alone.

**What this rules out.** A free crossing on the navigable Medway, on Watling Street, does not by itself make a town there. Since no settlement usually exists near the bridge at all, the failure comes before growth. Either founding never chooses the site, or the cost-based shadow suppresses whatever is founded there: a bridged crossing makes the site cheaper to reach from its neighbours, so it falls into their shadow, the same mechanism that pushes settlements off the roads. That is the designer's option (b). No further change was tried.

### Designer's reading of the bridges result

Run once, input checked and committed first, roads-only run reproduced exactly: a clean test. It fails, and it closes the question it was set.

1. **(b) is not approved as the next round.** The implementer's diagnosis points to the travel-cost shadow, but the roads test already holds a bound on that: with the shadow switched off entirely, roads gave Rochester 0/20 and an on-road share of 29%, still under the land share. A shadow measured in km suppresses less than the cost shadow and more than none, so it cannot by itself bring Rochester to 5/20. It also changes the generated maps, so as a default it needs a re-tune, full validation and Divish's approval. It stays recorded as a known defect (a road deepens the shadow along itself), not as the fix for this result.
2. **The Rochester and Canterbury expectation is withdrawn, and the error was the designer's.** Both were Roman towns (Durobrivae, Durovernum Cantiacorum) and early sees. Their place in 1300 is persistence from before the model's start, which the model takes from its starting state, not from founding. A model that founds hamlets on food and water and grows them over 500 years cannot be asked to place them. Rule: a site expectation is set only for places whose rise falls inside the model's period and mechanisms.
3. **Kent defaults.** Roads and bridges stay options, as soil did. They lower the gateway result (Gravesend 15 → 8) and add nothing to placement. Kent with roads runs at 42–49 ms a year natively, near the 50 ms budget; acceptable while they are opt-in.

## Diagnostic: why nothing settles at the Rochester bridge (approved, before running)

One round, no model change, nothing tuned, roads + bridges configuration, the same 20 seeds. Its purpose is to say whether founding or the shadow keeps the site empty; that decides whether (b) goes to Divish as a candidate default change.

1. Per seed, every settlement founded within 3 km of the Rochester bridge cells (54, 57) and (55, 57): founding year, peak population, and final population or year and cause of failure.
2. The founding score at those two cells at years 100, 250 and 500, split into its terms (food, water, shadow, access), against the best score and the threshold in that step.
3. The same 20 seeds with the shadow off, as in the roads diagnostic, plus bridges: Rochester count and on-road share.

**Stop.** Report, then stop. Nothing is tuned whatever the result. The next round after this is the agreed step (2), the outside world as a seller, to be pre-registered by the designer.

### Diagnostic round at the Rochester bridge (approved, no model change)

`hinterlands probe` (new, read-only) follows every settlement founded within 3 km of the bridge cells (54, 57) and (55, 57). At the founding steps of years 100, 250 and 500 it scores those cells on a copy of the run advanced to the moment founding runs. Configuration: v1.1 defaults, roads + bridges (`map-bridges.json`), seeds 1–20. Raw output: `docs/real/kent/experiments/probe-rochester.md`.

**(a) Settlements at the bridge.** Founding does choose the site:
- 35 settlements were founded within 3 km of the bridge, in 14 of the 20 seeds, most of them in the first 20 years.
- 15 grew to 1,000 or more (peaks 1,020–4,280).
- **14 of those 15 were then removed by the failure rule as unfed for 20 years** ("Starved"). Six of the removals fall in years 209–230, the rest in years 331–485.
- Only one town of 1,000+ survives at year 500 (seed 15, 2,162). In all, 11 settlements near the bridge are alive at year 500, nearly all small.

**(b) The founding score at the bridge cells** (medians over 20 seeds and both cells; the score shown is without noise, and noise adds up to 0.31):

| Step | Cells that are candidates | Food | Water | Shadow | Access | Score | Best score of the step | Threshold |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Year 100 | 20 of 40 | 0.61 | 1.14 | 1.33 | 0.41 | 0.79 | 1.83 | 0.33 |
| Year 250 | 24 of 40 | 0.64 | 1.14 | 29.9 | 8.05 | −20.0 | 5.06 | 0.33 |
| Year 500 | 30 of 40 | 0.64 | 1.14 | 31.3 | 10.4 | −16.9 | 9.17 | 0.33 |

Early on the site clears the threshold but loses to better sites in the same step. Founding takes up to 6 sites per step, so it is still chosen in most seeds. From year 250 on, the cost-based shadow at the bridge is 10 to 210. The Medway, the Thames and the roads put much of the region within travel cost 10, so a site there cannot be founded again once its first town has gone.

**(c) Shadow off (both founding shadow and drain), roads + bridges:** Rochester 3 of 20 seeds (median largest place within 3 km: 656); on or next to a road 28% of all settlements and 23% of places of 1,000+. Near the bridge, 91 settlements were founded across the 20 seeds, 24 peaked at 1,000+, 32 starved, and 58 were alive at year 500.

**Reading.** The failure at Rochester is not founding. Bridged and on Watling Street, the site is chosen early in most seeds, and its towns grow to thousands. They are then deleted whole by the failure rule ("unfed for 20 years"), which was written for failing hamlets. A town of 2,846 that runs short of food is removed with all its people, rather than shrinking to the size its food supports. The shrink rate (`r_shrink` 0.02 × shortfall a year) is too slow to bring a starving town back to fed within the 20 years. Once the town has gone, the cost-based shadow keeps the site empty. Why towns at the bridge run short around years 210–230 (competition for the food that feeds their market people, presumably from the tidal-Thames city growing at the same time) is not measured here. No change was tried.

### Designer's reading of the Rochester diagnostic

The probe found the cause; credit to the implementer. It also disproves both earlier readings: the implementer's (the shadow keeps the site from being founded) and the designer's (a food-and-water founding model cannot choose the site). Founding chooses the bridge in 14 of 20 seeds, and towns of up to 4,280 grow there. They are then deleted.

1. **The failure rule is a specification error, and it was the designer's.** It was written for hamlets and applied to every size. It also misreads its evidence: the 39% survival of English markets from 1300 to 1600 counts markets that lapsed, not places that vanished; most lapsed markets stayed villages. Deleting a town of 2,846 because it ran short of food for 17 years contradicts the brief's own expected behaviour "towns outlive their reason" (population inertia). This is the scale rule again: a mechanism matched to hamlets was applied to towns.
2. **Corrected rule.** An underfed place sheds the people its food does not feed, a third of the gap each year, which is the brief's three-year inertia. The leavers are lost, as under the present shrink. Removal is only for places that fall below `fail_pop` for `fail_low_years`; the unfed rule removes nothing. A starving hamlet shrinks below 20 and goes by that rule; a starving town shrinks to the village or town its food supports and stays.
3. **The Rochester expectation is re-set for this round, with its mechanism confirmed.** The earlier withdrawal rested on reasoning that the probe disproved. The mechanism (towns form at the bridge and are deleted by the failure rule) is now measured, which the pre-registration rule requires.

## Failure by shrinking, not deletion (pre-registered, before running)

Written by the designer and committed before any run.

**Change.** A switch `shed_unfed`, default off. When on, a place with food ratio below 1 loses a third of `(1 − ratio) × P` each year, and the unfed rule (`fail_unfed_years`) no longer removes anything; `fail_pop` and `fail_low_years` are unchanged. Golden hash with the switch off: `68beae826cf800cd`. Nothing is re-tuned.

**Report, in this order.**
1. **Scale check, v1.1 unchanged:** removals by cause and by peak population (< 100, 100–499, 500–999, 1,000+), over the 3 generated maps × 20 seeds and over Kent (uniform soil, no roads). This says whether the defect is general or a Kent effect.
2. **Generated maps with the switch on:** the full validation table, pass rates per check and per map, against v1.1, with the failure rate stated; time per year.
3. **Kent with the switch on,** both without roads and with roads + bridges: the usual table (lowland AUC; count and spacing of inland places over 500; on-road share; largest city with Gravesend's count) and Rochester.

**Expectation (the designer's).** On Kent with roads + bridges, a place of 1,000 or more within 3 km of Rochester in at least 5 of 20 seeds. On the generated maps, the failure rate stays within 40–70%, since most failures are hamlets that fall below 20. The count of inland places over 500 on Kent rises toward the real 29.

**Stop.** Run once, then stop and report. Nothing is tuned. Making `shed_unfed` the default, and any re-tune it needs, is then the designer's decision (design authority delegated by Divish on 2026-10-21).

### Results (run once, nothing tuned)

`shed_unfed` is implemented as pre-registered. With it on, an underfed place loses `(1 − ratio) × P / 3` a year, in place of the smoothed `r_shrink` loss, and the unfed rule removes nothing. With it off, the golden hash is still `68beae826cf800cd`. Parameters: `data/real/experiments/shed-unfed.json`. A new read-only `hinterlands failures` command lists every removal with its cause and the settlement's peak population.

**1. Scale check, v1.1 unchanged: removals by cause and peak population.**

| Runs | Cause | Peak < 100 | 100–499 | 500–999 | 1,000+ | Total |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| Generated maps, 3 × 20 | Depopulated | 3,243 | 890 | 70 | 49 | 4,252 |
| | Starved | 4,382 | 903 | 2,360 | **10,334** | 17,979 |
| Kent, 20 (uniform soil, no roads) | Depopulated | 1,802 | 233 | 20 | 18 | 2,073 |
| | Starved | 0 | 54 | 841 | **3,551** | 4,446 |

**The defect is general, not a Kent effect.** On the generated maps, v1.1 deletes about 170 settlements per run that had peaked at 1,000 or more, by the unfed rule, nearly all after year 250. In map 1, seed 1, one of them peaked at 5,608. Towns grow beyond what their food supports, starve and are removed, and new places grow in their place. The validation checks did not catch this: they look at year 500 and at the share of founded places that failed, not at what size the failures were.

**2. Generated maps with `shed_unfed` on (20 seeds per map): validation FAILED on one aggregate.**

| Check | v1.1, maps 1 / 2 / 3 | `shed_unfed`, maps 1 / 2 / 3 |
| --- | --- | --- |
| Settlements (median) | 20/20/20 (198/217/197) | 20/20/20 (240/242/239) |
| Over 2,000 | 16/18/19 | 20/20/20 |
| Over 5,000 (median) | 19/20/20 (2/2/2) | 20/20/20 (2/2/2) |
| Largest share | 20/20/20 | 20/20/20 |
| Rank-size slope (median) | 19/15/20 (−0.91/−0.91/−0.94) | 20/20/20 (−0.98/−1.03/−1.00) |
| Market spacing | 15/18/17 | 19/20/18 |
| Siting | 20/20/20 | 20/20/20 |
| Failure rate (median) | 16/20/17 (0.66/0.62/0.66) | 20/20/20 (0.59/0.55/0.58) |
| Forest near cities | 19/20/20 | 20/20/20 |
| Stability | 20/20/20 | 20/20/20 |
| **Largest varies (≥ 0.30, gated)** | 0.75/0.45/0.55 | 0.30/**0.20**/**0.00** |
| River-corridor share (reported) | 0.35/0.40/0.45 | 0.00/0.45/0.20 |
| Runs passing every check | 7/13/14 | **19/20/18** |
| Slowest run, native | 30.6 ms/year | 30.4 ms/year |

Every per-seed check improves, and the failure rate stays within 40–70%, as expected. **The gated contingency check fails on maps 2 and 3.** With towns no longer deleted, the same place becomes the largest city in nearly every seed. On map 3 it is the same cluster in all 20 seeds. In v1.1, much of the variation between seeds came from big towns being deleted and replaced.

**3. Kent with `shed_unfed` on (20 seeds).**

| Run | Lowland AUC: all / ≥1,000 / <1,000 | Inland towns found | Places on rivers | Inland >500: count, spacing | On or next to a road (all / ≥1,000) | Rochester | Canterbury | Maidstone | Largest city near (seeds) | Largest share |
| --- | --- | ---: | ---: | --- | --- | ---: | ---: | ---: | --- | ---: |
| v1.1 | 0.44 / 0.54 / 0.37 | 31% | 32% | 20, 9.1 km | 26% / 26% | 1/20 | 1/20 | 3/20 | Gravesend 15 | 0.30 |
| v1.1 + roads + bridges | 0.46 / 0.41 / 0.49 | 26% | 29% | 20, 8.6 km | 20% / 22% | 1/20 | 1/20 | 1/20 | Gravesend 8 | 0.27 |
| `shed_unfed` | 0.45 / 0.49 / 0.42 | 34% | 49% | 27, 7.2 km | 28% / 30% | 0/20 | 0/20 | 0/20 | Gravesend 16 | 0.33 |
| `shed_unfed` + roads + bridges | 0.49 / 0.53 / 0.46 | 39% | 44% | 32, 6.2 km | 21% / **53%** | **1/20** | 0/20 | 4/20 | Gravesend 10, Faversham 4 | 0.30 |
| Real | | | 26% | 29, 10.6 km | 47% of towns; 32% of land | | | | London | ≈0.3 |

**Against the expectation:**
- **Rochester fails** at 1 of 20 against a bar of 5.
- **The failure rate holds** (0.55–0.59).
- **Kent's inland places over 500 rise toward 29:** 27 without roads and 32 with roads + bridges, but closer together than the real ones (6.2–7.2 km against 10.6).

Other effects:
- With roads + bridges, **53% of places of 1,000+ sit on or next to a Roman road**, against 47% of real towns and 32% of land; in v1.1 it was 22%.
- That run has the highest small-town lowland AUC so far (0.46) and the highest inland find rate (39%).

**Why Rochester still misses.** The probe on the same configuration (read-only, stepping exactly as the run does) shows:
- Places are still founded at the bridge early: 21 places in 13 of 20 seeds, 7 of which reach 1,000 or more.
- They still starve. Two that peaked at 2,846 and 4,280 shrink to below 20 and are removed as depopulated in year 279. Three shrink to 822–914, just under the bar. One holds at 1,216.

So `shed_unfed` changes how a starving town ends, but the town at the bridge still loses the food that supported it. What drains it was not measured in this round. Nothing else was tried.

### Designer's decision on `shed_unfed`: it becomes the default (v1.2)

Run once as pre-registered, nothing tuned, with a scale check that turned a Kent question into a general finding. The decision is the designer's under the design authority delegated on 2026-10-21.

1. **`shed_unfed` becomes the default.** The defect is general: v1.1 deletes about 170 settlements of 1,000+ per generated run, which is wrong as history and looks wrong on screen, where towns of thousands vanish in a year. With the switch on, every per-seed check improves (runs passing every check 19/20/18 against 7/13/14) and the failure rate stays in band. No re-tune.
2. **The contingency check is revised, and the revision is made after seeing it fail, so the reasons have to stand on their own.** v1.1 passed it only through the defect: much of the variation between seeds came from big towns being deleted and replaced. The record puts persistence at the top (the largest cities are the most persistent places; Bosker & Buringh 2017) and contingency in the middle tier, which is where Evans & Rivers (2017) find it among sites of similar quality. The check now gates on the ten largest places: for each pair of seeds, the share of one run's ten largest with one of the other's within 5 km; the mean over pairs must be at most 0.80. The old largest-city check is reported. The threshold is set before any run: 1.0 means geography alone decides the hierarchy, and 0.80 means that on average at least two of the ten differ. `docs/foundation.md` and `docs/validation.md` carry the new wording.
3. **Rochester is closed as a target.** Towns form at the bridge and lose the market competition with the Thames and Medway towns; that is the model's normal contingency, not a defect. Rochester stays a row in the Kent table; no further rounds aim at single Kent sites.
4. **Kent's standard configuration becomes uniform soil + Roman roads + bridges.** The roads are real input data, and with `shed_unfed` they give 53% of places of 1,000+ on a Roman road (real 47%, land 32%) and the best small-town AUC so far. Gravesend's lower count (10 against 16) and the tight inland spacing (6.2 km against 10.6) are reported, not chased. From here Kent is a regression report at each milestone.

## v1.2 milestone (approved)

Make `shed_unfed` the default; implement the revised contingency check in `validate` (the old one reported); switch `just real` to the roads + bridges configuration. Then, as for any milestone: `just test` and `just validate` pass, the new golden hash is recorded natively and in the browser (`?selftest=500`), and `docs/milestones.md` gets the table with the new contingency figures per map alongside the old ones, the time per year in the release wasm build, and the Kent table. Record the default change as D35. If the revised contingency check fails on any map, stop and report; nothing is tuned to pass it.

## v2 G3: specialists, and the gate (stopped on a failed check)

Specialist founding (D40) is implemented as specified. For each good, at each founding step, the free resource site with the most reachable demand `Σ demand_k × exp(−β_g c)` gets a hamlet of `found_pop`, if that demand would feed the founders. Its kind is Mine, Saltern or Quarry, it is logged as new works, and it fails by the ordinary rules. `validate` now gates "sites producing ≥ 2/3" and "largest not specialist" (both trivially true with goods off), and reports the v2 measures.

**Gate, goods on, starting goods values (20 seeds per map): FAILED.**

| | Map 1 | Map 2 | Map 3 |
| --- | --- | --- | --- |
| v1.2 per-seed checks | pass | pass | pass |
| Over 5,000 median ≥ 2 | 2 | 2 | **1** |
| Top-10 overlap ≤ 0.80 | 0.55 | 0.53 | 0.52 |
| Sites producing ≥ 2/3 (median) | 0.87 | 0.80 | 0.73 |
| **Largest is not a specialist** | **2/20** | **9/20** | 20/20 |
| Specialists per run; size median / p90 / max | 8.8; 858 / 6,322 / 8,569 | 6.4; 258 / 5,297 / 7,716 | 8.0; 773 / 3,654 / 4,450 |
| Payment share of market inflow (expected 5–25%) | 0.074 | 0.071 | 0.070 |
| Forest within 5 km of iron / land | 0.00 / 0.41 | 0.00 / 0.41 | 0.00 / 0.41 |
| Settlements > 2,000 on a goods route | 1.00 | 1.00 | 1.00 |
| Slowest run, native | 27.6 ms/year | 28.7 | 27.3 |

**Cause.** The oversized specialists are salterns. In map 1, seed 1, the two largest places (Mereport 6,738 and Holport 3,761) sit on salt sites producing at the 1,000 cap, with no through-trade. The specification's production-constrained gravity sells a site's whole output whatever the demand. Four or five salterns at cap supply 4,000–5,000 grain units of salt a year against a regional salt demand of about 2,000, and buyers pay for all of it, up to their marketed share. A saltern therefore earns about 1,000 grain a year whether or not the salt is wanted. That builds its market size W, which then draws ordinary grain trade by gravity: Mereport receives 6,271 food a year, of which the salt pays about 1,000. The saltern becomes the regional market town.

**Options (for the designer).**
- (a) **Demand-limited sales.** A site sells at most the demand it reaches: output `min(cap, prod × workers, Σ demand_k e^{−β_g c})`, and each buyer gets at most its demand. Competing sites then share the market, which is Weber's market areas and what the founding test already uses. This is a change of shape, so it is the designer's.
- (b) **A lower cap per site** (tuning, mine): this bounds each saltern's income, but buyers would still pay for salt they do not need, which inflates the payment share.

Recommendation: (a). It makes oversupply impossible rather than smaller.

### Designer's decision on the G3 gate

The implementer stopped correctly on a failed check and found the cause. **The specification error is the designer's.** I wrote the goods step as "the mirror of the grain step", with the producer as the origin. But grain is sold by farmers who must sell; goods are bought by people who need a fixed amount. In Wilson's family of models that is the shopping model, with the buyer as the origin. With the producer as origin, oversupply was paid for, and a saltern at its cap became a market town on salt nobody needed.

**Decided:** the implementer's option (a), in the published form now written in `docs/foundation.md` ("Goods trade"). Each consumer buys its demand, split among sites by capacity × `exp(−β_g c)`. Orders above a site's capacity are scaled down in one pass, and output is what is sold. This is what the implementer proposed, with the sharing between competing sites made explicit, so that two salterns cannot both count the same buyers. Option (b) alone is declined, because it would make the overpayment smaller rather than impossible. Goods parameters, the caps included, remain the implementer's to tune afterwards.

**Next:** implement it (D41), re-run the v2 gate once, and report. If the gate passes, carry on to G4. If it fails, stop and report as before. Separately, forest within 5 km of iron sites is 0.00: mines strip their woods entirely. That is reported, not gated; look at it in G4. Wealden iron was run on managed coppice, so a mine that settles at its woods' regrowth would be the better picture.

### Re-run with the shopping model (D41): the gate fails again, on a label

Goods trade is rewritten as specified. Buyers are the origin. A holder's own demand is met first from its own sites. The rest of each buyer's demand is split among working sites in proportion to `capacity × exp(−β_g c)`, orders beyond a site's capacity are scaled down in one pass, and unmet demand stays unmet. A site's output is what it sells: trade writes `Site.sold`, and "sites producing" now counts sites that sold anything. Payment and route loading are unchanged. With goods off, the golden hash is unchanged.

| | Map 1 | Map 2 | Map 3 |
| --- | --- | --- | --- |
| v1.2 per-seed checks | pass | pass | pass |
| Over 5,000 median ≥ 2 | 2 | 2 | **1** |
| Top-10 overlap ≤ 0.80 | 0.38 | 0.41 | 0.48 |
| Sites producing ≥ 2/3 (median) | 0.87 | 0.73 | 0.73 |
| **Largest is not a specialist** | **1/20** | **12/20** | 20/20 |
| Specialists per run; size median / p90 / max | 8.7; 347 / 6,909 / 13,459 | 6.7; 211 / 3,357 / 15,387 | 7.9; 426 / 2,071 / 4,025 |
| Payment share of market inflow | 0.031 | 0.029 | 0.030 |
| Forest within 5 km of iron / land | 0.00 / 0.42 | 0.00 / 0.42 | 0.00 / 0.41 |
| Settlements > 2,000 on a goods route | 1.00 | 1.00 | 1.00 |
| Slowest run, native | 29.4 ms/year | 27.5 | 28.8 |

The shopping model removes oversupply. The payment share falls from 7% to 3%, and salt sites now sell 340–520 against a capacity of 1,000.

**But the largest places are still salterns, and salt no longer explains them.** In map 1, seed 1, the saltern Mereport at (20, 75) has 12,446 people and receives 11,689 food a year, of which salt sales bring about 516 (4%). The rest is ordinary grain trade drawn by its market size. Without goods (v1.2, same map and seed), the largest place is Wenenford at (18, 70), 11,870 people, a coastal city 5 km away. On map 2, the largest place is on the same coastal cell with and without goods.

So the gate is catching a label, not an oversized works. Salt sites are placed on the coast, next to the best city sites. A saltern founded early on such a spot grows into the city that would have grown there anyway, and keeps the kind `Saltern` for life.

**For the designer:** what "specialist" means in the gate. Options:
- (a) **The kind follows the economy:** a specialist becomes an ordinary settlement once goods sales are a small share of its inflow (salt towns and mining towns that became market towns);
- (b) **Gate on the economy, not the label:** no settlement whose goods sales are most of its inflow is the largest place;
- (c) **Keep the label, and keep specialists off the best town sites**, for example by not founding works where an ordinary founding would score higher.

(a) or (b) measure what the gate is for. (c) changes where works go.

### Designer's decision on the re-run

The shopping model did its job: oversupply is gone, and the payment share fell to 3%. The remaining failure is in how the gate was defined, and the definition was the designer's. "Specialist" was a label set at founding and never revisited, so a saltern on the best harbour site grew into the city that would have grown there anyway and was still counted as salt works. The implementer's reading is right: the gate was catching a label.

**Decided: (a), with the gate measured the same way, so (b) follows from it.**
- A specialist stays one while its goods sales are at least 25% of its gross inflow (10-year average). Below that it becomes ordinary for good, still working its site, and the chronicle notes it. The wich towns and mining boroughs did exactly this.
- The gate "largest is not a specialist" therefore tests the economy: no place living mainly on goods sales is the largest.
- (c) is declined. Salt belongs on harbour coasts, and a port growing beside its salt pans is a good thing to watch.
- **Over-5,000 median on map 3 (1, needs 2):** this is a real v1.2 check failing with goods on, not a label. The spec allows the v1.2 parameters to be re-tuned in exactly this case, so it is the implementer's to fix by tuning, with every change recorded.

**Next:** implement (D43), tune for map 3 if needed, re-run the gate once. If it passes, carry on to G4; if not, report. D42 is held for the graphics merge.

### v2 gate after D44 and a re-tune (stopped: map 3 over-5,000 median)

**Re-tune** (goods on, allowed by the specification because a v1.2 check failed). This was a search over α, K, ε, β, market share and stone demand on the three generated maps, 4 seeds each. The search now also scores the over-5,000 median. Best setting: α 1.088, K 3.39, ε 0.137, β 0.139, market share 0.487, stone demand 0.016 (`data/real/experiments/v2-candidate.json`). In the search it passed every per-seed check in every run.

**Gate, 20 seeds per map:**
- every per-seed check passes on every map; runs passing every check 20 / 20 / 18;
- sites producing, median 0.87 / 0.80 / 0.73;
- largest is not a specialist, 20/20 on each map;
- top-10 overlap 0.43 / 0.39 / 0.50;
- specialists per run 1.4 / 3.1 / 0.3, size median 60 / 90 / 28, max 404;
- payment share of market inflow 0.026 / 0.027 / 0.025;
- forest near iron 0.00 against land 0.41;
- settlements over 2,000 on a goods route 1.00;
- slowest run 24.8–27.6 ms a year, native.

**Failing: map 3's over-5,000 median is 1.**

Count of places over 5,000 per seed on map 3 (20 seeds, sorted):

| Configuration | 1 | 2 | 3–4 | Median |
| --- | ---: | ---: | ---: | ---: |
| v1.2, goods off | 8 | 8 | 4 | 2 |
| Goods on, starting values | 15 | 4 | 1 | 1 |
| Goods on, re-tuned | 12 | 8 | 0 | 1 |

Map 3 tends to a single dominant city (largest share 0.21; the largest city is the same in every seed). v1.2 passed this check there by one seed. Goods move a few more seeds to one city over 5,000, and the re-tune recovers some but not all of them.

**For the designer:**
- (a) another tuning round aimed at map 3, judged on 20 seeds; or
- (b) treat this as a property of map 3. D20 already holds that one city over 5,000 is historically normal for a region this size, and the per-seed check (1–5 over 5,000) passes in every seed on every map. The median rule could be reported, not gated, on maps where v1.2 itself sits at the edge.

No further tuning was tried.

### Designer's decision: the over-5,000 median becomes reported (D48), and v2 goes to G4

Every gated v2 check now passes, and so does every per-seed check on every map. The one failure is an aggregate on map 3: the median count of cities over 5,000 is 1 against a floor of 2.

**Decided: option (b), and the change to the acceptance test is made after seeing the failure, so the reasons are stated in full.**
- **v1.2 itself passed this check on map 3 by one seed** (8 seeds with one city, 8 with two, 4 with more). The aggregate sat on the edge before goods arrived, so it is not measuring a change goods made.
- **What the check was for is still gated per seed.** It was added (D20) to stop hierarchies with no second tier. That job is done by the per-seed checks, which pass in every seed: 1–5 cities over 5,000, 5–12 over 2,000, the rank-size slope, and the largest share under 25%.
- **Map 3 has one dominant site,** the same largest in every seed, with a share of 0.21. A region with one city over 5,000 is historically normal; the brief already says so.
- **Option (a) is declined.** A tuning round aimed at one map's aggregate would be tuning toward a test.

The median stays in the report. The re-tuned values in `data/real/experiments/v2-candidate.json` become the v2 defaults when the milestone is recorded.

**Next:** G4 (view, then the milestone) as specified, with goods on by default at the milestone and a new golden hash. Then v2.1, the Kent resources, as planned in `docs/foundation.md` ("Plan after v2").
