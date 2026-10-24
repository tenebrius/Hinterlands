# Note: keeping raw wool leaving: wool grades or bounded cloth demand (D148)

From the implementer, 24 October 2026, for the designer to decide. **No code.** Each figure says where it comes from; *to check* marks what I have not read in the original.

## The problem (D147)
- **The region has one wool at one price.**
  - Cloth's netback beats raw wool's for every seller once the price falls (from about 1370).
  - So the only thing that keeps raw wool leaving is the mills' capacity.
- **Under putting-out, capacity follows returns,** and covers the clip by 1390. Every lb then goes to cloth: the share is 1.0 from 1400.
- **The record says otherwise.** Raw wool kept leaving through 1550, about 14k sacks in the 1420s and 7–9k in the 1480s–90s, beside 40–60k cloths (Carus-Wilson & Coleman, as tabulated by Munro). In the 1420s cloth took about 0.4 of the wool leaving.
- So the model needs a reason that some wool is still worth more raw.

## (a) Wool grades

**The record.**
- **Prices differed widely by grade, set in official schedules:**
  - the 1343 and 1454 schedules (Lloyd 1973; Munro 1978, "Wool-price schedules and the qualities of English wools", *Textile History* 9) *to check: the 1343 and 1454 figures themselves*;
  - the Calais lists of 1475 and 1499 (Munro, Table 15, in `target/d2/munro_textile_stats.txt`).
- **Spread between districts:**
  - 1475, English sack: March (Shropshire and Herefordshire) £15.4, fine Cotswold £13.9, Lincolnshire £11–13, Kent £9.2, Norfolk £8.9, Yorkshire £8.1. The best is 1.9 times the worst.
  - 1499, against Leominster: from 100% down to 25% (middle Holland and Rutland). Kent is 34%, Surrey and Norfolk 33%, fine Cotswold 58%.
- **Spread within a district's own clip** (good, middle and refuse wool, sorted from the same fleeces):
  - Leominster 1499: 100 : 70 : 49;
  - Cotswold 1499: fine 58, fine young 46, middle 40, middle young 31, refuse 28.
- **The model's present price is already a mean of grades:** Lloyd's 1294 mean of £5.02 is weighted to the inferior wools; the best abbey wools were £8.80 (Munro 1978; `docs/wool/l0.md`).
- **The economic reading is well established:**
  - the finest English wools went raw to the luxury draperies of Flanders and Brabant and to Italy;
  - English broadcloth used middling and coarser wools;
  - the export duty made raw wool dearer abroad, which the model already has in its observed prices.

  (Munro 1973, *Wool, Cloth and Gold*; Munro 1978; Lloyd 1977, *The English Wool Trade in the Middle Ages*; *to check: the shares of each grade in the raw-wool trade.*)

**How it fits the model.**
- **The clip of each pasture is split into G grades** (three, say: good, middle, refuse). The port pays each grade its own price, `wool_price_sack × r_g`.
  - The relative prices r_g come from the 1499 within-district ratios.
  - They are normalised so the clip's mean is Lloyd's £5.02, so the mean is unchanged.
- **The mills' netback is the same for every grade** (D148's framing): cloth at Munro's value less making cost and carriage.
  - **Implementer's reading:** this understates the record, where fine wool made dearer cloth. It is the simple form.
- **The result.** Good wool's port price exceeds cloth's netback, so it leaves raw. Middle and refuse wool go to cloth once the netback beats them.
  - The cloth share is then bounded by the share of the clip below the cloth netback.
  - It rises as Clark's fall takes more grades under the netback, and with Munro's later price.
- **The wool market's entropy split** already handles several buyers for one seller. Grades would be several goods from one seller, as salt, iron and stone are.
- **Grade by land, the alternative assignment:** each pasture's clip gets a grade mix by its land (chalk and limestone uplands better, marsh and heavy clay worse). This is the county pattern of 1475: March and Cotswold limestone high, Lincolnshire marsh, Kent and Norfolk low.

**What it fixes.**
- Raw wool keeps leaving in every decade, at the share of good wool, whatever the mills' capacity.
- The all-or-nothing share goes, because the choice is made per grade, not per region.
- The pace of the shift follows prices: Clark's fall, then Munro's rise, against fixed grade ratios.

**What it needs calibrated.**
1. **The grade shares of a clip:** what fraction of a fleece is good, middle and refuse. This is the one real calibration. *To source:* Munro 1978 or Lloyd 1977 may give sorting proportions. Otherwise the share that reproduces the 1300s raw/cloth split, set at the start only, as κ_c was.
2. **If grades are assigned by land,** a rule from the model's terrain (yield, slope, elevation) to a grade mix. This is a second calibration, and weakly sourced at the region's scale. **I would not do it.** The within-clip split is sourced, and needs no geology the model does not have.
3. **The price ratios r_g:** sourced (1499).

## (b) Bounded port demand for English cloth

**The record.**
- **English cloth's export markets were political and volatile:**
  - Flemish and Burgundian bans on English cloth (1359, 1434, 1447, 1464, *to check the years*);
  - the Hanse's privileges and the war of 1468–74;
  - the Low Countries' re-export through Antwerp after about 1490;
  - the mid-century slump (Hatcher 1996, "The great slump of the mid-fifteenth century").

  (Munro 1973, *Wool, Cloth and Gold*; Munro 1999, on Bruges and the English cloth staple, *to check*.)
- **The quantities are well sourced:** Carus-Wilson & Coleman's customs series, annual and national. Raw wool exports are in the same series.

**How it fits the model.**
- The ports stop taking any amount of cloth at Munro's price. Each year they take at most a quantity, or take more only at a lower price.
- The quantity would be the region's share of England's cloth exports that year: the customs series × the region's share of national population or wool. The price response would be an elasticity about Munro's price.
- Cloth over the bound drops in price, so mills bid less and wool goes raw again.

**What it fixes.**
- The cloth share is held below 1 when the bound binds.
- It reproduces the 1450s–60s dip, and the slumps of the bans.

**What it needs calibrated.**
1. **The region's share of England's exports.** This is weakly founded for a generic region.
2. **The elasticity of the outside demand.** No source I know of.
3. **Most important:** the bound is the record's own series. **The cloth share would then partly be fitted:** it follows the customs series because it is given it. The pace stops being a test. This is the main weakness.

## Recommendation: (a) wool grades
- **Stronger source.** The price schedules give the grade ratios directly, from several dated lists (1343, 1454, 1475, 1499). The economic reading (fine wool raw, middling wool in cloth) is the standard one.
- **Fewer calibrations,** and none that sets the outcome: the grade shares of a clip, at the start only. (b) needs a scale share and an elasticity, and feeds the record into the result it would be tested against.
- **The fit to the model is natural.** Grades are goods with a seller, a price and buyers, which the entropy split already handles.
- **(b) stays available as data for v4 shocks.** The bans and the Hanse war are events, and the event queue is their place.

## Questions for the designer
1. Grades within the clip (sourced ratios, one share calibration), or by land as well?
2. **Cloth's netback by grade:** the same for every grade (simple), or rising with grade as the record suggests? The second needs a grade-to-cloth-value source, *to find*.
3. With grades, does the capacity stock (putting-out) come back on, or is it judged separately?
