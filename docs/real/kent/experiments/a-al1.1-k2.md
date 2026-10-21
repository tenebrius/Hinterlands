# Real-terrain scores: kent

## All 70 real towns against the model's top 70

20 seeds, final year; random baselines drop the same 70 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.32 | 0.37 | 0.30 | 0% |
| Real towns with a model place within 10 km | 0.71 | 0.82 | 0.65 | 0% |
| Model places with a real town within 5 km | 0.39 | 0.41 | 0.38 | 5% |
| Model places with a real town within 10 km | 0.78 | 0.90 | 0.85 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.30 | 0.29 | 0.38 | 0% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.14 | -0.03 | 0.20 | 50% |

## The 27 real towns of 1,000 or more against the model's top 27

20 seeds, final year; random baselines drop the same 27 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.19 | 0.15 | 0.17 | 0% |
| Real towns with a model place within 10 km | 0.47 | 0.45 | 0.46 | 10% |
| Model places with a real town within 5 km | 0.17 | 0.15 | 0.18 | 0% |
| Model places with a real town within 10 km | 0.44 | 0.48 | 0.52 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.30 | 0.29 | 0.39 | 0% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.01 | 0.01 | 0.14 | 0% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (70) | 0.57 | 0.49 | 0.18 | 0.13 |
| 1,000 or more (27) | 0.68 | 0.61 | 0.26 | 0.13 |
| Under 1,000 (43) | 0.50 | 0.41 | 0.13 | 0.13 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 1; any top-70 place within 10 km: 13. Median distance from the model's largest place to London (City): 66 km.

Nearest real town to the model's largest place, by seeds: Gravesend 6, Rochford 3, Milton Regis 3, Fordwich 2, Colchester 2, Sandwich 2, Wingham 1, Canterbury 1.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 18 | 26% | 21.7 | 31% | 37% |
| coast | 6 | 9% | 24.4 | 35% | 52% |
| inland | 46 | 66% | 23.9 | 34% | 27% |

Inland places over 500 people, nearest other place over 500 (median): real 10.6 km (29 towns); model 13.7 km (median over seeds; 10 such places per seed).

Mean nearest-neighbour spacing of the 70 places: real 8.6 km, model 6.3 km, random land 6.9 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.29 | 0.74 | 0.37 | 0.76 | 0.50 | 0.20 | Barmouth 20393 |
| 2 | 0.36 | 0.71 | 0.44 | 0.80 | 0.30 | 0.19 | Bradeley 10498 |
| 3 | 0.34 | 0.80 | 0.47 | 0.84 | 0.50 | 0.09 | Oakesmouth 14249 |
| 4 | 0.36 | 0.76 | 0.37 | 0.74 | 0.10 | 0.00 | Thornemouth 23964 |
| 5 | 0.27 | 0.71 | 0.27 | 0.73 | 0.20 | 0.24 | Chedford 19704 |
| 6 | 0.34 | 0.67 | 0.43 | 0.77 | 0.10 | 0.04 | Toreport 15326 |
| 7 | 0.30 | 0.74 | 0.43 | 0.81 | 0.20 | -0.00 | Bradhurst 12142 |
| 8 | 0.23 | 0.61 | 0.30 | 0.74 | 0.20 | -0.09 | Burenham 12578 |
| 9 | 0.36 | 0.70 | 0.41 | 0.81 | 0.40 | 0.29 | Glenhaven 15138 |
| 10 | 0.30 | 0.79 | 0.39 | 0.81 | 0.50 | 0.20 | Shapenmouth 28149 |
| 11 | 0.34 | 0.74 | 0.40 | 0.76 | 0.10 | 0.04 | Veransea 10556 |
| 12 | 0.20 | 0.70 | 0.33 | 0.73 | 0.30 | 0.17 | Penport 13961 |
| 13 | 0.33 | 0.73 | 0.33 | 0.73 | 0.40 | 0.26 | Torhaven 12952 |
| 14 | 0.30 | 0.69 | 0.34 | 0.80 | 0.40 | 0.03 | Pickport 21600 |
| 15 | 0.40 | 0.70 | 0.46 | 0.84 | 0.40 | 0.18 | Roanhurst 7066 |
| 16 | 0.26 | 0.70 | 0.31 | 0.76 | 0.40 | 0.15 | Linport 25992 |
| 17 | 0.33 | 0.67 | 0.40 | 0.73 | 0.30 | 0.21 | Shapesford 17216 |
| 18 | 0.40 | 0.70 | 0.53 | 0.84 | 0.10 | 0.21 | Glenhaven 9553 |
| 19 | 0.30 | 0.74 | 0.31 | 0.79 | 0.50 | 0.19 | Winsesmouth 21656 |
| 20 | 0.34 | 0.67 | 0.41 | 0.74 | 0.10 | 0.10 | Ickenmouth 24001 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 5% |
| Canterbury | 10000 | city | 45% |
| Colchester | 4000 | large_town | 10% |
| Sandwich | 4000 | large_town | 70% |
| Winchelsea | 3500 | large_town | 45% |
| Dover | 3000 | large_town | 75% |
| New Romney | 2500 | town | 20% |
| Rye | 2500 | town | 50% |
| Lewes | 2500 | town | 20% |
| Maidstone | 2000 | town | 50% |
| Hythe | 2000 | town | 95% |
| Hastings | 2000 | town | 40% |
| Maldon | 2000 | town | 40% |
| Faversham | 2000 | town | 80% |
| Rochester | 1900 | town | 60% |
| Lydd | 1500 | town | 25% |
| Chelmsford | 1500 | town | 35% |
| New Shoreham | 1500 | town | 35% |
| Battle | 1200 | town | 0% |
| Tonbridge | 1200 | town | 45% |
| Dartford | 1200 | town | 20% |
| Hertford | 1200 | town | 40% |
| Ware | 1200 | town | 40% |
| Croydon | 1000 | town | 50% |
| Reigate | 1000 | town | 25% |
| Folkestone | 1000 | town | 65% |
| Bishop's Stortford | 1000 | town | 20% |
| Seaford | 800 | small_town | 35% |
| East Grinstead | 800 | small_town | 0% |
| Milton Regis | 800 | small_town | 50% |
| Barking | 800 | small_town | 40% |
| Waltham Abbey | 800 | small_town | 30% |
| Pevensey | 600 | small_town | 55% |
| Fordwich | 600 | small_town | 20% |
| Tenterden | 600 | small_town | 0% |
| Ashford | 600 | small_town | 40% |
| Sevenoaks | 600 | small_town | 75% |
| West Malling | 600 | small_town | 50% |
| Cranbrook | 600 | small_town | 0% |
| Gravesend | 600 | small_town | 35% |
| Romford | 600 | small_town | 0% |
| Coggeshall | 600 | small_town | 70% |
| Braintree | 600 | small_town | 0% |
| Great Dunmow | 600 | small_town | 40% |
| Wye | 500 | small_town | 25% |
| Witham | 500 | small_town | 40% |
| Brentwood | 500 | small_town | 0% |
| Chipping Barnet | 500 | small_town | 0% |
| Bletchingley | 500 | small_town | 0% |
| Westerham | 400 | small_town | 0% |
| Charing | 400 | small_town | 0% |
| Wrotham | 400 | small_town | 0% |
| Lenham | 400 | small_town | 0% |
| Elham | 400 | small_town | 0% |
| Wingham | 400 | small_town | 10% |
| Bromley | 400 | small_town | 40% |
| Robertsbridge | 400 | small_town | 15% |
| Hailsham | 400 | small_town | 55% |
| Mayfield | 400 | small_town | 15% |
| Cuckfield | 400 | small_town | 30% |
| Crawley | 400 | small_town | 30% |
| Epping | 400 | small_town | 0% |
| Billericay | 400 | small_town | 30% |
| Rayleigh | 400 | small_town | 35% |
| Rochford | 400 | small_town | 30% |
| Harlow | 400 | small_town | 60% |
| Chipping Ongar | 400 | small_town | 30% |
| Hoddesdon | 400 | small_town | 35% |
| Sawbridgeworth | 400 | small_town | 50% |
| Hatfield | 400 | small_town | 45% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 110 | 6 | 95% | Colchester | 28 | -2 |
| 105 | 2 | 95% | Colchester | 23 | -0 |
| 118 | 59 | 85% | Sandwich | 12 | 41 |
| 104 | 11 | 85% | Colchester | 23 | 2 |
| 98 | 15 | 85% | Colchester | 20 | 1 |
| 89 | 11 | 75% | Colchester | 11 | 1 |
| 30 | 89 | 60% | East Grinstead | 12 | 70 |
| 79 | 36 | 55% | Rochford | 10 | 0 |
