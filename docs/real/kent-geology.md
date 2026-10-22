# Kent geology (D116)

- **Source:** BGS Geology 625k (DiGMapGB-625), bedrock and superficial layers, by the BGS ArcGIS REST service. Open Government Licence: "Contains NERC materials © NERC 2026." Fetched and rasterised by `tools/real/fetch_geology.py` to `data/real/kent/geology.asc`. Each cell centre is classed:
  - **chalk** (1,777 cells): Chalk bedrock with no superficial cover, or only clay-with-flints;
  - **marsh and alluvium** (1,258 cells): alluvium, marine, tidal or estuarine deposits, or peat. This includes river alluvium inland as well as Romney Marsh and the Thames and Medway marshes;
  - **other**.
- **Yield multipliers** (`geology_chalk`, `geology_marsh` in `MapParams`; approximate, recorded as calibrations):
  - **chalk ×0.6:** thin rendzina under sheep-and-corn husbandry gave about two-thirds of lowland grain yields (Campbell 2000);
  - **marsh ×0.35:** undrained marsh and alluvium were grazed rather than ploughed before drainage.
- **Run setting:** `data/real/kent/map-geology.json` is map-bridges plus the geology layer. Generated maps are unchanged, and so is the golden.

## Pre-registered: pasture's share of used land in 1450, Downs and Marsh above the Weald and lowland. Met.

| Region (3 seeds, median) | Pasture / used land |
| --- | --- |
| Chalk (by geology) | 0.40 |
| Marsh and alluvium (by geology) | 0.53 |
| Weald and lowland (other geology) | 0.00 |
| North Downs (lat/lon region, `tools/wool/kent.py`) | 0.53 |
| Romney Marsh (lat/lon region) | 0.60 |
| Weald (lat/lon region) | 0.10 |
| Rest | 0.15 |

Before the layer, Kent had 1–4 pasture cells in all.

## Kent's towns against v7 (20 seeds, scored against the 70 towns of c.1300; `docs/real/kent-geology/scores.md`)

| | v7 (map-bridges) | With geology |
| --- | --- | --- |
| Largest place near Gravesend | 14/20 | 15/20 |
| Largest place's share | 0.30 | 0.28 |
| AUC, all towns / 1,000 or more / under 1,000 | 0.53 / 0.57 / 0.50 | **0.56** / 0.54 / **0.57** |
| Real towns found within 5 km: river / coast / inland | 57% / 34% / 25% | 53% / 40% / **34%** |
| Model top places on a river / coast / inland (real 26 / 9 / 66%) | 51 / 21 / 28% | 42 / 14 / **44%** |
| Mean nearest-neighbour spacing (real 8.6 km) | 7.5 km | 7.4 km |

- **The geology moves the model's places inland and finds more of the small inland towns:** AUC under 1,000 goes from 0.50 to 0.57, and inland recall from 25% to 34%.
- **Large towns score a little lower** (0.57 → 0.54).
