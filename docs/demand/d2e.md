# D1b + D2 the default, and the cloth-pace trace (D135)

## 1. Over 2,000 reported, not gated (D135.1)
- `validate` drops "over 2,000" from the gated checks. It now prints two reported rows a map:
  - the count over 2,000 (seed 1, median, range);
  - the urban share, people in places over 2,000 in the final year (1600), against England's c. 0.10 c. 1300.
- `Metrics` gains `urban_share`. The tuning score in `sweep` no longer counts over 2,000.
- "Runs passing every check" now counts the gated checks only, so it is not comparable with earlier counts that included over 2,000.

## 2. The formal gate: D1b + D2 on, 40 seeds. **PASSED**
`target/d2/v135.txt`, binary built from 9b9e361 with `port_reach` and `cloth` on by default (the same model as D133's run).

| Check (gated) | Map 1 | Map 2 | Map 3 |
| --- | --- | --- | --- |
| rank-size slope | 38/40 | 40/40 | 40/40 |
| largest share | 40/40 | 40/40 | 39/40 |
| over 5,000 | 40/40 | 40/40 | 40/40 |
| market spacing | 37/40 | 37/40 | 34/40 |
| stability | 40/40 | 39/40 | 40/40 |
| top-10 vs off | 37/40 | 40/40 | 36/40 |
| every good worked | 40/40 | 40/40 | 36/40 |
| pasture outward | 36/40 | 40/40 | 40/40 |
| pasture 1450>1340 | 35/40 | 40/40 | 40/40 |
| road length 300–500 | 36/40 | 40/40 | 36/40 |
| outside trade | 40/40 | 40/40 | 40/40 |
| every other check | 40/40 | 40/40 | 40/40 |
| top-10 overlap (≤ 0.80) | 0.35 | 0.43 | 0.34 |
| **runs passing every check** | **25/40** | **36/40** | **28/40** |

D1 for comparison: 27 / 28 / 17 runs passing every check, with over 2,000 gated then.

| Reported (D135) | Map 1 | Map 2 | Map 3 |
| --- | --- | --- | --- |
| over 2,000 in 1600: median (range) | 6 (4–9) | 6 (5–9) | 5 (3–8) |
| urban share in 1600: median (seed 1) | 0.356 (0.319) | 0.369 (0.351) | 0.373 (0.321) |
| England c. 1300 | 0.10 | 0.10 | 0.10 |

- **Golden:** `5e773d6f60251a23`. The browser selftest (`?selftest=500`) gives the same hash at 96.6 ms a year.
- **Exports a head c. 1300:** 7.6 / 8.4 / 8.7 d (England about 14 d).
- **The D133 misses stay missed:**
  - map 3's population in 1600 is 11% below D1b's (10% allowed);
  - coin is above D1b's from 1550;
  - the cloth share is about half the record's after 1420.

## 3. The cloth-pace trace (D135.3). Nothing changed, nothing tuned
**Method.**
- `hinterlands clothpace --map-seed M --seed S` runs maps 1–3 × seeds 1–3 (`target/d2/pace/`).
- Each figure is the mean over a decade's ten years; the tables give the median of the 9 runs.
- The wool market writes `state.cloth_trace` each year. It is reported only and leaves the hash unchanged.
- **Netbacks are in d a lb of wool, at the clip-weighted median seller:**
  - raw wool: the port price less wool carriage to its port;
  - cloth: the best mill bid in its reach (Munro's 5.7 d less cloth carriage to the mill's port, less the 2.7 d making cost) less wool carriage to that mill;
  - realised: the price the seller's clip actually clears at.
- **Mill towns** are the places that milled in the year before the decade. Their W growth is over the decade, for the same places.

### Mill capacity against use
| Decade | Mills bidding | At capacity | Capacity lb | Milled lb | Use / capacity |
| --- | --- | --- | --- | --- | --- |
| 1350 | 101 | 86 | 28,000 | 17,000 | 0.60 |
| 1360 | 101 | 99 | 30,700 | 28,100 | 0.93 |
| 1370 | 104 | 104 | 29,400 | 29,400 | 1.00 |
| 1380 | 104 | 104 | 33,800 | 33,800 | 1.00 |
| 1390 | 104 | 104 | 34,800 | 34,800 | 1.00 |
| 1400 | 103 | 103 | 34,600 | 34,600 | 1.00 |
| 1410 | 101 | 101 | 23,300 | 23,300 | 1.00 |
| 1420 | 99 | 99 | 24,200 | 24,200 | 1.00 |
| 1430 | 99 | 99 | 23,700 | 23,700 | 1.00 |
| 1440 | 101 | 101 | 20,200 | 20,200 | 1.00 |
| 1450 | 102 | 102 | 20,400 | 20,400 | 1.00 |
| 1460 | 102 | 102 | 19,400 | 19,400 | 1.00 |
| 1470 | 101 | 101 | 19,200 | 19,200 | 1.00 |
| 1480 | 100 | 100 | 21,800 | 21,800 | 1.00 |
| 1490 | 100 | 100 | 28,700 | 28,700 | 1.00 |
| 1500 | 104 | 104 | 33,000 | 33,000 | 1.00 |
| 1510 | 106 | 106 | 37,900 | 37,900 | 1.00 |
| 1520 | 112 | 112 | 42,700 | 42,700 | 1.00 |
| 1530 | 112 | 112 | 47,100 | 47,100 | 1.00 |
| 1540 | 112 | 112 | 50,400 | 50,400 | 1.00 |

**From the 1370s every mill bidding is at capacity, in every run.**

### Mill towns' W against their cloth margin
| Decade | Mill towns | Their W at start | W growth over the decade | Region's W growth | Cloth margin, d a head of W a year |
| --- | --- | --- | --- | --- | --- |
| 1350 | 100 | 9,550 | +0.39 | +0.31 | 7.4 |
| 1360 | 101 | 13,300 | −0.04 | −0.06 | 9.2 |
| 1370 | 103 | 12,500 | +0.01 | −0.01 | 9.1 |
| 1380 | 105 | 12,400 | +0.17 | +0.12 | 11.8 |
| 1390 | 104 | 14,500 | +0.01 | −0.02 | 10.6 |
| 1400 | 102 | 14,900 | −0.09 | −0.10 | 10.6 |
| 1410 | 101 | 13,600 | +0.03 | +0.00 | 11.6 |
| 1420 | 99 | 9,660 | +0.03 | +0.01 | 11.9 |
| 1430 | 98 | 9,820 | −0.09 | −0.09 | 11.2 |
| 1440 | 100 | 7,910 | +0.08 | +0.05 | 12.0 |
| 1450 | 103 | 8,660 | +0.03 | +0.01 | 11.9 |
| 1460 | 101 | 8,010 | −0.02 | −0.02 | 11.5 |
| 1470 | 102 | 7,860 | −0.04 | −0.04 | 11.3 |
| 1480 | 99 | 7,530 | +0.07 | +0.09 | 12.2 |
| 1490 | 98 | 10,800 | +0.13 | +0.09 | 13.6 |
| 1500 | 101 | 12,300 | +0.13 | +0.11 | 13.8 |
| 1510 | 106 | 14,400 | +0.12 | +0.10 | 14.1 |
| 1520 | 109 | 16,200 | +0.12 | +0.08 | 13.8 |
| 1530 | 112 | 18,100 | +0.09 | +0.08 | 13.8 |
| 1540 | 111 | 19,600 | +0.07 | +0.06 | 13.6 |

### Netbacks and prices (d a lb)
| Decade | Wool's port price | Cloth value | Raw wool netback | Cloth netback | Realised | Clip share where cloth beats wool | Median mill bid | Price mills paid |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1350 | 3.22 | 5.70 | 3.22 | 2.99 | 3.02 | 0.0 | 3.00 | 1.86 |
| 1360 | 3.06 | 5.70 | 3.05 | 3.00 | 2.88 | 0.1 | 3.00 | 1.05 |
| 1370 | 2.89 | 5.70 | 2.89 | 3.00 | 2.72 | 1.0 | 3.00 | 2.39 |
| 1380 | 2.73 | 5.70 | 2.72 | 3.00 | 2.57 | 1.0 | 3.00 | 0.69 |
| 1390 | 2.56 | 5.70 | 2.56 | 3.00 | 2.42 | 1.0 | 3.00 | 0.13 |
| 1400–1540 | 2.47 | 5.70 | 2.47 | 3.00 | 2.33–2.37 | 1.0 | 3.00 | 0.00–0.51 |

**Price mills paid, per run** (milled-weighted median; runs map/seed 1/1 … 3/3):
- 1360: 2.04, 2.24, 2.76, 0.00, 1.05, 0.69, 2.86, 0.42, 0.52;
- 1400: 1.54, 1.16, 0.00, 0.00, 0.00, 0.10, 2.34, 0.00, 2.33;
- 1450: 0.00, 0.00, 0.00, 2.33, 0.00, 0.64, 1.90, 0.00, 2.34;
- 1500: 0.00 in all nine.

### Wool's port price against cloth's
- **In the model:**
  - wool is Lloyd's £5 a sack, 3.30 d a lb. Clark's fall of a quarter against wheat runs in a straight line from 1350 to 1400, to 2.47 d, then stays flat;
  - cloth is Munro's 5.7 d, flat throughout.
- **Clark's series** (from `docs/wool/l0.md`): wool a lb against wheat a bushel was about 0.92 in the 1300s and 0.67–0.68 from the 1340s to the 1440s. That is the same fall of a quarter, done by the 1340s in the record. The model has it done by 1400.

### A later English cloth price: found
- **Source:** Munro, *English textile prices, 1360–1560*, `EngclothPrices2.xls` (sheet AllPorts6M, "Cloth exports from all ports in £ sterling", from the TNA Particular Customs Accounts). It is listed at https://www.economics.utoronto.ca/munro5/MedClothPrices.htm.
- **Values** are the mean value a cloth, by decade (quinquennia averaged), in £ sterling and nominal:

| Decade | £ a cloth | Against the 1371–1450 mean (1.88) | At 84 lb, d a lb | Cambridge 1st quality (domestic), £ |
| --- | --- | --- | --- | --- |
| 1370s | 2.03 | 1.08 | 5.81 | 2.32 |
| 1380s | 2.01 | 1.07 | 5.74 | 2.47 |
| 1390s | 1.55 | 0.82 | 4.42 | 1.99 |
| 1400s | 1.65 | 0.88 | 4.72 | 2.14 |
| 1410s | 1.35 | 0.72 | 3.86 | 2.12 |
| 1420s | 2.04 | 1.08 | 5.81 | 2.27 |
| 1430s | 2.19 | 1.17 | 6.27 | 2.27 |
| 1440s | 2.21 | 1.18 | 6.32 | 2.39 |
| 1450s | 1.86 | 0.99 | 5.32 | 2.57 |
| 1460s | 1.86 | 0.99 | 5.32 | 3.15 |
| 1470s | 2.13 | 1.13 | 6.09 | 2.96 |
| 1480s | 2.35 | 1.25 | 6.72 | 3.39 |
| 1490s | 2.55 | 1.36 | 7.28 | 3.56 |
| 1500s | 3.24 | 1.72 | 9.25 | 3.43 |
| 1510s | 3.61 | 1.92 | 10.30 | 3.71 |

- The series ends at 1511–15.
- **Two cautions:**
  - It is nominal. The model's coin is tied to wheat, so the series would have to be set against wheat. Clark's wheat series is not in the repo for 1450–1550, and I have not deflated it.
  - Munro gives the standard broadcloth's weight rising from about 38 lb at the start of the fourteenth century to 64 lb later (Toronto working papers; the dates of the later figure not checked), so a value a cloth is not a value a lb.
- **Read:** flat to about 1470, then rising to about 1.25–1.36 times in the 1480s–90s and 1.7–1.9 times in the 1500s–10s.

### What the trace says sets the pace (readings marked)
1. **Mill capacity binds from the 1370s, everywhere** (fact, table 1).
   - Cloth wool equals 2.5 lb × the mill towns' W.
   - From 1370, cloth's netback (3.00 d) beats raw wool's (2.47–2.89 d) for every seller's clip.
   - So the share is set by the capacity, not by prices. The cloth share by decade (D133, the same model) moves with capacity: 0.26–0.28 at 1390–1410 (capacity about 34.7k lb), 0.19–0.22 through 1420–80 (19–24k lb), 0.33–0.35 from 1490 (29–38k lb).
2. **Mill towns' W grows at about the region's rate** (fact, table 2).
   - It is 0.00–0.05 a decade faster, falling from about 14.9k in 1400 to 7.5k in 1480 with the population trough, then rising again.
   - **Reading:** the cloth margin (11–14 d a head of W a year) raises the mill towns' market inflow, but not enough to make them grow faster than the region. Capacity, and so the share, follows the population curve.
3. **Mills pay sellers far less than the clearing price** (fact, table 3 and per run). From 1450 they mostly pay nothing.
   - **Code path:** a mill pays `min(want, mpurse − food)` out of its market purse in the wool market. Its cloth proceeds are credited to that purse only after all wool is settled. So a mill whose market purse is spent on food takes its capacity of wool unpaid, and the shortfall falls on the seller's receipts.
   - **Reading:** the "cloth margin" is then nearly the whole cloth value (5.7 d a lb) rather than value added (3.0 d), and sellers lose about 2.3 d a lb on mill-bound wool. This is a payment-order effect, not a price. It does not set the pace (capacity does), but it moves coin from wool sellers to mill towns. **I have not changed it.**
4. **Prices.** Under the capacity rule, a higher cloth price (Munro's rise after 1470) would not raise the share directly, since every mill is already full.
   - **Reading:** it would act only through the margin, if the margin raised mill towns' W. Point 2 suggests that channel is weak.
   - The model's wool fall finishes in 1400, the record's by the 1340s. That matters only for 1350–70, before capacity binds.

`cloth_lb_per_w` was not tuned.
