# Note: the growth rule, frontier against settled growth (D180)

From the implementer, 24 October 2026, for the designer to decide. **No code.** *To check* marks what I have not read in the original.

## 0. Summary
- **In this model only births add people.** Founding, the Rihll–Wilson pull, the urban drain and resettlement all move people from one place to another; plague, famine and endemic disease remove them. So the region can never grow faster than its natural increase.
- **The region fills from 500 people in 1100 to about 51,000 in 1300,** at 2.0–2.5% a year (§1). England grew at about 0.5% a year from Domesday to 1290.
- **So "settled increase at a sourced rate, with frontier growth through migration and founding" cannot fill the region on its own.** At 0.6% a year, 500 people become about 2,100 by 1340, not 60,000. Frontier growth needs a source of people. There are only three:
  - (A) more births on frontier land;
  - (B) settlers from outside the region;
  - (C) a region that is already partly settled when the clock starts.
- **I recommend (C), the settled start.**
  - The run still starts from ten hamlets and fills as now, as a spin-up before 1100.
  - The clock starts when the region reaches Domesday's share of its 1300 population.
  - From then on, births follow each place's own food ratio at a settled rate (at most 1%), and nothing depends on reach.
- **Before any result exists, the paired spacing and roads references need a decision** (§5). They were taken from a default that overshoots, at 1.25 × the peak in 1600. Any model that lands near the record's 0.85 will thin the markets and fail them.

## 1. The current rule
**Natural increase** (`growth.rs`, Pass 1):
- the gain is `r_grow` × (min(ratio, `food_cap`) − 1) / (`food_cap` − 1). That is 2.7% a year at a food ratio of 1.2 or more, and zero when a place is exactly fed;
- a shortfall below `hunger_tolerance` sheds people.

**What moves people, without adding any:**
- **the Rihll–Wilson pull:** `epsilon` × (inflow − K·W), met by migrants from the town's partners;
- **the urban drain:** a small place near a much larger one loses people to it;
- **resettlement** after a plague, towards vacant land;
- **founding:** at most six hamlets of 30 people every five years, which is about 36 people a year, taken from a donor.

**What removes people:** the plague calendar, famine shedding, and endemic mortality (1.5% a year, 1349–1480).

**The default's path** (map 1, seed 1; `just run`):

| Year | 1100 | 1150 | 1200 | 1250 | 1300 | 1350 | 1400 | 1450 | 1500 | 1550 | 1600 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| People (k) | 0.5 | 1.7 | 5.8 | 18.9 | 51.2 | 40.8 | 39.4 | 37.4 | 46.0 | 69.3 | 88.2 |
| Rate over the previous 50 years | | 2.5% | 2.4% | 2.4% | 2.0% | | | | 0.42% | 0.82% | 0.48% |

**Why one rate does both colonisation and recovery:**
- A place's food ratio rises above 1 whenever it can claim free land within reach. That happens in the fill, on land never cleared, and after 1348, on land vacated by the plague.
- In both cases births run at up to 2.7%. When the land runs out the food ratio falls back to about 1 and growth stops.
- So in this model the Malthusian stop works, but the rate below the ceiling is the frontier's, in every era.

**The earlier attempts:**
- **D165:** one ceiling of 1% everywhere. The region never fills (4.9k in 1340), because the region can grow only by births.
- **D170:** a ceiling set by each reach's never-cleared share. Colonisation fell by 40%:
  - a place's own reach fills while free land lies beyond it;
  - only founding can carry people there, and it moves about 36 people a year.

  The regional variant did not bind after 1348. Both are the same arithmetic.

## 2. What the record says
**Settled growth:**
- Domesday to 1290: about 1.71 million to 4.75 million, about 0.50% a year (Broadberry et al. 2015, Table 1.x; *to check*: the 1086 figure and its table).
- 1490–1600: 0.59% a year (Broadberry, Table 18, read).
- Wrigley & Schofield (1981): 1541–1601 at about 0.66% a year, with the fastest five-year spells near 1% (*to check*).
- `r_grow_pc` 1% stays the named upper bound.

**Internal colonisation:**
- Hallam (1965, *Settlement and Society*), on the south Lincolnshire fens: about a three- to fourfold rise in recorded tenants, 1086 to the 1260s, about 0.7–0.8% a year (*to check*).
- Assarting in the Weald and the woodlands (Dyer 2002, *Making a Living*).
- Colonisation ran at about 1.5 times the national rate, and it was fed largely by migration from nearby settled villages, not by its own births alone.

**Migration:**
- Poos (1991, *A Rural Society after the Black Death*, Essex): high turnover of names in the tithing lists, but most moves were short, within about 10–20 km (*to check*: the distances).
- Dyer: towns and colonised land drew people from their surroundings.
- So migration moves people a short way, as the model's partner-based moves do; it does not create them.

**The frontier's own births:**
- Settler populations on open land grew by natural increase at about 2.5–3% a year: colonial New England, 1650–1750 (Wrigley 1969; Potter, *to check*).
- So 2.7% is a credible frontier birth rate, but England after 1086 was not a frontier at that scale.
- After 1348, England had vacant land and did not regrow for a century and a half (Hatcher 1977). Land alone did not raise births.

**Reading:**
- The model's 1100–1300 fill, from 0.5k to 51k, is a colonisation from near-empty land that England had finished long before 1086.
- What the record gives for 1100–1600 is settled growth at 0.4–0.7%, with internal colonisation at the margin, fed by short-distance migration.

## 3. The options
**(A) More births on frontier land: the assart rule.**
- A place's births run at the frontier rate on the share of its own fields first cleared within the last T years, and at the settled rate on the rest.
- Its own fields carry a per-cell year of first clearing (`ever_owned` with a year), so reaches never overlap.
- After 1348, vacated land is not newly cleared, so recovery runs at the settled rate.
- **Against it: T sets how fast the region fills, and nothing sources it.**
  - For a place growing steadily at rate g, the share of new fields is 1 − e^(−gT). So g = r_s + (r_f − r_s)(1 − e^(−gT)).
  - With r_s 0.5% and r_f 2.7%: T = 25 years gives g ≈ 1.0%, so the region does not fill by 1340. T = 50 years gives about 1.8%.
  - T would be fitted to the fill.
- **Not recommended.**

**(B) Settlers from outside.**
- Founding into land never cleared brings its founders from beyond the region, not from a donor.
- The fill needs about 2% of the region a year from outside, by 1250 about 400 people a year, against today's 36 founders.
- No source gives that flux for an English region after 1086. It would be a fitted rate.
- It also changes coin, since founders bring coin, and the settlement count and failure rate, since there would be far more foundings.
- **Not recommended.**

**(C) A settled start (recommended).**
- **Spin-up:** the run starts from ten hamlets as now, and fills with today's rule and no events. The clock reads 1100 from the first year the region's population reaches a fraction φ of its 1300 level.
  - Because the model's arable a head is flat (D162: 14–16 cells per 1,000 people), the trigger can be land in use: φ × the default's share of farmable land in use in 1300.
  - φ = 1.71 / 4.75 ≈ 0.36 (Domesday against 1290; *to check*, as above). It is named, not fitted.
- **From 1100:** births follow the place's own food ratio at most at `r_settled`, 1%: `r_grow_pc`, the named bound, with Broadberry's 0.27–0.60% and Wrigley & Schofield's 0.66% as the checks.
  - Nothing depends on reach, so overlap does not arise.
  - Internal colonisation goes on as now: cultivation claims free land, founding takes free sites, and the pull and the drain move people. All of it is now fed by settled births, as in the record.
- **Endemic mortality drops to its sourced part,** about 0.007, the middle of Hatcher's 0.005–0.008. Its compensating half has nothing left to compensate.
- **What it changes in the brief:** the start is still ten hamlets, but 1100 is no longer year 0. The judged run is 500 years from the trigger.
  - The spin-up is deterministic: same Rng, same order.
  - Its length varies by seed, at about 130–150 years at today's rates (1.7k is reached by about year 50, and 18.9k by year 150, so φ × 51k ≈ 18k falls near year 150).
  - The calendar is keyed to calendar years, so the events land as now.

**(D) A date switch** (r_grow before a fixed year, settled after) is (C) with an arbitrary date. (C) puts the switch where the record does, at a settled share, and makes it the start of the judged run.

## 4. What (C) would do, predicted
**Filling to 1340:**
- From φ ≈ 0.36 at 1100, reaching the land's limit by about 1300 needs about 0.5% a year realised. At most 1% a year at a food ratio of 1.2 or more, falling as land fills, should give about that.
- **Risk:** the realised rate depends on how food ratios sit. If most places sit near 1, growth may fall to 0.2–0.3% and the peak comes late or low. The peak year and the 1300 level are pre-registered (§6).

**The recovery rate:**
- With the sourced calendar's trough (0.38 of the peak in the 1470s) and settled births, 1480–1590 at about 0.6% gives 1600 ≈ 0.38 × e^(0.006 × 125) ≈ 0.80 of the peak. The record is 0.85, at 0.59% a year.
- With the default calendar (trough about 0.6), 1600 would be about 1.2: still too high. So (C) and the sourced calendar go together.

**Endemic mortality's compensating half:** retired. Endemic falls to about 0.007. The trough then rests on the sourced calendar and the sourced endemic part.

**The sourced calendar:** re-gated with (C). Under today's rule its deeper trough freed land and drove a 0.91% recovery (D169). Under (C) the recovery is capped by births, not by land.

**Land per head, and so D3:**
- Not changed by (C). Claims stay need-led, so arable a head stays flat (D162).
- D3's income per head will still not rise after 1348. That needs the labour-led land rule or the preventive check, a separate piece of work.
- I'd keep it out of this change, so that one thing moves at a time.

**The gated checks** (the 1600 values are judged, so the question is what a region at about 0.85 of its peak in 1600 looks like):
- **Hierarchy** (over 5,000, rank-size, largest share):
  - towns still grow by the pull, while villages grow more slowly;
  - the urban share may rise and over 5,000 may fall, since the region in 1600 is about 0.65 of today's.
  - **Medium risk.**
- **Market spacing and road persistence (paired): high risk.**
  - Lower population in 1600 means fewer market places, and a deeper trough moves routes. That is the same mechanism as the sourced calendar (D177).
  - This is why §5 asks for the references to be settled first.
- **Recovery ≥ 0.75:** predicted about 0.80, near the floor. **Medium risk.**
- **Settlement count and failure rate:**
  - founding still runs, but with slower village growth there are fewer donors over 2 × `found_pop` at a food ratio above `donor_min_ratio`;
  - the failure rate needs a ruling: are spin-up foundings counted?
  - **Low to medium risk.**
- **Stability, plague drop, poor land deserted, pasture, Clark's tests, price level, exports / output, unsold grain:**
  - these are mostly about the 1300–1450 economy, which (C) leaves near its present size. **Low risk.**
  - Top-10 against off: **low risk.**
- **Performance:** the spin-up adds about 150 cheap years of a small region, roughly +5–10% of run time.

## 5. A decision needed before any result: the paired references
- **The spacing and roads references** (D176: 25 / 35 / 25 and 23 / 35 / 17) come from a default that ends at 1.21–1.29 × its peak.
  - A model that ends at 0.85 has fewer places over the market threshold and a deeper, longer trough.
  - D177 showed what that does to both counts.
- **Settling this now is not a change after the result:** no result for (C) exists yet. The options:
  1. **Keep D176's references.** (C) likely fails them, and the record is then judged against an overshooting model.
  2. **Pair against the default run with the sourced calendar and its parked results** (D177: spacing 20 on map 3; roads 8 / 13 / 5). This is the nearest reference to a model on the record.
  3. **Gate spacing on an absolute band again,** against Everitt's market density (*to check*, as in the spacing note), now that the band would be judged at the record's population.

  **I'd propose (3) if Everitt is read first, else (2).** Whichever is chosen is recorded before the exploration runs.

## 6. A staged plan
**S0 — measure the default, no change** (exploration, 9 runs a map; `landuse` and `run`, plus one probe for the decomposition):
1. The share of farmable land in use, by decade, 1100–1340.
2. The year, by seed, in which population first reaches 0.36 of its 1300 level, and how much that year varies.
3. A decomposition of each decade's change into births, deaths (shedding, endemic, plague) and moves (founding, pull, drain, resettlement). It confirms that moves sum to zero and shows what the fill rests on.

**S1 — build (C) behind `settled_start` (off), and explore** (9 runs a map):
- **Variants:** endemic 0.007 and 0.015; the default and the sourced calendar.
- **Pre-registered**, judged on the sourced calendar with endemic 0.007:
  - 1100–1300 realised growth 0.35–0.65% a year;
  - the peak year within 20 years of the default's, at 1.0 ± 0.15 of the default's peak population;
  - the trough 0.33–0.48;
  - recovery 1480–1590 at 0.45–0.75% a year;
  - 1600 at 0.75–1.0 × the peak;
  - spin-up length reported.
- Every gated check reported at 9 runs; none is called a pass or fail (D146).

**S2 — the gate,** 40 seeds a map, under the references settled in §5. Only if S1 meets its pre-registrations.

**S3 — release:** the view shows the spin-up as "before 1100", and the README and parameters are updated.

**Stop rules:**
- If S1 misses the 1100–1300 band, I report before anything else, and no knob is turned to meet it.
- If the hierarchy breaks, I report its attribution: villages slower against towns unchanged.

## 7. The browser's time (D180 asks for its cause)
- **v7: 45.9 ms a year;** D1 (stocks, the ports' salt and iron): 47.5. **The jump to 96.6 came at D136,** when `port_reach` and `cloth` became the default together.
- **Native, map 1 seed 1, 500 years** (four runs side by side, so the ratios matter more than the absolute figures):

  | Switches on | Neither | `port_reach` only | `cloth` only | Both (default) |
  | --- | --- | --- | --- | --- |
  | ms a year | 42.0 | 55.1 | 53.5 | 75.3 |

- **Each adds about 12–13 ms a year natively,** and their costs add up:
  - `port_reach` runs a bounded Dijkstra to the nearest port at each reach refresh, for every place with no port in reach, and adds a goods arc for each;
  - `cloth` solves the wool market with four times the goods' passes (D129) over home, mill and port buyers.
- **The browser runs at about 1.2 × native,** which gives 92 ms.
- **Not changed:** performance is reported only (D91). Both could be made cheaper (cache the port search between refreshes; fewer wool passes once the market converges). I would raise that only if you want it.

## Questions for the designer
1. Accept the arithmetic: frontier growth needs births, settlers from outside, or a settled start. And so (C)?
2. φ from Domesday against 1290 (0.36), with the trigger on land in use?
3. Endemic to its sourced part (about 0.007) with (C), as one change?
4. Which paired references (§5), decided before S1?
5. Are spin-up foundings counted in the failure rate?
