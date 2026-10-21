# Real-terrain scores: kent

## All 70 real towns against the model's top 70

20 seeds, final year; random baselines drop the same 70 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.31 | 0.37 | 0.30 | 0% |
| Real towns with a model place within 10 km | 0.72 | 0.82 | 0.65 | 0% |
| Model places with a real town within 5 km | 0.39 | 0.41 | 0.38 | 10% |
| Model places with a real town within 10 km | 0.84 | 0.90 | 0.85 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.28 | 0.29 | 0.38 | 10% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.06 | -0.03 | 0.20 | 25% |

## The 27 real towns of 1,000 or more against the model's top 27

20 seeds, final year; random baselines drop the same 27 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.16 | 0.15 | 0.17 | 5% |
| Real towns with a model place within 10 km | 0.51 | 0.45 | 0.46 | 5% |
| Model places with a real town within 5 km | 0.16 | 0.15 | 0.18 | 0% |
| Model places with a real town within 10 km | 0.48 | 0.48 | 0.52 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.28 | 0.29 | 0.39 | 10% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.07 | 0.01 | 0.14 | 5% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (70) | 0.52 | 0.43 | 0.15 | 0.14 |
| 1,000 or more (27) | 0.56 | 0.48 | 0.17 | 0.14 |
| Under 1,000 (43) | 0.50 | 0.40 | 0.14 | 0.14 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 0; any top-70 place within 10 km: 12. Median distance from the model's largest place to London (City): 45 km. Largest place's share of the region's people (median): 0.25.

Nearest real town to the model's largest place, by seeds: Gravesend 6, Rayleigh 4, Faversham 2, Milton Regis 2, Rochester 2, Maldon 1, Rochford 1, Sandwich 1, Brentwood 1.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 18 | 26% | 22.3 | 32% | 41% |
| coast | 6 | 9% | 11.2 | 16% | 26% |
| inland | 46 | 66% | 36.5 | 52% | 28% |

Inland places over 500 people, nearest other place over 500 (median): real 10.6 km (29 towns); model 8.1 km (median over seeds; 20 such places per seed).

Mean nearest-neighbour spacing of the 70 places: real 8.6 km, model 6.3 km, random land 6.9 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.27 | 0.70 | 0.33 | 0.83 | 0.50 | 0.35 | Oxenhaven 27837 |
| 2 | 0.43 | 0.86 | 0.54 | 0.91 | 0.30 | 0.11 | Marton 21235 |
| 3 | 0.31 | 0.74 | 0.37 | 0.84 | 0.20 | 0.14 | Oakden 16836 |
| 4 | 0.27 | 0.71 | 0.39 | 0.84 | 0.60 | 0.30 | Thetsea 25496 |
| 5 | 0.27 | 0.64 | 0.33 | 0.71 | 0.20 | 0.20 | Shapesworth 35500 |
| 6 | 0.26 | 0.70 | 0.30 | 0.91 | 0.10 | -0.03 | Dunholt 22256 |
| 7 | 0.27 | 0.73 | 0.39 | 0.87 | 0.20 | 0.25 | Oxecote 22830 |
| 8 | 0.37 | 0.77 | 0.53 | 0.86 | 0.20 | -0.15 | Grimelham 38586 |
| 9 | 0.27 | 0.71 | 0.33 | 0.81 | 0.30 | -0.28 | Bureshurst 28519 |
| 10 | 0.33 | 0.67 | 0.33 | 0.76 | 0.60 | 0.09 | Melden 22988 |
| 11 | 0.40 | 0.74 | 0.43 | 0.87 | 0.30 | -0.02 | Selanden 29968 |
| 12 | 0.34 | 0.73 | 0.49 | 0.93 | 0.30 | -0.11 | Verley 21429 |
| 13 | 0.23 | 0.63 | 0.26 | 0.80 | 0.40 | -0.01 | Brackingwick 28872 |
| 14 | 0.30 | 0.81 | 0.36 | 0.84 | 0.10 | 0.17 | Darenham 26886 |
| 15 | 0.27 | 0.69 | 0.31 | 0.76 | 0.10 | -0.02 | Marhaven 31177 |
| 16 | 0.36 | 0.76 | 0.41 | 0.89 | 0.50 | -0.06 | Oakeley 17643 |
| 17 | 0.30 | 0.73 | 0.43 | 0.84 | 0.10 | 0.13 | Gilelfield 34909 |
| 18 | 0.26 | 0.60 | 0.34 | 0.79 | 0.20 | 0.28 | Astesden 24179 |
| 19 | 0.36 | 0.67 | 0.50 | 0.84 | 0.20 | -0.16 | Hartwick 35075 |
| 20 | 0.30 | 0.76 | 0.36 | 0.81 | 0.30 | 0.00 | Roenford 35432 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 15% |
| Canterbury | 10000 | city | 30% |
| Colchester | 4000 | large_town | 25% |
| Sandwich | 4000 | large_town | 25% |
| Winchelsea | 3500 | large_town | 50% |
| Dover | 3000 | large_town | 35% |
| New Romney | 2500 | town | 65% |
| Rye | 2500 | town | 50% |
| Lewes | 2500 | town | 20% |
| Maidstone | 2000 | town | 40% |
| Hythe | 2000 | town | 25% |
| Hastings | 2000 | town | 30% |
| Maldon | 2000 | town | 85% |
| Faversham | 2000 | town | 45% |
| Rochester | 1900 | town | 65% |
| Lydd | 1500 | town | 40% |
| Chelmsford | 1500 | town | 40% |
| New Shoreham | 1500 | town | 5% |
| Battle | 1200 | town | 25% |
| Tonbridge | 1200 | town | 45% |
| Dartford | 1200 | town | 25% |
| Hertford | 1200 | town | 0% |
| Ware | 1200 | town | 5% |
| Croydon | 1000 | town | 25% |
| Reigate | 1000 | town | 25% |
| Folkestone | 1000 | town | 40% |
| Bishop's Stortford | 1000 | town | 15% |
| Seaford | 800 | small_town | 5% |
| East Grinstead | 800 | small_town | 10% |
| Milton Regis | 800 | small_town | 35% |
| Barking | 800 | small_town | 50% |
| Waltham Abbey | 800 | small_town | 50% |
| Pevensey | 600 | small_town | 45% |
| Fordwich | 600 | small_town | 35% |
| Tenterden | 600 | small_town | 25% |
| Ashford | 600 | small_town | 35% |
| Sevenoaks | 600 | small_town | 30% |
| West Malling | 600 | small_town | 45% |
| Cranbrook | 600 | small_town | 15% |
| Gravesend | 600 | small_town | 20% |
| Romford | 600 | small_town | 20% |
| Coggeshall | 600 | small_town | 75% |
| Braintree | 600 | small_town | 20% |
| Great Dunmow | 600 | small_town | 35% |
| Wye | 500 | small_town | 15% |
| Witham | 500 | small_town | 30% |
| Brentwood | 500 | small_town | 25% |
| Chipping Barnet | 500 | small_town | 15% |
| Bletchingley | 500 | small_town | 20% |
| Westerham | 400 | small_town | 10% |
| Charing | 400 | small_town | 40% |
| Wrotham | 400 | small_town | 30% |
| Lenham | 400 | small_town | 40% |
| Elham | 400 | small_town | 20% |
| Wingham | 400 | small_town | 20% |
| Bromley | 400 | small_town | 10% |
| Robertsbridge | 400 | small_town | 45% |
| Hailsham | 400 | small_town | 45% |
| Mayfield | 400 | small_town | 10% |
| Cuckfield | 400 | small_town | 0% |
| Crawley | 400 | small_town | 10% |
| Epping | 400 | small_town | 15% |
| Billericay | 400 | small_town | 40% |
| Rayleigh | 400 | small_town | 35% |
| Rochford | 400 | small_town | 25% |
| Harlow | 400 | small_town | 40% |
| Chipping Ongar | 400 | small_town | 45% |
| Hoddesdon | 400 | small_town | 65% |
| Sawbridgeworth | 400 | small_town | 45% |
| Hatfield | 400 | small_town | 20% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 91 | 9 | 65% | Colchester | 11 | 12 |
| 107 | 6 | 60% | Colchester | 25 | 8 |
| 83 | 13 | 55% | Colchester | 11 | 4 |
| 118 | 60 | 50% | Sandwich | 11 | 39 |
| 59 | 50 | 50% | Rochester | 10 | 39 |
