# D174: the re-founded checks on the default. It fails them; stopped before the 40-seed re-read

## Built in validate (D174)
- **Market spacing (gated 8–15 km):** the median nearest-neighbour distance among market places (W ≥ `market_min`, 100) in 1600. The old measure (places over 500) is a reported row, "spacing, over 500".
- **Road persistence (gated ≥ 0.90):** the share of year-300 road cells still road at year 500. Net length change ("road length 300-500") is a reported row. Also reported: "road persist ±1", a year-300 road cell counted as kept if it or a neighbour is road in 1600.
- **D120's no-worse entry** for road length on map 1 is removed, since that check is no longer gated.

## The default (D161) under them: 3 seeds a map (exploration, not a gate; the 40-seed re-read was not run)
| | Map 1 | Map 2 | Map 3 |
| --- | --- | --- | --- |
| **market spacing, market places** | **1/3**, median 16.1 km | 3/3, 11.7 | 3/3, 14.4 |
| spacing, over 500 (old) | 2/3, 13.0 | 3/3, 12.4 | 2/3, 14.1 |
| **road persistence (≥ 0.90)** | **0/3, 0.558** | **0/3, 0.590** | **0/3, 0.555** |
| road persist ±1 (reported) | 0.780 | 0.803 | 0.786 |
| road length change 300–500 (old) | 3/3, 0.029 | 2/3, 0.049 | 3/3, 0.064 |
| **recovery by 500 (0.75–1.15, D168)** | **1/3**, 1.29 | **1/3**, 1.23 | **2/3**, 1.15 |

**The default fails road persistence on every map, and three seeds are enough to show it:**
- 0.56–0.59 of the 1400 road cells are still road in 1600;
- 0.78–0.81 if a road shifted by one cell counts as kept;
- net length changes by only 3–6%.

**The network keeps its size and moves.** This is v3(b)'s rule as designed (D54):
- every `road_interval` the top trading pairs' paths are raised to full;
- cells under `road_keep_load` lose `road_decay` (0.05 a step, about 200 years to nothing).

v3 already reported "share of road cells abandoned: median 0.42" (validate's v3 row).

**The default also fails `recovery by 500`** under D168's band. Its 1600 stands at about 1.2–1.3 × the peak, above 1.15. **This is an error of mine at D168:** I re-founded the band for the sourced calendar's gate and did not re-read the default. **`just validate` on the default fails since D168.**

## What this means
1. **Road persistence at 0.90 cannot pass with the model's roads as they are.** Its routes shift with the trading pairs and the decay rule. The record's persistence (Dalgaard) is of routes. The model's 1-km cells move even where the route does not, and ±1 cell recovers only about 0.2 of the 0.44 lost.
   - **Options:**
     - (a) persistence measured on routes, for example ±2 cells or on the graph of road segments between places;
     - (b) a lower threshold, with a source for the record's medieval road churn (*to find*);
     - (c) a change to the road rule: slower decay, or keeping a road while either end is a market. That is a mechanism.
2. **Market spacing on market places fails map 1 on the default** (16.1 km). There are fewer market places than places over 500 there.
3. **Recovery by 500:** the default (D108 calendar) overshoots the re-founded band. Either the band is held and the default fails until the sourced calendar is adopted, or the gate's band is applied only with `plague_sourced`. **I'd suggest the first:** the default's 1.3 is the known overshoot, and the sourced calendar fixes it to about 1.03.

## Stopped, per D174 ("if the default fails a new measure, stop and report")
- **Validate now carries D174's checks.** `just validate` fails on the default: road persistence (all maps), market spacing (map 1) and recovery (D168).
- No model change; golden `5de3d525bb23a427`.
