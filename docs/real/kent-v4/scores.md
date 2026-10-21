# Real-terrain scores: kent

## All 70 real towns against the model's top 70

20 seeds, final year; random baselines drop the same 70 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.42 | 0.37 | 0.30 | 30% |
| Real towns with a model place within 10 km | 0.87 | 0.82 | 0.65 | 25% |
| Model places with a real town within 5 km | 0.45 | 0.41 | 0.38 | 10% |
| Model places with a real town within 10 km | 0.90 | 0.90 | 0.85 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.47 | 0.29 | 0.38 | 15% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.09 | -0.03 | 0.20 | 15% |

## The 27 real towns of 1,000 or more against the model's top 27

20 seeds, final year; random baselines drop the same 27 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.25 | 0.15 | 0.17 | 15% |
| Real towns with a model place within 10 km | 0.61 | 0.45 | 0.46 | 50% |
| Model places with a real town within 5 km | 0.24 | 0.15 | 0.18 | 15% |
| Model places with a real town within 10 km | 0.55 | 0.48 | 0.52 | 5% |
| Real top 10 with a model top-15 place within 10 km | 0.47 | 0.29 | 0.39 | 15% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.25 | 0.01 | 0.14 | 30% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (70) | 0.60 | 0.52 | 0.25 | 0.16 |
| 1,000 or more (27) | 0.64 | 0.57 | 0.30 | 0.16 |
| Under 1,000 (43) | 0.57 | 0.49 | 0.22 | 0.16 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 0; any top-70 place within 10 km: 20. Median distance from the model's largest place to London (City): 39 km. Largest place's share of the region's people (median): 0.33.

Nearest real town to the model's largest place, by seeds: Gravesend 13, Milton Regis 4, Faversham 3.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 18 | 26% | 33.5 | 48% | 55% |
| coast | 6 | 9% | 9.1 | 13% | 45% |
| inland | 46 | 66% | 27.4 | 39% | 37% |

Inland places over 500 people, nearest other place over 500 (median): real 10.6 km (29 towns); model 11.9 km (median over seeds; 10 such places per seed).

Mean nearest-neighbour spacing of the 70 places: real 8.6 km, model 7.4 km, random land 6.9 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.41 | 0.86 | 0.47 | 0.91 | 0.30 | 0.07 | Winshaven 59971 |
| 2 | 0.39 | 0.90 | 0.40 | 0.90 | 0.50 | 0.17 | Talemouth 57150 |
| 3 | 0.39 | 0.87 | 0.41 | 0.87 | 0.50 | 0.01 | Aldewood 62462 |
| 4 | 0.46 | 0.86 | 0.46 | 0.91 | 0.50 | 0.02 | Grimbourne 62396 |
| 5 | 0.46 | 0.91 | 0.50 | 0.94 | 0.60 | 0.19 | Penesport 65031 |
| 6 | 0.41 | 0.87 | 0.47 | 0.90 | 0.40 | 0.01 | Halwick 61307 |
| 7 | 0.43 | 0.86 | 0.50 | 0.90 | 0.40 | 0.06 | Bromwick 63770 |
| 8 | 0.36 | 0.83 | 0.41 | 0.89 | 0.50 | 0.06 | Ordewood 57090 |
| 9 | 0.40 | 0.93 | 0.44 | 0.93 | 0.60 | 0.12 | Croermouth 63442 |
| 10 | 0.47 | 0.84 | 0.50 | 0.94 | 0.40 | 0.03 | Noresport 64509 |
| 11 | 0.40 | 0.86 | 0.46 | 0.93 | 0.50 | 0.23 | Torsea 52650 |
| 12 | 0.46 | 0.81 | 0.47 | 0.87 | 0.40 | 0.13 | Winsford 60333 |
| 13 | 0.43 | 0.90 | 0.51 | 0.96 | 0.60 | -0.08 | Mereerham 63951 |
| 14 | 0.31 | 0.83 | 0.34 | 0.90 | 0.50 | -0.03 | Fairerhaven 46139 |
| 15 | 0.41 | 0.86 | 0.46 | 0.89 | 0.50 | 0.06 | Wexelmouth 62616 |
| 16 | 0.50 | 0.87 | 0.49 | 0.87 | 0.40 | 0.16 | Ashenmouth 57588 |
| 17 | 0.43 | 0.87 | 0.39 | 0.86 | 0.40 | 0.10 | Sutesbourne 55149 |
| 18 | 0.51 | 0.93 | 0.54 | 0.91 | 0.40 | 0.21 | Asherham 61621 |
| 19 | 0.37 | 0.83 | 0.39 | 0.91 | 0.50 | 0.16 | Winshaven 33214 |
| 20 | 0.43 | 0.86 | 0.41 | 0.90 | 0.50 | 0.08 | Kinport 57536 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 30% |
| Canterbury | 10000 | city | 65% |
| Colchester | 4000 | large_town | 15% |
| Sandwich | 4000 | large_town | 90% |
| Winchelsea | 3500 | large_town | 100% |
| Dover | 3000 | large_town | 55% |
| New Romney | 2500 | town | 80% |
| Rye | 2500 | town | 95% |
| Lewes | 2500 | town | 25% |
| Maidstone | 2000 | town | 95% |
| Hythe | 2000 | town | 95% |
| Hastings | 2000 | town | 5% |
| Maldon | 2000 | town | 25% |
| Faversham | 2000 | town | 60% |
| Rochester | 1900 | town | 20% |
| Lydd | 1500 | town | 5% |
| Chelmsford | 1500 | town | 90% |
| New Shoreham | 1500 | town | 0% |
| Battle | 1200 | town | 0% |
| Tonbridge | 1200 | town | 35% |
| Dartford | 1200 | town | 85% |
| Hertford | 1200 | town | 15% |
| Ware | 1200 | town | 15% |
| Croydon | 1000 | town | 50% |
| Reigate | 1000 | town | 20% |
| Folkestone | 1000 | town | 30% |
| Bishop's Stortford | 1000 | town | 50% |
| Seaford | 800 | small_town | 40% |
| East Grinstead | 800 | small_town | 0% |
| Milton Regis | 800 | small_town | 45% |
| Barking | 800 | small_town | 10% |
| Waltham Abbey | 800 | small_town | 80% |
| Pevensey | 600 | small_town | 100% |
| Fordwich | 600 | small_town | 85% |
| Tenterden | 600 | small_town | 5% |
| Ashford | 600 | small_town | 55% |
| Sevenoaks | 600 | small_town | 40% |
| West Malling | 600 | small_town | 30% |
| Cranbrook | 600 | small_town | 0% |
| Gravesend | 600 | small_town | 55% |
| Romford | 600 | small_town | 0% |
| Coggeshall | 600 | small_town | 60% |
| Braintree | 600 | small_town | 10% |
| Great Dunmow | 600 | small_town | 15% |
| Wye | 500 | small_town | 25% |
| Witham | 500 | small_town | 50% |
| Brentwood | 500 | small_town | 5% |
| Chipping Barnet | 500 | small_town | 25% |
| Bletchingley | 500 | small_town | 5% |
| Westerham | 400 | small_town | 0% |
| Charing | 400 | small_town | 5% |
| Wrotham | 400 | small_town | 5% |
| Lenham | 400 | small_town | 0% |
| Elham | 400 | small_town | 0% |
| Wingham | 400 | small_town | 70% |
| Bromley | 400 | small_town | 75% |
| Robertsbridge | 400 | small_town | 85% |
| Hailsham | 400 | small_town | 0% |
| Mayfield | 400 | small_town | 20% |
| Cuckfield | 400 | small_town | 100% |
| Crawley | 400 | small_town | 70% |
| Epping | 400 | small_town | 50% |
| Billericay | 400 | small_town | 40% |
| Rayleigh | 400 | small_town | 75% |
| Rochford | 400 | small_town | 5% |
| Harlow | 400 | small_town | 50% |
| Chipping Ongar | 400 | small_town | 80% |
| Hoddesdon | 400 | small_town | 75% |
| Sawbridgeworth | 400 | small_town | 70% |
| Hatfield | 400 | small_town | 85% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 81 | 16 | 100% | Colchester | 14 | 1 |
| 47 | 88 | 95% | Cranbrook | 10 | 94 |
| 3 | 47 | 90% | London (City) | 11 | 7 |
| 90 | 9 | 85% | Colchester | 11 | 20 |
| 29 | 89 | 80% | East Grinstead | 11 | 50 |
| 27 | 104 | 65% | Mayfield | 12 | 32 |
| 104 | 12 | 55% | Colchester | 24 | 0 |
