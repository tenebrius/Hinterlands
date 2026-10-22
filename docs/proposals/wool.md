# Proposal: wool and land use

From the owner (Divish), drafted with the proposal session on 22 October 2026, for the designer to decide. Nothing here is decided. The owner has taken a position on each open decision (section 9); the designer decides. It is not a request to interrupt v7: the earliest stage would start after M4 passes.

## 1. What is asked

The designer is asked to decide whether wool and a choice of land use come into the model, in what form and order, and with which gates. Section 9 gives the owner's position on each decision.

It stays inside the standing exclusions: no wages, no labour market, no agents, no floating money. Farmers choose land use by price, but the settlement stays the unit, and the labour a pasture or a field needs comes from the settlement's own people.

## 2. Why now

v7 gave the model prices, purses and coin. Two things it now lacks stand out against the record:

1. **The region earns its coin by selling grain.** The outside ports buy grain up to `port_world_price`, and, besides the starting stock, that is how coin comes in (D96–D98). England's silver came in chiefly through wool. The foundation already says so (v7, "Where money comes from and goes"). Grain was the smaller export.
2. **Land has one use.** Cultivation claims cells for grain until need is covered, and every other cell is forest or waste. The model's land use responds to people, never to prices. The viewer already draws pasture and sheep, but as a view layer, beyond the fields, with no economic meaning.

With prices in place, both can be fixed by the model the brief started from. von Thünen's rings are land use by price minus transport: each cell goes to the use that pays the most rent at its distance from market. The project has used his geometry since v1, but not his economics.

## 3. What the viewer should see

- Downs, wolds and uplands carrying sheep. Lowland near markets carrying corn.
- Wool moving down to the ports, and coin coming back up.
- After 1348, with fewer people and cheap grain, fields going to grass, and poor-land villages emptying into sheep runs. This is the desertion of villages that Beresford documented, in the place and for the reason he found it.
- In 1315–25 the sheep murrain hitting the pastoral districts as the famine hit the arable ones.

## 4. The evidence

To be checked against the sources before any number is used as a gate. The direction of each is well established; the figures are not given here for that reason.

- **Wool was England's great export and its source of silver.** Exports peaked around 1300–1310 and declined in the fifteenth century as cloth exports replaced them (Power 1941, *The Wool Trade in English Medieval History*; Carus-Wilson & Coleman 1963, *England's Export Trade, 1275–1547*). Mayhew (1995) ties England's money stock to it.
- **Conversion to pasture after the Black Death.** Grain fell in price against wool and livestock from the 1370s, and land went to grass, especially on poor and heavy soils (Campbell 2000, *English Seigniorial Agriculture 1250–1450*; Dyer 2002). Villages were deserted, largely in the fifteenth century (Beresford 1954, *The Lost Villages of England*; Beresford & Hurst 1971).
- **Pasture needs far less labour than arable.** This is why it gained when people were scarce, and the reason it is the right response to the plague without a labour market (Campbell 2000).
- **Pasture lay where land was poor and markets far.** Chalk downs, wolds, uplands and marsh. In Kent, Romney Marsh and the Downs were sheep country.
- **The sheep murrain of the 1310s–1320s** cut flocks sharply in the same years as the Great Famine (Campbell 2000; Slavin, on the 1315–25 livestock panzootics).

## 5. The proposed design

### Land use per cell

Each claimed land cell is **arable** or **pasture**. A new field per cell, written by cultivation (which already owns `owner` and `forest`).
- **Arable** as now: grain at the cell's yield, `people_per_prime_cell` fed per prime cell.
- **Pasture:** wool, at `wool_per_cell` grain-value units a year, depending much less on soil yield than grain does. It needs `pasture_labour` people per cell against arable's far larger need, and grazing keeps its forest from regrowing.

### Wool, a fourth good

- `Good::Wool`, produced on each settlement's pasture, not at point sites.
- High value for its weight (`ρ` about 5): it was carted long distances to the ports.
- **Buyers:** the outside ports, at a world wool price `port_wool_price` with an elasticity, as grain is bought now (D98). Domestic demand for cloth waits for a later stage.
- **How it travels.** By the S1 goods machinery: from each producing settlement to the ports, at prices that clear, on the stored paths. Towns on the route earn handling and transshipment income on it, as on any good.
- **Who is paid.** The sale is split by the existing rent share: `market_share` of it goes to the market purse (the lords: demesne flocks and rent), the rest to the farmers' purse, less the toll. This needs no new parameter, and it follows the record: large demesne flocks before 1350, peasant flocks after (Campbell 2000).
- Wool then becomes the main way coin enters the region, and the price–specie anchor (D97) gets its historical export.
- **Shepherds** are counted with the farmers in the labour guard (D92): `pasture_labour` per pasture cell is added to the people the land needs, so pasture never draws on the market people.

### Choosing the use: two options, smaller first

**Option A: pasture on land arable does not use.** Cultivation claims arable by need, as now. Beyond that, a settlement grazes free cells within its reach where wool's farm-gate return (world price less transport to the port, less the rent share) is positive and it has the people to herd them. Nothing competes with food.
- After the plague, need falls, arable is released, and the freed land goes to grass. Conversion emerges from the demographic shock, not from a price comparison.
- It is the smallest change, and the hierarchy's grain supply is untouched before 1348.

**Option B: von Thünen by price.** On each claimed cell, the use with the higher rent wins:

```text
rent_arable  = yield × p_grain,local × (1 − τ c_market) − labour_arable × need × p_grain,local
rent_pasture = wool_per_cell × p_wool,port × (1 − τ_w c_port) − pasture_labour × need × p_grain,local
```

The labour term is what the workers eat, valued at the grain price: a cost in kind, not a wage. Each year a capped share of cells (`convert_rate`) may change use, with hysteresis, since enclosure and ploughing-up took years. A settlement may then grow less grain than it eats and buy the rest with wool money, which is how pastoral districts lived.
- This is the true model, and it puts the decision on price where it belongs.
- It touches the grain supply, and so the hierarchy (D78–D80 is the warning).

Recommended: Option A first, Option B only if A's checks pass and it is still wanted.

### What wool does to existing systems

- **Trade:** wool is traded by the S1 machinery (goods by price) and pays the carriage and handling of any good, so the route from the downs to a port gains handling income.
- **Growth:** wool does not count towards W at first. W grows on grain drawn from other places (D90, D92), and every hierarchy check is calibrated on that. Wool earns market towns tolls and handling income, which is real money, but not W. L1 reports what W would be if wool counted; that decides whether wool towns, as Salisbury and the Cotswold boroughs were, come in later.
- **Events:** a sheep murrain, in the v4 queue, cuts wool output by a share in its years (1315–25 in the Historical calendar).
- **Founding:** unchanged. Pastoral villages are founded as any other.

## 6. Staging: small, proven, gated

Each stage is behind its own switch, off by default, with the golden unchanged when off.

| Stage | What | Check |
| --- | --- | --- |
| **L0 Diagnostic** | Compute, for every claimed and free cell, the arable and pasture rents at this year's prices and a trial wool price. Record where pasture would go under A and B. No feedback. | Reported: pasture share by travel cost to market and by yield band; the wool price at which pasture first pays; the would-be coin from wool against today's coin from grain. |
| **L1 Wool on spare land (A)** | Land use per cell, pasture beyond need, wool sold to the ports. | Every v7 check; the reported items below. |
| **L2 The murrain** | Sheep murrain events in the Historical calendar. Kept apart from L1 because it strikes in the years of the 1315–17 famine that M4 tunes, and both at once could not be told apart. | Every check; the wool dip in 1315–25. |
| **L3 Price competition (B)** | Arable and pasture compete on rent. | Every check; the conversion and desertion checks gated. |
| **L4 View** | The viewer's pasture and sheep come from the model, as mills and bridges did; wool as a route colour; chronicle lines ("1420. The fields of Ashby are put down to grass."). | No model change. |

Cloth is named, not scheduled: domestic cloth, with fulling mills on the streams where mills already sit, taking over from raw wool exports in the fifteenth century. It is decided after L3.

### Proposed checks

**Gated at L1:**
- every v7 check, the price-level band (0.3–2.5) included, since wool adds a second export and so changes the money stock;
- **pasture lies outward and on poor land:** the pasture share rises with travel cost to the nearest market and falls with yield (von Thünen);
- **conversion after the plague:** the pasture share of claimed land in 1450 is above its 1340 level. Option A should give it through released arable.

**Reported** (gated later only once their figures are sourced):
- **wool is the main export:** coin from wool against all coin entering the region before 1348, expected above half;
- **desertion on poor land:** the D59 check already holds; reported is whether the deserted places' land goes to pasture more often than not;
- **W with wool:** what each market's W would be if wool counted (decision 3);
- **Kent:** the pasture share on the Downs and Romney Marsh against the Weald and the lowland.

## 7. Parameters

New: `wool_per_cell`, `pasture_labour`, `rho_wool`, `port_wool_price`, `elast_wool`, the murrain loss and years; under B also `convert_rate` and its hysteresis. Each needs a source before it is set: stocking rates, fleece weights and the price of a sack of wool, from Campbell (2000) and Lloyd (1973, *The Movement of Wool Prices in Medieval England*). L0 exists so that the wool price can be set where the land use looks right before anything depends on it.

## 8. Risks

- **The grain supply and the hierarchy.** Under B, a high wool price takes land out of grain and makes the towns' food dearer. A leaves pre-plague grain alone, and is the guard.
- **The coin balance.** A second export changes the money stock. If wool brings in far more coin than grain did, the price level rises, and the check (0.3–2.5) must still hold. Imports and luxuries are the existing outlets, as in Hume's mechanism.
- **Labour without a labour market.** Pasture labour comes from the settlement's own people, as the labour guard (D92) does for fields. No wage is set.
- **Scope.** Four stages plus a view stage. Each stands alone, and the designer may stop after any of them.

## 9. Decisions: the owner's positions

The owner has worked through each decision. The design authority stays with the designer, who may accept or change any of them; each is stated with its reason.

1. **When: after M4 passes** (owner). M4 is still iterating, and the wool stages need a settled famine baseline.
2. **Option A first, B after** (owner). B comes only if A passes its gate and L0 shows that price competition would not starve the towns of grain.
3. **Wool does not grow W at first.** It earns tolls and handling income; L1 reports W with wool, which decides whether it counts later (section 5).
4. **Wool income is split by the existing rent share:** `market_share` to the market purse, the rest to the farmers' purse, less the toll (section 5).
5. **Gates at L1** as in section 6: every v7 check, pasture outward and on poor land, conversion after the plague. Wool's share of coin and the Kent comparison are reported until their figures are sourced.
6. **The murrain waits for L2,** apart from the famine years M4 tunes.
7. **Cloth is named, not scheduled** (owner). It is decided after L3.
8. **Wool travels by the S1 goods machinery** to the ports, and shepherds count in the labour guard (section 5). This was not in the first list.

What the designer is asked: to accept, change or decline these, and if accepted, to brief L0.

## References

- von Thünen, J. H. (1826). *Der isolierte Staat*.
- Power, E. (1941). *The Wool Trade in English Medieval History*. Oxford University Press.
- Carus-Wilson, E. M. & Coleman, O. (1963). *England's Export Trade, 1275–1547*. Clarendon Press.
- Lloyd, T. H. (1973). *The Movement of Wool Prices in Medieval England*. Economic History Review Supplement 6.
- Beresford, M. W. (1954). *The Lost Villages of England*. Lutterworth. Beresford, M. W. & Hurst, J. G. (eds.) (1971). *Deserted Medieval Villages*. Lutterworth.
- Campbell, B. M. S. (2000). *English Seigniorial Agriculture, 1250–1450*. Cambridge University Press.
- Dyer, C. (2002). *Making a Living in the Middle Ages*. Yale University Press.
- Mayhew, N. J. (1995). Modelling medieval monetisation. In Britnell & Campbell (eds.), *A Commercialising Economy*. Manchester University Press.
- Slavin, P. On the livestock panzootics of 1315–25 (e.g. "The Great Bovine Pestilence and its economic and environmental consequences in England and Wales, 1318–50", *Economic History Review* 65(4), 2012).
