# The preventive check: source check (D164), before any code

From the implementer, 24 October 2026. D164 asks for this before any code: whether the sources support the mechanism.

## 1. Wrigley & Schofield's lag
- **W&S** (1981) compared turning points in real wages and fertility, 16th to 19th centuries, and reported an average lag of about **50 years** (as given by Olney 1983, *JEH* 43, 71–77; W&S not read directly).
- **Olney (1983)** re-estimated it by a formal procedure: **about 16 years** over the three centuries, and 12–16 with 18th-century regional wages.
- **Nuptiality moved with real wages:** crude first-marriage rates and real wages "moved in tandem" (LSE working paper, 2023). The direction is well supported; the length of the lag is not settled.
- **There is no single sourced `marriage_lag`.** It is 16 or 50 years depending on the method, and the choice matters: a 50-year lag would make the response to the 1348 plague arrive around 1400.

## 2. The evidence before 1541 (the marriage pattern in medieval England)
- **For:**
  - **Smith,** on the 1377 poll tax: about 60% of adult women married, close to the early-modern pattern. The Spalding serf lists (a century earlier) are similar (Campop summary, 2024).
  - **Poos (1991, *A Rural Society after the Black Death*, Essex):** many unmarried servants in husbandry after 1348.
- **Against:**
  - **Razi (1980, Halesowen 1280–1349):** early marriage before the plague. His method has been criticised: 285 reconstructed families, and a re-estimate of male age at marriage of about 25 (La Poutré; Poos & Smith 1984).
  - **Bailey (2024):** "no convincing evidence" for a rise in single female servants after the plague, or for the pattern before 1550.
- **Read:** contested. Some evidence for late marriage c. 1377, none settled for 1100–1348, and a recent sceptical review. **Applying the lagged mechanism "in all eras" rests on a disputed claim for the medieval centuries.**

## 3. What is well sourced: the sustained growth rates
**Broadberry et al. Table 18** (English population, from parish data and Wrigley et al.):

| Period | Growth, % a year |
| --- | --- |
| 1270–1300 | 0.27 |
| 1300–1348 | 0.04 |
| 1400–1450 | −0.18 |
| 1450–1490 | 0.29 |
| 1490–1560 | 0.55 |
| 1560–1600 | 0.60 |

No English period up to 1700 grew faster than about 0.6% a year sustained. **The model's `r_grow` is 2.7% a year at a full food surplus,** four to five times any recorded sustained rate.

## 4. Options for the designer
1. **(b) as decided, all eras, with a named lag** (16 years, Olney, or 50, W&S). Its medieval footing is contested, and the result depends on the lag chosen.
2. **(b′) the sourced part only:**
   - the ceiling on natural growth, `r_grow_pc`, from Broadberry's sustained rates: about 0.6% a year at a full surplus, or 1% as a round upper bound;
   - births answer the food ratio over a short average (W2's ten years), with no marriage lag.

   This needs no claim about medieval marriage: it rests on the observed rates, whatever the mechanism behind them. **Risk:** the pre-plague growth to the peak slows. The model's 1100–1300 colonisation would reach its peak later, which the pre-registration (peak year within 20 years) will show.
3. **Leave growth as it is** and take up the trough instead. Hatcher's 1450/1340 is 0.55 in the model; the record's trough is 0.40.

**My recommendation: (b′).** It is the sourced core of (b): the rates are measured, the lag is not. If the pre-plague peak moves more than 20 years, the ceiling could apply from 1348 only. That is a cleaner era split than the marriage pattern, because it rests on the post-plague rates being measured.

Sources: Olney 1983; Broadberry et al. 2010/2015; Campop blog, "Marriage in the Middle Ages" (2024); Bailey (2024, UEA eprints); La Poutré, reply to Razi (*Law and History Review*). Most were read from abstracts and summaries, not in full.
