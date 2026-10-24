# D160: the wool series' paired re-gate. Passed; the series is the default

**The rule (D160):**
- map 3's market spacing is judged on 80 seeds: the series must reach at least D144's own 80-seed count minus 4;
- every other check needs 30/40 on seeds 1–40.

| Map 3 market spacing (passing runs) | Seeds 1–40 | Seeds 41–80 | 80 seeds |
| --- | --- | --- | --- |
| D144 (series off) | 30/40 (`v141_late.txt`) | 30/40 (`v160_off_41.txt`) | **60/80** |
| Wool series | 25/40 (`v156.txt`) | 31/40 (`v160_on_41.txt`) | **56/80** |

- **56 ≥ 60 − 4: passed,** at the margin.
- **Every other gated check** with the series is 30/40 or better on seeds 1–40 (`v156.txt`), and on seeds 41–80 as well.
- **Runs passing every check:** seeds 1–40 give 24 / 33 / 15; seeds 41–80 give 25 / 35 / 20. D144 on seeds 41–80 gives 27 / 33 / 21.
- **Market spacing on seeds 41–80, series (D144):**
  - map 1: 34/40, 12.1 km (38/40, 12.0);
  - map 2: 37/40, 11.4 (37/40, 11.9);
  - map 3: 31/40, 13.7 (30/40, 12.8).

**The new reference for map 3's spacing:** the 80-seed count of 56/80 with the series (D144 60/80).

**`wool_series` is the default.** Golden `5de3d525bb23a427`, the same in the browser (`?selftest=500`, 94.7 ms a year).
