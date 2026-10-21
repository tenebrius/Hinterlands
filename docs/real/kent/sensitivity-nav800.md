# Real-terrain scores: kent, sensitivity with nav_area 800 km²

## All 70 real towns against the model's top 70

20 seeds, final year; random baselines drop the same 70 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.38 | 0.37 | 0.30 | 10% |
| Real towns with a model place within 10 km | 0.84 | 0.82 | 0.65 | 10% |
| Model places with a real town within 5 km | 0.42 | 0.41 | 0.38 | 0% |
| Model places with a real town within 10 km | 0.86 | 0.90 | 0.85 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.41 | 0.29 | 0.38 | 20% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.09 | -0.03 | 0.20 | 25% |

## The 27 real towns of 1,000 or more against the model's top 27

20 seeds, final year; random baselines drop the same 27 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.20 | 0.15 | 0.17 | 0% |
| Real towns with a model place within 10 km | 0.54 | 0.45 | 0.46 | 20% |
| Model places with a real town within 5 km | 0.20 | 0.15 | 0.18 | 0% |
| Model places with a real town within 10 km | 0.53 | 0.48 | 0.52 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.41 | 0.29 | 0.39 | 20% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.06 | 0.01 | 0.14 | 5% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (70) | 0.56 | 0.47 | 0.20 | 0.15 |
| 1,000 or more (27) | 0.65 | 0.57 | 0.26 | 0.15 |
| Under 1,000 (43) | 0.51 | 0.42 | 0.17 | 0.15 |

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 10 | 14% | 16.4 | 23% | 67% |
| coast | 6 | 9% | 11.6 | 17% | 36% |
| inland | 54 | 77% | 42.0 | 60% | 33% |

Mean nearest-neighbour spacing of the 70 places: real 8.6 km, model 7.3 km, random land 6.9 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.39 | 0.84 | 0.46 | 0.89 | 0.30 | 0.07 | Thornesea 27040 |
| 2 | 0.44 | 0.86 | 0.46 | 0.84 | 0.40 | 0.04 | Ordmouth 20228 |
| 3 | 0.41 | 0.84 | 0.46 | 0.91 | 0.30 | 0.05 | Holenhaven 21945 |
| 4 | 0.37 | 0.83 | 0.46 | 0.81 | 0.40 | 0.08 | Belerport 24526 |
| 5 | 0.33 | 0.84 | 0.40 | 0.90 | 0.40 | 0.03 | Upensea 20017 |
| 6 | 0.29 | 0.80 | 0.37 | 0.83 | 0.40 | 0.19 | Talerport 21375 |
| 7 | 0.36 | 0.81 | 0.36 | 0.83 | 0.10 | 0.18 | Calhaven 27477 |
| 8 | 0.33 | 0.83 | 0.44 | 0.87 | 0.50 | -0.05 | Harnesport 23655 |
| 9 | 0.33 | 0.86 | 0.41 | 0.86 | 0.50 | 0.13 | Ickelwick 18034 |
| 10 | 0.41 | 0.86 | 0.41 | 0.90 | 0.80 | 0.08 | Calelport 14347 |
| 11 | 0.41 | 0.80 | 0.46 | 0.93 | 0.40 | 0.01 | Croeswick 15354 |
| 12 | 0.46 | 0.90 | 0.47 | 0.87 | 0.10 | 0.20 | Bromport 17149 |
| 13 | 0.34 | 0.89 | 0.43 | 0.91 | 0.20 | 0.01 | Harterhaven 17486 |
| 14 | 0.36 | 0.83 | 0.37 | 0.84 | 0.50 | 0.10 | Darhaven 27502 |
| 15 | 0.36 | 0.90 | 0.39 | 0.90 | 0.30 | 0.00 | Wexport 18067 |
| 16 | 0.36 | 0.81 | 0.39 | 0.83 | 0.30 | 0.02 | Crosea 14995 |
| 17 | 0.46 | 0.81 | 0.50 | 0.84 | 0.70 | 0.16 | Brackehaven 12987 |
| 18 | 0.40 | 0.86 | 0.39 | 0.83 | 0.70 | 0.11 | Upelsea 16019 |
| 19 | 0.41 | 0.81 | 0.39 | 0.86 | 0.20 | 0.20 | Cranenwick 20936 |
| 20 | 0.37 | 0.80 | 0.49 | 0.84 | 0.70 | 0.21 | Glenhaven 13287 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 10% |
| Canterbury | 10000 | city | 55% |
| Colchester | 4000 | large_town | 5% |
| Sandwich | 4000 | large_town | 75% |
| Winchelsea | 3500 | large_town | 50% |
| Dover | 3000 | large_town | 45% |
| New Romney | 2500 | town | 70% |
| Rye | 2500 | town | 35% |
| Lewes | 2500 | town | 50% |
| Maidstone | 2000 | town | 70% |
| Hythe | 2000 | town | 35% |
| Hastings | 2000 | town | 35% |
| Maldon | 2000 | town | 75% |
| Faversham | 2000 | town | 50% |
| Rochester | 1900 | town | 85% |
| Lydd | 1500 | town | 50% |
| Chelmsford | 1500 | town | 55% |
| New Shoreham | 1500 | town | 25% |
| Battle | 1200 | town | 0% |
| Tonbridge | 1200 | town | 55% |
| Dartford | 1200 | town | 65% |
| Hertford | 1200 | town | 70% |
| Ware | 1200 | town | 30% |
| Croydon | 1000 | town | 70% |
| Reigate | 1000 | town | 30% |
| Folkestone | 1000 | town | 25% |
| Bishop's Stortford | 1000 | town | 25% |
| Seaford | 800 | small_town | 45% |
| East Grinstead | 800 | small_town | 0% |
| Milton Regis | 800 | small_town | 50% |
| Barking | 800 | small_town | 35% |
| Waltham Abbey | 800 | small_town | 60% |
| Pevensey | 600 | small_town | 65% |
| Fordwich | 600 | small_town | 55% |
| Tenterden | 600 | small_town | 0% |
| Ashford | 600 | small_town | 75% |
| Sevenoaks | 600 | small_town | 35% |
| West Malling | 600 | small_town | 30% |
| Cranbrook | 600 | small_town | 0% |
| Gravesend | 600 | small_town | 50% |
| Romford | 600 | small_town | 0% |
| Coggeshall | 600 | small_town | 55% |
| Braintree | 600 | small_town | 0% |
| Great Dunmow | 600 | small_town | 20% |
| Wye | 500 | small_town | 15% |
| Witham | 500 | small_town | 30% |
| Brentwood | 500 | small_town | 0% |
| Chipping Barnet | 500 | small_town | 0% |
| Bletchingley | 500 | small_town | 0% |
| Westerham | 400 | small_town | 0% |
| Charing | 400 | small_town | 5% |
| Wrotham | 400 | small_town | 0% |
| Lenham | 400 | small_town | 0% |
| Elham | 400 | small_town | 0% |
| Wingham | 400 | small_town | 30% |
| Bromley | 400 | small_town | 45% |
| Robertsbridge | 400 | small_town | 75% |
| Hailsham | 400 | small_town | 85% |
| Mayfield | 400 | small_town | 20% |
| Cuckfield | 400 | small_town | 25% |
| Crawley | 400 | small_town | 45% |
| Epping | 400 | small_town | 20% |
| Billericay | 400 | small_town | 30% |
| Rayleigh | 400 | small_town | 45% |
| Rochford | 400 | small_town | 15% |
| Harlow | 400 | small_town | 70% |
| Chipping Ongar | 400 | small_town | 55% |
| Hoddesdon | 400 | small_town | 90% |
| Sawbridgeworth | 400 | small_town | 65% |
| Hatfield | 400 | small_town | 70% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 104 | 11 | 75% | Colchester | 23 | 2 |
| 2 | 45 | 70% | London (City) | 11 | 10 |
| 59 | 46 | 60% | Rayleigh | 11 | -1 |
| 2 | 59 | 60% | Croydon | 10 | 34 |
| 105 | 60 | 55% | Wingham | 11 | 2 |
| 64 | 55 | 50% | Rochester | 10 | -2 |
| 29 | 89 | 50% | East Grinstead | 11 | 50 |
| 28 | 105 | 50% | Mayfield | 12 | 44 |
