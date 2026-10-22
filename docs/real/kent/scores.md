# Real-terrain scores: kent

## All 70 real towns against the model's top 70

20 seeds, final year; random baselines drop the same 70 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.34 | 0.37 | 0.30 | 0% |
| Real towns with a model place within 10 km | 0.80 | 0.82 | 0.65 | 0% |
| Model places with a real town within 5 km | 0.38 | 0.41 | 0.38 | 0% |
| Model places with a real town within 10 km | 0.86 | 0.90 | 0.85 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.38 | 0.29 | 0.38 | 15% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.08 | -0.03 | 0.20 | 5% |

## The 27 real towns of 1,000 or more against the model's top 27

20 seeds, final year; random baselines drop the same 27 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.19 | 0.15 | 0.17 | 0% |
| Real towns with a model place within 10 km | 0.58 | 0.45 | 0.46 | 35% |
| Model places with a real town within 5 km | 0.18 | 0.15 | 0.18 | 0% |
| Model places with a real town within 10 km | 0.60 | 0.48 | 0.52 | 45% |
| Real top 10 with a model top-15 place within 10 km | 0.38 | 0.29 | 0.39 | 15% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.10 | 0.01 | 0.14 | 5% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (70) | 0.53 | 0.45 | 0.19 | 0.15 |
| 1,000 or more (27) | 0.57 | 0.50 | 0.25 | 0.15 |
| Under 1,000 (43) | 0.50 | 0.41 | 0.15 | 0.15 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 1; any top-70 place within 10 km: 20. Median distance from the model's largest place to London (City): 38 km. Largest place's share of the region's people (median): 0.30.

Nearest real town to the model's largest place, by seeds: Gravesend 14, Milton Regis 4, Hastings 1, Maldon 1.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 18 | 26% | 35.6 | 51% | 57% |
| coast | 6 | 9% | 14.7 | 21% | 34% |
| inland | 46 | 66% | 19.6 | 28% | 25% |

Inland places over 500 people, nearest other place over 500 (median): real 10.6 km (29 towns); model 13.9 km (median over seeds; 2 such places per seed).

Mean nearest-neighbour spacing of the 70 places: real 8.6 km, model 7.5 km, random land 6.9 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.36 | 0.80 | 0.36 | 0.89 | 0.40 | -0.05 | Ulaport 14829 |
| 2 | 0.36 | 0.77 | 0.37 | 0.89 | 0.50 | 0.02 | Kiningmouth 16040 |
| 3 | 0.27 | 0.81 | 0.31 | 0.89 | 0.30 | 0.16 | Kiningmouth 13999 |
| 4 | 0.37 | 0.79 | 0.43 | 0.90 | 0.20 | 0.10 | Sutelmouth 16204 |
| 5 | 0.36 | 0.76 | 0.37 | 0.83 | 0.40 | 0.04 | Rayingford 14445 |
| 6 | 0.37 | 0.74 | 0.41 | 0.84 | 0.20 | -0.05 | Bareley 16728 |
| 7 | 0.31 | 0.79 | 0.30 | 0.83 | 0.30 | 0.21 | Evermouth 16349 |
| 8 | 0.39 | 0.84 | 0.40 | 0.86 | 0.50 | 0.07 | Toraney 16295 |
| 9 | 0.34 | 0.79 | 0.37 | 0.86 | 0.30 | -0.00 | Toraney 13380 |
| 10 | 0.37 | 0.84 | 0.36 | 0.86 | 0.50 | 0.14 | Yarhurst 21324 |
| 11 | 0.39 | 0.86 | 0.41 | 0.89 | 0.50 | 0.05 | Toraney 15622 |
| 12 | 0.30 | 0.77 | 0.39 | 0.87 | 0.20 | 0.06 | Rayelport 15606 |
| 13 | 0.36 | 0.87 | 0.43 | 0.89 | 0.60 | 0.14 | Evermouth 16866 |
| 14 | 0.27 | 0.77 | 0.31 | 0.81 | 0.30 | 0.10 | Orderhaven 9515 |
| 15 | 0.29 | 0.79 | 0.34 | 0.83 | 0.60 | 0.17 | Kiningmouth 18270 |
| 16 | 0.34 | 0.77 | 0.39 | 0.87 | 0.30 | 0.14 | Selanden 11206 |
| 17 | 0.36 | 0.79 | 0.37 | 0.87 | 0.60 | 0.06 | Kiningmouth 16508 |
| 18 | 0.36 | 0.81 | 0.37 | 0.84 | 0.30 | 0.07 | Falanmouth 16068 |
| 19 | 0.33 | 0.84 | 0.40 | 0.84 | 0.30 | 0.07 | Hartingham 18174 |
| 20 | 0.37 | 0.77 | 0.41 | 0.84 | 0.30 | 0.08 | Holmouth 15045 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 0% |
| Canterbury | 10000 | city | 25% |
| Colchester | 4000 | large_town | 0% |
| Sandwich | 4000 | large_town | 100% |
| Winchelsea | 3500 | large_town | 90% |
| Dover | 3000 | large_town | 15% |
| New Romney | 2500 | town | 90% |
| Rye | 2500 | town | 90% |
| Lewes | 2500 | town | 10% |
| Maidstone | 2000 | town | 60% |
| Hythe | 2000 | town | 45% |
| Hastings | 2000 | town | 0% |
| Maldon | 2000 | town | 95% |
| Faversham | 2000 | town | 75% |
| Rochester | 1900 | town | 15% |
| Lydd | 1500 | town | 50% |
| Chelmsford | 1500 | town | 95% |
| New Shoreham | 1500 | town | 5% |
| Battle | 1200 | town | 0% |
| Tonbridge | 1200 | town | 90% |
| Dartford | 1200 | town | 90% |
| Hertford | 1200 | town | 10% |
| Ware | 1200 | town | 20% |
| Croydon | 1000 | town | 15% |
| Reigate | 1000 | town | 0% |
| Folkestone | 1000 | town | 55% |
| Bishop's Stortford | 1000 | town | 0% |
| Seaford | 800 | small_town | 10% |
| East Grinstead | 800 | small_town | 10% |
| Milton Regis | 800 | small_town | 10% |
| Barking | 800 | small_town | 25% |
| Waltham Abbey | 800 | small_town | 65% |
| Pevensey | 600 | small_town | 20% |
| Fordwich | 600 | small_town | 25% |
| Tenterden | 600 | small_town | 0% |
| Ashford | 600 | small_town | 15% |
| Sevenoaks | 600 | small_town | 60% |
| West Malling | 600 | small_town | 45% |
| Cranbrook | 600 | small_town | 0% |
| Gravesend | 600 | small_town | 40% |
| Romford | 600 | small_town | 5% |
| Coggeshall | 600 | small_town | 10% |
| Braintree | 600 | small_town | 0% |
| Great Dunmow | 600 | small_town | 20% |
| Wye | 500 | small_town | 5% |
| Witham | 500 | small_town | 50% |
| Brentwood | 500 | small_town | 20% |
| Chipping Barnet | 500 | small_town | 10% |
| Bletchingley | 500 | small_town | 0% |
| Westerham | 400 | small_town | 10% |
| Charing | 400 | small_town | 10% |
| Wrotham | 400 | small_town | 15% |
| Lenham | 400 | small_town | 5% |
| Elham | 400 | small_town | 0% |
| Wingham | 400 | small_town | 80% |
| Bromley | 400 | small_town | 55% |
| Robertsbridge | 400 | small_town | 75% |
| Hailsham | 400 | small_town | 35% |
| Mayfield | 400 | small_town | 40% |
| Cuckfield | 400 | small_town | 85% |
| Crawley | 400 | small_town | 50% |
| Epping | 400 | small_town | 45% |
| Billericay | 400 | small_town | 35% |
| Rayleigh | 400 | small_town | 10% |
| Rochford | 400 | small_town | 5% |
| Harlow | 400 | small_town | 45% |
| Chipping Ongar | 400 | small_town | 20% |
| Hoddesdon | 400 | small_town | 75% |
| Sawbridgeworth | 400 | small_town | 45% |
| Hatfield | 400 | small_town | 75% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 15 | 124 | 100% | Lewes | 11 | -5 |
| 4 | 48 | 100% | London (City) | 10 | 5 |
| 104 | 4 | 95% | Colchester | 22 | 0 |
| 120 | 59 | 85% | Sandwich | 13 | 27 |
| 106 | 60 | 85% | Wingham | 11 | 3 |
| 29 | 89 | 80% | East Grinstead | 11 | 50 |
| 27 | 104 | 80% | Mayfield | 12 | 32 |
| 104 | 12 | 70% | Colchester | 24 | 0 |
| 90 | 10 | 65% | Colchester | 11 | 11 |
