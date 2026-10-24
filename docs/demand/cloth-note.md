# Cloth: a design note for D2 (D118, D121)

For the designer to decide before any D2 code. It covers the four points D121 asks to be settled, with the evidence found and what is still missing.

## 1. Cloth per head, before and after 1348
- **The one figure found** is Dyer's estimate for c. 1500: about three yards of cloth a year per adult (1,250,000 adults buying three yards each would take 160,000 cloths, twice that year's exports). It is a figure for a population with higher living standards after the plague, the rise Dyer documents.
- **No per-head figure was found for c. 1300.**
- **The record gives the direction:**
  - wages and living standards rose after 1348 (Dyer 1989; Hatcher 1977);
  - the 1363 sumptuary law limited what labourers could spend on cloth, which suggests they were buying more;
  - cloth exports rose from a few thousand cloths in the 1350s to the 1500 figure.
- **Proposed** (a calibration bracketed by Dyer, to be tested by D0-style reporting before D3):
  - 1.5 yards per adult before 1348;
  - rising in a straight line to 3 yards by 1500;
  - this as a stand-in for D3 (wealth). Once D3 exists, cloth per head follows income per head, and the straight line is retired.
- **The adult share** (0.6 of people) is a calibration.

## 2. Wool per yard, by kind of cloth
- **Broadcloth:** 24 yards × 2 yards wide (statute of 1464/65). At the exchequer's reckoning of 4⅓ cloths to the sack of 364 lb, that is about 84 lb a cloth, or 3.5 lb a yard. A 1689 Commons record gives about 72 lb for a Gloucestershire cloth, 3 lb a yard.
- **Kersey:** 18 yards × about 1 yard (same statute); three kerseys were reckoned as one cloth by c. 1618. So a kersey is about 28 lb, or 1.6 lb a yard.
- **Most domestic cloth was the narrow, cheaper kind** (kerseys, russets, straits), and the broadcloth was the export cloth.
- **Proposed:** domestic cloth at 2 lb of wool a yard, between kersey and broadcloth; export cloth at 3.5 lb a yard (broadcloth). Both are approximate.

## 3. Wool per cell against England's clip
- **In the model,** pasture carries 1.5 sheep an acre at 1.5 lb a fleece on dedicated pasture only (L1). The region produces about 2 lb of wool a head (D0: 76,000 lb for 50,000 people in 1300).
- **England c. 1300 exported 25,000–45,000 sacks a year,** 9–16 million lb, for about 4.5–5 million people: 2–3.5 lb a head from exports alone, before any domestic cloth. National flock estimates in the sources found conflict (5–10 million sheep, uncited); Slavin (2020, *EHR*) is the source to consult.
- **The gap:** most English sheep grazed fallow, stubble and commons in sheep-and-corn husbandry, not dedicated pasture. The model counts only dedicated pasture.
- **Proposed:** before D2, add a fallow flock to arable cells: sheep on the fallow and commons at a rate per arable km², its wool counted with the pasture's. Sources: Campbell 2000 on sheep-corn systems; the rate is to be sourced or named as a calibration. Without it, D2 would starve either the exports or the home cloth.
- **D0's own sizing:** at 2 lb a yard and 1.5 yards per adult, home cloth needs about 1.8 lb a head before 1348. That is about the model's whole clip.

## 4. Ports selling wool or cloth when the region lacks wool
- **Historically,** England exported wool and imported little. A region short of wool bought from its neighbours, and fine cloth came in from Flanders and Italy as a luxury.
- **Proposed:**
  - **Ports sell neither wool nor cloth by default.** Cloth is made from the region's own wool, so a wool-short region makes less cloth and exports less wool, which is the trade-off D2 is meant to show.
  - **Imported fine cloth is already in "imports"** (the market people's luxuries).
  - If the D2 gate shows regions starved of cloth, the fallback is to let the ports sell wool at the world price, as D121 does for iron and salt.

## What D2 would need settled
1. Cloth per adult: 1.5 yards rising to 3 by 1500, or another path.
2. Wool per yard: 2 lb for home cloth, 3.5 lb for export cloth.
3. The fallow flock: added before D2 or not, and its rate.
4. Ports selling wool or cloth: no, or yes.

## Sources
- Dyer, C. (1989). *Standards of Living in the Later Middle Ages* (the three-yards figure as cited; the book was not consulted directly).
- Statute 4 Edw. IV c. 1 (1464/65) on the sizes of broadcloth and kersey ([Narrow cloth](https://en.wikipedia.org/wiki/Narrow_cloth); [Alnage](https://en.wikipedia.org/wiki/Alnage)).
- House of Commons Journal vol. 10, 11 June 1689 (a cloth of about 72 lb) ([british-history.ac.uk](https://www.british-history.ac.uk/node/21995)).
- Slavin, P. (2020). Mites and merchants: the crisis of English wool and textile trade revisited, c. 1275–1330. *Economic History Review* ([Wiley](https://onlinelibrary.wiley.com/doi/full/10.1111/ehr.12969)).
- Carus-Wilson, E. M. & Coleman, O. (1963). *England's Export Trade, 1275–1547* (not consulted directly).
