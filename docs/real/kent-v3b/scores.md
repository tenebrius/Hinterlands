# Real-terrain scores: kent

## All 70 real towns against the model's top 70

20 seeds, final year; random baselines drop the same 70 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.42 | 0.37 | 0.30 | 15% |
| Real towns with a model place within 10 km | 0.88 | 0.82 | 0.65 | 40% |
| Model places with a real town within 5 km | 0.44 | 0.41 | 0.38 | 10% |
| Model places with a real town within 10 km | 0.92 | 0.90 | 0.85 | 10% |
| Real top 10 with a model top-15 place within 10 km | 0.42 | 0.29 | 0.38 | 10% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.06 | -0.03 | 0.20 | 10% |

## The 27 real towns of 1,000 or more against the model's top 27

20 seeds, final year; random baselines drop the same 27 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.26 | 0.15 | 0.17 | 20% |
| Real towns with a model place within 10 km | 0.58 | 0.45 | 0.46 | 35% |
| Model places with a real town within 5 km | 0.24 | 0.15 | 0.18 | 15% |
| Model places with a real town within 10 km | 0.53 | 0.48 | 0.52 | 5% |
| Real top 10 with a model top-15 place within 10 km | 0.42 | 0.29 | 0.39 | 10% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.19 | 0.01 | 0.14 | 5% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (70) | 0.59 | 0.53 | 0.25 | 0.16 |
| 1,000 or more (27) | 0.60 | 0.54 | 0.29 | 0.16 |
| Under 1,000 (43) | 0.59 | 0.52 | 0.23 | 0.16 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 0; any top-70 place within 10 km: 20. Median distance from the model's largest place to London (City): 76 km. Largest place's share of the region's people (median): 0.26.

Nearest real town to the model's largest place, by seeds: Faversham 11, Gravesend 9.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 18 | 26% | 32.2 | 46% | 51% |
| coast | 6 | 9% | 8.0 | 11% | 45% |
| inland | 46 | 66% | 29.8 | 42% | 37% |

Inland places over 500 people, nearest other place over 500 (median): real 10.6 km (29 towns); model 10.8 km (median over seeds; 12 such places per seed).

Mean nearest-neighbour spacing of the 70 places: real 8.6 km, model 7.6 km, random land 6.9 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.37 | 0.86 | 0.44 | 0.86 | 0.20 | -0.00 | Dunelpit 46282 |
| 2 | 0.41 | 0.91 | 0.43 | 0.96 | 0.50 | 0.15 | Glenhaven 35472 |
| 3 | 0.43 | 0.86 | 0.50 | 0.93 | 0.40 | 0.05 | Kinanhurst 44621 |
| 4 | 0.47 | 0.90 | 0.47 | 0.91 | 0.30 | 0.02 | Verestone 42785 |
| 5 | 0.40 | 0.90 | 0.40 | 0.91 | 0.60 | 0.09 | Dunerford 46935 |
| 6 | 0.44 | 0.89 | 0.41 | 0.94 | 0.50 | -0.12 | Fairerhaven 55148 |
| 7 | 0.46 | 0.84 | 0.47 | 0.87 | 0.10 | 0.16 | Wexpit 38574 |
| 8 | 0.41 | 0.87 | 0.49 | 0.96 | 0.40 | 0.18 | Reneholt 51377 |
| 9 | 0.43 | 0.94 | 0.43 | 0.87 | 0.30 | 0.09 | Walanwood 60145 |
| 10 | 0.40 | 0.87 | 0.41 | 0.94 | 0.30 | 0.11 | Kelendale 44016 |
| 11 | 0.37 | 0.90 | 0.37 | 0.94 | 0.40 | -0.01 | Fairerhaven 59224 |
| 12 | 0.49 | 0.90 | 0.50 | 0.91 | 0.50 | 0.10 | Winshaven 50543 |
| 13 | 0.40 | 0.87 | 0.46 | 0.94 | 0.40 | 0.07 | Croelwood 45859 |
| 14 | 0.39 | 0.86 | 0.43 | 0.97 | 0.50 | -0.00 | Oxingwick 49188 |
| 15 | 0.39 | 0.91 | 0.41 | 0.90 | 0.60 | -0.14 | Ewerbourne 51497 |
| 16 | 0.43 | 0.83 | 0.51 | 0.93 | 0.50 | 0.15 | Aldingden 37074 |
| 17 | 0.41 | 0.89 | 0.41 | 0.97 | 0.50 | 0.05 | Loxensea 58670 |
| 18 | 0.44 | 0.80 | 0.46 | 0.93 | 0.40 | 0.18 | Leyingholt 60825 |
| 19 | 0.34 | 0.91 | 0.36 | 0.89 | 0.50 | 0.07 | Kelenholt 52754 |
| 20 | 0.43 | 0.87 | 0.51 | 0.93 | 0.40 | 0.03 | Yarenhaven 37371 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 45% |
| Canterbury | 10000 | city | 5% |
| Colchester | 4000 | large_town | 0% |
| Sandwich | 4000 | large_town | 85% |
| Winchelsea | 3500 | large_town | 100% |
| Dover | 3000 | large_town | 35% |
| New Romney | 2500 | town | 65% |
| Rye | 2500 | town | 100% |
| Lewes | 2500 | town | 50% |
| Maidstone | 2000 | town | 85% |
| Hythe | 2000 | town | 100% |
| Hastings | 2000 | town | 0% |
| Maldon | 2000 | town | 25% |
| Faversham | 2000 | town | 35% |
| Rochester | 1900 | town | 30% |
| Lydd | 1500 | town | 15% |
| Chelmsford | 1500 | town | 100% |
| New Shoreham | 1500 | town | 15% |
| Battle | 1200 | town | 0% |
| Tonbridge | 1200 | town | 45% |
| Dartford | 1200 | town | 85% |
| Hertford | 1200 | town | 10% |
| Ware | 1200 | town | 15% |
| Croydon | 1000 | town | 40% |
| Reigate | 1000 | town | 10% |
| Folkestone | 1000 | town | 25% |
| Bishop's Stortford | 1000 | town | 50% |
| Seaford | 800 | small_town | 30% |
| East Grinstead | 800 | small_town | 0% |
| Milton Regis | 800 | small_town | 55% |
| Barking | 800 | small_town | 10% |
| Waltham Abbey | 800 | small_town | 85% |
| Pevensey | 600 | small_town | 100% |
| Fordwich | 600 | small_town | 25% |
| Tenterden | 600 | small_town | 35% |
| Ashford | 600 | small_town | 85% |
| Sevenoaks | 600 | small_town | 65% |
| West Malling | 600 | small_town | 45% |
| Cranbrook | 600 | small_town | 10% |
| Gravesend | 600 | small_town | 25% |
| Romford | 600 | small_town | 0% |
| Coggeshall | 600 | small_town | 45% |
| Braintree | 600 | small_town | 20% |
| Great Dunmow | 600 | small_town | 65% |
| Wye | 500 | small_town | 50% |
| Witham | 500 | small_town | 40% |
| Brentwood | 500 | small_town | 10% |
| Chipping Barnet | 500 | small_town | 40% |
| Bletchingley | 500 | small_town | 10% |
| Westerham | 400 | small_town | 0% |
| Charing | 400 | small_town | 0% |
| Wrotham | 400 | small_town | 0% |
| Lenham | 400 | small_town | 10% |
| Elham | 400 | small_town | 0% |
| Wingham | 400 | small_town | 55% |
| Bromley | 400 | small_town | 75% |
| Robertsbridge | 400 | small_town | 80% |
| Hailsham | 400 | small_town | 0% |
| Mayfield | 400 | small_town | 45% |
| Cuckfield | 400 | small_town | 95% |
| Crawley | 400 | small_town | 55% |
| Epping | 400 | small_town | 60% |
| Billericay | 400 | small_town | 55% |
| Rayleigh | 400 | small_town | 45% |
| Rochford | 400 | small_town | 5% |
| Harlow | 400 | small_town | 45% |
| Chipping Ongar | 400 | small_town | 75% |
| Hoddesdon | 400 | small_town | 65% |
| Sawbridgeworth | 400 | small_town | 60% |
| Hatfield | 400 | small_town | 65% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 81 | 16 | 100% | Colchester | 14 | 1 |
| 46 | 89 | 100% | Cranbrook | 11 | 78 |
| 23 | 105 | 95% | Lewes | 10 | 19 |
| 90 | 9 | 85% | Colchester | 11 | 20 |
| 3 | 43 | 85% | London (City) | 10 | 18 |
| 29 | 89 | 70% | East Grinstead | 11 | 50 |
| 104 | 11 | 55% | Colchester | 23 | 2 |
| 12 | 2 | 50% | Ware | 10 | 109 |
