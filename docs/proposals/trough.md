# Note: the trough, the plague calendar after 1348 (D167)

From the implementer, 24 October 2026. A small code change was needed for the exploration: the sourced calendar behind `plague_sourced` (off, golden unchanged).

## 1. The national outbreaks after 1369, against the model's list
**Sources:** a secondary compilation (urbanrim.org.uk, "Plague in England") citing Gottfried 1978 (*Epidemic Disease in Fifteenth-Century England*), Gottfried 1983, Shrewsbury 1970 and Creighton; and Hatcher 1986 (*EcHR* 39, "Mortality in the fifteenth century: some new evidence", read in full) for Canterbury.

| Model (D108) | Sources | Extent |
| --- | --- | --- |
| 1375 | 1375 | national ("pestis quarta") |
| — | 1379–83 | north 1379, London 1382, Kent 1383: regional |
| 1390 | 1390–91 | national |
| 1400 | 1399–1400 | national |
| 1407 | 1405–06 | national |
| 1413 | 1411–12 | national |
| — | 1420–23 | Norfolk and the north: regional |
| — | **1428–29** | **national** |
| 1434 | 1433–35 | national |
| 1439 | 1438–39 | national |
| — | 1447–54 | "parts of England"; 11 of the 18 years 1442–59 had epidemics |
| — | **1457** | **severe at Canterbury "and other parts of the realm"** (Hatcher 1986, from John Stone) |
| 1464 | 1463–65 | the whole kingdom |
| — | **1467** | parts, possibly national |
| 1471 | 1471 | all England |
| 1479 | 1479–80 | London and many parts; Gottfried: "the most virulent epidemic of the fifteenth century" |

- **The model's ten later years are confirmed within one or two years.**
- **Missing:** 1428–29 (national), 1457 (severe), 1467 (possibly national), and the regional clusters.
- **The sourced calendar** (`SOURCED_PLAGUES`, 13 outbreaks, each at the first year of its span): 1375, 1390, 1399, 1405, 1411, 1428, 1433, 1438, 1457, 1463, 1467, 1471, 1479.

## 2. Mortality of each outbreak
- **National mortality is not measured for the outbreaks after 1369.** The secondary list gives none, and Gottfried's figures (from wills) are not in hand.
- **Hatcher 1986 gives Christ Church, Canterbury's monks** (about 70–90 adult men; crude death rate per 1,000):
  - **1457: 189** (16 deaths, the highest);
  - **1471: 139** (plague, 11 deaths);
  - 1485: 116 (the sweat);
  - lesser plague in 1487 and 1501.
- **Crises over 40 per 1,000:** 27 in 1395–1505, 17 of them before 1450 (peaks of 40–60) and 10 after (steeper; 8 above 60).
- **Mortality did not fall over the century:** "After 1450 … [the death rate] was usually above 30 per thousand and often much higher." The model's rule (10% falling to 6% from 1375 to 1479) runs the other way.
- **In the exploration,** 1457 and 1471 take the monks' rates (0.189 and 0.139). The other outbreaks keep the model's falling rule. **One community's rates are an upper bound for a region.**

## 3. The endemic baseline the tables imply, against 0.015
**Hatcher 1986:**
- **The monks' crude death rate** averaged 33.1 per 1,000 over the century, crises included.
- **The 9-year moving average** was under 20 in the 1420s, 20–30 in 1430–50, and above 30 after 1450.
- **Life expectancy at birth** implied by the monks' adult mortality: 21–23 years.
- **For comparison, Wrigley & Schofield (1541–1871):** the national quinquennial crude death rate never above 37, and above 33 only three times.

**Read:**
- The monks' excess over the early-modern national rate is about 5–8 per 1,000 a year, outbreaks included. Between outbreaks it is near 0 before 1450, and about 5 per 1,000 after.
- **So the sourced excess is about 0.005–0.008, not 0.015.**

**But `endemic_mortality` is not a crude death rate in the model.**
- The model has no deaths and births separately. Natural growth is `r_grow` (2.7%) at a full food ratio, and 0.015 nets against that.
- **0.015 stands for two things:** the endemic disease of the record, and the low fertility the model lacks (D165: r_grow is a frontier's rate).
- **It cannot be checked against a death rate alone.** Its sourced part is about half of it; the other half is the growth rule's excess.

## 4. Exploration: 9 runs (maps 1–3 × seeds 1–3), `landwork --from 0`, `target/d2/tr167` (not a gate)
| Calendar, endemic | 1450 / 1340 (range) | Trough / peak (decade) | 1590s / peak | Recovery 1480–1590 |
| --- | --- | --- | --- | --- |
| D108, 0.015 (default) | 0.55 (0.48–0.65) | 0.53 (1470s) | 1.30 | 0.77% a year |
| D108, 0 | 1.27 (1.19–1.43) | 0.78 (1350s) | 1.56 | 0.16% |
| **Sourced, 0.015** | **0.49 (0.42–0.58)** | **0.38 (1470s)** | **1.04** | 0.91% |
| Sourced, 0 ("the calendar alone") | 1.17 (1.08–1.27) | 0.78 (1350s) | 1.50 | 0.34% |

**The record:** 1450/1348 is 0.40 (Broadberry, 1.90 / 4.81); Hatcher's half to two-thirds fall gives 0.33–0.50; 1600 / peak is 0.85; recovery 1490–1600 is 0.59% a year.

**Read:**
- **The calendar alone does not make the trough.** Without the endemic term the model regrows between outbreaks at up to 2.7%, and 1450 is above 1340.
- **The sourced calendar with today's endemic term lands the trough at the record's level** (0.38 against 0.40):
  - 1450 / 1340 is 0.49, inside D114's band (0.40–0.65) and Hatcher's (0.33–0.50);
  - **the 1600 overshoot falls from 1.30 to 1.04;**
  - the recovery is faster (0.91% against the record's 0.59%), because there are more survivors' vacancies.
- **Of the remaining overshoot (1.04 against 0.85), most is now the recovery rate.** That is your option 1 later: a land-dependent ceiling.

## 5. The gated band
- **D114's band for 1450 / 1340 is 0.40–0.65,** pre-registered and reported (D114 met it in 60/60 runs). It is not a gated check in `validate`, which gates "plague drop" (≥ 25%) and "recovery by 500" (≥ 0.90 of the pre-plague peak).
- **The record's 0.40 sits on the band's lower edge.** Hatcher's range (0.33–0.50) overlaps it only in 0.40–0.50.

**A conflict to resolve:** "recovery by 500 ≥ 0.90" is gated, and the record's 0.85 would fail it. With the sourced calendar the model reaches 1.04, so it passes, but a further move toward 0.85 would break it.
- **Proposal:** "recovery by 500" becomes 0.75–1.15 of the peak. The record's 0.85 then sits inside, and the D164 ceiling of 1.15 is the upper bound. This is a re-founded band with a referent (Broadberry), decided before any run against it.

## 6. Recommendation
1. **Adopt the sourced calendar,** with 1457 and 1471 at the Canterbury rates as named upper bounds, and `endemic_mortality` 0.015 as now. Gate it on 40 seeds, reporting:
   - 1450 / 1340;
   - trough / peak;
   - 1600 / peak;
   - the recovery rate.
2. **Re-found "recovery by 500" as 0.75–1.15** before the gate, if you agree.
3. **Record that `endemic_mortality` 0.015 is about half sourced** (the 15th-century excess, about 0.005–0.008). The rest stands in for the growth rule. It is not re-tuned now.
4. **The recovery's rate** (0.91% against 0.59%) is the next step, in the land-dependent form (your option 1).
