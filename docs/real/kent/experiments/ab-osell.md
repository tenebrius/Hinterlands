# Real-terrain scores: kent

## All 70 real towns against the model's top 70

20 seeds, final year; random baselines drop the same 70 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.34 | 0.37 | 0.30 | 0% |
| Real towns with a model place within 10 km | 0.73 | 0.82 | 0.65 | 0% |
| Model places with a real town within 5 km | 0.41 | 0.41 | 0.38 | 5% |
| Model places with a real town within 10 km | 0.84 | 0.90 | 0.85 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.33 | 0.29 | 0.38 | 5% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.16 | -0.03 | 0.20 | 45% |

## The 27 real towns of 1,000 or more against the model's top 27

20 seeds, final year; random baselines drop the same 27 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.22 | 0.15 | 0.17 | 10% |
| Real towns with a model place within 10 km | 0.51 | 0.45 | 0.46 | 5% |
| Model places with a real town within 5 km | 0.21 | 0.15 | 0.18 | 10% |
| Model places with a real town within 10 km | 0.51 | 0.48 | 0.52 | 10% |
| Real top 10 with a model top-15 place within 10 km | 0.33 | 0.29 | 0.39 | 5% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.06 | 0.01 | 0.14 | 10% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (70) | 0.57 | 0.48 | 0.18 | 0.14 |
| 1,000 or more (27) | 0.70 | 0.61 | 0.27 | 0.14 |
| Under 1,000 (43) | 0.49 | 0.39 | 0.13 | 0.14 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 3; any top-70 place within 10 km: 19. Median distance from the model's largest place to London (City): 39 km. Largest place's share of the region's people (median): 0.29.

Nearest real town to the model's largest place, by seeds: Gravesend 16, Milton Regis 1, Dartford 1, Barking 1, Rochford 1.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 18 | 26% | 37.7 | 54% | 53% |
| coast | 6 | 9% | 14.2 | 20% | 41% |
| inland | 46 | 66% | 18.1 | 26% | 26% |

Inland places over 500 people, nearest other place over 500 (median): real 10.6 km (29 towns); model 10.3 km (median over seeds; 10 such places per seed).

Mean nearest-neighbour spacing of the 70 places: real 8.6 km, model 6.5 km, random land 6.9 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.39 | 0.76 | 0.43 | 0.81 | 0.30 | 0.21 | Barmouth 26914 |
| 2 | 0.31 | 0.70 | 0.39 | 0.83 | 0.30 | 0.10 | Barmouth 29405 |
| 3 | 0.27 | 0.63 | 0.31 | 0.79 | 0.10 | 0.20 | Langerport 24489 |
| 4 | 0.27 | 0.67 | 0.39 | 0.80 | 0.40 | 0.18 | Ewenport 23746 |
| 5 | 0.43 | 0.74 | 0.51 | 0.86 | 0.30 | 0.10 | Langerport 25351 |
| 6 | 0.31 | 0.71 | 0.41 | 0.84 | 0.70 | 0.22 | Langerport 23671 |
| 7 | 0.30 | 0.76 | 0.34 | 0.87 | 0.40 | 0.17 | Ashenmouth 29266 |
| 8 | 0.33 | 0.70 | 0.40 | 0.71 | 0.50 | 0.16 | Wexford 20439 |
| 9 | 0.33 | 0.77 | 0.37 | 0.90 | 0.40 | 0.02 | Barmouth 34841 |
| 10 | 0.37 | 0.77 | 0.37 | 0.83 | 0.30 | -0.00 | Shapenmouth 25541 |
| 11 | 0.39 | 0.79 | 0.49 | 0.86 | 0.50 | 0.09 | Barmouth 27647 |
| 12 | 0.40 | 0.83 | 0.44 | 0.89 | 0.30 | 0.17 | Glenelbourne 16232 |
| 13 | 0.24 | 0.67 | 0.31 | 0.69 | 0.20 | 0.10 | Chalelhaven 22210 |
| 14 | 0.36 | 0.70 | 0.44 | 0.89 | 0.30 | 0.06 | Calingmouth 17657 |
| 15 | 0.36 | 0.81 | 0.37 | 0.89 | 0.10 | 0.37 | Barmouth 28897 |
| 16 | 0.34 | 0.66 | 0.39 | 0.80 | 0.30 | 0.12 | Falmouth 23505 |
| 17 | 0.34 | 0.79 | 0.44 | 0.84 | 0.40 | 0.15 | Pickingmouth 26319 |
| 18 | 0.33 | 0.73 | 0.39 | 0.87 | 0.30 | 0.19 | Falmouth 29334 |
| 19 | 0.40 | 0.74 | 0.49 | 0.91 | 0.30 | 0.16 | Barmouth 27056 |
| 20 | 0.34 | 0.71 | 0.43 | 0.90 | 0.30 | 0.34 | Wyamouth 34441 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 50% |
| Canterbury | 10000 | city | 40% |
| Colchester | 4000 | large_town | 10% |
| Sandwich | 4000 | large_town | 25% |
| Winchelsea | 3500 | large_town | 50% |
| Dover | 3000 | large_town | 35% |
| New Romney | 2500 | town | 60% |
| Rye | 2500 | town | 50% |
| Lewes | 2500 | town | 10% |
| Maidstone | 2000 | town | 50% |
| Hythe | 2000 | town | 55% |
| Hastings | 2000 | town | 35% |
| Maldon | 2000 | town | 60% |
| Faversham | 2000 | town | 55% |
| Rochester | 1900 | town | 65% |
| Lydd | 1500 | town | 45% |
| Chelmsford | 1500 | town | 50% |
| New Shoreham | 1500 | town | 30% |
| Battle | 1200 | town | 0% |
| Tonbridge | 1200 | town | 60% |
| Dartford | 1200 | town | 55% |
| Hertford | 1200 | town | 85% |
| Ware | 1200 | town | 75% |
| Croydon | 1000 | town | 45% |
| Reigate | 1000 | town | 10% |
| Folkestone | 1000 | town | 45% |
| Bishop's Stortford | 1000 | town | 20% |
| Seaford | 800 | small_town | 30% |
| East Grinstead | 800 | small_town | 0% |
| Milton Regis | 800 | small_town | 40% |
| Barking | 800 | small_town | 75% |
| Waltham Abbey | 800 | small_town | 70% |
| Pevensey | 600 | small_town | 70% |
| Fordwich | 600 | small_town | 25% |
| Tenterden | 600 | small_town | 0% |
| Ashford | 600 | small_town | 60% |
| Sevenoaks | 600 | small_town | 35% |
| West Malling | 600 | small_town | 35% |
| Cranbrook | 600 | small_town | 0% |
| Gravesend | 600 | small_town | 15% |
| Romford | 600 | small_town | 0% |
| Coggeshall | 600 | small_town | 55% |
| Braintree | 600 | small_town | 0% |
| Great Dunmow | 600 | small_town | 55% |
| Wye | 500 | small_town | 20% |
| Witham | 500 | small_town | 45% |
| Brentwood | 500 | small_town | 0% |
| Chipping Barnet | 500 | small_town | 0% |
| Bletchingley | 500 | small_town | 0% |
| Westerham | 400 | small_town | 0% |
| Charing | 400 | small_town | 0% |
| Wrotham | 400 | small_town | 0% |
| Lenham | 400 | small_town | 0% |
| Elham | 400 | small_town | 0% |
| Wingham | 400 | small_town | 15% |
| Bromley | 400 | small_town | 55% |
| Robertsbridge | 400 | small_town | 25% |
| Hailsham | 400 | small_town | 25% |
| Mayfield | 400 | small_town | 20% |
| Cuckfield | 400 | small_town | 20% |
| Crawley | 400 | small_town | 10% |
| Epping | 400 | small_town | 50% |
| Billericay | 400 | small_town | 45% |
| Rayleigh | 400 | small_town | 45% |
| Rochford | 400 | small_town | 10% |
| Harlow | 400 | small_town | 35% |
| Chipping Ongar | 400 | small_town | 55% |
| Hoddesdon | 400 | small_town | 60% |
| Sawbridgeworth | 400 | small_town | 45% |
| Hatfield | 400 | small_town | 70% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 107 | 5 | 85% | Colchester | 25 | 1 |
| 4 | 48 | 85% | London (City) | 10 | 5 |
| 104 | 11 | 70% | Colchester | 23 | 2 |
| 85 | 24 | 65% | Maldon | 19 | 1 |
