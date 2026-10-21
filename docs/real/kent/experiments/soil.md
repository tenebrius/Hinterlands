# Real-terrain scores: kent

## All 70 real towns against the model's top 70

20 seeds, final year; random baselines drop the same 70 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.34 | 0.37 | 0.30 | 0% |
| Real towns with a model place within 10 km | 0.75 | 0.82 | 0.65 | 0% |
| Model places with a real town within 5 km | 0.41 | 0.41 | 0.38 | 15% |
| Model places with a real town within 10 km | 0.87 | 0.90 | 0.85 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.40 | 0.29 | 0.38 | 20% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.07 | -0.03 | 0.20 | 30% |

## The 27 real towns of 1,000 or more against the model's top 27

20 seeds, final year; random baselines drop the same 27 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.16 | 0.15 | 0.17 | 0% |
| Real towns with a model place within 10 km | 0.53 | 0.45 | 0.46 | 15% |
| Model places with a real town within 5 km | 0.16 | 0.15 | 0.18 | 0% |
| Model places with a real town within 10 km | 0.53 | 0.48 | 0.52 | 5% |
| Real top 10 with a model top-15 place within 10 km | 0.40 | 0.29 | 0.39 | 20% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.05 | 0.01 | 0.14 | 0% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (70) | 0.55 | 0.46 | 0.17 | 0.15 |
| 1,000 or more (27) | 0.60 | 0.51 | 0.20 | 0.15 |
| Under 1,000 (43) | 0.52 | 0.43 | 0.15 | 0.15 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 1; any top-70 place within 10 km: 20. Median distance from the model's largest place to London (City): 41 km. Largest place's share of the region's people (median): 0.30.

Nearest real town to the model's largest place, by seeds: Gravesend 15, Rochford 2, Faversham 1, Sevenoaks 1, Fordwich 1.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 18 | 26% | 23.5 | 34% | 46% |
| coast | 6 | 9% | 8.9 | 13% | 29% |
| inland | 46 | 66% | 37.6 | 54% | 30% |

Inland places over 500 people, nearest other place over 500 (median): real 10.6 km (29 towns); model 9.0 km (median over seeds; 21 such places per seed).

Mean nearest-neighbour spacing of the 70 places: real 8.6 km, model 6.6 km, random land 6.9 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.29 | 0.81 | 0.33 | 0.87 | 0.40 | -0.01 | Kinesford 38755 |
| 2 | 0.44 | 0.77 | 0.51 | 0.91 | 0.30 | 0.06 | Hayey 36652 |
| 3 | 0.33 | 0.73 | 0.39 | 0.87 | 0.60 | 0.24 | Elmerfield 22660 |
| 4 | 0.31 | 0.73 | 0.51 | 0.87 | 0.40 | 0.07 | Ewanley 11238 |
| 5 | 0.39 | 0.74 | 0.51 | 0.87 | 0.40 | 0.22 | Kinelhurst 35745 |
| 6 | 0.33 | 0.77 | 0.39 | 0.83 | 0.40 | 0.05 | Gilelham 41610 |
| 7 | 0.31 | 0.79 | 0.31 | 0.89 | 0.40 | 0.00 | Walholt 33637 |
| 8 | 0.33 | 0.74 | 0.41 | 0.94 | 0.50 | 0.01 | Oakey 31635 |
| 9 | 0.36 | 0.81 | 0.39 | 0.89 | 0.50 | 0.13 | Thetwick 33610 |
| 10 | 0.36 | 0.70 | 0.41 | 0.90 | 0.30 | 0.13 | Aldanholt 27823 |
| 11 | 0.27 | 0.67 | 0.46 | 0.93 | 0.40 | 0.28 | Roanden 29983 |
| 12 | 0.34 | 0.71 | 0.36 | 0.89 | 0.60 | -0.19 | Ewenhurst 41600 |
| 13 | 0.31 | 0.77 | 0.47 | 0.89 | 0.80 | -0.06 | Fenanley 25344 |
| 14 | 0.36 | 0.73 | 0.37 | 0.77 | 0.20 | 0.13 | Iselelsea 37996 |
| 15 | 0.37 | 0.73 | 0.37 | 0.81 | 0.30 | 0.18 | Everingford 36056 |
| 16 | 0.43 | 0.79 | 0.47 | 0.87 | 0.40 | 0.17 | Craningsea 32718 |
| 17 | 0.30 | 0.77 | 0.31 | 0.86 | 0.60 | -0.29 | Sutelwood 35512 |
| 18 | 0.30 | 0.70 | 0.34 | 0.87 | 0.10 | 0.02 | Beleden 36294 |
| 19 | 0.40 | 0.77 | 0.50 | 0.89 | 0.20 | -0.01 | Talenhaven 37096 |
| 20 | 0.30 | 0.70 | 0.40 | 0.84 | 0.30 | 0.21 | Upowford 41058 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 45% |
| Canterbury | 10000 | city | 35% |
| Colchester | 4000 | large_town | 30% |
| Sandwich | 4000 | large_town | 25% |
| Winchelsea | 3500 | large_town | 50% |
| Dover | 3000 | large_town | 50% |
| New Romney | 2500 | town | 65% |
| Rye | 2500 | town | 35% |
| Lewes | 2500 | town | 20% |
| Maidstone | 2000 | town | 65% |
| Hythe | 2000 | town | 35% |
| Hastings | 2000 | town | 25% |
| Maldon | 2000 | town | 85% |
| Faversham | 2000 | town | 50% |
| Rochester | 1900 | town | 40% |
| Lydd | 1500 | town | 70% |
| Chelmsford | 1500 | town | 50% |
| New Shoreham | 1500 | town | 0% |
| Battle | 1200 | town | 10% |
| Tonbridge | 1200 | town | 65% |
| Dartford | 1200 | town | 25% |
| Hertford | 1200 | town | 20% |
| Ware | 1200 | town | 15% |
| Croydon | 1000 | town | 60% |
| Reigate | 1000 | town | 5% |
| Folkestone | 1000 | town | 35% |
| Bishop's Stortford | 1000 | town | 5% |
| Seaford | 800 | small_town | 30% |
| East Grinstead | 800 | small_town | 10% |
| Milton Regis | 800 | small_town | 30% |
| Barking | 800 | small_town | 50% |
| Waltham Abbey | 800 | small_town | 45% |
| Pevensey | 600 | small_town | 35% |
| Fordwich | 600 | small_town | 50% |
| Tenterden | 600 | small_town | 25% |
| Ashford | 600 | small_town | 40% |
| Sevenoaks | 600 | small_town | 35% |
| West Malling | 600 | small_town | 45% |
| Cranbrook | 600 | small_town | 30% |
| Gravesend | 600 | small_town | 35% |
| Romford | 600 | small_town | 25% |
| Coggeshall | 600 | small_town | 65% |
| Braintree | 600 | small_town | 25% |
| Great Dunmow | 600 | small_town | 30% |
| Wye | 500 | small_town | 15% |
| Witham | 500 | small_town | 60% |
| Brentwood | 500 | small_town | 35% |
| Chipping Barnet | 500 | small_town | 5% |
| Bletchingley | 500 | small_town | 10% |
| Westerham | 400 | small_town | 20% |
| Charing | 400 | small_town | 30% |
| Wrotham | 400 | small_town | 35% |
| Lenham | 400 | small_town | 40% |
| Elham | 400 | small_town | 10% |
| Wingham | 400 | small_town | 35% |
| Bromley | 400 | small_town | 35% |
| Robertsbridge | 400 | small_town | 55% |
| Hailsham | 400 | small_town | 45% |
| Mayfield | 400 | small_town | 20% |
| Cuckfield | 400 | small_town | 0% |
| Crawley | 400 | small_town | 30% |
| Epping | 400 | small_town | 25% |
| Billericay | 400 | small_town | 25% |
| Rayleigh | 400 | small_town | 35% |
| Rochford | 400 | small_town | 45% |
| Harlow | 400 | small_town | 20% |
| Chipping Ongar | 400 | small_town | 55% |
| Hoddesdon | 400 | small_town | 40% |
| Sawbridgeworth | 400 | small_town | 15% |
| Hatfield | 400 | small_town | 30% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 105 | 8 | 65% | Colchester | 23 | 15 |
| 29 | 81 | 60% | Tonbridge | 10 | 71 |
| 118 | 59 | 55% | Sandwich | 12 | 41 |
| 99 | 13 | 55% | Colchester | 20 | 14 |
| 2 | 47 | 55% | London (City) | 12 | 8 |
