# Note: market spacing (maps 1 and 3) and road length 300–500, under the sourced plague calendar (D173)

From the implementer, 24 October 2026, for the designer to decide. **No code.** *To check* marks what I have not read in the original.

## 1. The failures, and what they measure
**Under `plague_sourced`** (D168's gate, `v168.txt`):
- market spacing 26/40 on map 1 and 17/40 on map 3;
- road length 300–500 27/40 on maps 1 and 3;
- recovery by 500 28/40 on map 1.

**The two checks:**
- **Market spacing (gated 8–15 km):** the median nearest-neighbour distance among places of **more than 500 people**, in 1600 (`metrics.rs`).
- **Road length 300–500 (gated < 10%):** |road length in 1600 − road length in 1400| / road length in 1400 (`validate.rs`, v3(b), D54). It was meant to show "a road network that settles rather than churns".

## 2. What the trace shows (maps 1 and 3, seeds 1–10, default and sourced; `spacetrace`, `target/d2/sp173`)
| | Over 500 (median) | Spacing km (runs > 15) | Places 200–500 | 500–1,000 | 1,000–2,000 | Over 2,000 |
| --- | --- | --- | --- | --- | --- | --- |
| Map 1, default | 32 | 12.3 (2/10) | 56 | 21 | 6 | 5 |
| Map 1, sourced | 27.5 | 13.9 (3/10) | 52 | 18 | 4 | 5.5 |
| Map 3, default | 21 | 14.4 (3/10) | 64 | 14.5 | 3 | 3.5 |
| Map 3, sourced | 17 | 14.8 (5/10) | 54 | 10 | 1 | 3 |

**The places over 500 are no different in kind** under the sourced calendar:
- distance to the largest place 50–59 km (default) against 51–58 (sourced);
- no pasture;
- yield 1.43–1.51 × the region's.

**So the outer districts are not emptying.** There are fewer places in the 500–1,000 band everywhere, by about the fall in total population. 1600 is 1.04 × the peak with the sourced calendar, against 1.30 by default, a ratio of 0.80. Places over 500: 27.5/32 = 0.86 (map 1) and 17/21 = 0.81 (map 3).

**Map 3 is the weaker map** because its population sits lower in the size scale: 64 places of 200–500 against 14.5 of 500–1,000. A small fall in total population drops many places below 500.

## 3. The reading
**(a) Spacing on a population threshold tracks total population.**
- The 8–15 km band and the 500 threshold were set when the model's 1600 population stood at 1.3–1.46 × its pre-plague peak (D114 onwards).
- **The record's 1600 was about 0.85 of the peak** (Broadberry). As the model moves toward the record, places fall below 500 and the measured spacing widens. No change in the market network is needed for that.
- **The check is measuring population density, not the market network.** Bracton's 6⅔ miles, the band's footing (foundation table, "Towns keep their distance"), is about markets, not about places of 500 people.

**(b) Road length 1400 → 1600 is trough to recovery under the historical calendar.** With the sourced calendar, 1400 sits near the bottom of the 15th-century trough (0.38 of the peak, reached in the 1470s). By 1600 the population has more than doubled from its low. Roads are rebuilt on the new traffic, so their length changes by more than 10%. **The check reads a real recovery as churn.**

**(c) Recovery on map 1** is the known miss (D172): the recovery rate is 0.9–1.0% against 0.59%. It is not taken up here.

## 4. Candidate fixes, with sources
**For spacing:**
1. **Measure spacing on market places, not on population.**
   - The model has its own market function: places with a market of at least `market_min` W (100), which trade at a price (v7, D88–D92).
   - The check would take the median nearest-neighbour distance among those, in 1600.
   - **Sources:** Bracton's rule (6⅔ miles, about 10.7 km, between a market and a new one); **Everitt** (1967, *Agrarian History* IV): about 760 market towns in England and Wales c. 1500–1640, many of them small, an average of about one per 70 square miles, implying a spacing of about 10–13 km (*to check*: Everitt's figures); **Dyer** on small towns (300–1,000 people) as market centres.
   - **This keeps the band** (8–15 km) and changes what is measured to what the band is about. **Risk:** a different set of places, so the check must be re-read on the default before any gate. It is a re-founding of a gated check, so it is your decision.
2. **Market persistence (a mechanism).** Markets granted by charter persisted through the depopulation, or at least lapsed more slowly than population fell (Britnell 1981, "The proliferation of markets in England, 1200–1349"; Dyer on markets after 1349, *to check*: the share of markets that lapsed by 1500, which I recall as large).
   - In the model, W would fall more slowly than its inflow after a mortality: hysteresis on W.
   - **Against it:** the record also shows many small markets lapsing after 1349, so it would not obviously keep the 500–1,000 band. **Weaker footing than 1.**
3. **Lower the population threshold** (for example 300). It has no source for any particular number; it only moves the problem. **Not recommended.**

**For road length:**
4. **Measure the road network's stability over a steady window,** for example 1500–1600 or 1550–1600, or relative to the change in population or traffic. D54's intent was churn under steady conditions.
   - **Source** for persistence: Dalgaard et al. (2022) on Roman roads persisting where traffic kept them in use. That is a statement about churn at constant traffic, not across a population doubling.
   - **Again a re-founding of a gated check.** I'd propose road length per head of population (or per unit of traffic) changing by under 10% from 1400 to 1600. That keeps the dates and asks what D54 meant.

## 5. Recommendation
1. **Re-found market spacing on market places** (option 1). Re-read the default against it on 40 seeds (maps 1–3) before any calendar gate, and report how it compares with today's measure.
2. **Re-found road length** as length per head, 1400–1600 (option 4), with the same re-reading of the default first.
3. **Then re-gate the sourced calendar** (endemic 0.015) under the re-founded checks. Recovery on map 1 stays the known miss; if it alone fails, that is a decision for you, with the known cause.
4. **The over-2,000 lesson (D135) applies:** a threshold check that tracks total population was re-founded on its referent before. Both checks here have referents (Bracton and Everitt; Dalgaard), so neither is moved by result.

## Questions for the designer
1. Re-found spacing on market places (`market_min` W), keeping 8–15 km?
2. Re-found road length per head (1400–1600), or over a later steady window?
3. Re-read the default on both before the calendar is gated?
