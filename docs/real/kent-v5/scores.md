# Real-terrain scores: kent

## All 70 real towns against the model's top 70

20 seeds, final year; random baselines drop the same 70 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.43 | 0.37 | 0.30 | 25% |
| Real towns with a model place within 10 km | 0.87 | 0.82 | 0.65 | 25% |
| Model places with a real town within 5 km | 0.47 | 0.41 | 0.38 | 10% |
| Model places with a real town within 10 km | 0.92 | 0.90 | 0.85 | 5% |
| Real top 10 with a model top-15 place within 10 km | 0.45 | 0.29 | 0.38 | 20% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.11 | -0.03 | 0.20 | 20% |

## The 27 real towns of 1,000 or more against the model's top 27

20 seeds, final year; random baselines drop the same 27 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.30 | 0.15 | 0.17 | 30% |
| Real towns with a model place within 10 km | 0.61 | 0.45 | 0.46 | 50% |
| Model places with a real town within 5 km | 0.29 | 0.15 | 0.18 | 35% |
| Model places with a real town within 10 km | 0.59 | 0.48 | 0.52 | 20% |
| Real top 10 with a model top-15 place within 10 km | 0.45 | 0.29 | 0.39 | 20% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.21 | 0.01 | 0.14 | 20% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (70) | 0.59 | 0.52 | 0.24 | 0.16 |
| 1,000 or more (27) | 0.63 | 0.55 | 0.28 | 0.16 |
| Under 1,000 (43) | 0.57 | 0.50 | 0.21 | 0.16 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 0; any top-70 place within 10 km: 20. Median distance from the model's largest place to London (City): 39 km. Largest place's share of the region's people (median): 0.35.

Nearest real town to the model's largest place, by seeds: Gravesend 17, Milton Regis 2, Faversham 1.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 18 | 26% | 30.9 | 44% | 50% |
| coast | 6 | 9% | 9.2 | 13% | 48% |
| inland | 46 | 66% | 29.9 | 43% | 39% |

Inland places over 500 people, nearest other place over 500 (median): real 10.6 km (29 towns); model 12.0 km (median over seeds; 10 such places per seed).

Mean nearest-neighbour spacing of the 70 places: real 8.6 km, model 7.3 km, random land 6.9 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.46 | 0.93 | 0.49 | 0.93 | 0.50 | 0.10 | Bureswick 62718 |
| 2 | 0.41 | 0.86 | 0.46 | 0.94 | 0.30 | 0.06 | Glenhurst 60682 |
| 3 | 0.41 | 0.89 | 0.44 | 0.91 | 0.40 | 0.14 | Malden 60256 |
| 4 | 0.43 | 0.87 | 0.49 | 0.91 | 0.40 | 0.05 | Harnmouth 69932 |
| 5 | 0.49 | 0.86 | 0.50 | 0.90 | 0.50 | 0.07 | Harnerford 69310 |
| 6 | 0.44 | 0.89 | 0.49 | 0.93 | 0.50 | 0.22 | Uleley 61631 |
| 7 | 0.39 | 0.86 | 0.44 | 0.91 | 0.40 | 0.10 | Reneholt 64444 |
| 8 | 0.43 | 0.83 | 0.46 | 0.91 | 0.50 | 0.04 | Toremouth 66392 |
| 9 | 0.39 | 0.87 | 0.43 | 0.99 | 0.20 | -0.01 | Melelhaven 37625 |
| 10 | 0.43 | 0.84 | 0.43 | 0.94 | 0.40 | -0.00 | Cranenport 74299 |
| 11 | 0.41 | 0.91 | 0.46 | 0.90 | 0.60 | 0.08 | Wexesport 73262 |
| 12 | 0.43 | 0.81 | 0.51 | 0.89 | 0.40 | 0.04 | Chalehurst 56892 |
| 13 | 0.43 | 0.90 | 0.49 | 0.93 | 0.50 | 0.19 | Wexesport 72399 |
| 14 | 0.41 | 0.86 | 0.47 | 0.90 | 0.40 | 0.33 | Kirkemouth 75897 |
| 15 | 0.36 | 0.89 | 0.43 | 0.90 | 0.70 | 0.11 | Norebourne 61628 |
| 16 | 0.40 | 0.90 | 0.47 | 0.90 | 0.20 | 0.13 | Iselmouth 68813 |
| 17 | 0.47 | 0.91 | 0.51 | 0.93 | 0.60 | 0.17 | Falelport 68855 |
| 18 | 0.46 | 0.86 | 0.50 | 0.93 | 0.50 | 0.03 | Mereerham 68371 |
| 19 | 0.46 | 0.89 | 0.44 | 0.87 | 0.60 | 0.20 | Harnerford 73104 |
| 20 | 0.43 | 0.86 | 0.47 | 0.89 | 0.40 | 0.15 | Kirkesham 63903 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 15% |
| Canterbury | 10000 | city | 60% |
| Colchester | 4000 | large_town | 10% |
| Sandwich | 4000 | large_town | 90% |
| Winchelsea | 3500 | large_town | 100% |
| Dover | 3000 | large_town | 60% |
| New Romney | 2500 | town | 85% |
| Rye | 2500 | town | 100% |
| Lewes | 2500 | town | 30% |
| Maidstone | 2000 | town | 75% |
| Hythe | 2000 | town | 95% |
| Hastings | 2000 | town | 0% |
| Maldon | 2000 | town | 10% |
| Faversham | 2000 | town | 85% |
| Rochester | 1900 | town | 5% |
| Lydd | 1500 | town | 10% |
| Chelmsford | 1500 | town | 100% |
| New Shoreham | 1500 | town | 0% |
| Battle | 1200 | town | 0% |
| Tonbridge | 1200 | town | 30% |
| Dartford | 1200 | town | 85% |
| Hertford | 1200 | town | 15% |
| Ware | 1200 | town | 0% |
| Croydon | 1000 | town | 35% |
| Reigate | 1000 | town | 40% |
| Folkestone | 1000 | town | 60% |
| Bishop's Stortford | 1000 | town | 65% |
| Seaford | 800 | small_town | 35% |
| East Grinstead | 800 | small_town | 0% |
| Milton Regis | 800 | small_town | 80% |
| Barking | 800 | small_town | 15% |
| Waltham Abbey | 800 | small_town | 75% |
| Pevensey | 600 | small_town | 100% |
| Fordwich | 600 | small_town | 75% |
| Tenterden | 600 | small_town | 5% |
| Ashford | 600 | small_town | 90% |
| Sevenoaks | 600 | small_town | 35% |
| West Malling | 600 | small_town | 20% |
| Cranbrook | 600 | small_town | 0% |
| Gravesend | 600 | small_town | 35% |
| Romford | 600 | small_town | 5% |
| Coggeshall | 600 | small_town | 85% |
| Braintree | 600 | small_town | 10% |
| Great Dunmow | 600 | small_town | 45% |
| Wye | 500 | small_town | 20% |
| Witham | 500 | small_town | 40% |
| Brentwood | 500 | small_town | 15% |
| Chipping Barnet | 500 | small_town | 20% |
| Bletchingley | 500 | small_town | 10% |
| Westerham | 400 | small_town | 0% |
| Charing | 400 | small_town | 0% |
| Wrotham | 400 | small_town | 0% |
| Lenham | 400 | small_town | 5% |
| Elham | 400 | small_town | 0% |
| Wingham | 400 | small_town | 75% |
| Bromley | 400 | small_town | 80% |
| Robertsbridge | 400 | small_town | 70% |
| Hailsham | 400 | small_town | 10% |
| Mayfield | 400 | small_town | 25% |
| Cuckfield | 400 | small_town | 100% |
| Crawley | 400 | small_town | 65% |
| Epping | 400 | small_town | 55% |
| Billericay | 400 | small_town | 45% |
| Rayleigh | 400 | small_town | 60% |
| Rochford | 400 | small_town | 15% |
| Harlow | 400 | small_town | 50% |
| Chipping Ongar | 400 | small_town | 40% |
| Hoddesdon | 400 | small_town | 70% |
| Sawbridgeworth | 400 | small_town | 70% |
| Hatfield | 400 | small_town | 75% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 80 | 17 | 100% | Maldon | 14 | 2 |
| 46 | 89 | 100% | Cranbrook | 11 | 78 |
| 4 | 48 | 100% | London (City) | 10 | 5 |
| 29 | 89 | 95% | East Grinstead | 11 | 50 |
| 27 | 104 | 85% | Mayfield | 12 | 32 |
| 104 | 11 | 50% | Colchester | 23 | 2 |
| 104 | 4 | 50% | Colchester | 22 | 0 |
