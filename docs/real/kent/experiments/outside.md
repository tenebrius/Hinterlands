# Real-terrain scores: kent

## All 70 real towns against the model's top 70

20 seeds, final year; random baselines drop the same 70 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.34 | 0.37 | 0.30 | 0% |
| Real towns with a model place within 10 km | 0.78 | 0.82 | 0.65 | 0% |
| Model places with a real town within 5 km | 0.40 | 0.41 | 0.38 | 5% |
| Model places with a real town within 10 km | 0.83 | 0.90 | 0.85 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.37 | 0.29 | 0.38 | 0% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.12 | -0.03 | 0.20 | 35% |

## The 27 real towns of 1,000 or more against the model's top 27

20 seeds, final year; random baselines drop the same 27 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.21 | 0.15 | 0.17 | 5% |
| Real towns with a model place within 10 km | 0.51 | 0.45 | 0.46 | 0% |
| Model places with a real town within 5 km | 0.20 | 0.15 | 0.18 | 5% |
| Model places with a real town within 10 km | 0.50 | 0.48 | 0.52 | 5% |
| Real top 10 with a model top-15 place within 10 km | 0.37 | 0.29 | 0.39 | 0% |
| Rank agreement (Spearman, real size vs model size within 5 km) | -0.01 | 0.01 | 0.14 | 10% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (70) | 0.57 | 0.49 | 0.20 | 0.14 |
| 1,000 or more (27) | 0.69 | 0.61 | 0.26 | 0.14 |
| Under 1,000 (43) | 0.50 | 0.41 | 0.16 | 0.14 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 3; any top-70 place within 10 km: 18. Median distance from the model's largest place to London (City): 39 km.

Nearest real town to the model's largest place, by seeds: Cuckfield 5, Hoddesdon 4, East Grinstead 3, London (City) 2, Lewes 2, Ware 1, Reigate 1, Tonbridge 1, Hailsham 1.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 18 | 26% | 32.6 | 47% | 59% |
| coast | 6 | 9% | 14.6 | 21% | 44% |
| inland | 46 | 66% | 22.8 | 32% | 23% |

Mean nearest-neighbour spacing of the 70 places: real 8.6 km, model 6.6 km, random land 6.9 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.31 | 0.77 | 0.39 | 0.84 | 0.50 | 0.35 | Fairenford 3473 |
| 2 | 0.36 | 0.76 | 0.43 | 0.84 | 0.40 | 0.31 | Thetenbourne 2989 |
| 3 | 0.37 | 0.80 | 0.41 | 0.86 | 0.40 | 0.13 | Winsford 3641 |
| 4 | 0.33 | 0.76 | 0.34 | 0.84 | 0.30 | 0.20 | Everford 4975 |
| 5 | 0.36 | 0.79 | 0.43 | 0.86 | 0.50 | -0.00 | Norham 4495 |
| 6 | 0.36 | 0.76 | 0.43 | 0.80 | 0.30 | 0.29 | Astford 3095 |
| 7 | 0.33 | 0.74 | 0.39 | 0.86 | 0.10 | 0.11 | Rayholt 3025 |
| 8 | 0.33 | 0.83 | 0.37 | 0.86 | 0.40 | -0.14 | Calford 5890 |
| 9 | 0.37 | 0.76 | 0.44 | 0.87 | 0.30 | -0.02 | Penbourne 2767 |
| 10 | 0.26 | 0.81 | 0.31 | 0.80 | 0.40 | 0.13 | Merebourne 3272 |
| 11 | 0.36 | 0.77 | 0.46 | 0.87 | 0.40 | 0.10 | Melaney 3928 |
| 12 | 0.29 | 0.83 | 0.34 | 0.80 | 0.40 | 0.33 | Fairingey 3852 |
| 13 | 0.41 | 0.77 | 0.51 | 0.80 | 0.40 | 0.10 | Loxeney 5108 |
| 14 | 0.40 | 0.80 | 0.41 | 0.77 | 0.50 | 0.18 | Burwick 3873 |
| 15 | 0.37 | 0.81 | 0.39 | 0.89 | 0.30 | 0.07 | Calford 5702 |
| 16 | 0.29 | 0.83 | 0.34 | 0.74 | 0.50 | 0.15 | Wexbourne 5606 |
| 17 | 0.37 | 0.80 | 0.46 | 0.86 | 0.20 | 0.08 | Malenford 6554 |
| 18 | 0.34 | 0.77 | 0.41 | 0.80 | 0.20 | -0.09 | Marford 4704 |
| 19 | 0.23 | 0.74 | 0.31 | 0.83 | 0.40 | -0.04 | Thetingwick 5517 |
| 20 | 0.34 | 0.73 | 0.43 | 0.79 | 0.40 | 0.22 | Croford 2976 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 20% |
| Canterbury | 10000 | city | 50% |
| Colchester | 4000 | large_town | 0% |
| Sandwich | 4000 | large_town | 40% |
| Winchelsea | 3500 | large_town | 60% |
| Dover | 3000 | large_town | 40% |
| New Romney | 2500 | town | 60% |
| Rye | 2500 | town | 40% |
| Lewes | 2500 | town | 50% |
| Maidstone | 2000 | town | 75% |
| Hythe | 2000 | town | 45% |
| Hastings | 2000 | town | 25% |
| Maldon | 2000 | town | 45% |
| Faversham | 2000 | town | 40% |
| Rochester | 1900 | town | 55% |
| Lydd | 1500 | town | 30% |
| Chelmsford | 1500 | town | 65% |
| New Shoreham | 1500 | town | 60% |
| Battle | 1200 | town | 0% |
| Tonbridge | 1200 | town | 45% |
| Dartford | 1200 | town | 60% |
| Hertford | 1200 | town | 55% |
| Ware | 1200 | town | 65% |
| Croydon | 1000 | town | 55% |
| Reigate | 1000 | town | 20% |
| Folkestone | 1000 | town | 35% |
| Bishop's Stortford | 1000 | town | 20% |
| Seaford | 800 | small_town | 40% |
| East Grinstead | 800 | small_town | 0% |
| Milton Regis | 800 | small_town | 35% |
| Barking | 800 | small_town | 45% |
| Waltham Abbey | 800 | small_town | 60% |
| Pevensey | 600 | small_town | 95% |
| Fordwich | 600 | small_town | 40% |
| Tenterden | 600 | small_town | 0% |
| Ashford | 600 | small_town | 60% |
| Sevenoaks | 600 | small_town | 55% |
| West Malling | 600 | small_town | 15% |
| Cranbrook | 600 | small_town | 0% |
| Gravesend | 600 | small_town | 55% |
| Romford | 600 | small_town | 0% |
| Coggeshall | 600 | small_town | 40% |
| Braintree | 600 | small_town | 0% |
| Great Dunmow | 600 | small_town | 25% |
| Wye | 500 | small_town | 10% |
| Witham | 500 | small_town | 60% |
| Brentwood | 500 | small_town | 0% |
| Chipping Barnet | 500 | small_town | 0% |
| Bletchingley | 500 | small_town | 0% |
| Westerham | 400 | small_town | 0% |
| Charing | 400 | small_town | 10% |
| Wrotham | 400 | small_town | 0% |
| Lenham | 400 | small_town | 0% |
| Elham | 400 | small_town | 0% |
| Wingham | 400 | small_town | 40% |
| Bromley | 400 | small_town | 25% |
| Robertsbridge | 400 | small_town | 40% |
| Hailsham | 400 | small_town | 70% |
| Mayfield | 400 | small_town | 5% |
| Cuckfield | 400 | small_town | 35% |
| Crawley | 400 | small_town | 40% |
| Epping | 400 | small_town | 10% |
| Billericay | 400 | small_town | 15% |
| Rayleigh | 400 | small_town | 25% |
| Rochford | 400 | small_town | 0% |
| Harlow | 400 | small_town | 35% |
| Chipping Ongar | 400 | small_town | 35% |
| Hoddesdon | 400 | small_town | 70% |
| Sawbridgeworth | 400 | small_town | 45% |
| Hatfield | 400 | small_town | 80% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 105 | 3 | 90% | Colchester | 23 | 0 |
| 105 | 10 | 80% | Colchester | 24 | 0 |
| 3 | 47 | 65% | London (City) | 11 | 7 |
| 2 | 58 | 65% | Croydon | 11 | 24 |
| 31 | 89 | 60% | Mayfield | 12 | 72 |
| 119 | 59 | 55% | Sandwich | 13 | 45 |
