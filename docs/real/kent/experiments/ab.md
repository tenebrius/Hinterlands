# Real-terrain scores: kent

## All 70 real towns against the model's top 70

20 seeds, final year; random baselines drop the same 70 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.36 | 0.37 | 0.30 | 10% |
| Real towns with a model place within 10 km | 0.76 | 0.82 | 0.65 | 0% |
| Model places with a real town within 5 km | 0.43 | 0.41 | 0.38 | 10% |
| Model places with a real town within 10 km | 0.86 | 0.90 | 0.85 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.35 | 0.29 | 0.38 | 15% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.15 | -0.03 | 0.20 | 45% |

## The 27 real towns of 1,000 or more against the model's top 27

20 seeds, final year; random baselines drop the same 27 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.23 | 0.15 | 0.17 | 10% |
| Real towns with a model place within 10 km | 0.51 | 0.45 | 0.46 | 15% |
| Model places with a real town within 5 km | 0.22 | 0.15 | 0.18 | 5% |
| Model places with a real town within 10 km | 0.50 | 0.48 | 0.52 | 5% |
| Real top 10 with a model top-15 place within 10 km | 0.35 | 0.29 | 0.39 | 15% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.01 | 0.01 | 0.14 | 5% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (70) | 0.58 | 0.49 | 0.21 | 0.14 |
| 1,000 or more (27) | 0.69 | 0.61 | 0.28 | 0.14 |
| Under 1,000 (43) | 0.51 | 0.42 | 0.17 | 0.14 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 2; any top-70 place within 10 km: 19. Median distance from the model's largest place to London (City): 39 km. Largest place's share of the region's people (median): 0.28.

Nearest real town to the model's largest place, by seeds: Gravesend 15, Chipping Ongar 1, Dartford 1, Barking 1, Canterbury 1, Milton Regis 1.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 18 | 26% | 37.7 | 54% | 54% |
| coast | 6 | 9% | 12.9 | 18% | 43% |
| inland | 46 | 66% | 19.4 | 28% | 27% |

Inland places over 500 people, nearest other place over 500 (median): real 10.6 km (29 towns); model 10.3 km (median over seeds; 11 such places per seed).

Mean nearest-neighbour spacing of the 70 places: real 8.6 km, model 6.5 km, random land 6.9 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.43 | 0.87 | 0.50 | 0.91 | 0.50 | 0.19 | Barmouth 31856 |
| 2 | 0.37 | 0.80 | 0.40 | 0.93 | 0.40 | 0.22 | Barmouth 28841 |
| 3 | 0.46 | 0.83 | 0.57 | 0.91 | 0.70 | 0.15 | Langerport 30967 |
| 4 | 0.33 | 0.77 | 0.37 | 0.86 | 0.70 | 0.34 | Thornemouth 27350 |
| 5 | 0.37 | 0.74 | 0.46 | 0.84 | 0.50 | 0.17 | Langerport 19852 |
| 6 | 0.31 | 0.74 | 0.43 | 0.84 | 0.50 | 0.18 | Langerport 26470 |
| 7 | 0.34 | 0.80 | 0.41 | 0.84 | 0.10 | 0.03 | Ashenmouth 26963 |
| 8 | 0.34 | 0.63 | 0.37 | 0.84 | 0.30 | 0.42 | Shapenmouth 29217 |
| 9 | 0.37 | 0.83 | 0.50 | 0.89 | 0.20 | 0.08 | Barmouth 27243 |
| 10 | 0.36 | 0.66 | 0.47 | 0.80 | 0.40 | 0.08 | Elmanden 14453 |
| 11 | 0.39 | 0.74 | 0.50 | 0.87 | 0.20 | 0.06 | Barmouth 28809 |
| 12 | 0.33 | 0.74 | 0.40 | 0.87 | 0.10 | 0.06 | Ashenmouth 24786 |
| 13 | 0.36 | 0.76 | 0.43 | 0.86 | 0.30 | -0.04 | Fenford 10753 |
| 14 | 0.31 | 0.73 | 0.43 | 0.83 | 0.30 | 0.18 | Kinenhurst 13715 |
| 15 | 0.33 | 0.79 | 0.37 | 0.84 | 0.10 | 0.12 | Barmouth 25329 |
| 16 | 0.30 | 0.73 | 0.34 | 0.81 | 0.60 | 0.20 | Loxey 8842 |
| 17 | 0.47 | 0.79 | 0.54 | 0.86 | 0.30 | 0.18 | Leyholt 13578 |
| 18 | 0.31 | 0.67 | 0.40 | 0.84 | 0.30 | 0.21 | Loxesmouth 18200 |
| 19 | 0.36 | 0.81 | 0.40 | 0.87 | 0.30 | 0.15 | Barmouth 30715 |
| 20 | 0.30 | 0.76 | 0.37 | 0.89 | 0.20 | 0.12 | Wyamouth 31269 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 35% |
| Canterbury | 10000 | city | 35% |
| Colchester | 4000 | large_town | 0% |
| Sandwich | 4000 | large_town | 35% |
| Winchelsea | 3500 | large_town | 50% |
| Dover | 3000 | large_town | 35% |
| New Romney | 2500 | town | 60% |
| Rye | 2500 | town | 45% |
| Lewes | 2500 | town | 15% |
| Maidstone | 2000 | town | 60% |
| Hythe | 2000 | town | 70% |
| Hastings | 2000 | town | 55% |
| Maldon | 2000 | town | 70% |
| Faversham | 2000 | town | 55% |
| Rochester | 1900 | town | 50% |
| Lydd | 1500 | town | 45% |
| Chelmsford | 1500 | town | 75% |
| New Shoreham | 1500 | town | 20% |
| Battle | 1200 | town | 0% |
| Tonbridge | 1200 | town | 90% |
| Dartford | 1200 | town | 35% |
| Hertford | 1200 | town | 70% |
| Ware | 1200 | town | 55% |
| Croydon | 1000 | town | 35% |
| Reigate | 1000 | town | 10% |
| Folkestone | 1000 | town | 45% |
| Bishop's Stortford | 1000 | town | 50% |
| Seaford | 800 | small_town | 20% |
| East Grinstead | 800 | small_town | 0% |
| Milton Regis | 800 | small_town | 75% |
| Barking | 800 | small_town | 40% |
| Waltham Abbey | 800 | small_town | 60% |
| Pevensey | 600 | small_town | 55% |
| Fordwich | 600 | small_town | 40% |
| Tenterden | 600 | small_town | 0% |
| Ashford | 600 | small_town | 55% |
| Sevenoaks | 600 | small_town | 55% |
| West Malling | 600 | small_town | 55% |
| Cranbrook | 600 | small_town | 0% |
| Gravesend | 600 | small_town | 30% |
| Romford | 600 | small_town | 0% |
| Coggeshall | 600 | small_town | 70% |
| Braintree | 600 | small_town | 0% |
| Great Dunmow | 600 | small_town | 40% |
| Wye | 500 | small_town | 10% |
| Witham | 500 | small_town | 70% |
| Brentwood | 500 | small_town | 0% |
| Chipping Barnet | 500 | small_town | 0% |
| Bletchingley | 500 | small_town | 0% |
| Westerham | 400 | small_town | 0% |
| Charing | 400 | small_town | 5% |
| Wrotham | 400 | small_town | 0% |
| Lenham | 400 | small_town | 0% |
| Elham | 400 | small_town | 0% |
| Wingham | 400 | small_town | 20% |
| Bromley | 400 | small_town | 40% |
| Robertsbridge | 400 | small_town | 35% |
| Hailsham | 400 | small_town | 35% |
| Mayfield | 400 | small_town | 15% |
| Cuckfield | 400 | small_town | 20% |
| Crawley | 400 | small_town | 5% |
| Epping | 400 | small_town | 20% |
| Billericay | 400 | small_town | 50% |
| Rayleigh | 400 | small_town | 60% |
| Rochford | 400 | small_town | 15% |
| Harlow | 400 | small_town | 40% |
| Chipping Ongar | 400 | small_town | 60% |
| Hoddesdon | 400 | small_town | 70% |
| Sawbridgeworth | 400 | small_town | 70% |
| Hatfield | 400 | small_town | 65% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 107 | 6 | 75% | Colchester | 25 | 8 |
| 66 | 53 | 75% | Milton Regis | 11 | -10 |
| 62 | 81 | 70% | Lenham | 10 | 24 |
| 4 | 48 | 70% | London (City) | 10 | 5 |
| 47 | 88 | 60% | Cranbrook | 10 | 94 |
| 120 | 60 | 50% | Sandwich | 13 | 31 |
