# Proposal: demand from stocks, and processing chains

From the owner (Divish), drafted with the proposal session on 22 October 2026, for the designer to decide. Nothing here is decided. Development is stopped (D117); this is written for when it resumes, and it takes the place of the cloth design note recorded as a next step (D116.3), of which it is the general form.

## 1. What is asked

The designer is asked to decide whether demand for goods gets a physical basis (replacement of the stocks people, fields, workshops and buildings hold), whether processing chains come in where a separate trade sits between the raw material and its user, and, if so, in what order. Section 8 lists the decisions, with the owner's position where he has taken one.

It stays inside the standing exclusions: no wages, no labour market, no agents, no floating money.

## 2. The problem: demand has no basis

Demand for each good is a fixed quantity per head (or per market person), falling with price:

```text
demand = base × people × price^(−ε)
```

Only grain's numbers come from a source. The others are starting values or calibrations:

| Good | Base | Where it came from | Elasticity | Where it came from |
| --- | --- | --- | --- | --- |
| Grain | need, 1 per person | the model's food unit | 0.75 | Clark (1999) |
| Salt | 0.02 per person | designer's v2 starting value | 0.2 | designer's S0 starting value |
| Iron | 0.0013 per person | calibrated in S1 so its price sits at 1–3 against coppiced output (D71); was 0.02 | 0.5 | designer's S0 starting value |
| Stone | 0.016 per market person | tuned in v2 | 1.0 | designer's S0 starting value |
| Imports | 0.07 per market person | tuned in v5 to the outside-trade band | 1.0 | implementer's choice |

Three faults follow:
1. **Demand is for nothing.** Goods are bought because the formula says so. Nothing they would be used for exists in the model, so nothing about a place's activity changes what it buys: a town of ploughmen and a town of merchants want the same iron per head.
2. **Demand ignores wealth.** A rich town wants the same goods per head as a poor village. Wealth matters only when a purse cannot pay. Dyer documents the opposite after the Black Death: higher living standards spent on more cloth, meat and ale.
3. **Raw demand is not derived.** Iron, wool and stone are demanded for themselves. A change in what people want of finished goods cannot reach the mines, the pastures or the quarries. The cloth trade, the late-medieval change Carus-Wilson describes, has no way into the model.

Part of this spending already exists, untraced: the purses' yearly `purse_loss` is described as "cloth, dues, the church" (D97), and it recirculates to the markets. The proposal makes the goods part of it traced.

## 3. Demand as replacement of stocks

Most goods were bought to replace something that wears out: clothes, tools, ploughs, horseshoes, the fabric of houses. Demand is the wear on a stock, plus new stock as the holders grow:

```text
demand_g = Σ over holders ( stock_g per holder × wear_g )  +  new stock for growth
```

The holders are not only people. Each good is held by what uses it:

| Good | Held by | Wears out by |
| --- | --- | --- |
| Cloth (from wool) | each person: clothing, bedding | garments replaced every year or two |
| Iron (as tools) | each **arable field**: ploughshares, coulters, sickles, horseshoes; each **worker at a site or workshop**: tools; each household: knives, pots | plough irons and horseshoes often; household goods slowly |
| Stone (and timber) | each **built-up cell**: houses, walls, churches | yearly repair, plus **new building as a town grows** |
| Salt | not a stock; used up per person, and per animal slaughtered and catch preserved | consumed as used |
| Grain | not a stock; eaten, as now | consumed as used |

What this changes:
- **Iron follows farming.** More ploughland means more iron. When fields go to grass after the plague (L1), iron demand falls and wool demand rises.
- **Stone follows building.** A growing town buys stone for new houses; a stagnant one only repairs. Demand gains a growth term, the accelerator of the economics literature.
- **Wealth enters without a separate rule.** Richer holders keep larger stocks: more clothes, better houses. The stock per person rises with income per head (from the purses), which is Engel's law for goods.

## 4. Processing chains, only where a separate trade sits between

A good is final for whoever consumes it. Most processing was done by its user and needs no chain:
- **Grain:** villages boiled, baked and brewed it at home. Towns bought bread and ale from bakers and brewers, at a markup the Assize of Bread and Ale (c. 1266) tied to the grain price, so town bread demand is grain demand passed through. The model's K already includes ale and fodder (D90). The one step households did not do was milling, at the lord's mill for a toll in grain (multure); the mills are already in the model (D56).
- **Iron:** most villages had a smith. Iron is bought by the place that uses it, as now.

A chain is modelled only where a separate trade, in another place, sits between the raw material and its user. On the record that is chiefly **cloth**: wool from the sheep country, spun and woven, fulled at water mills on the streams, in cloth towns often far from both the flocks and the buyers. The cloth trade took over from raw wool exports in the fifteenth century (Carus-Wilson & Coleman 1963), and the Weald of Kent was itself cloth country.

**How a chain works** (Leontief 1936 for the coefficients; Weber 1909 for where the step sits):
1. People demand cloth by section 3.
2. Cloth towns supply it. A settlement with a fulling mill (a stream with a mill, D56) and market people can make cloth. Its capacity scales with its market people, as a mine's does with its miners.
3. Each unit of cloth needs a fixed amount of wool (the input coefficient). A cloth town's wool orders are derived demand.
4. The cloth town's margin, the price of cloth less the cost of its wool, goes to its market purse. It feeds the craftsmen and grows the town, so towns live on crafts as well as rents.
5. **Solving:** each year, from the finished end to the raw end, on last year's prices: cloth first (which sets each cloth town's sales), then the cloth towns' wool orders, then the pastures and the ports. Each step uses the existing S1 solver. The one-year lag avoids a joint solve.
6. The outside ports buy cloth as they buy wool (L1), so cloth exports can replace raw wool exports.

No wages are needed: craftsmen live on the margin, as miners on their sales.

## 5. Staging: small, proven, gated

Each stage behind its own switch, off by default, golden unchanged when off.

| Stage | What | Check |
| --- | --- | --- |
| **D0 Diagnostic** | Compute stock-based demand for every place each year beside the current demand. No feedback. | Reported: stock-based against current demand by good and place type; iron demand per arable cell before and after 1348; stone demand against town growth. |
| **D1 Stock demand** | Salt, iron and stone demand from section 3, replacing the per-head numbers. | Every v7 check; the D0 items, gated where sourced. |
| **D2 Cloth** | Cloth as a good, made at fulling-mill towns from wool, bought by people and the ports. | Every check; cloth towns sit on mill streams; the share of wool going to cloth rises after 1400. |
| **D3 Wealth** | Stock per person rising with income per head. | Every check; goods spending per head after 1348 against before. |

## 6. Parameters and sources

D105's rule applies: no value is set until D0 has reported a source for it; where none is found, it is set where its check holds and recorded as a calibration. Starting points, all to be checked before use:
- **Clothing:** yearly cloth and garment allowances in peasant maintenance (retirement) agreements (Dyer 1989, *Standards of Living in the Later Middle Ages*).
- **Plough irons and horseshoes:** smithing costs per plough in manorial accounts (Langdon 1986, *Horses, Oxen and Technological Innovation*).
- **Building repair:** repair outlays in borough and manorial accounts; stone per built cell from the footprint.
- **Cloth:** wool per cloth and cloth output (Carus-Wilson & Coleman 1963).

## 7. Risks

- **The footprint becomes an input.** Built-up cells are drawn from `State.built`, which the foundation describes as rendering only. Stone demand from built cells makes it feed back. The alternative is market people (W) as the proxy for buildings, with no change to the footprint's role. Decision 4.
- **Iron demand moves with the land.** If iron demand follows arable fields, the iron price and the mines respond to every change in farming. S1's calibration (D71) has to be redone at D1.
- **The hierarchy.** Cloth gives towns a second income beside rents, so it can move which towns lead. Every v7 check stays gated.
- **Scope.** Four stages. Each stands alone, and the designer may stop after any of them.

## 8. Decisions

Decisions 1 and 2 are the owner's positions; the rest are the proposal's. The design authority stays with the designer, who may accept or change any of them.
1. **Demand is replacement of stocks plus growth** (owner), held by people, fields, workers and buildings as in section 3.
2. **Chains only where a separate trade in another place sits between** (owner), so cloth first and alone. Grain, bread, ale and village smithing stay as they are.
3. **Wealth raises stocks** (D3), after the chain.
4. **Building stock:** from built-up cells, or from W as a proxy. Proposed: W first, so the footprint stays rendering only; built cells only if W fails the check.
5. **When:** when development resumes, in the place of the cloth note (D116.3), after the robustness re-tune.
6. **A milling toll** in grain to the market purse, as a faithful form of the mills: in scope, or left out.

## References

- Leontief, W. (1936). Quantitative input and output relations in the economic systems of the United States. *Review of Economics and Statistics* 18(3), 105–125.
- Weber, A. (1909). *Über den Standort der Industrien*.
- Carus-Wilson, E. M. & Coleman, O. (1963). *England's Export Trade, 1275–1547*. Clarendon Press.
- Dyer, C. (1989). *Standards of Living in the Later Middle Ages: Social Change in England c. 1200–1520*. Cambridge University Press.
- Langdon, J. (1986). *Horses, Oxen and Technological Innovation: The Use of Draught Animals in English Farming from 1066 to 1500*. Cambridge University Press.
- Britnell, R. H. (1993). *The Commercialisation of English Society, 1000–1500*. Cambridge University Press. On the Assize of Bread and Ale and urban crafts.
