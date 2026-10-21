# Proposal: markets, consumers and stores

From the owner (Divish), drafted with a third Claude session on 21 October 2026, for the designer to decide. Nothing here is decided. It is not a request to interrupt v5: the earliest stage would start after the v5 release, and the release itself is unchanged.

## 1. What is asked

The designer is asked to decide whether Hinterlands goes beyond v5 with a market layer, and if so, in what form, in what order and with which gates. Section 8 lists the decisions.

This runs against three standing rules, and the proposal names them rather than working around them:
- the foundation lists "a price economy: markets clearing, money, wages" as out of scope, and "a full price economy" as never planned;
- v5 is specified as the last simulation version, with no new mechanisms after the release;
- the designer's standing instruction is small, proven, gated.

The case below is that a market-clearing grain trade has a published, recognised footing (it is the gravity model the project already uses, with prices added), that it fixes faults the current model has against the historical record, and that it can be staged so each step is small and gated. If the designer judges otherwise, that is the decision.

## 2. The vision

Today a region fills in: villages on food and water, towns where transport is cheap, a hierarchy by competition, shocks answered by the existing rules. What it cannot show is people responding to scarcity. A town with coin cannot buy grain in a dearth; a hungry valley is not fed by a full one next door; nobody keeps a store from a good year; salt, iron and stone are bought and then do nothing.

The aim is a region whose consumers decide:
- **Grain reaches the hungry.** A town short of food has dear grain, and grain flows to it from wherever the price gap covers the carriage. A local bad harvest is relieved by neighbours; a regional famine (1315) is not, because no one has grain to spare.
- **Stores carry good years into bad ones.** Granaries keep a share of cheap harvests at a cost (interest and storage), as Clark finds they did.
- **Prices tell consumers what is scarce.** Stone becomes dear when three towns build bridges at once; the distant town waits.
- **Goods are used.** Iron makes tools, salt preserves food, stone builds bridges and walls. A blacksmith buys iron and sells tools: Weber's processing step.
- **Decisions belong to consumers.** Each settlement is a consumer with demand that responds to price. A mayor, later, is one consumer among them (buying stone and labour for a bridge), not a ruler setting policy.

What the viewer would see: grain moving up valleys in a dearth, price colours on the map, a quarry town growing while towns build, granaries filling after good harvests and emptying in 1315.

Nothing here adds individual agents. The settlement stays the unit.

## 3. Problems in the current model

Each is checked against the code on `main` at `55b843e`.

**P1. Grain is pushed, never bought.** Each settlement sends `market_share` of its harvest to markets by gravity on market size W (`crates/hinterlands-core/src/systems/trade.rs`, the domestic grain loop). The sender gets nothing for it, and the receiver pays nothing. A town is fed because it is a big market, not because it is short or can pay. A hungry village with a small market receives almost nothing and sheds people (D34).

**P2. Food is not passed on.** A settlement's marketed share is computed from its own harvest only (`food_prod × market_share − goods payments − grain sold to outside ports`, `trade.rs:100`). Grain it received cannot be sent on to a place in deficit. Every flow is one step, farm to market.

**P3. Surplus is burned.** The food ratio is `(food_prod + food_in − food_out) / need` (`growth.rs`). Food above `food_cap` (1.2 × need) does nothing and is gone at the year's end. Gross inflow (`market_in`) still counts all of it towards W, so a large market can take in grain far beyond its needs: the excess disappears as food but grows the market.

**P4. There are no stocks.** No field on `Settlement` carries grain, coin or goods from one year to the next. The only famine buffer is the extra land `cult_margin` keeps under the plough.

**P5. Goods do nothing for the buyer.** Demand for salt, iron and stone is a fixed amount per head. What is delivered is not recorded or used, and unmet demand has no consequence. Goods matter only to the seller (fed by grain payment), the routes (handling income, roads, bridges) and the woods (iron fuel).

**P6. Prices are fixed.** Every good trades at `value_g` = 1 grain unit. Scarcity rations orders but never moves a price, so no consumer has a signal to respond to.

**P7. Transport loss is about 2.5 times Clark's carriage cost.** `transport_loss` is 0.5% of cargo per unit of cost, so 0.5% per km on flat land. Clark (below) gives about 0.2 s per quarter per 10 miles on wheat at about 6 s a quarter: about 0.2% per km. This is reported, not proposed for change; the current tuning rests on it.

## 4. The evidence: Clark (1999)

Gregory Clark, *Markets and Economic Growth: The Grain Market of Medieval England* (UC Davis working paper; in the foundation's references). Wheat prices at 227 manors 1208–1500, and yields at 65 of them.
- **Local prices followed the national price.** A 1% rise in the national price gave a 0.997% local rise (s.e. 0.007). With the national price controlled for, doubling a manor's own yield changed its price by about 2.5%. The national price explains 86% of local price variation, from Devon to Durham. The same holds before 1300: no integration over time.
- **Price gaps were partly transport.** Average price levels differed by up to two to one between manors; transport accounts for gaps of about 20% of the price, the rest is local bushel measures and grain quality. Water carriage cost about a tenth of land carriage.
- **Grain was stored within the year.** The rise from harvest to the following summer was about 12.5% a year for wheat before 1350, close to the cost of holding it (interest of 10–11% before 1350, about 5% after, plus storage of at most 5% a year).
- **Grain was stored between years.** Price autocorrelation (0.48 for wheat, 1208–1453) is far above yield autocorrelation (0.16). After cheap years the carry-over reached about 40% of an average harvest (previous price below 0.6 of trend).
- **Grain prices and yields.** For years with no storage, `ln P = 0.05 − 1.34 ln Y` (R² 0.56): a usable grain demand response.

The model currently has the opposite of the first finding: a settlement's own harvest decides whether it eats. P1–P4 are why.

## 5. The proposed solution: spatial price equilibrium with entropy

### Two published models that are one

- **Spatial price equilibrium** (Samuelson 1952; Takayama & Judge 1964, 1971). For each good, each place has a supply and a demand that depend on its local price. Each year, prices and shipments are such that every market clears, no price gap between two places exceeds the transport cost between them, and goods move only where the gap equals that cost. Samuelson showed this is the solution of one optimisation: consumer benefit minus production cost minus transport cost, maximised. Alone, it gives corner solutions: each place trades with one or two partners, and flows jump between routes when costs change.
- **Wilson's gravity model** (Wilson 1970; Erlander & Stewart 1990). The doubly constrained gravity model is the solution of the same transport problem with an entropy term, which spreads trade over many routes. Its balancing factors are, in the dual, prices.

Adding the entropy term to Samuelson's objective gives flows

```text
x_ij = exp( β (p_j − p_i − t_ij) )
```

where `p_i` is the supply price at i, `p_j` the demand price at j, and `t_ij = τ_g × c_ij` the transport cost per unit of good g. This is the gravity form the model already uses, `A_i B_j exp(−β c_ij)`, with `A_i = exp(−β p_i)` and `B_j = exp(β p_j)`. As β grows it tends to Samuelson's exact equilibrium; the price-gap bound holds approximately, with the gap on a route `(1/β) ln x_ij` above transport cost. Combined distribution models of this kind are standard in transport planning (Evans 1976).

### In Hinterlands terms

- **Goods:** salt, iron, stone, imports, and grain.
- **Supply:** a site's capacity for goods (as now); a settlement's marketed harvest plus any store it releases for grain.
- **Demand:** per consumer, a demand curve. Grain: need, with an elasticity near 0.75 (Clark's 1.34 inverted). Salt low, stone high, iron between; starting values are the designer's to set.
- **Transport:** the existing cost grid and stored paths, so flows load routes and earn handling and transshipment income as now.
- **Grain is the unit of account.** Prices are in grain units; coin, if it comes (stage S5), is a stock counted in grain units, never a floating currency.

### Solving it

A fixed number of balancing passes a year: compute flows from prices, then move each supply price towards clearing its supply and each demand price towards clearing its demand, damped. This is the Furness (Sinkhorn) method. It uses only multiplication and `libm::expf`, iterates over sorted settlement lists, and runs a fixed count, so it is deterministic natively and in wasm. Warm-started from last year's prices, a few dozen passes over the pairs within `C_max` should fit the 50 ms budget; that is an estimate to be measured at stage S0.

### What happens to the Rihll–Wilson market

This is the main design question. Today trade is attracted by `W_j^α`, and W grows on gross inflow plus handling income. In the equilibrium, flows go where the demand price is high, not where the market is large. Three options, for the designer:
1. **W as demand:** market people demand grain and goods; W grows on the turnover the market handles, as now. Hierarchy comes from handling and transshipment income and from market people's demand. Closest to the published equilibrium; most risk to the town hierarchy.
2. **W as a market premium:** keep `W_j^α` as a multiplier on `B_j`, read as the convenience of a large market (more buyers, fairs, credit). Keeps the current hierarchy dynamics; the price-gap bound then holds only up to that premium.
3. **Grain stays as now:** equilibrium for goods only (stage S1), grain unchanged. Smallest change; does not fix P1–P3.

## 6. Staging: small, proven, gated

Each stage is behind its own switch, off by default, with the golden hash unchanged when off. Each is built and checked alone, then reported.

| Stage | What | Check |
| --- | --- | --- |
| **S0 Shadow prices** | Solve the equilibrium each year alongside the current trade and record prices and would-be flows. No feedback into the model. | Determinism; time per year; Clark's regressions on the shadow prices (below), reported. Zero risk to validation. |
| **S1 Goods by equilibrium** | Salt, iron, stone and imports trade by the equilibrium, replacing the shopping model (D41); consumer demand responds to price. | Every v5 check; price gaps against transport, reported. |
| **S2 Grain by equilibrium** | Grain trades by the equilibrium; places in deficit buy, grain passes on, surplus is not burned. W per the chosen option. | Every v5 check, and the Clark checks gated. |
| **S3 Stores** | A grain store per settlement, filled when the price is below trend and drawn when above, with a holding cost (interest plus storage). | The 1315 famine: depth of the dip against S2; carry-over after cheap years against Clark's 40%. |
| **S4 Goods in use** | Iron raises yield (tools); salt lowers store losses; stone is an input to bridges and walls; a processing step (blacksmith: iron to tools). | Every check; the effect of each good reported. |
| **S5 Coin and projects** | A treasury per settlement in grain units, fed by tolls; a bridge or wall built by buying stone and labour; consumers, mayors among them, choose what to buy. | Every check; bridges per run against D55. |

Stopping points: S0 alone tells the designer whether the model's prices look like Clark's England, without changing a run. If S2 cannot pass the v5 checks after tuning, stop and keep S1.

### Proposed checks from Clark

On grain, per run, regressing each settlement's yearly price on its own yield and the regional price:
- local price response to the regional price between 0.8 and 1.2 (Clark: 0.997);
- local price response to local yield, with the regional price controlled for, between −0.15 and 0 (Clark: −0.025);
- the regional price explains at least 60% of local price variation (Clark: 86%);
- reported: the price gap between pairs that trade, against their transport cost.

## 7. Risks

- **The town hierarchy.** Every v1–v5 check was tuned with gravity on W. Option 1 in section 5 may flatten the hierarchy or move it. Mitigation: S0 shows the prices before anything feeds back; option 2 is the fallback.
- **New parameters.** Demand elasticities per good, the entropy weight (β, existing), damping and pass count, store holding cost. Only the grain elasticity and storage costs have a medieval estimate (Clark).
- **Performance.** The balancing passes are the heaviest step in the year. Measured at S0, before anything depends on it.
- **Determinism.** Fixed pass count, sorted iteration, `libm`. Checked natively and with `?selftest=N` from S0.
- **Scope.** Six stages is the over-ambition the owner has seen sink projects before. Each stage stands alone, and the designer may stop after any of them.

## 8. Decisions asked of the designer

1. Whether to go beyond v5 at all, given that v5 is specified as the last version. If yes, whether this starts after the v5 release (proposed) or replaces part of it.
2. Whether market-clearing prices are admitted, amending "never planned: a full price economy". Wages and floating money stay excluded under this proposal.
3. The form: entropy-regularised spatial equilibrium (proposed), the simpler lagged price index (prices nudged each year by orders against capacity; no clearing, no price-gap bound, no grain to the hungry), or no change.
4. The role of the Rihll–Wilson W in the equilibrium (section 5, options 1–3).
5. Which stages to approve, in what order, and the stop condition for each. Proposed first step: S0 only, then stop and report.
6. Whether the Clark checks become gates at S2, and their ranges.
7. Whether P7 (transport loss against Clark) is reported only or looked at.

## References

- Clark, G. (c. 1999). Markets and economic growth: the grain market of medieval England. UC Davis working paper. [PDF](https://faculty.econ.ucdavis.edu/faculty/gclark/papers/market99.pdf)
- Samuelson, P. A. (1952). Spatial price equilibrium and linear programming. *American Economic Review* 42(3), 283–303.
- Takayama, T. & Judge, G. G. (1971). *Spatial and Temporal Price and Allocation Models*. North-Holland.
- Wilson, A. G. (1970). *Entropy in Urban and Regional Modelling*. Pion.
- Erlander, S. & Stewart, N. F. (1990). *The Gravity Model in Transportation Analysis: Theory and Extensions*. VSP.
- Evans, S. P. (1976). Derivation and analysis of some models for combining trip distribution and assignment. *Transportation Research* 10, 37–57.
- Nagurney, A. (1999). *Network Economics: A Variational Inequality Approach*. Kluwer. Projection methods for spatial price equilibrium.
- Britnell, R. H. (1993). *The Commercialisation of English Society, 1000–1500*. Cambridge University Press.
- Galloway, J. A., Keene, D. & Murphy, M. (1996). Fuelling the city: production and distribution of firewood and fuel in London's region, 1290–1400. *Economic History Review* 49(3), 447–472. For a fuel good, if one is wanted later.
