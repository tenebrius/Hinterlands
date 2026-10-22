# Real-terrain scores: kent

Interior only: everything within 10 km of the map edge left out.

## All 58 real towns against the model's top 58

20 seeds, final year; random baselines drop the same 58 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.38 | 0.38 | 0.32 | 0% |
| Real towns with a model place within 10 km | 0.84 | 0.83 | 0.69 | 0% |
| Model places with a real town within 5 km | 0.42 | 0.42 | 0.40 | 0% |
| Model places with a real town within 10 km | 0.91 | 0.91 | 0.88 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.41 | 0.32 | 0.41 | 5% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.15 | -0.00 | 0.20 | 20% |

## The 23 real towns of 1,000 or more against the model's top 23

20 seeds, final year; random baselines drop the same 23 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.23 | 0.17 | 0.20 | 0% |
| Real towns with a model place within 10 km | 0.57 | 0.47 | 0.48 | 10% |
| Model places with a real town within 5 km | 0.22 | 0.17 | 0.20 | 0% |
| Model places with a real town within 10 km | 0.55 | 0.49 | 0.54 | 5% |
| Real top 10 with a model top-15 place within 10 km | 0.41 | 0.33 | 0.42 | 5% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.07 | -0.01 | 0.11 | 15% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (58) | 0.53 | 0.46 | 0.20 | 0.16 |
| 1,000 or more (23) | 0.61 | 0.55 | 0.30 | 0.16 |
| Under 1,000 (35) | 0.48 | 0.40 | 0.14 | 0.16 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 2; any top-58 place within 10 km: 20. Median distance from the model's largest place to London (City): 38 km. Largest place's share of the region's people (median): 0.30.

Nearest real town to the model's largest place, by seeds: Gravesend 14, Milton Regis 5, Maldon 1.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 17 | 29% | 33.8 | 58% | 59% |
| coast | 4 | 7% | 9.4 | 16% | 48% |
| inland | 37 | 64% | 14.8 | 26% | 27% |

Inland places over 500 people, nearest other place over 500 (median): real 10.6 km (22 towns); model 13.2 km (median over seeds; 2 such places per seed).

Mean nearest-neighbour spacing of the 58 places: real 8.5 km, model 7.3 km, random land 6.7 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.38 | 0.81 | 0.36 | 0.91 | 0.40 | 0.03 | Ulaport 14829 |
| 2 | 0.38 | 0.78 | 0.41 | 0.91 | 0.70 | 0.13 | Kiningmouth 16040 |
| 3 | 0.29 | 0.88 | 0.34 | 0.97 | 0.30 | 0.30 | Kiningmouth 13999 |
| 4 | 0.40 | 0.78 | 0.48 | 0.91 | 0.30 | 0.18 | Sutelmouth 16204 |
| 5 | 0.41 | 0.83 | 0.40 | 0.90 | 0.40 | 0.06 | Rayingford 14445 |
| 6 | 0.41 | 0.83 | 0.45 | 0.90 | 0.40 | 0.00 | Bareley 16728 |
| 7 | 0.34 | 0.83 | 0.33 | 0.90 | 0.30 | 0.26 | Evermouth 16349 |
| 8 | 0.41 | 0.86 | 0.43 | 0.91 | 0.50 | 0.15 | Toraney 16295 |
| 9 | 0.36 | 0.84 | 0.41 | 0.88 | 0.30 | 0.07 | Toraney 13380 |
| 10 | 0.41 | 0.88 | 0.40 | 0.88 | 0.60 | 0.25 | Yarhurst 21324 |
| 11 | 0.45 | 0.91 | 0.48 | 0.93 | 0.60 | 0.10 | Toraney 15622 |
| 12 | 0.33 | 0.81 | 0.43 | 0.95 | 0.30 | 0.16 | Rayelport 15606 |
| 13 | 0.43 | 0.88 | 0.52 | 0.95 | 0.60 | 0.16 | Evermouth 16866 |
| 14 | 0.29 | 0.78 | 0.33 | 0.83 | 0.20 | 0.18 | Mereport 6655 |
| 15 | 0.33 | 0.83 | 0.40 | 0.88 | 0.60 | 0.23 | Kiningmouth 18270 |
| 16 | 0.40 | 0.81 | 0.45 | 0.91 | 0.30 | 0.19 | Selanden 11206 |
| 17 | 0.38 | 0.90 | 0.38 | 0.91 | 0.60 | 0.15 | Kiningmouth 16508 |
| 18 | 0.38 | 0.84 | 0.40 | 0.88 | 0.30 | 0.17 | Falanmouth 16068 |
| 19 | 0.38 | 0.88 | 0.47 | 0.88 | 0.40 | 0.10 | Hartingham 18174 |
| 20 | 0.41 | 0.81 | 0.47 | 0.93 | 0.20 | 0.14 | Holmouth 15045 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 0% |
| Canterbury | 10000 | city | 30% |
| Sandwich | 4000 | large_town | 100% |
| Winchelsea | 3500 | large_town | 90% |
| Dover | 3000 | large_town | 15% |
| New Romney | 2500 | town | 90% |
| Rye | 2500 | town | 90% |
| Lewes | 2500 | town | 10% |
| Faversham | 2000 | town | 85% |
| Hythe | 2000 | town | 45% |
| Hastings | 2000 | town | 0% |
| Maidstone | 2000 | town | 65% |
| Maldon | 2000 | town | 95% |
| Rochester | 1900 | town | 15% |
| Lydd | 1500 | town | 50% |
| Chelmsford | 1500 | town | 95% |
| Battle | 1200 | town | 0% |
| Tonbridge | 1200 | town | 90% |
| Dartford | 1200 | town | 90% |
| Hertford | 1200 | town | 10% |
| Ware | 1200 | town | 15% |
| Croydon | 1000 | town | 15% |
| Folkestone | 1000 | town | 55% |
| East Grinstead | 800 | small_town | 10% |
| Milton Regis | 800 | small_town | 10% |
| Barking | 800 | small_town | 25% |
| Waltham Abbey | 800 | small_town | 65% |
| Fordwich | 600 | small_town | 25% |
| Tenterden | 600 | small_town | 0% |
| Ashford | 600 | small_town | 15% |
| Sevenoaks | 600 | small_town | 60% |
| West Malling | 600 | small_town | 40% |
| Cranbrook | 600 | small_town | 0% |
| Gravesend | 600 | small_town | 45% |
| Romford | 600 | small_town | 5% |
| Wye | 500 | small_town | 5% |
| Witham | 500 | small_town | 50% |
| Brentwood | 500 | small_town | 20% |
| Bletchingley | 500 | small_town | 0% |
| Westerham | 400 | small_town | 10% |
| Charing | 400 | small_town | 10% |
| Wrotham | 400 | small_town | 10% |
| Lenham | 400 | small_town | 5% |
| Elham | 400 | small_town | 10% |
| Wingham | 400 | small_town | 80% |
| Bromley | 400 | small_town | 50% |
| Robertsbridge | 400 | small_town | 75% |
| Hailsham | 400 | small_town | 30% |
| Mayfield | 400 | small_town | 40% |
| Cuckfield | 400 | small_town | 85% |
| Epping | 400 | small_town | 45% |
| Billericay | 400 | small_town | 35% |
| Rayleigh | 400 | small_town | 15% |
| Rochford | 400 | small_town | 5% |
| Harlow | 400 | small_town | 40% |
| Chipping Ongar | 400 | small_town | 25% |
| Hoddesdon | 400 | small_town | 75% |
| Sawbridgeworth | 400 | small_town | 30% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 106 | 60 | 90% | Wingham | 11 | 3 |
| 29 | 89 | 80% | East Grinstead | 11 | 50 |
| 27 | 104 | 80% | Mayfield | 12 | 32 |
| 105 | 11 | 70% | Maldon | 39 | 0 |
