# Real-terrain scores: kent

## All 70 real towns against the model's top 70

20 seeds, final year; random baselines drop the same 70 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.37 | 0.37 | 0.30 | 5% |
| Real towns with a model place within 10 km | 0.83 | 0.82 | 0.65 | 5% |
| Model places with a real town within 5 km | 0.43 | 0.41 | 0.38 | 10% |
| Model places with a real town within 10 km | 0.87 | 0.90 | 0.85 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.40 | 0.29 | 0.38 | 15% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.10 | -0.03 | 0.20 | 30% |

## The 27 real towns of 1,000 or more against the model's top 27

20 seeds, final year; random baselines drop the same 27 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.19 | 0.15 | 0.17 | 5% |
| Real towns with a model place within 10 km | 0.53 | 0.45 | 0.46 | 15% |
| Model places with a real town within 5 km | 0.18 | 0.15 | 0.18 | 0% |
| Model places with a real town within 10 km | 0.48 | 0.48 | 0.52 | 5% |
| Real top 10 with a model top-15 place within 10 km | 0.40 | 0.29 | 0.39 | 15% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.04 | 0.01 | 0.14 | 5% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (70) | 0.57 | 0.47 | 0.20 | 0.15 |
| 1,000 or more (27) | 0.64 | 0.55 | 0.25 | 0.15 |
| Under 1,000 (43) | 0.52 | 0.42 | 0.18 | 0.15 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 2; any top-70 place within 10 km: 20. Median distance from the model's largest place to London (City): 54 km.

Nearest real town to the model's largest place, by seeds: Maldon 5, Gravesend 4, Dartford 3, Dover 3, Rochester 2, Romford 1, Milton Regis 1, Colchester 1.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 18 | 26% | 34.1 | 49% | 59% |
| coast | 6 | 9% | 9.6 | 14% | 28% |
| inland | 46 | 66% | 26.3 | 38% | 29% |

Mean nearest-neighbour spacing of the 70 places: real 8.6 km, model 7.1 km, random land 6.9 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.46 | 0.89 | 0.46 | 0.86 | 0.40 | 0.27 | Langerham 22254 |
| 2 | 0.33 | 0.86 | 0.39 | 0.84 | 0.30 | -0.02 | Walanport 15243 |
| 3 | 0.39 | 0.87 | 0.49 | 0.86 | 0.30 | 0.11 | Kirkport 18165 |
| 4 | 0.37 | 0.89 | 0.40 | 0.86 | 0.40 | 0.21 | Noranwick 17175 |
| 5 | 0.36 | 0.83 | 0.47 | 0.87 | 0.60 | -0.01 | Wexbourne 11008 |
| 6 | 0.34 | 0.80 | 0.33 | 0.80 | 0.40 | 0.27 | Rayaney 13807 |
| 7 | 0.44 | 0.74 | 0.57 | 0.84 | 0.10 | 0.19 | Elmeford 27017 |
| 8 | 0.39 | 0.83 | 0.41 | 0.86 | 0.60 | -0.01 | Kinholt 22605 |
| 9 | 0.39 | 0.93 | 0.41 | 0.87 | 0.40 | -0.02 | Aldesea 14920 |
| 10 | 0.34 | 0.84 | 0.40 | 0.90 | 0.50 | -0.01 | Wyeford 19998 |
| 11 | 0.37 | 0.89 | 0.44 | 0.89 | 0.50 | 0.04 | Wickingey 15004 |
| 12 | 0.33 | 0.76 | 0.43 | 0.84 | 0.20 | 0.13 | Holingport 35243 |
| 13 | 0.31 | 0.83 | 0.37 | 0.80 | 0.50 | 0.18 | Grimsea 20493 |
| 14 | 0.34 | 0.76 | 0.44 | 0.91 | 0.70 | 0.14 | Kirkenport 19873 |
| 15 | 0.36 | 0.74 | 0.43 | 0.91 | 0.40 | 0.13 | Iselenhaven 16050 |
| 16 | 0.34 | 0.86 | 0.40 | 0.89 | 0.40 | -0.09 | Belanmouth 20033 |
| 17 | 0.31 | 0.83 | 0.41 | 0.96 | 0.50 | 0.19 | Dunelport 12451 |
| 18 | 0.44 | 0.79 | 0.51 | 0.86 | 0.30 | 0.01 | Ulowmouth 16705 |
| 19 | 0.39 | 0.83 | 0.47 | 0.90 | 0.40 | 0.16 | Rayey 17012 |
| 20 | 0.31 | 0.89 | 0.39 | 0.87 | 0.20 | 0.11 | Roaey 17548 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 20% |
| Canterbury | 10000 | city | 60% |
| Colchester | 4000 | large_town | 5% |
| Sandwich | 4000 | large_town | 65% |
| Winchelsea | 3500 | large_town | 50% |
| Dover | 3000 | large_town | 45% |
| New Romney | 2500 | town | 80% |
| Rye | 2500 | town | 45% |
| Lewes | 2500 | town | 45% |
| Maidstone | 2000 | town | 60% |
| Hythe | 2000 | town | 20% |
| Hastings | 2000 | town | 25% |
| Maldon | 2000 | town | 60% |
| Faversham | 2000 | town | 85% |
| Rochester | 1900 | town | 65% |
| Lydd | 1500 | town | 40% |
| Chelmsford | 1500 | town | 60% |
| New Shoreham | 1500 | town | 30% |
| Battle | 1200 | town | 0% |
| Tonbridge | 1200 | town | 45% |
| Dartford | 1200 | town | 55% |
| Hertford | 1200 | town | 65% |
| Ware | 1200 | town | 55% |
| Croydon | 1000 | town | 35% |
| Reigate | 1000 | town | 10% |
| Folkestone | 1000 | town | 5% |
| Bishop's Stortford | 1000 | town | 20% |
| Seaford | 800 | small_town | 35% |
| East Grinstead | 800 | small_town | 0% |
| Milton Regis | 800 | small_town | 70% |
| Barking | 800 | small_town | 60% |
| Waltham Abbey | 800 | small_town | 40% |
| Pevensey | 600 | small_town | 65% |
| Fordwich | 600 | small_town | 40% |
| Tenterden | 600 | small_town | 0% |
| Ashford | 600 | small_town | 55% |
| Sevenoaks | 600 | small_town | 40% |
| West Malling | 600 | small_town | 35% |
| Cranbrook | 600 | small_town | 0% |
| Gravesend | 600 | small_town | 70% |
| Romford | 600 | small_town | 0% |
| Coggeshall | 600 | small_town | 65% |
| Braintree | 600 | small_town | 0% |
| Great Dunmow | 600 | small_town | 25% |
| Wye | 500 | small_town | 25% |
| Witham | 500 | small_town | 55% |
| Brentwood | 500 | small_town | 0% |
| Chipping Barnet | 500 | small_town | 0% |
| Bletchingley | 500 | small_town | 0% |
| Westerham | 400 | small_town | 0% |
| Charing | 400 | small_town | 0% |
| Wrotham | 400 | small_town | 0% |
| Lenham | 400 | small_town | 0% |
| Elham | 400 | small_town | 0% |
| Wingham | 400 | small_town | 30% |
| Bromley | 400 | small_town | 70% |
| Robertsbridge | 400 | small_town | 55% |
| Hailsham | 400 | small_town | 90% |
| Mayfield | 400 | small_town | 10% |
| Cuckfield | 400 | small_town | 25% |
| Crawley | 400 | small_town | 30% |
| Epping | 400 | small_town | 15% |
| Billericay | 400 | small_town | 40% |
| Rayleigh | 400 | small_town | 40% |
| Rochford | 400 | small_town | 15% |
| Harlow | 400 | small_town | 60% |
| Chipping Ongar | 400 | small_town | 60% |
| Hoddesdon | 400 | small_town | 45% |
| Sawbridgeworth | 400 | small_town | 65% |
| Hatfield | 400 | small_town | 80% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 82 | 34 | 85% | Rochford | 13 | 2 |
| 105 | 10 | 70% | Colchester | 24 | 0 |
| 4 | 48 | 70% | London (City) | 10 | 5 |
| 118 | 59 | 60% | Sandwich | 12 | 41 |
| 54 | 45 | 60% | Gravesend | 11 | -1 |
| 31 | 87 | 60% | Tonbridge | 11 | 81 |
| 2 | 61 | 50% | Croydon | 11 | 34 |
