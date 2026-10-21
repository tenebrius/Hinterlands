# Real-terrain scores: kent

## All 70 real towns against the model's top 70

20 seeds, final year; random baselines drop the same 70 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.34 | 0.37 | 0.30 | 0% |
| Real towns with a model place within 10 km | 0.80 | 0.82 | 0.65 | 5% |
| Model places with a real town within 5 km | 0.42 | 0.41 | 0.38 | 0% |
| Model places with a real town within 10 km | 0.87 | 0.90 | 0.85 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.35 | 0.29 | 0.38 | 0% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.05 | -0.03 | 0.20 | 10% |

## The 27 real towns of 1,000 or more against the model's top 27

20 seeds, final year; random baselines drop the same 27 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.19 | 0.15 | 0.17 | 5% |
| Real towns with a model place within 10 km | 0.52 | 0.45 | 0.46 | 15% |
| Model places with a real town within 5 km | 0.19 | 0.15 | 0.18 | 0% |
| Model places with a real town within 10 km | 0.50 | 0.48 | 0.52 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.35 | 0.29 | 0.39 | 0% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.04 | 0.01 | 0.14 | 5% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (70) | 0.55 | 0.46 | 0.19 | 0.15 |
| 1,000 or more (27) | 0.62 | 0.53 | 0.21 | 0.15 |
| Under 1,000 (43) | 0.51 | 0.42 | 0.17 | 0.15 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 5; any top-70 place within 10 km: 19. Median distance from the model's largest place to London (City): 56 km.

Nearest real town to the model's largest place, by seeds: Maldon 4, Rayleigh 4, Rochford 4, Gravesend 4, Colchester 2, Milton Regis 1, Faversham 1.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 18 | 26% | 32.4 | 46% | 54% |
| coast | 6 | 9% | 9.7 | 14% | 25% |
| inland | 46 | 66% | 27.9 | 40% | 28% |

Mean nearest-neighbour spacing of the 70 places: real 8.6 km, model 6.9 km, random land 6.9 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.36 | 0.90 | 0.39 | 0.91 | 0.20 | 0.02 | Ewansea 20056 |
| 2 | 0.33 | 0.73 | 0.34 | 0.79 | 0.20 | 0.19 | Thethaven 19614 |
| 3 | 0.39 | 0.87 | 0.44 | 0.89 | 0.40 | 0.01 | Halsea 16684 |
| 4 | 0.39 | 0.80 | 0.43 | 0.87 | 0.50 | -0.06 | Normouth 25897 |
| 5 | 0.36 | 0.83 | 0.46 | 0.86 | 0.30 | -0.06 | Hartsea 13342 |
| 6 | 0.31 | 0.71 | 0.33 | 0.80 | 0.30 | 0.10 | Sutanport 24824 |
| 7 | 0.37 | 0.81 | 0.44 | 0.87 | 0.30 | 0.15 | Belanhaven 16868 |
| 8 | 0.31 | 0.71 | 0.47 | 0.87 | 0.50 | 0.17 | Iselsea 15575 |
| 9 | 0.37 | 0.80 | 0.47 | 0.89 | 0.30 | -0.05 | Halingsea 26076 |
| 10 | 0.30 | 0.84 | 0.44 | 0.94 | 0.40 | -0.08 | Buringwick 17822 |
| 11 | 0.31 | 0.80 | 0.41 | 0.83 | 0.40 | -0.16 | Cranerport 14605 |
| 12 | 0.34 | 0.80 | 0.43 | 0.90 | 0.50 | 0.13 | Langesea 11251 |
| 13 | 0.36 | 0.81 | 0.46 | 0.89 | 0.40 | 0.05 | Harnelsea 26963 |
| 14 | 0.33 | 0.77 | 0.41 | 0.87 | 0.20 | 0.05 | Iselport 14828 |
| 15 | 0.34 | 0.83 | 0.39 | 0.83 | 0.50 | 0.20 | Kirkport 16460 |
| 16 | 0.40 | 0.81 | 0.39 | 0.86 | 0.50 | 0.15 | Penensea 16167 |
| 17 | 0.30 | 0.79 | 0.40 | 0.84 | 0.10 | 0.09 | Hartsea 12756 |
| 18 | 0.37 | 0.79 | 0.44 | 0.87 | 0.40 | 0.08 | Ulowmouth 16198 |
| 19 | 0.33 | 0.81 | 0.46 | 0.86 | 0.30 | 0.08 | Talelport 19371 |
| 20 | 0.27 | 0.76 | 0.41 | 0.89 | 0.20 | -0.03 | Ludingport 17809 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 20% |
| Canterbury | 10000 | city | 55% |
| Colchester | 4000 | large_town | 5% |
| Sandwich | 4000 | large_town | 55% |
| Winchelsea | 3500 | large_town | 35% |
| Dover | 3000 | large_town | 20% |
| New Romney | 2500 | town | 70% |
| Rye | 2500 | town | 25% |
| Lewes | 2500 | town | 50% |
| Maidstone | 2000 | town | 65% |
| Hythe | 2000 | town | 35% |
| Hastings | 2000 | town | 5% |
| Maldon | 2000 | town | 55% |
| Faversham | 2000 | town | 65% |
| Rochester | 1900 | town | 75% |
| Lydd | 1500 | town | 30% |
| Chelmsford | 1500 | town | 50% |
| New Shoreham | 1500 | town | 30% |
| Battle | 1200 | town | 0% |
| Tonbridge | 1200 | town | 70% |
| Dartford | 1200 | town | 40% |
| Hertford | 1200 | town | 25% |
| Ware | 1200 | town | 15% |
| Croydon | 1000 | town | 70% |
| Reigate | 1000 | town | 15% |
| Folkestone | 1000 | town | 15% |
| Bishop's Stortford | 1000 | town | 30% |
| Seaford | 800 | small_town | 30% |
| East Grinstead | 800 | small_town | 0% |
| Milton Regis | 800 | small_town | 45% |
| Barking | 800 | small_town | 50% |
| Waltham Abbey | 800 | small_town | 45% |
| Pevensey | 600 | small_town | 80% |
| Fordwich | 600 | small_town | 45% |
| Tenterden | 600 | small_town | 0% |
| Ashford | 600 | small_town | 75% |
| Sevenoaks | 600 | small_town | 50% |
| West Malling | 600 | small_town | 35% |
| Cranbrook | 600 | small_town | 0% |
| Gravesend | 600 | small_town | 55% |
| Romford | 600 | small_town | 0% |
| Coggeshall | 600 | small_town | 60% |
| Braintree | 600 | small_town | 0% |
| Great Dunmow | 600 | small_town | 10% |
| Wye | 500 | small_town | 35% |
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
| Robertsbridge | 400 | small_town | 70% |
| Hailsham | 400 | small_town | 70% |
| Mayfield | 400 | small_town | 10% |
| Cuckfield | 400 | small_town | 60% |
| Crawley | 400 | small_town | 30% |
| Epping | 400 | small_town | 5% |
| Billericay | 400 | small_town | 45% |
| Rayleigh | 400 | small_town | 45% |
| Rochford | 400 | small_town | 10% |
| Harlow | 400 | small_town | 35% |
| Chipping Ongar | 400 | small_town | 40% |
| Hoddesdon | 400 | small_town | 50% |
| Sawbridgeworth | 400 | small_town | 55% |
| Hatfield | 400 | small_town | 75% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 31 | 87 | 75% | Tonbridge | 11 | 81 |
| 105 | 10 | 70% | Colchester | 24 | 0 |
| 64 | 55 | 70% | Rochester | 10 | -2 |
| 104 | 60 | 60% | Fordwich | 10 | 2 |
| 89 | 10 | 60% | Colchester | 11 | 3 |
| 2 | 62 | 60% | Croydon | 11 | 39 |
| 4 | 48 | 55% | London (City) | 10 | 5 |
| 54 | 45 | 50% | Gravesend | 11 | -1 |
| 3 | 41 | 50% | London (City) | 10 | 48 |
