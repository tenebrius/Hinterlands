# Design note: putting-out labour in cloth (D141, D142)

From the implementer, 24 October 2026, for the designer to decide. **Nothing here is built,** and no code comes before the decision. The figures marked *to source* have not been checked.

## 1. The problem the trace found
- **The cloth share follows the population, not the returns** (D136, D138, D140).
  - Every mill is at capacity from the 1370s. Capacity is `cloth_lb_per_w` × W of the mill towns.
  - Mill towns' W grows only 0.00–0.08 a decade faster than the region's. That held before the payment fix (a margin of 11–14 d a head of W a year) and after it (8–9 d).
  - With the payment fix, the default since D140, the share is 0.15–0.18 through 1440–80 and 0.34–0.41 from 1490.
  - With the later cloth price (D138) it reaches the record's level, but about 40 years late.
- **The record's rise from 1420 to 1500 came while population was flat.** It came from more of the countryside's time going into cloth:
  - spinning and weaving put out by clothiers to rural households as by-employment;
  - fulling at rural water mills.

  Carus-Wilson 1941 (rural fulling); Oldland 2019, *The English Woollen Industry, c. 1200–c. 1560*; Hatcher and Bailey on Suffolk and the West Country. *To source:* the shares and the chronology.
- **In the model the making cost is earned only by W in the mill town.** Rural labour cannot enter cloth at all. A farmer's household has no use for its spare time, and the labour guard (D92) counts every person either as a farmer or as W.

## 2. What it should add, within the standing exclusions
There are to be no wages, no labour market and no agents. The settlement stays the unit.

The idea is a **cloth capacity that is a stock of its own.**
- It is the clothiers' putting-out network, held by each mill settlement.
- It grows with the margin it earns and decays with its upkeep: the Rihll–Wilson rule the model already uses for W, applied to cloth.
- It is bounded by the spare time of the farm households within reach.
- The making cost it pays goes to those households' purses, not to the mill town's market.

## 3. The proposed mechanism
**State:** `cloth_cap[j]`, lb of wool a year, for each mill settlement j.

1. **Growth** (each year, after the wool market):

   `Δcap_j = ε_c (π_j − κ_c · cap_j)`
   - π_j: the clothier's margin this year, coin a lb: cloth value at the port, less carriage, less the wool paid at the clearing price (D140), less the making cost paid out.
   - κ_c: the upkeep a lb of capacity, in the same coin: the margin at which a network neither grows nor shrinks. **Calibration:** it is set so that the start (1350–59) gives the same capacity as `cloth_lb_per_w` 2.5 × W does now. The start is then unchanged, and `cloth_lb_per_w` becomes the initial condition, not a cap.
   - ε_c: the speed. Rihll–Wilson's ε for W is the model's only precedent. **Calibration:** use the same ε, reported, not tuned.
2. **Labour bound.** The capacity cannot exceed what the spare time of the households in reach can spin and weave:

   `cap_j ≤ s × Σ_{i in reach of j} adults_i × (farm households' share of i's people)`
   - s: lb of wool a rural adult can spin and weave as by-employment in a year.
     - **A first bracket.** Spinning is the bottleneck: a weaver takes about 6 lb of yarn a day. A married woman spinning part-time manages about 2.5 lb a week (a late seventeenth-century study, giving about 40% of her time to clothmaking); a single woman manages about 6 lb. The medieval rate is not known well (Kania).
     - On that bracket, a household's spare time spins about 50–100 lb a year, or roughly 25–50 lb a rural adult. The mills' present capacity is 2.5 lb a head of the mill town's W.
     - **So the labour bound would hardly bind.** The region's farm adults could spin several times its whole clip (about 0.2–0.3 M lb). The pace would then be set by ε_c and κ_c, and by the wool supply. That makes the dynamics, not the labour bound, the calibration that matters (decision 2 below).
     - *To source before any run:* Munro's or Oldland's medieval figures, if they exist.
   - Where several mills reach one place, its labour is split among them by the S1 entropy form, by their margins, as the wool is split now.
3. **Payment.**
   - The making cost (2.7 d a lb, unchanged) is paid by the mill to the farmers' purses of the places whose labour it uses, in proportion to the labour drawn.
   - Its market keeps π_j.
   - Fulling and finishing stay in the mill town: a share f of the making cost, *to source* (fulling and finishing against spinning and weaving in Munro's Ypres and Leuven costings).
4. **Where it binds.**
   - At the start, capacity binds, as now.
   - When returns are high, capacity grows until the labour bound or the wool supply binds. When the margin falls, it shrinks.
   - The cloth share then answers the margin, and so the price (Munro's rise after 1470). It no longer answers W alone.
5. **The switch:** `putting_out` (off until the gate passes), as for every system.

## 4. What changes for what the gates guard
- **Hierarchy:** the coin of the making cost moves from mill towns to their hinterlands.
  - Mill towns keep the margin and the finishing.
  - Rural places in reach of a mill gain purse coin. Through the purse rules (D95) that may hold more people on poor and pastoral land.
  - **Risk:** spacing and over 5,000. Mill towns get less, and rural districts more.
- **Coin:** the making cost stays in the region either way. The channel changes, not the total.
- **Exports over output (D139, value added from D141):** the cloth share rises, so exports rise. The band of 1–35% has room: the medians were 0.066–0.078 at D140.
- **The labour guard (D92) is unchanged.** By-employment uses farm households' spare time, not their field time, and W is still the people the fields do not need.

## 5. What would show it works (to be pre-registered by the designer)
- **The cloth share:** 0.35–0.65 in 1440–60, and 0.55–0.85 in 1490–1510 with the later price. Without it, the share holds about 0.2 after 1420.
- **The share of cloth made with rural labour** rises from 1400. There are as yet no sourced figures. If one is found (Oldland's rural/urban split of the cloth tax, the alnage accounts), it can become a reported check.
- **Mill towns' W** no longer sets the capacity: capacity over W should rise across 1420–1500.
- **Where the clothing districts are:** the places whose farm purses gain most should be the pastoral, poorer-land places near mills, as in Suffolk, the Cotswolds and the Weald. This is reported, not gated.

## 6. Decisions for the designer
1. **Whether capacity becomes a stock that grows with margin** (this note), or stays tied to W with a rural term added (a simpler variant: `cap_j = a·W_j + b·rural adults in reach`, with no dynamics). The variant does not answer returns, so it would not close the lag through the 1400s.
2. **ε_c and κ_c:** the same speed as W and calibrated to the start, or sourced otherwise.
3. **The by-employment bound s,** and whether to source it before any run.
4. **The finishing share f** kept in the town.
5. **The order:** after the later cloth price is settled (D141 step 2), so the price and the mechanism can be read apart.
6. **What sets the pace:** if the labour bound does not bind (section 3.2), the pace comes from ε_c and κ_c.
   - Using W's own ε keeps them unfitted.
   - The cost is a pace that may still be wrong for a reason the note cannot test in advance.
   - A sourced figure for how fast clothiers' networks grew (for example the rise of cloth exports per decade, 1420–1500, about 1.5 times a decade *to source*) would be a check on ε_c, not a fit.

## 7. What it does not do
- No wages, no labour market, no individual clothiers.
- Home cloth is unchanged: households still spin their own (D128).
- No new export market or price. Munro's price and the capacity rule's start stay as they are.

## Sources for the spinning bracket (secondary, to be checked against the primary)
- The late seventeenth-century spinning study (married women 2.5 lb a week, single women 6 lb, about 40% of married women's time in clothmaking), as cited in "The Economic Impact of Clothmaking on Medieval Rural Society, 1300–1550", brewminate.com.
- J. S. Lee, as cited by Bret Devereaux, "Clothing, How Did They Make It? Part III", acoup.blog (2021): about 1 lb of yarn a spinner-day; a fourteenth-century weaver about 2 yards from about 6 lb of yarn a day.
- Katrin Kania, "Was Medieval Spinning Women's Work?", thefreelancehistorywriter.com (2015): medieval spinning rates are not known; her trials gave 40–100 m of yarn an hour.
