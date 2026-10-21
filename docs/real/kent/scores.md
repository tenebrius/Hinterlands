# Real-terrain scores: kent

## All 70 real towns against the model's top 70

20 seeds, final year; random baselines drop the same 70 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.34 | 0.37 | 0.30 | 0% |
| Real towns with a model place within 10 km | 0.72 | 0.82 | 0.65 | 0% |
| Model places with a real town within 5 km | 0.40 | 0.41 | 0.38 | 0% |
| Model places with a real town within 10 km | 0.85 | 0.90 | 0.85 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.32 | 0.29 | 0.38 | 5% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.16 | -0.03 | 0.20 | 40% |

## The 27 real towns of 1,000 or more against the model's top 27

20 seeds, final year; random baselines drop the same 27 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.21 | 0.15 | 0.17 | 5% |
| Real towns with a model place within 10 km | 0.51 | 0.45 | 0.46 | 10% |
| Model places with a real town within 5 km | 0.21 | 0.15 | 0.18 | 5% |
| Model places with a real town within 10 km | 0.52 | 0.48 | 0.52 | 5% |
| Real top 10 with a model top-15 place within 10 km | 0.32 | 0.29 | 0.39 | 5% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.06 | 0.01 | 0.14 | 5% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (70) | 0.54 | 0.43 | 0.17 | 0.15 |
| 1,000 or more (27) | 0.64 | 0.55 | 0.23 | 0.15 |
| Under 1,000 (43) | 0.48 | 0.36 | 0.13 | 0.15 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 0; any top-70 place within 10 km: 12. Median distance from the model's largest place to London (City): 70 km. Largest place's share of the region's people (median): 0.24.

Nearest real town to the model's largest place, by seeds: Faversham 7, Gravesend 7, Sandwich 2, Fordwich 1, Milton Regis 1, Rochford 1, Charing 1.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 18 | 26% | 21.8 | 31% | 43% |
| coast | 6 | 9% | 9.6 | 14% | 38% |
| inland | 46 | 66% | 38.7 | 55% | 29% |

Inland places over 500 people, nearest other place over 500 (median): real 10.6 km (29 towns); model 9.3 km (median over seeds; 20 such places per seed).

Mean nearest-neighbour spacing of the 70 places: real 8.6 km, model 6.5 km, random land 6.9 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.27 | 0.74 | 0.24 | 0.86 | 0.30 | 0.06 | Brackford 25477 |
| 2 | 0.36 | 0.77 | 0.39 | 0.86 | 0.50 | 0.38 | Wexenby 38475 |
| 3 | 0.30 | 0.67 | 0.34 | 0.81 | 0.20 | 0.07 | Shapeley 40798 |
| 4 | 0.36 | 0.69 | 0.44 | 0.90 | 0.60 | 0.08 | Nedingport 15918 |
| 5 | 0.36 | 0.73 | 0.41 | 0.87 | 0.50 | 0.36 | Vereby 21636 |
| 6 | 0.36 | 0.71 | 0.41 | 0.91 | 0.20 | 0.10 | Mereey 42629 |
| 7 | 0.37 | 0.83 | 0.39 | 0.89 | 0.30 | 0.05 | Chedley 22805 |
| 8 | 0.34 | 0.71 | 0.41 | 0.93 | 0.50 | 0.13 | Bradwick 29363 |
| 9 | 0.26 | 0.57 | 0.36 | 0.83 | 0.20 | 0.18 | Ickeford 30406 |
| 10 | 0.36 | 0.81 | 0.40 | 0.76 | 0.10 | 0.09 | Yarerhurst 20028 |
| 11 | 0.27 | 0.70 | 0.31 | 0.80 | 0.30 | 0.10 | Loxhurst 24935 |
| 12 | 0.37 | 0.73 | 0.40 | 0.84 | 0.30 | -0.06 | Penesbourne 23878 |
| 13 | 0.39 | 0.70 | 0.49 | 0.87 | 0.20 | 0.09 | Roerhaven 30150 |
| 14 | 0.34 | 0.70 | 0.43 | 0.83 | 0.30 | 0.19 | Loxerford 37088 |
| 15 | 0.36 | 0.70 | 0.43 | 0.77 | 0.40 | 0.27 | Bareford 30602 |
| 16 | 0.44 | 0.84 | 0.44 | 0.84 | 0.50 | 0.07 | Ulowford 20700 |
| 17 | 0.31 | 0.77 | 0.39 | 0.84 | 0.40 | 0.34 | Belelport 23393 |
| 18 | 0.26 | 0.64 | 0.41 | 0.84 | 0.30 | 0.34 | Dewmouth 28129 |
| 19 | 0.36 | 0.71 | 0.43 | 0.86 | 0.10 | 0.06 | Maresford 33717 |
| 20 | 0.30 | 0.71 | 0.41 | 0.86 | 0.20 | 0.27 | Dewenwood 22845 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 35% |
| Canterbury | 10000 | city | 25% |
| Colchester | 4000 | large_town | 40% |
| Sandwich | 4000 | large_town | 55% |
| Winchelsea | 3500 | large_town | 35% |
| Dover | 3000 | large_town | 50% |
| New Romney | 2500 | town | 75% |
| Rye | 2500 | town | 25% |
| Lewes | 2500 | town | 30% |
| Maidstone | 2000 | town | 60% |
| Hythe | 2000 | town | 60% |
| Hastings | 2000 | town | 25% |
| Maldon | 2000 | town | 75% |
| Faversham | 2000 | town | 50% |
| Rochester | 1900 | town | 75% |
| Lydd | 1500 | town | 80% |
| Chelmsford | 1500 | town | 20% |
| New Shoreham | 1500 | town | 20% |
| Battle | 1200 | town | 25% |
| Tonbridge | 1200 | town | 55% |
| Dartford | 1200 | town | 55% |
| Hertford | 1200 | town | 15% |
| Ware | 1200 | town | 10% |
| Croydon | 1000 | town | 20% |
| Reigate | 1000 | town | 35% |
| Folkestone | 1000 | town | 55% |
| Bishop's Stortford | 1000 | town | 15% |
| Seaford | 800 | small_town | 30% |
| East Grinstead | 800 | small_town | 20% |
| Milton Regis | 800 | small_town | 10% |
| Barking | 800 | small_town | 45% |
| Waltham Abbey | 800 | small_town | 25% |
| Pevensey | 600 | small_town | 50% |
| Fordwich | 600 | small_town | 60% |
| Tenterden | 600 | small_town | 15% |
| Ashford | 600 | small_town | 50% |
| Sevenoaks | 600 | small_town | 30% |
| West Malling | 600 | small_town | 45% |
| Cranbrook | 600 | small_town | 20% |
| Gravesend | 600 | small_town | 30% |
| Romford | 600 | small_town | 30% |
| Coggeshall | 600 | small_town | 35% |
| Braintree | 600 | small_town | 50% |
| Great Dunmow | 600 | small_town | 20% |
| Wye | 500 | small_town | 45% |
| Witham | 500 | small_town | 40% |
| Brentwood | 500 | small_town | 50% |
| Chipping Barnet | 500 | small_town | 10% |
| Bletchingley | 500 | small_town | 20% |
| Westerham | 400 | small_town | 30% |
| Charing | 400 | small_town | 25% |
| Wrotham | 400 | small_town | 15% |
| Lenham | 400 | small_town | 10% |
| Elham | 400 | small_town | 25% |
| Wingham | 400 | small_town | 50% |
| Bromley | 400 | small_town | 25% |
| Robertsbridge | 400 | small_town | 40% |
| Hailsham | 400 | small_town | 35% |
| Mayfield | 400 | small_town | 10% |
| Cuckfield | 400 | small_town | 10% |
| Crawley | 400 | small_town | 15% |
| Epping | 400 | small_town | 10% |
| Billericay | 400 | small_town | 30% |
| Rayleigh | 400 | small_town | 35% |
| Rochford | 400 | small_town | 45% |
| Harlow | 400 | small_town | 15% |
| Chipping Ongar | 400 | small_town | 30% |
| Hoddesdon | 400 | small_town | 20% |
| Sawbridgeworth | 400 | small_town | 20% |
| Hatfield | 400 | small_town | 10% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 115 | 59 | 60% | Sandwich | 11 | 19 |
| 105 | 3 | 60% | Colchester | 23 | 0 |
| 104 | 10 | 60% | Colchester | 23 | 5 |
| 67 | 53 | 60% | Milton Regis | 10 | -8 |
| 84 | 22 | 55% | Maldon | 18 | 1 |
