# Real-terrain scores: kent

## All 70 real towns against the model's top 70

20 seeds, final year; random baselines drop the same 70 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.41 | 0.37 | 0.30 | 25% |
| Real towns with a model place within 10 km | 0.88 | 0.82 | 0.65 | 50% |
| Model places with a real town within 5 km | 0.45 | 0.41 | 0.38 | 0% |
| Model places with a real town within 10 km | 0.91 | 0.90 | 0.85 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.48 | 0.29 | 0.38 | 30% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.03 | -0.03 | 0.20 | 5% |

## The 27 real towns of 1,000 or more against the model's top 27

20 seeds, final year; random baselines drop the same 27 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.25 | 0.15 | 0.17 | 5% |
| Real towns with a model place within 10 km | 0.61 | 0.45 | 0.46 | 50% |
| Model places with a real town within 5 km | 0.23 | 0.15 | 0.18 | 10% |
| Model places with a real town within 10 km | 0.56 | 0.48 | 0.52 | 10% |
| Real top 10 with a model top-15 place within 10 km | 0.48 | 0.29 | 0.39 | 30% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.19 | 0.01 | 0.14 | 10% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (70) | 0.59 | 0.52 | 0.25 | 0.16 |
| 1,000 or more (27) | 0.60 | 0.53 | 0.28 | 0.16 |
| Under 1,000 (43) | 0.58 | 0.51 | 0.24 | 0.16 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 0; any top-70 place within 10 km: 20. Median distance from the model's largest place to London (City): 49 km. Largest place's share of the region's people (median): 0.28.

Nearest real town to the model's largest place, by seeds: Gravesend 10, Faversham 9, Milton Regis 1.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 18 | 26% | 32.1 | 46% | 50% |
| coast | 6 | 9% | 8.3 | 12% | 40% |
| inland | 46 | 66% | 29.6 | 42% | 37% |

Inland places over 500 people, nearest other place over 500 (median): real 10.6 km (29 towns); model 10.4 km (median over seeds; 12 such places per seed).

Mean nearest-neighbour spacing of the 70 places: real 8.6 km, model 7.5 km, random land 6.9 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.41 | 0.90 | 0.43 | 0.91 | 0.70 | -0.05 | Veranwood 35457 |
| 2 | 0.40 | 0.90 | 0.43 | 0.91 | 0.50 | 0.06 | Calsea 51251 |
| 3 | 0.37 | 0.87 | 0.40 | 0.90 | 0.30 | 0.05 | Aldewood 53887 |
| 4 | 0.46 | 0.91 | 0.50 | 0.93 | 0.40 | -0.00 | Loxensea 60640 |
| 5 | 0.40 | 0.89 | 0.49 | 0.94 | 0.60 | 0.04 | Ordsea 59958 |
| 6 | 0.43 | 0.90 | 0.49 | 0.87 | 0.50 | -0.08 | Halwick 53429 |
| 7 | 0.37 | 0.90 | 0.41 | 0.93 | 0.30 | 0.01 | Kinaney 40866 |
| 8 | 0.46 | 0.86 | 0.49 | 0.93 | 0.50 | 0.04 | Woolestone 31506 |
| 9 | 0.36 | 0.90 | 0.46 | 0.91 | 0.30 | -0.05 | Fenelholt 41695 |
| 10 | 0.43 | 0.79 | 0.49 | 0.89 | 0.50 | 0.03 | Ashanford 44558 |
| 11 | 0.37 | 0.89 | 0.43 | 0.93 | 0.60 | 0.03 | Dunanham 34985 |
| 12 | 0.47 | 0.89 | 0.47 | 0.89 | 0.30 | -0.04 | Elmelden 57022 |
| 13 | 0.31 | 0.91 | 0.39 | 0.94 | 0.50 | -0.04 | Glenhurst 55460 |
| 14 | 0.37 | 0.87 | 0.39 | 0.93 | 0.70 | 0.03 | Fairerhaven 55929 |
| 15 | 0.44 | 0.87 | 0.44 | 0.94 | 0.60 | 0.11 | Melelhaven 59617 |
| 16 | 0.47 | 0.90 | 0.49 | 0.87 | 0.40 | 0.24 | Oxingwick 46155 |
| 17 | 0.36 | 0.80 | 0.39 | 0.90 | 0.50 | -0.04 | Ordsea 53358 |
| 18 | 0.41 | 0.90 | 0.47 | 0.91 | 0.40 | 0.15 | Stowanley 59988 |
| 19 | 0.46 | 0.93 | 0.44 | 0.87 | 0.50 | 0.07 | Ulowham 51286 |
| 20 | 0.40 | 0.84 | 0.43 | 0.90 | 0.60 | 0.09 | Loxensea 30528 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 25% |
| Canterbury | 10000 | city | 15% |
| Colchester | 4000 | large_town | 15% |
| Sandwich | 4000 | large_town | 85% |
| Winchelsea | 3500 | large_town | 100% |
| Dover | 3000 | large_town | 25% |
| New Romney | 2500 | town | 70% |
| Rye | 2500 | town | 95% |
| Lewes | 2500 | town | 45% |
| Maidstone | 2000 | town | 75% |
| Hythe | 2000 | town | 90% |
| Hastings | 2000 | town | 0% |
| Maldon | 2000 | town | 25% |
| Faversham | 2000 | town | 35% |
| Rochester | 1900 | town | 25% |
| Lydd | 1500 | town | 10% |
| Chelmsford | 1500 | town | 90% |
| New Shoreham | 1500 | town | 5% |
| Battle | 1200 | town | 0% |
| Tonbridge | 1200 | town | 40% |
| Dartford | 1200 | town | 70% |
| Hertford | 1200 | town | 5% |
| Ware | 1200 | town | 10% |
| Croydon | 1000 | town | 50% |
| Reigate | 1000 | town | 20% |
| Folkestone | 1000 | town | 15% |
| Bishop's Stortford | 1000 | town | 45% |
| Seaford | 800 | small_town | 30% |
| East Grinstead | 800 | small_town | 0% |
| Milton Regis | 800 | small_town | 45% |
| Barking | 800 | small_town | 5% |
| Waltham Abbey | 800 | small_town | 80% |
| Pevensey | 600 | small_town | 100% |
| Fordwich | 600 | small_town | 20% |
| Tenterden | 600 | small_town | 5% |
| Ashford | 600 | small_town | 75% |
| Sevenoaks | 600 | small_town | 65% |
| West Malling | 600 | small_town | 35% |
| Cranbrook | 600 | small_town | 5% |
| Gravesend | 600 | small_town | 50% |
| Romford | 600 | small_town | 10% |
| Coggeshall | 600 | small_town | 65% |
| Braintree | 600 | small_town | 25% |
| Great Dunmow | 600 | small_town | 65% |
| Wye | 500 | small_town | 50% |
| Witham | 500 | small_town | 40% |
| Brentwood | 500 | small_town | 5% |
| Chipping Barnet | 500 | small_town | 50% |
| Bletchingley | 500 | small_town | 10% |
| Westerham | 400 | small_town | 0% |
| Charing | 400 | small_town | 5% |
| Wrotham | 400 | small_town | 0% |
| Lenham | 400 | small_town | 0% |
| Elham | 400 | small_town | 0% |
| Wingham | 400 | small_town | 65% |
| Bromley | 400 | small_town | 75% |
| Robertsbridge | 400 | small_town | 85% |
| Hailsham | 400 | small_town | 0% |
| Mayfield | 400 | small_town | 45% |
| Cuckfield | 400 | small_town | 100% |
| Crawley | 400 | small_town | 60% |
| Epping | 400 | small_town | 45% |
| Billericay | 400 | small_town | 55% |
| Rayleigh | 400 | small_town | 55% |
| Rochford | 400 | small_town | 25% |
| Harlow | 400 | small_town | 45% |
| Chipping Ongar | 400 | small_town | 60% |
| Hoddesdon | 400 | small_town | 70% |
| Sawbridgeworth | 400 | small_town | 75% |
| Hatfield | 400 | small_town | 70% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 81 | 16 | 100% | Colchester | 14 | 1 |
| 47 | 88 | 95% | Cranbrook | 10 | 94 |
| 27 | 104 | 85% | Mayfield | 12 | 32 |
| 3 | 47 | 85% | London (City) | 11 | 7 |
| 29 | 89 | 65% | East Grinstead | 11 | 50 |
| 104 | 11 | 60% | Colchester | 23 | 2 |
