# Real-terrain scores: kent

## All 70 real towns against the model's top 70

20 seeds, final year; random baselines drop the same 70 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.39 | 0.37 | 0.30 | 5% |
| Real towns with a model place within 10 km | 0.83 | 0.82 | 0.65 | 0% |
| Model places with a real town within 5 km | 0.41 | 0.41 | 0.38 | 0% |
| Model places with a real town within 10 km | 0.87 | 0.90 | 0.85 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.30 | 0.29 | 0.38 | 0% |
| Rank agreement (Spearman, real size vs model size within 5 km) | -0.04 | -0.03 | 0.20 | 0% |

## The 27 real towns of 1,000 or more against the model's top 27

20 seeds, final year; random baselines drop the same 27 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.14 | 0.15 | 0.17 | 0% |
| Real towns with a model place within 10 km | 0.50 | 0.45 | 0.46 | 0% |
| Model places with a real town within 5 km | 0.14 | 0.15 | 0.18 | 0% |
| Model places with a real town within 10 km | 0.55 | 0.48 | 0.52 | 5% |
| Real top 10 with a model top-15 place within 10 km | 0.30 | 0.29 | 0.39 | 0% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.02 | 0.01 | 0.14 | 0% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (70) | 0.56 | 0.47 | 0.18 | 0.16 |
| 1,000 or more (27) | 0.54 | 0.45 | 0.18 | 0.16 |
| Under 1,000 (43) | 0.57 | 0.48 | 0.19 | 0.16 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 0; any top-70 place within 10 km: 20. Median distance from the model's largest place to London (City): 38 km. Largest place's share of the region's people (median): 0.28.

Nearest real town to the model's largest place, by seeds: Gravesend 15, Rochford 2, Milton Regis 2, Rochester 1.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 18 | 26% | 29.8 | 42% | 53% |
| coast | 6 | 9% | 9.4 | 14% | 40% |
| inland | 46 | 66% | 30.8 | 44% | 34% |

Inland places over 500 people, nearest other place over 500 (median): real 10.6 km (29 towns); model 10.6 km (median over seeds; 10 such places per seed).

Mean nearest-neighbour spacing of the 70 places: real 8.6 km, model 7.4 km, random land 6.9 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.41 | 0.86 | 0.43 | 0.86 | 0.30 | -0.06 | Kiningmouth 18767 |
| 2 | 0.44 | 0.84 | 0.49 | 0.90 | 0.30 | -0.18 | Ordermouth 25145 |
| 3 | 0.36 | 0.86 | 0.39 | 0.86 | 0.30 | 0.08 | Calport 19811 |
| 4 | 0.40 | 0.80 | 0.41 | 0.86 | 0.40 | 0.07 | Kiningmouth 21920 |
| 5 | 0.40 | 0.83 | 0.43 | 0.83 | 0.20 | -0.16 | Calport 20379 |
| 6 | 0.36 | 0.86 | 0.36 | 0.90 | 0.40 | 0.08 | Ludenmouth 23503 |
| 7 | 0.41 | 0.83 | 0.46 | 0.87 | 0.30 | -0.07 | Kiningmouth 21585 |
| 8 | 0.39 | 0.84 | 0.41 | 0.89 | 0.30 | -0.03 | Croerport 19477 |
| 9 | 0.40 | 0.84 | 0.47 | 0.89 | 0.30 | -0.09 | Kiningmouth 18679 |
| 10 | 0.31 | 0.83 | 0.30 | 0.90 | 0.20 | 0.07 | Pickford 7096 |
| 11 | 0.34 | 0.79 | 0.40 | 0.81 | 0.30 | -0.05 | Ulaport 23391 |
| 12 | 0.37 | 0.84 | 0.39 | 0.84 | 0.30 | -0.18 | Ickport 21750 |
| 13 | 0.40 | 0.86 | 0.40 | 0.89 | 0.40 | -0.09 | Bareley 21134 |
| 14 | 0.37 | 0.84 | 0.43 | 0.86 | 0.40 | 0.08 | Ludenwick 19821 |
| 15 | 0.43 | 0.83 | 0.41 | 0.86 | 0.30 | 0.01 | Evermouth 21839 |
| 16 | 0.47 | 0.86 | 0.46 | 0.93 | 0.30 | 0.10 | Maringhaven 24225 |
| 17 | 0.44 | 0.83 | 0.46 | 0.91 | 0.30 | -0.09 | Evermouth 24201 |
| 18 | 0.34 | 0.74 | 0.41 | 0.83 | 0.30 | -0.04 | Linmouth 13225 |
| 19 | 0.44 | 0.87 | 0.43 | 0.91 | 0.30 | -0.04 | Rayingford 22423 |
| 20 | 0.33 | 0.81 | 0.36 | 0.89 | 0.20 | -0.24 | Wyermouth 12800 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 15% |
| Canterbury | 10000 | city | 35% |
| Colchester | 4000 | large_town | 20% |
| Sandwich | 4000 | large_town | 90% |
| Winchelsea | 3500 | large_town | 60% |
| Dover | 3000 | large_town | 20% |
| New Romney | 2500 | town | 10% |
| Rye | 2500 | town | 55% |
| Lewes | 2500 | town | 0% |
| Maidstone | 2000 | town | 80% |
| Hythe | 2000 | town | 75% |
| Hastings | 2000 | town | 0% |
| Maldon | 2000 | town | 65% |
| Faversham | 2000 | town | 90% |
| Rochester | 1900 | town | 10% |
| Lydd | 1500 | town | 0% |
| Chelmsford | 1500 | town | 70% |
| New Shoreham | 1500 | town | 5% |
| Battle | 1200 | town | 0% |
| Tonbridge | 1200 | town | 80% |
| Dartford | 1200 | town | 80% |
| Hertford | 1200 | town | 30% |
| Ware | 1200 | town | 25% |
| Croydon | 1000 | town | 25% |
| Reigate | 1000 | town | 10% |
| Folkestone | 1000 | town | 95% |
| Bishop's Stortford | 1000 | town | 40% |
| Seaford | 800 | small_town | 5% |
| East Grinstead | 800 | small_town | 10% |
| Milton Regis | 800 | small_town | 35% |
| Barking | 800 | small_town | 70% |
| Waltham Abbey | 800 | small_town | 85% |
| Pevensey | 600 | small_town | 10% |
| Fordwich | 600 | small_town | 60% |
| Tenterden | 600 | small_town | 5% |
| Ashford | 600 | small_town | 20% |
| Sevenoaks | 600 | small_town | 10% |
| West Malling | 600 | small_town | 50% |
| Cranbrook | 600 | small_town | 25% |
| Gravesend | 600 | small_town | 35% |
| Romford | 600 | small_town | 40% |
| Coggeshall | 600 | small_town | 80% |
| Braintree | 600 | small_town | 0% |
| Great Dunmow | 600 | small_town | 20% |
| Wye | 500 | small_town | 5% |
| Witham | 500 | small_town | 65% |
| Brentwood | 500 | small_town | 30% |
| Chipping Barnet | 500 | small_town | 5% |
| Bletchingley | 500 | small_town | 0% |
| Westerham | 400 | small_town | 0% |
| Charing | 400 | small_town | 55% |
| Wrotham | 400 | small_town | 15% |
| Lenham | 400 | small_town | 15% |
| Elham | 400 | small_town | 0% |
| Wingham | 400 | small_town | 80% |
| Bromley | 400 | small_town | 55% |
| Robertsbridge | 400 | small_town | 60% |
| Hailsham | 400 | small_town | 10% |
| Mayfield | 400 | small_town | 30% |
| Cuckfield | 400 | small_town | 80% |
| Crawley | 400 | small_town | 20% |
| Epping | 400 | small_town | 60% |
| Billericay | 400 | small_town | 60% |
| Rayleigh | 400 | small_town | 65% |
| Rochford | 400 | small_town | 50% |
| Harlow | 400 | small_town | 55% |
| Chipping Ongar | 400 | small_town | 35% |
| Hoddesdon | 400 | small_town | 85% |
| Sawbridgeworth | 400 | small_town | 75% |
| Hatfield | 400 | small_town | 85% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 4 | 48 | 100% | London (City) | 10 | 5 |
| 16 | 124 | 95% | Lewes | 10 | 4 |
| 103 | 2 | 90% | Colchester | 21 | 4 |
| 104 | 11 | 85% | Colchester | 23 | 2 |
| 2 | 59 | 75% | Croydon | 10 | 34 |
| 27 | 104 | 65% | Mayfield | 12 | 32 |
