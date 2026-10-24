# Note: the recovery rate, full growth on free land and a settled rate where land is taken (D168)

From the implementer, 24 October 2026, for the designer to decide. **No code.**

## The problem
- **After the sourced calendar** (D167 exploration), the model recovers at 0.91% a year in 1480–1590, against the record's 0.59% (Broadberry Table 18: 2.14 million in 1490 to 4.11 million in 1600). 1600 reaches 1.04 × the pre-plague peak, against 0.85.
- **D165 showed that one national ceiling cannot work:** the region fills from hamlets at about 1.5% a year, a frontier's rate, and a 1% ceiling empties it.
- **So the rate must depend on the land:** a frontier's rate where land has never been taken, a settled country's rate where it has.

## 1. What "free" means, and why it cannot be "unowned"
**After 1348 half the fields are vacated.** If "free" meant unowned cells, the plague would make the land free again, and survivors would grow at the frontier rate: exactly today's fast recovery.

**The record says the opposite.** England after 1348 had abundant vacant land and did not regrow for a century and a half. That is Hatcher's puzzle, answered by high mortality and low fertility, not by a lack of land.

**Proposed: "free" is land never cleared.** A cell is free if no place has ever owned it as a field: a new per-cell flag, `ever_owned`, set when cultivation first claims it and never cleared.
- **A place's free share** = the free cells in its field order (its reach, at or above `min_field_yield`) over all such cells.
- **The ceiling on natural growth:** r_max = `r_grow_pc` + (`r_grow` − `r_grow_pc`) × free share.
- **A new hamlet in untouched land** has a free share near 1 and grows at `r_grow` (2.7%), as today. A village whose reach has all been cleared once grows at most `r_grow_pc`.
- **After 1348,** vacated fields stay "ever owned", so recovery runs at the settled rate.

**Sources for the two rates:**
- **Settled:** Broadberry et al. Table 18. England's sustained rates were 0.27–0.60% a year, 1270–1600. `r_grow_pc` 1% (D165) stays the round upper bound, since a place can exceed the national mean.
- **Frontier:**
  - colonisation of the fens and the Weald in the 12th–13th centuries, *to check*: Hallam (*Settlement and Society*, 1965) on the Lincolnshire fens, about three- to four-fold growth 1086–1260;
  - and the model's own need: about 1.5% a year averaged to fill from hamlets.
  - **The frontier rate stays `r_grow` (2.7%), as calibrated since v1.** It is fast because new places also gain by migration, which the record's rates net out.

**The free share has no separate calibration.** It is a count of cells, read from the model's own land; its only threshold is `min_field_yield`, which already exists.

## 2. What it does to endemic mortality's compensating half
- **Today 0.015 nets against `r_grow` 2.7%** between outbreaks. About half of it is sourced, the 15th century's excess deaths of about 0.005–0.008 (Hatcher 1986 against Wrigley & Schofield); the other half compensates for the frontier rate (D167 note).
- **Under this rule,** by 1348 the region's land has all been cleared once (the model fills to its peak), so every place has a free share near 0. Recovery runs at `r_grow_pc` (1%), not 2.7%.
- **The compensating half then has nothing to compensate.** At 0.015 with births capped at 1%, the trough would deepen far below the record. D165's exploration showed the same at 1%: Hatcher's ratio fell to 0.22.
- **So the rule and endemic go together.** I'd propose endemic mortality drop to its sourced part, about 0.007, the middle of Hatcher's 0.005–0.008, named and not fitted. The test is whether the trough and the recovery then land on the record without tuning.
- **Pre-registrations I'd suggest:**
  - trough 0.33–0.48;
  - recovery 1480–1590 at 0.45–0.75%;
  - 1600 at 0.75–1.15 × peak (the re-founded gate);
  - the peak year within 20 years;
  - the 1300 population within 10% of today's. Colonisation is unchanged while land is free.

## 3. Risks
- **The pre-plague peak.** Places that fill their reach early (the oldest, central ones) slow to 1% before 1348, while the frontier is still open elsewhere. The peak may come later or lower. Reported: the peak year and the 1300 population.
- **The famine (1315–17).** Recovery from it would be at the settled rate, which is slower. That is closer to the record (*to check*: Jordan's *The Great Famine*).
- **The hierarchy.** Towns grow by migration (Rihll–Wilson pull), which is unchanged, so towns keep growing while villages slow. That could raise the urban share; reported.
- **Map 3's spacing** (reference 56/80): slower village growth after 1480 means fewer places over 500 by 1600, which widens spacing. **This is the main risk to the gate.**
- **New founding** onto never-cleared land after 1348 would grow at the frontier rate. That is right in the record's sense: new colonisation of waste. Rare after 1348; reported.

## 4. Questions for the designer
1. "Free" as never-cleared land (`ever_owned`), not unowned land?
2. Endemic mortality to its sourced part (about 0.007) with the rule, as one change? Or the rule alone first, as an exploration, to show the over-deep trough?
3. The frontier rate stays `r_grow` (2.7%, v1's calibration), or should it be sourced (Hallam's fens, about 0.7–0.8% a year), at the cost of the region filling more slowly?
