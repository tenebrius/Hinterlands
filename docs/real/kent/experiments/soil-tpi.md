# Real-terrain scores: kent

## All 70 real towns against the model's top 70

20 seeds, final year; random baselines drop the same 70 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.32 | 0.37 | 0.30 | 0% |
| Real towns with a model place within 10 km | 0.69 | 0.82 | 0.65 | 0% |
| Model places with a real town within 5 km | 0.42 | 0.41 | 0.38 | 15% |
| Model places with a real town within 10 km | 0.84 | 0.90 | 0.85 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.37 | 0.29 | 0.38 | 20% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.11 | -0.03 | 0.20 | 35% |

## The 27 real towns of 1,000 or more against the model's top 27

20 seeds, final year; random baselines drop the same 27 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.17 | 0.15 | 0.17 | 5% |
| Real towns with a model place within 10 km | 0.47 | 0.45 | 0.46 | 5% |
| Model places with a real town within 5 km | 0.17 | 0.15 | 0.18 | 5% |
| Model places with a real town within 10 km | 0.48 | 0.48 | 0.52 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.37 | 0.29 | 0.39 | 20% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.15 | 0.01 | 0.14 | 15% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (70) | 0.57 | 0.47 | 0.17 | 0.14 |
| 1,000 or more (27) | 0.58 | 0.51 | 0.21 | 0.14 |
| Under 1,000 (43) | 0.56 | 0.44 | 0.15 | 0.14 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 1; any top-70 place within 10 km: 17. Median distance from the model's largest place to London (City): 43 km. Largest place's share of the region's people (median): 0.27.

Nearest real town to the model's largest place, by seeds: Gravesend 9, Faversham 3, Milton Regis 2, Wingham 1, Rayleigh 1, Ashford 1, Rochford 1, Rochester 1, West Malling 1.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 18 | 26% | 21.6 | 31% | 41% |
| coast | 6 | 9% | 11.8 | 17% | 32% |
| inland | 46 | 66% | 36.6 | 52% | 28% |

Inland places over 500 people, nearest other place over 500 (median): real 10.6 km (29 towns); model 9.0 km (median over seeds; 18 such places per seed).

Mean nearest-neighbour spacing of the 70 places: real 8.6 km, model 6.2 km, random land 6.9 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.24 | 0.61 | 0.30 | 0.76 | 0.20 | 0.21 | Thornanton 27292 |
| 2 | 0.27 | 0.67 | 0.36 | 0.83 | 0.60 | 0.21 | Ulahaven 28126 |
| 3 | 0.31 | 0.69 | 0.43 | 0.84 | 0.10 | 0.05 | Aldelby 19600 |
| 4 | 0.34 | 0.63 | 0.41 | 0.91 | 0.30 | 0.04 | Torport 25913 |
| 5 | 0.33 | 0.67 | 0.51 | 0.90 | 0.40 | 0.19 | Pickhaven 30896 |
| 6 | 0.31 | 0.69 | 0.41 | 0.86 | 0.50 | 0.14 | Penelden 31885 |
| 7 | 0.30 | 0.70 | 0.34 | 0.80 | 0.20 | 0.22 | Walesbourne 25450 |
| 8 | 0.36 | 0.71 | 0.44 | 0.86 | 0.40 | 0.00 | Alderham 32314 |
| 9 | 0.33 | 0.67 | 0.37 | 0.86 | 0.30 | 0.08 | Weneholt 34199 |
| 10 | 0.27 | 0.64 | 0.46 | 0.81 | 0.20 | 0.05 | Walsea 26606 |
| 11 | 0.41 | 0.76 | 0.54 | 0.90 | 0.70 | 0.05 | Torwick 25771 |
| 12 | 0.27 | 0.63 | 0.34 | 0.86 | 0.30 | 0.07 | Yarerport 27561 |
| 13 | 0.26 | 0.64 | 0.39 | 0.80 | 0.10 | 0.16 | Peningford 38959 |
| 14 | 0.23 | 0.57 | 0.30 | 0.77 | 0.30 | 0.22 | Kirkenford 17840 |
| 15 | 0.33 | 0.74 | 0.41 | 0.81 | 0.20 | 0.07 | Cranerden 20246 |
| 16 | 0.36 | 0.66 | 0.47 | 0.80 | 0.50 | 0.10 | Norenley 24793 |
| 17 | 0.33 | 0.76 | 0.46 | 0.84 | 0.70 | -0.11 | Brackden 28725 |
| 18 | 0.44 | 0.79 | 0.53 | 0.86 | 0.70 | 0.23 | Bromewick 37760 |
| 19 | 0.26 | 0.69 | 0.34 | 0.84 | 0.50 | -0.02 | Malport 20805 |
| 20 | 0.39 | 0.79 | 0.49 | 0.90 | 0.20 | 0.21 | Nedenford 35977 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 30% |
| Canterbury | 10000 | city | 25% |
| Colchester | 4000 | large_town | 45% |
| Sandwich | 4000 | large_town | 20% |
| Winchelsea | 3500 | large_town | 45% |
| Dover | 3000 | large_town | 45% |
| New Romney | 2500 | town | 85% |
| Rye | 2500 | town | 40% |
| Lewes | 2500 | town | 10% |
| Maidstone | 2000 | town | 45% |
| Hythe | 2000 | town | 35% |
| Hastings | 2000 | town | 45% |
| Maldon | 2000 | town | 80% |
| Faversham | 2000 | town | 50% |
| Rochester | 1900 | town | 60% |
| Lydd | 1500 | town | 70% |
| Chelmsford | 1500 | town | 25% |
| New Shoreham | 1500 | town | 5% |
| Battle | 1200 | town | 10% |
| Tonbridge | 1200 | town | 60% |
| Dartford | 1200 | town | 5% |
| Hertford | 1200 | town | 5% |
| Ware | 1200 | town | 5% |
| Croydon | 1000 | town | 30% |
| Reigate | 1000 | town | 30% |
| Folkestone | 1000 | town | 35% |
| Bishop's Stortford | 1000 | town | 0% |
| Seaford | 800 | small_town | 25% |
| East Grinstead | 800 | small_town | 20% |
| Milton Regis | 800 | small_town | 20% |
| Barking | 800 | small_town | 60% |
| Waltham Abbey | 800 | small_town | 40% |
| Pevensey | 600 | small_town | 55% |
| Fordwich | 600 | small_town | 30% |
| Tenterden | 600 | small_town | 10% |
| Ashford | 600 | small_town | 55% |
| Sevenoaks | 600 | small_town | 50% |
| West Malling | 600 | small_town | 45% |
| Cranbrook | 600 | small_town | 30% |
| Gravesend | 600 | small_town | 50% |
| Romford | 600 | small_town | 10% |
| Coggeshall | 600 | small_town | 40% |
| Braintree | 600 | small_town | 35% |
| Great Dunmow | 600 | small_town | 25% |
| Wye | 500 | small_town | 60% |
| Witham | 500 | small_town | 40% |
| Brentwood | 500 | small_town | 35% |
| Chipping Barnet | 500 | small_town | 0% |
| Bletchingley | 500 | small_town | 25% |
| Westerham | 400 | small_town | 40% |
| Charing | 400 | small_town | 45% |
| Wrotham | 400 | small_town | 5% |
| Lenham | 400 | small_town | 40% |
| Elham | 400 | small_town | 15% |
| Wingham | 400 | small_town | 30% |
| Bromley | 400 | small_town | 10% |
| Robertsbridge | 400 | small_town | 40% |
| Hailsham | 400 | small_town | 25% |
| Mayfield | 400 | small_town | 10% |
| Cuckfield | 400 | small_town | 15% |
| Crawley | 400 | small_town | 25% |
| Epping | 400 | small_town | 20% |
| Billericay | 400 | small_town | 25% |
| Rayleigh | 400 | small_town | 35% |
| Rochford | 400 | small_town | 35% |
| Harlow | 400 | small_town | 15% |
| Chipping Ongar | 400 | small_town | 35% |
| Hoddesdon | 400 | small_town | 35% |
| Sawbridgeworth | 400 | small_town | 15% |
| Hatfield | 400 | small_town | 5% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 107 | 6 | 75% | Colchester | 25 | 8 |
| 96 | 15 | 55% | Colchester | 19 | 1 |
| 3 | 47 | 55% | London (City) | 11 | 7 |
| 83 | 24 | 50% | Maldon | 17 | 4 |
