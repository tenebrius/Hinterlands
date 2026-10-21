# Real-terrain scores: kent

Interior only: everything within 10 km of the map edge left out.

## All 58 real towns against the model's top 58

20 seeds, final year; random baselines drop the same 58 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.39 | 0.38 | 0.32 | 5% |
| Real towns with a model place within 10 km | 0.83 | 0.83 | 0.69 | 0% |
| Model places with a real town within 5 km | 0.44 | 0.42 | 0.40 | 5% |
| Model places with a real town within 10 km | 0.89 | 0.91 | 0.88 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.43 | 0.32 | 0.41 | 0% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.12 | -0.00 | 0.20 | 25% |

## The 23 real towns of 1,000 or more against the model's top 23

20 seeds, final year; random baselines drop the same 23 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.22 | 0.17 | 0.20 | 5% |
| Real towns with a model place within 10 km | 0.57 | 0.47 | 0.48 | 0% |
| Model places with a real town within 5 km | 0.22 | 0.17 | 0.20 | 5% |
| Model places with a real town within 10 km | 0.53 | 0.49 | 0.54 | 5% |
| Real top 10 with a model top-15 place within 10 km | 0.43 | 0.33 | 0.42 | 0% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.10 | -0.01 | 0.11 | 15% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (58) | 0.58 | 0.49 | 0.22 | 0.16 |
| 1,000 or more (23) | 0.68 | 0.59 | 0.28 | 0.16 |
| Under 1,000 (35) | 0.52 | 0.43 | 0.18 | 0.16 |

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 17 | 29% | 32.8 | 57% | 62% |
| coast | 4 | 7% | 7.0 | 12% | 16% |
| inland | 37 | 64% | 18.2 | 31% | 30% |

Mean nearest-neighbour spacing of the 58 places: real 8.5 km, model 7.1 km, random land 6.7 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.36 | 0.83 | 0.38 | 0.90 | 0.60 | 0.15 | Wexanhaven 19317 |
| 2 | 0.40 | 0.79 | 0.45 | 0.86 | 0.40 | 0.16 | Ulowmouth 19566 |
| 3 | 0.36 | 0.90 | 0.43 | 0.93 | 0.50 | 0.10 | Nedenmouth 16998 |
| 4 | 0.48 | 0.83 | 0.50 | 0.90 | 0.50 | 0.24 | Oakesmouth 27465 |
| 5 | 0.43 | 0.79 | 0.47 | 0.93 | 0.50 | 0.08 | Hartsea 17519 |
| 6 | 0.34 | 0.83 | 0.43 | 0.83 | 0.20 | 0.29 | Yarelsea 23207 |
| 7 | 0.34 | 0.90 | 0.41 | 0.93 | 0.30 | -0.04 | Falwick 19183 |
| 8 | 0.31 | 0.84 | 0.34 | 0.90 | 0.40 | 0.15 | Torelport 27309 |
| 9 | 0.38 | 0.91 | 0.47 | 0.95 | 0.60 | 0.07 | Walanport 17027 |
| 10 | 0.34 | 0.76 | 0.41 | 0.79 | 0.50 | 0.22 | Everey 14429 |
| 11 | 0.33 | 0.76 | 0.43 | 0.91 | 0.40 | -0.13 | Wickhaven 20469 |
| 12 | 0.34 | 0.84 | 0.40 | 0.91 | 0.60 | 0.14 | Hartsea 12017 |
| 13 | 0.47 | 0.90 | 0.47 | 0.97 | 0.40 | 0.07 | Wickesham 10624 |
| 14 | 0.43 | 0.84 | 0.43 | 0.84 | 0.40 | -0.09 | Winsesford 16430 |
| 15 | 0.36 | 0.84 | 0.43 | 0.95 | 0.40 | 0.22 | Thorningport 18753 |
| 16 | 0.45 | 0.86 | 0.59 | 0.97 | 0.20 | 0.09 | Pickerwick 24342 |
| 17 | 0.40 | 0.81 | 0.41 | 0.86 | 0.40 | 0.17 | Calensea 22991 |
| 18 | 0.33 | 0.74 | 0.36 | 0.78 | 0.40 | 0.07 | Aldeley 27345 |
| 19 | 0.40 | 0.76 | 0.43 | 0.84 | 0.50 | 0.15 | Asherford 17440 |
| 20 | 0.45 | 0.90 | 0.47 | 0.91 | 0.40 | 0.24 | Ordansea 20454 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 20% |
| Canterbury | 10000 | city | 75% |
| Sandwich | 4000 | large_town | 40% |
| Winchelsea | 3500 | large_town | 50% |
| Dover | 3000 | large_town | 45% |
| New Romney | 2500 | town | 95% |
| Rye | 2500 | town | 30% |
| Lewes | 2500 | town | 30% |
| Faversham | 2000 | town | 65% |
| Hythe | 2000 | town | 10% |
| Hastings | 2000 | town | 0% |
| Maidstone | 2000 | town | 65% |
| Maldon | 2000 | town | 60% |
| Rochester | 1900 | town | 75% |
| Lydd | 1500 | town | 30% |
| Chelmsford | 1500 | town | 65% |
| Battle | 1200 | town | 0% |
| Tonbridge | 1200 | town | 45% |
| Dartford | 1200 | town | 40% |
| Hertford | 1200 | town | 55% |
| Ware | 1200 | town | 55% |
| Croydon | 1000 | town | 55% |
| Folkestone | 1000 | town | 5% |
| East Grinstead | 800 | small_town | 0% |
| Milton Regis | 800 | small_town | 70% |
| Barking | 800 | small_town | 55% |
| Waltham Abbey | 800 | small_town | 50% |
| Fordwich | 600 | small_town | 65% |
| Tenterden | 600 | small_town | 0% |
| Ashford | 600 | small_town | 55% |
| Sevenoaks | 600 | small_town | 60% |
| West Malling | 600 | small_town | 45% |
| Cranbrook | 600 | small_town | 0% |
| Gravesend | 600 | small_town | 85% |
| Romford | 600 | small_town | 0% |
| Wye | 500 | small_town | 10% |
| Witham | 500 | small_town | 60% |
| Brentwood | 500 | small_town | 0% |
| Bletchingley | 500 | small_town | 0% |
| Westerham | 400 | small_town | 0% |
| Charing | 400 | small_town | 10% |
| Wrotham | 400 | small_town | 0% |
| Lenham | 400 | small_town | 0% |
| Elham | 400 | small_town | 0% |
| Wingham | 400 | small_town | 45% |
| Bromley | 400 | small_town | 50% |
| Robertsbridge | 400 | small_town | 70% |
| Hailsham | 400 | small_town | 85% |
| Mayfield | 400 | small_town | 20% |
| Cuckfield | 400 | small_town | 50% |
| Epping | 400 | small_town | 20% |
| Billericay | 400 | small_town | 50% |
| Rayleigh | 400 | small_town | 35% |
| Rochford | 400 | small_town | 10% |
| Harlow | 400 | small_town | 50% |
| Chipping Ongar | 400 | small_town | 65% |
| Hoddesdon | 400 | small_town | 55% |
| Sawbridgeworth | 400 | small_town | 55% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 105 | 60 | 75% | Wingham | 11 | 2 |
| 32 | 86 | 70% | Tonbridge | 10 | 97 |
| 64 | 55 | 65% | Rochester | 10 | -2 |
| 81 | 14 | 50% | Maldon | 15 | 1 |
| 50 | 79 | 50% | Maidstone | 11 | 14 |
