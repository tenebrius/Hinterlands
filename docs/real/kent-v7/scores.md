# Real-terrain scores: kent

## All 70 real towns against the model's top 70

20 seeds, final year; random baselines drop the same 70 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.42 | 0.37 | 0.30 | 15% |
| Real towns with a model place within 10 km | 0.85 | 0.82 | 0.65 | 15% |
| Model places with a real town within 5 km | 0.44 | 0.41 | 0.38 | 0% |
| Model places with a real town within 10 km | 0.92 | 0.90 | 0.85 | 5% |
| Real top 10 with a model top-15 place within 10 km | 0.40 | 0.29 | 0.38 | 5% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.08 | -0.03 | 0.20 | 20% |

## The 27 real towns of 1,000 or more against the model's top 27

20 seeds, final year; random baselines drop the same 27 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.25 | 0.15 | 0.17 | 15% |
| Real towns with a model place within 10 km | 0.57 | 0.45 | 0.46 | 20% |
| Model places with a real town within 5 km | 0.23 | 0.15 | 0.18 | 10% |
| Model places with a real town within 10 km | 0.59 | 0.48 | 0.52 | 20% |
| Real top 10 with a model top-15 place within 10 km | 0.40 | 0.29 | 0.39 | 5% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.18 | 0.01 | 0.14 | 20% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (70) | 0.58 | 0.52 | 0.25 | 0.16 |
| 1,000 or more (27) | 0.60 | 0.54 | 0.29 | 0.16 |
| Under 1,000 (43) | 0.57 | 0.50 | 0.22 | 0.16 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 0; any top-70 place within 10 km: 20. Median distance from the model's largest place to London (City): 38 km. Largest place's share of the region's people (median): 0.25.

Nearest real town to the model's largest place, by seeds: Gravesend 12, Faversham 6, Milton Regis 1, Maldon 1.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 18 | 26% | 38.0 | 54% | 61% |
| coast | 6 | 9% | 7.0 | 10% | 39% |
| inland | 46 | 66% | 25.1 | 36% | 35% |

Inland places over 500 people, nearest other place over 500 (median): real 10.6 km (29 towns); model 11.7 km (median over seeds; 8 such places per seed).

Mean nearest-neighbour spacing of the 70 places: real 8.6 km, model 7.5 km, random land 6.9 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.44 | 0.83 | 0.43 | 0.90 | 0.40 | 0.00 | Ewemouth 34395 |
| 2 | 0.49 | 0.84 | 0.47 | 0.89 | 0.40 | 0.17 | Roaford 33055 |
| 3 | 0.41 | 0.84 | 0.47 | 0.96 | 0.50 | -0.03 | Ulemouth 31947 |
| 4 | 0.40 | 0.83 | 0.47 | 0.97 | 0.60 | 0.18 | Ordermouth 33073 |
| 5 | 0.44 | 0.89 | 0.49 | 0.93 | 0.40 | 0.08 | Evermouth 30866 |
| 6 | 0.43 | 0.84 | 0.43 | 0.94 | 0.50 | 0.12 | Loxandale 27027 |
| 7 | 0.44 | 0.86 | 0.43 | 0.93 | 0.50 | 0.06 | Uleport 26781 |
| 8 | 0.33 | 0.86 | 0.36 | 0.93 | 0.40 | 0.09 | Fairerhaven 13440 |
| 9 | 0.37 | 0.83 | 0.44 | 0.91 | 0.40 | -0.05 | Thornanmouth 26782 |
| 10 | 0.41 | 0.87 | 0.43 | 0.96 | 0.30 | 0.16 | Selespit 18771 |
| 11 | 0.46 | 0.90 | 0.44 | 0.94 | 0.50 | -0.03 | Leyingham 32772 |
| 12 | 0.41 | 0.79 | 0.49 | 0.93 | 0.30 | 0.00 | Ewemouth 32879 |
| 13 | 0.43 | 0.86 | 0.46 | 0.90 | 0.20 | -0.03 | Dewport 22174 |
| 14 | 0.43 | 0.91 | 0.40 | 0.91 | 0.50 | 0.04 | Nederdale 24909 |
| 15 | 0.43 | 0.81 | 0.44 | 0.93 | 0.40 | 0.25 | Ordenstone 20731 |
| 16 | 0.41 | 0.90 | 0.43 | 0.90 | 0.50 | 0.10 | Brackwick 18283 |
| 17 | 0.41 | 0.81 | 0.43 | 0.91 | 0.40 | 0.09 | Roaford 36231 |
| 18 | 0.46 | 0.83 | 0.41 | 0.93 | 0.30 | 0.02 | Iselepit 21284 |
| 19 | 0.41 | 0.84 | 0.46 | 0.90 | 0.20 | 0.30 | Thornanmouth 38741 |
| 20 | 0.39 | 0.89 | 0.43 | 0.91 | 0.20 | -0.03 | Dunelpit 23969 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 40% |
| Canterbury | 10000 | city | 10% |
| Colchester | 4000 | large_town | 0% |
| Sandwich | 4000 | large_town | 60% |
| Winchelsea | 3500 | large_town | 95% |
| Dover | 3000 | large_town | 10% |
| New Romney | 2500 | town | 95% |
| Rye | 2500 | town | 100% |
| Lewes | 2500 | town | 10% |
| Maidstone | 2000 | town | 90% |
| Hythe | 2000 | town | 90% |
| Hastings | 2000 | town | 0% |
| Maldon | 2000 | town | 40% |
| Faversham | 2000 | town | 85% |
| Rochester | 1900 | town | 15% |
| Lydd | 1500 | town | 35% |
| Chelmsford | 1500 | town | 100% |
| New Shoreham | 1500 | town | 5% |
| Battle | 1200 | town | 5% |
| Tonbridge | 1200 | town | 100% |
| Dartford | 1200 | town | 75% |
| Hertford | 1200 | town | 20% |
| Ware | 1200 | town | 10% |
| Croydon | 1000 | town | 35% |
| Reigate | 1000 | town | 30% |
| Folkestone | 1000 | town | 25% |
| Bishop's Stortford | 1000 | town | 45% |
| Seaford | 800 | small_town | 20% |
| East Grinstead | 800 | small_town | 0% |
| Milton Regis | 800 | small_town | 70% |
| Barking | 800 | small_town | 25% |
| Waltham Abbey | 800 | small_town | 80% |
| Pevensey | 600 | small_town | 100% |
| Fordwich | 600 | small_town | 45% |
| Tenterden | 600 | small_town | 10% |
| Ashford | 600 | small_town | 50% |
| Sevenoaks | 600 | small_town | 40% |
| West Malling | 600 | small_town | 35% |
| Cranbrook | 600 | small_town | 5% |
| Gravesend | 600 | small_town | 40% |
| Romford | 600 | small_town | 10% |
| Coggeshall | 600 | small_town | 75% |
| Braintree | 600 | small_town | 10% |
| Great Dunmow | 600 | small_town | 40% |
| Wye | 500 | small_town | 20% |
| Witham | 500 | small_town | 60% |
| Brentwood | 500 | small_town | 40% |
| Chipping Barnet | 500 | small_town | 20% |
| Bletchingley | 500 | small_town | 30% |
| Westerham | 400 | small_town | 0% |
| Charing | 400 | small_town | 15% |
| Wrotham | 400 | small_town | 0% |
| Lenham | 400 | small_town | 0% |
| Elham | 400 | small_town | 0% |
| Wingham | 400 | small_town | 80% |
| Bromley | 400 | small_town | 85% |
| Robertsbridge | 400 | small_town | 70% |
| Hailsham | 400 | small_town | 25% |
| Mayfield | 400 | small_town | 25% |
| Cuckfield | 400 | small_town | 95% |
| Crawley | 400 | small_town | 40% |
| Epping | 400 | small_town | 45% |
| Billericay | 400 | small_town | 15% |
| Rayleigh | 400 | small_town | 25% |
| Rochford | 400 | small_town | 5% |
| Harlow | 400 | small_town | 60% |
| Chipping Ongar | 400 | small_town | 75% |
| Hoddesdon | 400 | small_town | 70% |
| Sawbridgeworth | 400 | small_town | 70% |
| Hatfield | 400 | small_town | 95% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 4 | 48 | 100% | London (City) | 10 | 5 |
| 81 | 16 | 95% | Colchester | 14 | 1 |
| 47 | 92 | 95% | Cranbrook | 10 | 81 |
| 23 | 105 | 75% | Lewes | 10 | 19 |
| 29 | 89 | 70% | East Grinstead | 11 | 50 |
| 104 | 11 | 55% | Colchester | 23 | 2 |
