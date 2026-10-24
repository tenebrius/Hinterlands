# Note: land per worker after the plague (D160)

From the implementer, 24 October 2026, for the designer to decide. **No code.** Each figure says where it comes from; *to check* marks what I have not read in the original.

## 1. The cultivation rule, and why it stops at need
- **The rule** (`systems/cultivation.rs`): each place claims free cells in order of walking cost until its produce covers `fed × cult_margin / (1 − market_share)`.
  - `fed` is the farmers' need (people − W).
  - `cult_margin` is 1.2.
- **It releases its farthest cells** when produce exceeds that target by `release_hysteresis` (0.1).
- **So the land a place farms is set by its need, never by its labour.** When the plague halves a village, it releases about half its fields, and output per head returns to about 1.2 × need.

**Measured** (map 1, seeds 1–3; `landuse` and `cloth` probes on today's default; `target/d2/lpw`):

| Year | Arable cells per 1,000 people | People |
| --- | --- | --- |
| 1300 | 14.4 / 14.8 / 15.7 | 50.2k / 51.3k / 51.8k |
| 1450 | 14.4 / 15.0 / 15.0 | 38.8k / 34.0k / 36.5k |
| 1490 | 13.2 / 14.4 / 14.5 | 42.4k / 38.1k / 39.9k |
| 1600 | 13.5 / 13.2 / 13.6 | 94.2k / 84.3k / 91.1k |

**Arable per head is flat, within about 10%,** across the plague. This is why D3's income per head stayed at 1.0–1.24 (D159).

## 2. The record: land per head and output per head, 1300–1600
**Broadberry, Campbell, Klein, Overton & van Leeuwen** (2010 working paper, *English Economic Growth 1270–1870*, Tables 1, 18 and 19; the 2015 book). Read from the paper.

| | 1300 | 1380 | 1450 | 1490–1500 | 1600 |
| --- | --- | --- | --- | --- | --- |
| Total arable, M acres | 11.52 | 8.73 | 7.64 | 7.81 | 8.87 |
| Population, M | 4.73 | 2.50 (1377) | 1.90 | 2.14 (1490) | 4.11 |
| **Arable acres per head** | **2.44** | 3.49 | **4.02 (× 1.65)** | **3.65 (× 1.50)** | 2.16 (× 0.89) |
| Sown acres per head | 1.56 | 2.08 | 2.39 (× 1.53) | 2.27 (× 1.45) | 1.64 |

**GDP per head against 1340/48** (compounded from their Table 19):
- 1400/09 × 1.55;
- 1450/59 × 1.63;
- 1480/89 × 1.43;
- 1553/59 × 1.36;
- 1600/09 × 1.44.

**Real wages against 1300–1339** (Clark, `Wages 2014.xlsx`):
- farm labourers × 2.75 (1450s) and × 2.34 (1500s);
- building labourers × 2.30 and × 2.05.

**Holdings** (*to check*: the figures are from memory of the literature, not read here):
- **c. 1300:** most tenant holdings were a half-virgate (about 15 acres) or less. Many were cottars with under 5 acres (Kosminsky on the Hundred Rolls; Dyer 2002, *Making a Living*).
- **After 1348:** holdings were engrossed. Holdings of a virgate or more became common, and some of 60–100 acres (Dyer 2002; Hatcher 1977; Campbell 2000 on demesnes leased whole).
- **Arable fell, and the rest went to grass** (Campbell 2000), as L1 already models.

**Read:**
- **Land per head rose about 1.5–1.65 times by 1450–1500. Output per head rose about 1.4–1.6 times.** Both are well below the real wage, at about 2–2.75.
- Wages rose more than income per head, because labour's share rose as labour grew scarce.
- **This bears on D3's check.** Its income measure is income per head, so its referent should be GDP per head (× 1.4–1.6), not the real wage (× 2). At e = 1 that gives about 2.1–2.4 yards an adult in 1500, not 3.
  - Either clothing's elasticity is above 1: Dyer's 3 yards would need e ≈ 1.9 at × 1.43.
  - Or Dyer's 3 yards is high for an average adult.
  - **I'd raise this before D3 is gated.**

## 3. Output per head against land per worker
**Diminishing returns are already in the terrain.** Cells are claimed nearest first in walking cost, and the farther cells are poorer or farther. A place that keeps more fields gets less from each extra one, with no new parameter.

**The labour constraint is what the rule lacks.** A household can work only so much land.
- **The record:** a virgate household (about 30 acres of arable, with fallow) was near the upper end that a family with a plough team worked without hired labour.
- **Labour inputs** run at about 15–25 days an acre for arable by the fourteenth century (Campbell 2000, *to check*).
- **A labour-led rule:** a place keeps (or claims) fields while its farmers can work them, up to `acres_per_farmer` × farmers, and does not release them because need is met. Below that cap it still claims to need × margin, as now.
  - **Calibration:** `acres_per_farmer`, bracketed by the virgate (30 acres a household of about 4.5 people, about 6.7 acres a head of arable, about 4.4 sown at a third fallow).
  - **Check, not fit:** arable per head in 1450 against 1300, × 1.5–1.65 (Broadberry).
- **Yield response:** I would not add a separate yield-per-acre fall with land per worker. Broadberry's Table 2 shows yields per acre flat or rising after 1348 (wheat 7.99 bu in 1300–09; the post-plague decades, *to check*). The extra land's lower quality is the only diminishing return.

## 4. Where the surplus goes
- **Today:** food above `food_cap` (1.2 × need) adds no growth. It is stored, sent as relief, or lost ("surplus lost above food_cap per year 0" in validate). Food between 1 and 1.2 × need drives births at up to `r_grow`.
- **With labour-led land,** a village of half its pre-plague size keeps most of its fields, at about 2 × need.
  - **First, consumption.** Better diet, ale and meat after 1348 are the record's (Dyer 1989). Food above `food_cap` should count as income in kind for D3, uncapped. D3's measure capped own food at need, which hides it.
  - **Then sales.** More grain to market and to the ports, at lower prices (Clark's falling grain price against wool and cloth).
  - **Then D3's income:** stocks of cloth, iron and stone rise.

## 5. Interactions
- **`r_grow` and Hatcher's path (gated, must hold).**
  - **This is the main risk.** If more land per head lifts the food ratio to 1.2 everywhere, every village grows at the full `r_grow` 2.7% a year between outbreaks.
  - `endemic_mortality` was tuned (D111) against today's rule, where many villages sit near 1.0 after releasing land. **The tuned endemic rate would likely be too low, and the 1450 trough too shallow.**
  - **Two ways to hold it:**
    - (a) re-tune `endemic_mortality` once, as at D111, against the same Hatcher target;
    - (b) the record's own answer: the preventive check. People did not turn a doubled food supply into doubled births: late marriage, the European pattern (Hajnal 1965; for England, Smith; Poos), *to check*. It would mean births answer the food ratio at a lower slope, or with a lag, above need. That is a larger design change.

    **I'd recommend (a) first, and (b) only if (a) cannot hold Hatcher's path with the other checks.**
- **The famine.** More land per head is a buffer, so famines after 1348 bite less. That matches the record (no national famine on the 1315 scale after the plague). Reported.
- **Pasture (L1; "pasture 1450>1340" is gated).**
  - **Risk:** if villages keep their arable, less land goes to grass, and the gated rise in pasture could fail.
  - **The record's answer:** the extra land per worker went largely to grass, which needs a fifth of the labour (`pasture_labour` 5 a km² against `arable_labour` 40).
  - **Reading:** the labour rule should count pasture as a use of the same labour, so the land kept is split between arable and grass by their returns, as L1 does now. That keeps the conversion.
- **The hierarchy.** Richer farmers buy more at market (D3), so W grows on rural demand. That is broadly level across places, so it should not flatten the top as the cloth boom did (D148). Reported, with rank-size gated.
- **The 1600 overshoot** (population 1.43–1.46 × the pre-plague peak, D114, against the record's 0.85):
  - **The rule bears on it, in the wrong direction if nothing else changes.** Food above need speeds recovery after 1480, when the endemic era ends.
  - **With (b)** the overshoot would shrink, since births would no longer turn land into people at the full rate. This is the strongest argument for (b) in the long run.
  - **With (a)** the overshoot likely grows.

## 6. Recommendation
1. **Before any code, settle D3's referent:** GDP per head (Broadberry, × 1.4–1.6 by 1450–1500), not the real wage, as the income check. Then re-read Dyer's three yards at e = 1 against about 2.1 yards.
2. **The labour-led land rule** (`acres_per_farmer`, one calibration, bracketed by the virgate), with pasture counted as a labour use, behind its switch. Its checks:
   - arable per head 1450 / 1300 against × 1.5–1.65;
   - output per head against × 1.4–1.6;
   - Hatcher's path, gated.
3. **If Hatcher fails,** re-tune `endemic_mortality` once (a). If that cannot hold every check, the preventive check (b) comes next as its own note.
4. **D3's income counts own food uncapped** (consumption above need).

## Questions for the designer
1. D3's check against GDP per head, rather than the real wage?
2. The labour-led rule with one calibration (`acres_per_farmer`), or a rule tied to a sourced labour share (Cobb–Douglas, land share about 0.3–0.4, *to check*)? The first is simpler and its check is direct.
3. If Hatcher's path breaks, (a) a single re-tune of endemic mortality, or (b) the preventive check?
