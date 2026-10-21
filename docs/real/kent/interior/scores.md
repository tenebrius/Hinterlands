# Real-terrain scores: kent

Interior only: everything within 10 km of the map edge left out.

## All 58 real towns against the model's top 58

20 seeds, final year; random baselines drop the same 58 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.34 | 0.38 | 0.32 | 0% |
| Real towns with a model place within 10 km | 0.72 | 0.83 | 0.69 | 0% |
| Model places with a real town within 5 km | 0.41 | 0.42 | 0.40 | 0% |
| Model places with a real town within 10 km | 0.87 | 0.91 | 0.88 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.41 | 0.32 | 0.41 | 10% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.16 | -0.00 | 0.20 | 30% |

## The 23 real towns of 1,000 or more against the model's top 23

20 seeds, final year; random baselines drop the same 23 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.23 | 0.17 | 0.20 | 15% |
| Real towns with a model place within 10 km | 0.54 | 0.47 | 0.48 | 0% |
| Model places with a real town within 5 km | 0.22 | 0.17 | 0.20 | 0% |
| Model places with a real town within 10 km | 0.53 | 0.49 | 0.54 | 5% |
| Real top 10 with a model top-15 place within 10 km | 0.41 | 0.33 | 0.42 | 10% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.08 | -0.01 | 0.11 | 10% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (58) | 0.53 | 0.44 | 0.17 | 0.15 |
| 1,000 or more (23) | 0.64 | 0.57 | 0.24 | 0.15 |
| Under 1,000 (35) | 0.46 | 0.36 | 0.13 | 0.15 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 1; any top-58 place within 10 km: 11. Median distance from the model's largest place to London (City): 70 km. Largest place's share of the region's people (median): 0.24.

Nearest real town to the model's largest place, by seeds: Faversham 7, Gravesend 7, Sandwich 2, Fordwich 1, Milton Regis 1, Rochford 1, Charing 1.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 17 | 29% | 20.6 | 36% | 42% |
| coast | 4 | 7% | 7.0 | 12% | 40% |
| inland | 37 | 64% | 30.4 | 52% | 30% |

Inland places over 500 people, nearest other place over 500 (median): real 10.6 km (22 towns); model 9.5 km (median over seeds; 16 such places per seed).

Mean nearest-neighbour spacing of the 58 places: real 8.5 km, model 6.3 km, random land 6.7 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.31 | 0.78 | 0.28 | 0.86 | 0.50 | 0.07 | Brackford 25477 |
| 2 | 0.34 | 0.74 | 0.36 | 0.84 | 0.30 | 0.38 | Wexenby 38475 |
| 3 | 0.29 | 0.69 | 0.34 | 0.86 | 0.30 | 0.07 | Shapeley 40798 |
| 4 | 0.38 | 0.69 | 0.48 | 0.91 | 0.40 | 0.02 | Nedingport 15918 |
| 5 | 0.33 | 0.71 | 0.36 | 0.90 | 0.70 | 0.40 | Vereby 21636 |
| 6 | 0.40 | 0.72 | 0.45 | 0.93 | 0.50 | 0.12 | Mereey 42629 |
| 7 | 0.34 | 0.81 | 0.36 | 0.93 | 0.60 | 0.10 | Chedley 22805 |
| 8 | 0.36 | 0.71 | 0.45 | 0.95 | 0.70 | 0.10 | Bradwick 29363 |
| 9 | 0.26 | 0.60 | 0.38 | 0.88 | 0.20 | 0.13 | Ickeford 30406 |
| 10 | 0.36 | 0.74 | 0.43 | 0.81 | 0.40 | 0.11 | Yarerhurst 20028 |
| 11 | 0.29 | 0.69 | 0.33 | 0.81 | 0.30 | 0.18 | Loxhurst 24935 |
| 12 | 0.38 | 0.71 | 0.43 | 0.84 | 0.40 | -0.03 | Penesbourne 23878 |
| 13 | 0.34 | 0.66 | 0.45 | 0.86 | 0.40 | 0.06 | Roerhaven 30150 |
| 14 | 0.33 | 0.76 | 0.43 | 0.81 | 0.30 | 0.16 | Loxerford 37088 |
| 15 | 0.36 | 0.76 | 0.43 | 0.79 | 0.30 | 0.23 | Bareford 30602 |
| 16 | 0.41 | 0.81 | 0.43 | 0.91 | 0.50 | 0.11 | Ulowford 20700 |
| 17 | 0.31 | 0.83 | 0.38 | 0.84 | 0.40 | 0.37 | Belelport 23393 |
| 18 | 0.29 | 0.66 | 0.48 | 0.88 | 0.60 | 0.34 | Dewmouth 28129 |
| 19 | 0.36 | 0.69 | 0.45 | 0.86 | 0.20 | 0.07 | Maresford 33717 |
| 20 | 0.31 | 0.67 | 0.45 | 0.90 | 0.20 | 0.26 | Dewenwood 22845 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 35% |
| Canterbury | 10000 | city | 25% |
| Sandwich | 4000 | large_town | 55% |
| Winchelsea | 3500 | large_town | 35% |
| Dover | 3000 | large_town | 50% |
| New Romney | 2500 | town | 80% |
| Rye | 2500 | town | 25% |
| Lewes | 2500 | town | 20% |
| Faversham | 2000 | town | 45% |
| Hythe | 2000 | town | 55% |
| Hastings | 2000 | town | 15% |
| Maidstone | 2000 | town | 60% |
| Maldon | 2000 | town | 75% |
| Rochester | 1900 | town | 75% |
| Lydd | 1500 | town | 75% |
| Chelmsford | 1500 | town | 20% |
| Battle | 1200 | town | 25% |
| Tonbridge | 1200 | town | 55% |
| Dartford | 1200 | town | 55% |
| Hertford | 1200 | town | 10% |
| Ware | 1200 | town | 5% |
| Croydon | 1000 | town | 20% |
| Folkestone | 1000 | town | 55% |
| East Grinstead | 800 | small_town | 20% |
| Milton Regis | 800 | small_town | 10% |
| Barking | 800 | small_town | 45% |
| Waltham Abbey | 800 | small_town | 25% |
| Fordwich | 600 | small_town | 60% |
| Tenterden | 600 | small_town | 15% |
| Ashford | 600 | small_town | 50% |
| Sevenoaks | 600 | small_town | 30% |
| West Malling | 600 | small_town | 35% |
| Cranbrook | 600 | small_town | 25% |
| Gravesend | 600 | small_town | 30% |
| Romford | 600 | small_town | 30% |
| Wye | 500 | small_town | 45% |
| Witham | 500 | small_town | 40% |
| Brentwood | 500 | small_town | 55% |
| Bletchingley | 500 | small_town | 10% |
| Westerham | 400 | small_town | 30% |
| Charing | 400 | small_town | 25% |
| Wrotham | 400 | small_town | 15% |
| Lenham | 400 | small_town | 15% |
| Elham | 400 | small_town | 30% |
| Wingham | 400 | small_town | 50% |
| Bromley | 400 | small_town | 25% |
| Robertsbridge | 400 | small_town | 40% |
| Hailsham | 400 | small_town | 25% |
| Mayfield | 400 | small_town | 10% |
| Cuckfield | 400 | small_town | 10% |
| Epping | 400 | small_town | 10% |
| Billericay | 400 | small_town | 30% |
| Rayleigh | 400 | small_town | 35% |
| Rochford | 400 | small_town | 45% |
| Harlow | 400 | small_town | 15% |
| Chipping Ongar | 400 | small_town | 30% |
| Hoddesdon | 400 | small_town | 20% |
| Sawbridgeworth | 400 | small_town | 10% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 115 | 59 | 65% | Sandwich | 11 | 19 |
| 84 | 22 | 60% | Maldon | 18 | 1 |
| 67 | 53 | 60% | Milton Regis | 10 | -8 |
