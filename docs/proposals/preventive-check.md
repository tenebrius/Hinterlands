# Sketch: the preventive check, births answering food above need at a lower slope (D162 option b)

From the implementer, 24 October 2026. **No code.** *To check* marks what I have not read in the original.

## The record
- **Hajnal (1965, "European marriage patterns in perspective")** describes northwest Europe's late, non-universal marriage: women marrying at about 23–26, and 10–20% never marrying. Fertility was thus held well below its biological ceiling, and it answered economic conditions through marriage.
- **Wrigley & Schofield (1981, *The Population History of England, 1541–1871*):**
  - nuptiality followed the real wage with a lag of a generation or more (*to check*: the lag they give, about 30–40 years in their Figure 10.x);
  - population growth rarely exceeded about 1% a year;
  - England's growth in 1541–1601 was about 0.6–1.0% a year (*to check*). Broadberry's Table 18 gives 0.55% a year for 1490–1560 and 0.60% for 1560–1600.

## The model today
- **Natural growth:** `r_grow` (2.7% a year) × (food ratio − 1) / (`food_cap` − 1), capped at food ratio 1.2. A village at 1.2 × need grows at 2.7% a year at once: a positive check only, with no lag.
- **The 1600 overshoot** (D114) is 1.30 × the pre-plague peak in today's runs (9 runs; the record is 4.11 / 4.81 = 0.85). It has two parts:
  - **The trough.** The model falls to 0.53 of its peak by the 1450s, against the record's 0.40 (1.90 / 4.81). Hatcher's gated 1450/1340 is 0.55 in the model.
  - **The recovery.** The model grows 0.64–0.77% a year in 1480–1590, against the record's about 0.59% (2.14 million in 1490 to 4.11 million in 1600).
- **Starting from the model's trough at the record's rate** (0.53 × e^(0.0059 × 110)) would give about 1.01 × peak in 1600. **The recovery explains only about 0.3 of the 0.45 overshoot; the trough explains the rest.**

## The mechanism
- **Births answer a lagged, smoothed food ratio,** not the year's: `food_lagged` is an exponential average over `marriage_lag` years (*to source*: about 25–35, Wrigley & Schofield).
- **The slope above need is lower:** growth = `r_grow_pc` × (food_lagged − 1) / (`food_cap` − 1), where `r_grow_pc` is the sustained maximum rate of a late-marriage regime. Its source is the highest sustained English rates: about 1% a year (Wrigley & Schofield; Broadberry Table 18).
- **The colonisation era is unchanged:** the 1100–1300 growth of a frontier region may keep `r_grow`. **This is a choice for you:** either one rate for all eras, or the preventive check from the first great mortality (1348) on, as Hajnal's pattern is attested by then (*to check*: Smith, Poos on late medieval English marriage).
- **Parameters:**
  - `marriage_lag` (sourced, about 30 years);
  - `r_grow_pc` (sourced, about 1%);
  - the era switch.

## The expected effect
- **The 1600 overshoot:** recovery capped near 1% a year with a lag lowers 1600 from about 1.30 to about 1.0–1.1 × peak. **It does not reach the record's 0.85 unless the trough deepens too.** The trough is Hatcher's path, which is gated; the model's 0.55 sits at its upper end (0.33–0.50 for 1450/1340, D109).
- **Hatcher's path:** with slower growth between outbreaks, the trough deepens without re-tuning `endemic_mortality`, toward the record's 0.40. This is (b)'s advantage over (a): one mechanism moves the trough and the recovery the right way.
- **Risks:**
  - **The founding era and the hierarchy:** if the check applied before 1348, the 1300 population would fall.
  - **The famine:** recovery after 1315–17 would be slower, which is the record's own path (*to check*).
  - **The tuned endemic rate (D111)** may then overshoot the trough. One re-check, not a re-tune, should come first.

## Recommendation
**(b) is better founded than (a)**, and it moves both parts of the overshoot. Its parameters are sourced. The land rule (D162) is not needed for it.
