# D3 wealth: built (off); 9-run exploration

## Built per D158, behind `wealth_stocks` (off; golden `fc16de69c1855b30` unchanged)
**A new system, `systems/wealth.rs`,** runs after market. Each place's real income per head (`wealth_y`), in grain units, a ten-year average (`wealth_years` 10), is:
- the food its own fields give its people, up to their need;
- plus the coin its grain sales (`grain_earned`, now written by marketplace), wool (`wool_got`) and cloth margin earned, over its grain price.

**Implementer's reading, departing from the note:**
- The note counted earned coin only.
- With coin only, a subsistence village has an income near 0, and its cloth and iron demand would collapse.
- Own food up to need is counted as income in kind.

**The reference y₀** is the region's people-weighted mean of 1300–1339, as D158 says.

**The multiplier** is (y / y₀)^e, from 1340, with e = 1.0 (`wealth_elast`; one e for all, named). It scales:
- home cloth per adult (the wool market's home demand);
- household iron (`household_iron_d`);
- building stone (repair and growth).

Salt, plough iron and site iron are unchanged.

**Houthakker (1957)** was not checked against the original: its category figures are behind JSTOR, and no secondary source quoting them was found. So the named fallback e = 1.0 is used (D158).

## The record's check: real wages against 1300–1339 (Clark, `Wages 2014.xlsx`, decadal)
| Series | 1350 | 1400 | 1450 | 1480 | 1500 | 1550 | 1590 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Farm labourers | 1.77 | 2.35 | **2.75** | 2.35 | **2.34** | 1.62 | 1.37 |
| Building labourers | 1.26 | 1.88 | **2.30** | 2.03 | **2.05** | 1.47 | 1.14 |
| Building craftsmen | 1.18 | 1.47 | 1.72 | 1.53 | 1.57 | 1.12 | 0.88 |

The doubling c. 1300 → 1450–1500 holds for labourers (2.0–2.75). Phelps Brown & Hopkins' builders are in Clark's building series.

## Exploration: 9 runs (maps 1–3 × seeds 1–3), `{"wealth_stocks": true}`, `target/d2/rep_d3` (not a gate)
| Decade | Income per head / y₀ | Cloth per adult (yd) | Home wool use / need (lb) | People | Coin | People, D3 off | Coin, D3 off |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1300 | 1.06 | 1.50 | 98.0k / 96.7k | 54.2k | 52.5k | 54.2k | 52.5k |
| 1350 | 1.17 | 1.75 | 89.2k / 87.6k | 43.6k | 70.9k | 43.6k | 73.8k |
| 1370 | **1.24** | 1.86 | 83.6k / 82.3k | 37.5k | 75.9k | 37.7k | 82.0k |
| 1400 | 1.11 | 1.67 | 71.7k / 70.7k | 36.3k | 58.0k | 36.4k | 60.6k |
| 1450 | 1.07 | 1.61 | 64.0k / 63.1k | 33.2k | 51.0k | 33.9k | 52.2k |
| 1500 | **1.02** | **1.54** | 82.0k / 80.9k | 44.8k | 54.1k | 47.0k | 61.9k |
| 1550 | 0.91 | 1.36 | 111.9k / 110.3k | 67.1k | 97.7k | 71.1k | 98.6k |
| 1590 | 0.87 | 1.30 | 126.8k / 125.1k | 81.1k | 115.8k | 83.7k | 111.7k |

**The cloth share of wool leaving** (D3, 9 runs):
- 1400 0.26, 1450 0.18, 1500 0.32, 1550 0.43, 1600 0.43;
- D144: 1500 0.35, 1550 0.59, 1600 0.53.

**The checks (reported):**
- **Cloth per adult in 1500 against Dyer's 3 yd:** 1.54. **Missed.**
- **Real income per head against the doubling c. 1300 → 1450–1500:** 1.07 (1450), 1.02 (1500), at most 1.24 (1370). **Missed.**

**Why (fact, from the measure; the reading is mine):**
- Own food is capped at need, and cultivation claims fields until the need is covered (`cult_margin`).
- So output per head stays near need by construction. After the plague the model abandons fields rather than giving each survivor more land.
- Income per head rises only through coin from sales, and grain is cheap after 1348.
- The record's doubling of real wages came from more land and capital per worker. **The model's cultivation rule does not let output per head rise.**
- **Reading:** no elasticity can turn a 1.0–1.24 rise in income into three yards. D3's check needs the model's income per head to rise first, which is a question about cultivation and labour, not about stocks.
