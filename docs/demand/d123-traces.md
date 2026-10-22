# D123: the four traces before D2

Probes:
- `hinterlands specialists`, `ironprice` (now split by port reach), `outside` and `stores`.
- The tools are run at the D1 default (golden `4a9737eafc305561`).
- The data is in `target/d2/`.

## 1. Map 1's specialists, by good: iron mines, not salterns
40 seeds at year 500 (1600). Specialists by kind:

| Kind | Count | Median people | p90 | Max |
| --- | --- | --- | --- | --- |
| Mine (iron) | 129 | 1,100 | 2,362 | 3,437 |
| Saltern | 21 | 25 | 55 | 78 |
| Quarry | 2 | 32 | 46 | 46 |

- **Every specialist of 500 or more people is an iron mine** (125 of 125).
- **The salterns stay small.** The D1 rise in salt demand did not make Droitwich-like salt towns.
- **The mines grow on their price, not their volume.** A large mine sells about 50–80 units of iron a year. It receives 600–1,100 in coin, since its supply price is 20–30 (trace 2), and that coin grows its market.
  - Example: map 1 seed 33, year 500. The mine has 3,437 people and a W of 2,496. It sells 81 units at a supply price of 29.5.
- They stay specialists because goods remain more than a quarter of their inflow (`specialist_share`, D44).

**Per D123, large iron sites are not acceptable.** Their cause is trace 2.

## 2. The iron price spikes: neither a solver defect nor a supply cut, but a missing mechanism
- **The 16–21 run-years are the visible tip of a systematic split.**
  - Goods trade only between reach partners, places within `c_max` of each other on the cost grid (`reach.rs`). Nothing is resold onward.
  - A place with no port among its partners can buy salt and iron only from the region's own sites in its reach.

Over 40 seeds a map (`target/d2/ironprice40.txt`):

| Map | Runs with portless markets | People in portless places (median) | Iron price there (median over runs), 1300 / 1400 / 1500 / 1600 | Mine people in portless places |
| --- | --- | --- | --- | --- |
| 1 | 40/40 | 0.30–0.36 | 8.2 / 15.1 / 16.0 / 24.5 | 100% |
| 2 | 40/40 | 0.25–0.33 | 10.5 / 18.6 / 21.6 / 30.5 | 69–86% |
| 3 | 40/40 | 0.18–0.31 | 6.9 / 11.9 / 8.3 / 13.9 | 43–61% |

- **The region-wide median** over all markets goes over 2 only when portless places are most of the markets that year. That happened in 6 + 10 run-years on map 1 (1400, 1500), 3 + 1 on map 2 and 1 + 1 on map 3.
- **Before D1 this was hidden.** Iron demand was calibrated (D71) to what the coppiced mines could supply, so a portless district paid 1–3.
- **D1 raised demand about 40 times:** plough irons and shoeing at Langdon's costs, against a supply that is capped by coppice in exactly the districts the port cannot reach.
- **The same wall will stop D2's export cloth** from any portless district.
- Wool already reaches the ports by up to two hops (direct, or through one partner; `pasture.rs`). Goods do not.

**Options, for the designer:**
- **(a) Port carriage beyond reach for the port goods.** Each settlement gets an arc to its nearest port for salt and iron (and later cloth) at the full travel cost. The port's distance field is computed once per cost refresh; ports are few, so it is cheap. Carriage is charged as now, at τ/ρ per unit of cost, so iron (ρ 8) carries far and stone does not.
- **(b) The wool rule for goods.** Up to two hops, direct or through one partner, as wool already does.
- **(c) Resale:** a market's buying price becomes a supply price to its partners (a network equilibrium). This is the most faithful, and the most work and run time.
- **My recommendation is (a).** It is the smallest change and matches how iron and salt actually moved inland: by chains of carriers paid by the mile. The price inland is then the port price plus carriage, so a mine competes with that, not with nothing.

## 3. Market stores: not emptied; the measure lost its sample
**Validate's carry-over** is the mean store held in years when the regional grain price is below 0.8 of its 21-year trend, over a mean harvest. It reads 0 when there are no such years.

At the D1 default against the same runs with `demand_stocks` off (3 seeds a map; `hinterlands stores`):

| | D1 off | D1 on |
| --- | --- | --- |
| Years below 0.8 × trend, per run | 3–15 | 0–3 |
| Lowest price / trend | 0.59–0.78 | 0.74–0.84 |
| Stores held, mean over all years (of a mean harvest) | 0.034–0.054 | 0.025–0.047 |
| Stores held, peak | 0.37–0.52 | 0.30–0.45 |

- **The stores still fill and empty.** They hold about 15–25% less than with D1 off.
- **The carry-over reads 0 because cheap years have almost gone.** With iron and salt from the ports and wool exports up, the regional price falls less far below its trend. Most runs have no year below 0.8 × trend left to average over.
- **The order was tested and is not the cause.** A build that ran the grain market (and its stores) before the goods was tried on 10 seeds a map:
  - carry-over stayed 0.00;
  - outside trade broke (1 / 1 / 0 of 10), with the Clark checks and recovery.
  - The switch was removed.
- **What D1 did do: the market purse binds before the margin.** In the cheap market-years of map 1 seed 1, buying was limited by the purse in 69% of cases, by the margin in 25% and by room in 6%. Goods take everything above the food bill (D93) before the grain market opens.
- **For the designer:**
  - read stores as the mean and peak held over all years, or over the cheapest decile of years relative to trend, rather than below a fixed 0.8;
  - or decide whether the stores should have a claim on the purse ahead of the goods.

## 4. Map 3's five outside-trade failures: all over the ceiling, through wool
Year 500, 40 seeds (`hinterlands outside`):

| | Failing (5) | Passing (35) |
| --- | --- | --- |
| Share | 0.301–0.348 (median 0.319) | median 0.263 |
| Grain to ports / denominator | 0.002 | 0.005 |
| Imports (luxuries, salt, iron) / denominator | 0.070 | 0.061 |
| Wool / denominator | 0.251 | 0.194 |

- All five are above 30%, by 0.001–0.048. The difference is wool (+0.057). Imports add +0.009.
- Salt and iron from the ports are no higher in the failing runs (7,000 units against 7,800).
- D1 raised wool exports about 25% (the Hume adjustment), so its effect on this check runs through wool.
- The fallow flock (D2 step 1) will add wool, so this check will be tighter before cloth takes some of it.
