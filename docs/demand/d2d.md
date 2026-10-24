# D1b + D2 without duties (D132): gate failed on one check; stopped

## Built (D132)
- **No duties.**
  - Wool sells at the English price (Lloyd's £5 a sack, Clark's fall from 1350), and export cloth at Munro's customs valuation, 5.7 d a lb.
  - `duties_out` and the duty parameters are removed. **Rule 5:** no duty remains in the code (`grep` finds none outside comments).
- **Making cost:** 2.7 d a lb (Munro, Ypres 1500; Leuven 1434 would give 2.1).
- **Home wool elasticity:** 0.3 (calibration). Home use is now 1.01–1.02 of the need in every decade.
- **Capacity:** `cloth_lb_per_w` 2.5, the largest value tried that keeps cloth at or under 0.10 of the wool leaving in 1350–59.
  - No labour source was found.
  - Trials on 9 runs, median 1350–59 share: 0.1 → 0.005; 0.3 → 0.015; 1 → 0.040; 2 → 0.078; **2.5 → 0.089**; 2.8 → 0.109; 3 → 0.108; 10 → 0.254; 30 → 0.523.
- **Kept:** port reach, the fallow flock, the entropy split, the capacity rule, and the switches.

## Gate: 40 seeds. **FAILED on one check**
`target/d2/gate_d2d_40.txt`.

| Check | Map 1 | Map 2 | Map 3 | D1b alone |
| --- | --- | --- | --- | --- |
| over 2,000 | 32/40 | ≥ 38 | **22/40** | 31 / 37 / 25 |
| market spacing | 37/40 | 37/40 | 34/40 | 38 / 40 / 34 |
| outside trade | ≥ 38 | ≥ 38 | ≥ 38 | (30% ceiling then) |
| **runs passing every check** | **23/40** | **36/40** | **14/40** | 15 / 10 / 2 |

- **Every other check is at 34/40 or better on every map.**
- **Map 3's only failure is over 2,000.** Of its 26 failing runs, 18 fail it, 14 of them as their only failure.
- **Exports a head c. 1300:** 7.6 / 8.4 / 8.7 d.
- **Stores** (cheapest tenth / all / peak): 0.068–0.085 / 0.024–0.030 / 0.31–0.35.

## Pre-registered (D132), reported only
| | Result (medians of 9 runs unless said) | Met |
| --- | --- | --- |
| Wool leaving (raw and in cloth) in 1400–1500 at least half its 1300 level | 1300: 101k lb; 1400: 131k; 1450: 97k; 1500: 83k | **yes** |
| Coin stock after 1400 within 25% of D1b's | D2/D1b 1400 1.17, 1450 1.03, 1500 1.22, 1550 1.59, 1600 1.69 | to 1500 yes; **from 1550 no** (above) |
| Map 3's 1600 population within 10% of D1b's (10 seeds) | 78.9k against 88.8k (−11%) | **no**, by 1 point |
| Home wool at most 1.5 × need in every decade | 1.01–1.02 | **yes** |
| Cloth share 0.2–0.5 in 1390–1410 | 0.26, 0.28, 0.25 | **yes** |
| 0.35–0.65 in 1440–60 | 0.19, 0.20, 0.22 | **no** |
| 0.55–0.85 in 1490–1510 | 0.34, 0.35, 0.33 | **no** |
| Raw wool exported in every decade, at least a quarter of its 1300 volume through 1450 | 1300: 89k; 1350: 172k; 1400: 95k; 1450: 75k | **yes** |

**The cloth path:**
- 0.13–0.14 in 1300–40;
- 0.07 in 1350, as the start was calibrated;
- 0.14–0.28 from 1360 to 1410;
- about 0.20 through the 1400s;
- 0.33–0.40 from 1490.
It rises, as the record does, but at about half the record's pace after 1420.

## Over 2,000, by map (D132), 10 seeds each
`hinterlands diag`, `target/d2/diag2/`:

| Map | Build | People 1340 / 1450 / 1600 | Share in places over 2,000, 1340 / 1600 | Over 2,000 in 1600 (median; runs <5) | Spacing km | Top 8 in 1600 (median by rank) |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | D1 | 63,953 / 35,238 / 98,268 | 0.245 / 0.341 | 6; 1 | 10.4 | 8,599, 6,920, 5,616, 4,654, 3,064, 2,410, 2,096, 1,722 |
| 1 | D1b | 62,722 / 35,820 / 93,044 | 0.251 / 0.355 | 6; 1 | 11.7 | 8,694, 7,216, 6,324, 4,617, 3,783, 2,570, 1,504, 1,212 |
| 1 | D1b + D2 | 65,378 / 36,351 / 90,248 | 0.232 / 0.354 | 6; 1 | 11.5 | 10,668, 6,779, 4,866, 3,638, 2,866, 2,243, 1,659, 1,344 |
| 2 | D1 | 62,218 / 34,638 / 91,743 | 0.211 / 0.352 | 7; 1 | 12.3 | 8,539, 6,764, 5,782, 4,427, 3,164, 2,366, 2,103, 1,546 |
| 2 | D1b | 61,822 / 33,132 / 87,254 | 0.205 / 0.351 | 6; 1 | 11.1 | 9,980, 5,910, 4,965, 3,766, 2,678, 2,284, 1,914, 1,610 |
| 2 | D1b + D2 | 61,714 / 32,434 / 84,707 | 0.222 / 0.363 | 6; 0 | 12.1 | 9,720, 6,319, 5,046, 3,336, 3,135, 2,585, 1,858, 1,388 |
| 3 | D1 | 60,400 / 30,016 / 89,874 | 0.263 / 0.333 | 6; 1 | 11.4 | 9,099, 6,162, 4,624, 3,762, 3,081, 2,025, 1,620, 1,376 |
| 3 | D1b | 62,176 / 32,726 / 88,832 | 0.217 / 0.349 | 6; 3 | 11.8 | 9,638, 7,196, 4,476, 3,030, 2,291, 1,955, 1,626, 1,346 |
| 3 | D1b + D2 | 60,162 / 29,742 / 78,870 | 0.268 / 0.366 | 5; 4 | 13.8 | 12,025, 6,998, 4,086, 3,119, 2,166, 1,650, 1,317, 1,204 |

**Read:**
- **Map 3 does not lose urban people.** The share in places over 2,000 in 1600 is 0.366, against 0.349 for D1b and 0.333 for D1.
- **It loses the spread.**
  - The largest place is bigger (12.0k against 9.1–9.6k).
  - Ranks 5–8 are smaller (2,166 / 1,650 / 1,317 / 1,204 against 3,081 / 2,025 / 1,620 / 1,376 for D1).
  - The count over 2,000 falls to a median of 5, with 4 of 10 runs below 5.
- **Population in 1600** is 78.9k against D1's 89.9k and D1b's 88.8k.
- **Maps 1 and 2** keep their people (90.2k and 84.7k against D1b's 93.0k and 87.3k) and their counts.

## Tables (medians of 9 runs)
| Year | People | Clip lb | Home use lb (need) | To mills lb | To ports lb | Cloth share of wool leaving | Cloth made at W 100-2,000 | Cloth towns | Over 2,000 (median / max of runs) | Median / largest cloth town |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1150 | 1,743 | 4,690 | 3,087 (3,054) | 58 | 1,536 | 0.04 | -0.00 | 24 | 0 / 0 | 47 / 66 |
| 1200 | 5,796 | 32,848 | 10,413 (10,215) | 568 | 21,711 | 0.03 | -0.00 | 52 | 0 / 0 | 55 / 155 |
| 1250 | 18,630 | 116,896 | 33,315 (32,730) | 4,640 | 78,416 | 0.06 | 0.70 | 86 | 0 / 0 | 70 / 1,145 |
| 1300 | 50,508 | 188,815 | 91,518 (90,246) | 12,338 | 89,077 | 0.13 | 0.73 | 99 | 2 / 4 | 156 / 3,356 |
| 1350 | 40,059 | 253,792 | 71,571 (70,380) | 11,677 | 171,992 | 0.07 | 0.63 | 100 | 2 / 3 | 127 / 3,332 |
| 1400 | 37,895 | 186,176 | 67,516 (66,660) | 36,698 | 94,732 | 0.28 | 0.40 | 102 | 3 / 5 | 59 / 5,356 |
| 1450 | 32,101 | 155,863 | 57,560 (56,836) | 21,422 | 75,375 | 0.20 | 0.46 | 103 | 2 / 5 | 47 / 4,260 |
| 1500 | 38,898 | 153,752 | 70,140 (69,354) | 30,608 | 51,913 | 0.35 | 0.26 | 101 | 3 / 4 | 63 / 6,245 |
| 1550 | 65,553 | 237,904 | 119,023 (117,198) | 51,780 | 78,808 | 0.34 | 0.22 | 113 | 3 / 6 | 100 / 8,451 |
| 1600 | 83,047 | 308,480 | 149,931 (150,063) | 69,540 | 99,411 | 0.39 | 0.27 | 113 | 5 / 6 | 185 / 9,306 |

Cloth share of wool leaving, by decade 1300-1600 (median of runs): 1300 0.13, 1310 0.14, 1320 0.14, 1330 0.14, 1340 0.13, 1350 0.07, 1360 0.14, 1370 0.18, 1380 0.19, 1390 0.26, 1400 0.28, 1410 0.25, 1420 0.21, 1430 0.26, 1440 0.19, 1450 0.20, 1460 0.22, 1470 0.21, 1480 0.20, 1490 0.34, 1500 0.35, 1510 0.33, 1520 0.34, 1530 0.33, 1540 0.33, 1550 0.34, 1560 0.36, 1570 0.37, 1580 0.37, 1590 0.40, 1600 0.39

Cloth-making places over 2,000, by decade 1250-1340 (median / max of runs): 1250 0/0, 1260 0/0, 1270 1/1, 1280 1/2, 1290 2/4, 1300 2/4, 1310 3/4, 1320 2/4, 1330 2/5, 1340 3/5

| Year | Coin stock | Price level | Grain to ports | Wool in | Cloth in | Imports out | Luxuries out | Wear |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1150 | 10,495 | nan | 1,331 | 70 | 5 | 276 | 924 | 106 |
| 1200 | 24,284 | 0.50 | 1,936 | 994 | 45 | 896 | 1,666 | 245 |
| 1250 | 48,606 | 0.52 | 1,747 | 3,590 | 367 | 2,541 | 2,246 | 491 |
| 1300 | 53,769 | 0.93 | 366 | 4,079 | 977 | 3,659 | 1,211 | 543 |
| 1350 | 63,481 | 0.79 | 559 | 7,875 | 924 | 4,157 | 2,201 | 641 |
| 1400 | 74,092 | 1.07 | 231 | 3,275 | 2,905 | 3,836 | 1,624 | 748 |
| 1450 | 51,755 | 1.06 | 240 | 2,588 | 1,696 | 2,833 | 1,135 | 523 |
| 1500 | 51,018 | 1.08 | 245 | 1,783 | 2,423 | 2,971 | 832 | 515 |
| 1550 | 67,135 | 1.13 | 204 | 2,706 | 4,099 | 4,694 | 1,419 | 678 |
| 1600 | 84,914 | 1.40 | 109 | 3,414 | 5,505 | 5,278 | 2,452 | 858 |
