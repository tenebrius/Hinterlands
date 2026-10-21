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
