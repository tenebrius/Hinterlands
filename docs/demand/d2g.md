# D139 step 1: the mills' payment fix, alone; outside trade on output. Gate passed; paused after it

## Built (D139)
- **Mills pay the seller's clearing price out of the cloth proceeds** (D137 step 1). Each mill pays exactly its sellers' price, and its market keeps the cloth value less the wool.
- **The later cloth price (`cloth_price_late`) is built but off by default.** Step 3 (its gate) was not run: development was paused (designer, after D139).
- **Outside trade is gated as exports over output (D139),** at 1–35%, in the final year (1600).
  - Exports: what the ports paid for grain, wool and cloth.
  - Output: the harvest at each place's grain price, goods sold at their price, the wool clip at the port price, and cloth at its value.
  - The share of market inflow is reported, with its count inside the old 3–50%.
  - **Implementer's reading:** the wool that goes into cloth is counted in both the clip and the cloth, as D139 lists them.

## Gate: 40 seeds, defaults. **PASSED**
`target/d2/v139_step1.txt`.

| | Map 1 | Map 2 | Map 3 | D136 |
| --- | --- | --- | --- | --- |
| exports / output, median (1–35%) | 0.066, 40/40 | 0.078, 40/40 | 0.073, 40/40 | not measured |
| outside trade, old share (reported; inside 3–50%) | 0.425, 39/40 | 0.457, 39/40 | 0.432, 40/40 | 0.404 / 0.422 / 0.411, 40 each |
| lowest gated checks | spacing 33/40, pasture 1450>1340 34/40, top-10 vs off 35/40 | settlements 39/40, spacing 39/40 | spacing 34/40, stability 37/40 | |
| runs passing every check | 23/40 | 37/40 | 25/40 | 25 / 36 / 28 |
| over 2,000 in 1600, median (range) | 6 (3–9) | 7 (4–9) | 5 (3–8) | 6 / 6 / 5 |
| urban share in 1600, median | 0.339 | 0.354 | 0.360 | 0.356 / 0.369 / 0.373 |

- **The payment fix is now the default.** Golden `2e6856056e73a1c6`, the same in the browser (`?selftest=500`, 98.7 ms a year).
- **Pre-registered** (exports over 1600 output between 5% and 25% on every map): **met in the medians** (0.066 / 0.078 / 0.073). `validate` does not print the per-run range.
- **Pre-registered** (cloth share about 40 years late): this belongs to step 3, which was not run.

## Report tables, step 1 (medians of 9 runs; `target/d2/rep139a`, `pace139a`)
**Cloth share of wool leaving, by decade:**
- 1390 0.24, 1400 0.26, 1410 0.22, 1420 0.17, 1430 0.20, 1440 0.16, 1450 0.15, 1460 0.18, 1470 0.15, 1480 0.16;
- 1490 0.34, 1500 0.34, 1510 0.35, 1520 0.39, 1530 0.41, 1540 0.41, 1550 0.41, 1600 0.41;
- the record: 0.2–0.5 (1390–1410), 0.35–0.65 (1440–60), 0.55–0.85 (1490–1510).

**Mill capacity, use and towns:**
- Capacity binds in every mill from the 1370s, as before.

| Decade | Capacity lb | Mills at capacity / bidding | Mill towns' W | W growth / region's | Margin, d a head of W a year | Growers' wool receipts (coin) |
| --- | --- | --- | --- | --- | --- | --- |
| 1350 | 29,500 | 90 / 102 | 10,000 | +0.39 / +0.31 | 4.8 | 10,780 |
| 1400 | 35,900 | 100 / 100 | 15,500 | −0.10 / −0.09 | 7.7 | 5,300 |
| 1450 | 15,100 | 99 / 99 | 5,920 | +0.06 / +0.00 | 8.6 | 4,270 |
| 1470 | 13,600 | 102 / 102 | 5,620 | −0.03 / −0.02 | 8.1 | 4,320 |
| 1490 | 40,000 | 101 / 101 | 14,500 | +0.08 / +0.08 | 8.6 | 4,080 |
| 1500 | 46,900 | 106 / 106 | 17,800 | +0.09 / +0.09 | 8.6 | 4,390 |
| 1520 | 57,800 | 114 / 114 | 22,600 | +0.08 / +0.09 | 8.6 | 5,350 |
| 1540 | 64,100 | 117 / 117 | 25,200 | +0.05 / +0.05 | 8.4 | 6,520 |

**Coin by channel:**

| Year | Coin stock | Price level | Grain to ports | Wool in | Cloth in | Imports out | Luxuries out | Wear |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1350 | 62,079 | 0.80 | 574 | 7,829 | 1,023 | 4,171 | 2,194 | 627 |
| 1400 | 63,583 | 1.07 | 232 | 2,810 | 3,033 | 3,548 | 1,573 | 642 |
| 1450 | 52,483 | 1.08 | 257 | 2,414 | 1,161 | 2,831 | 1,143 | 530 |
| 1500 | 54,296 | 0.99 | 306 | 1,774 | 3,492 | 3,572 | 1,063 | 548 |
| 1550 | 72,656 | 1.09 | 229 | 2,451 | 5,198 | 5,408 | 1,484 | 734 |
| 1600 | 89,505 | 1.27 | 139 | 3,262 | 6,337 | 5,841 | 2,413 | 904 |

**Not done (paused):**
- step 3, the later cloth price's gate under the new measure;
- the putting-out note.
