# Real-terrain scores: kent

## All 70 real towns against the model's top 70

20 seeds, final year; random baselines drop the same 70 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.30 | 0.37 | 0.30 | 0% |
| Real towns with a model place within 10 km | 0.72 | 0.82 | 0.65 | 0% |
| Model places with a real town within 5 km | 0.40 | 0.41 | 0.38 | 0% |
| Model places with a real town within 10 km | 0.84 | 0.90 | 0.85 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.33 | 0.29 | 0.38 | 10% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.01 | -0.03 | 0.20 | 10% |

## The 27 real towns of 1,000 or more against the model's top 27

20 seeds, final year; random baselines drop the same 27 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.14 | 0.15 | 0.17 | 0% |
| Real towns with a model place within 10 km | 0.46 | 0.45 | 0.46 | 0% |
| Model places with a real town within 5 km | 0.14 | 0.15 | 0.18 | 0% |
| Model places with a real town within 10 km | 0.43 | 0.48 | 0.52 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.33 | 0.29 | 0.39 | 10% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.08 | 0.01 | 0.14 | 5% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (70) | 0.53 | 0.46 | 0.17 | 0.14 |
| 1,000 or more (27) | 0.49 | 0.41 | 0.15 | 0.14 |
| Under 1,000 (43) | 0.57 | 0.49 | 0.17 | 0.14 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 0; any top-70 place within 10 km: 14. Median distance from the model's largest place to London (City): 41 km. Largest place's share of the region's people (median): 0.27.

Nearest real town to the model's largest place, by seeds: Gravesend 8, Rayleigh 5, Billericay 2, Milton Regis 1, Faversham 1, Wingham 1, West Malling 1, Fordwich 1.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 18 | 26% | 20.4 | 29% | 39% |
| coast | 6 | 9% | 10.6 | 15% | 30% |
| inland | 46 | 66% | 39.0 | 56% | 26% |

Inland places over 500 people, nearest other place over 500 (median): real 10.6 km (29 towns); model 8.6 km (median over seeds; 20 such places per seed).

Mean nearest-neighbour spacing of the 70 places: real 8.6 km, model 6.4 km, random land 6.9 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.30 | 0.71 | 0.34 | 0.83 | 0.30 | 0.37 | Dunholt 38916 |
| 2 | 0.21 | 0.69 | 0.36 | 0.81 | 0.30 | -0.03 | Croeford 27973 |
| 3 | 0.30 | 0.74 | 0.40 | 0.87 | 0.10 | -0.12 | Fairesholt 35669 |
| 4 | 0.27 | 0.74 | 0.39 | 0.86 | 0.70 | -0.18 | Faleshaven 26208 |
| 5 | 0.29 | 0.74 | 0.39 | 0.81 | 0.50 | 0.10 | Wenhurst 21019 |
| 6 | 0.33 | 0.64 | 0.49 | 0.79 | 0.30 | 0.03 | Verhaven 26020 |
| 7 | 0.29 | 0.76 | 0.39 | 0.89 | 0.50 | 0.05 | Halhaven 29303 |
| 8 | 0.30 | 0.79 | 0.33 | 0.83 | 0.00 | -0.10 | Noreington 32861 |
| 9 | 0.33 | 0.73 | 0.49 | 0.91 | 0.10 | -0.04 | Roasea 30118 |
| 10 | 0.33 | 0.79 | 0.50 | 0.91 | 0.30 | -0.08 | Oakhurst 31120 |
| 11 | 0.33 | 0.64 | 0.44 | 0.80 | 0.40 | 0.04 | Craneley 28584 |
| 12 | 0.31 | 0.71 | 0.37 | 0.79 | 0.40 | 0.07 | Malden 19529 |
| 13 | 0.29 | 0.77 | 0.36 | 0.93 | 0.60 | -0.20 | Roingley 41563 |
| 14 | 0.21 | 0.61 | 0.36 | 0.87 | 0.00 | -0.02 | Chalwood 11220 |
| 15 | 0.37 | 0.73 | 0.41 | 0.84 | 0.40 | 0.07 | Dewesfield 31177 |
| 16 | 0.41 | 0.70 | 0.50 | 0.83 | 0.50 | -0.10 | Melesham 27003 |
| 17 | 0.21 | 0.70 | 0.29 | 0.77 | 0.30 | -0.04 | Gilerworth 17787 |
| 18 | 0.30 | 0.76 | 0.40 | 0.81 | 0.20 | -0.07 | Reningden 22172 |
| 19 | 0.36 | 0.73 | 0.46 | 0.77 | 0.30 | 0.36 | Stanenport 16632 |
| 20 | 0.26 | 0.66 | 0.36 | 0.84 | 0.40 | 0.07 | Verby 36336 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 15% |
| Canterbury | 10000 | city | 25% |
| Colchester | 4000 | large_town | 20% |
| Sandwich | 4000 | large_town | 15% |
| Winchelsea | 3500 | large_town | 30% |
| Dover | 3000 | large_town | 20% |
| New Romney | 2500 | town | 80% |
| Rye | 2500 | town | 30% |
| Lewes | 2500 | town | 5% |
| Maidstone | 2000 | town | 35% |
| Hythe | 2000 | town | 30% |
| Hastings | 2000 | town | 30% |
| Maldon | 2000 | town | 85% |
| Faversham | 2000 | town | 45% |
| Rochester | 1900 | town | 25% |
| Lydd | 1500 | town | 70% |
| Chelmsford | 1500 | town | 10% |
| New Shoreham | 1500 | town | 15% |
| Battle | 1200 | town | 10% |
| Tonbridge | 1200 | town | 60% |
| Dartford | 1200 | town | 10% |
| Hertford | 1200 | town | 10% |
| Ware | 1200 | town | 15% |
| Croydon | 1000 | town | 15% |
| Reigate | 1000 | town | 25% |
| Folkestone | 1000 | town | 40% |
| Bishop's Stortford | 1000 | town | 0% |
| Seaford | 800 | small_town | 35% |
| East Grinstead | 800 | small_town | 35% |
| Milton Regis | 800 | small_town | 45% |
| Barking | 800 | small_town | 75% |
| Waltham Abbey | 800 | small_town | 65% |
| Pevensey | 600 | small_town | 35% |
| Fordwich | 600 | small_town | 25% |
| Tenterden | 600 | small_town | 15% |
| Ashford | 600 | small_town | 25% |
| Sevenoaks | 600 | small_town | 40% |
| West Malling | 600 | small_town | 25% |
| Cranbrook | 600 | small_town | 15% |
| Gravesend | 600 | small_town | 20% |
| Romford | 600 | small_town | 20% |
| Coggeshall | 600 | small_town | 65% |
| Braintree | 600 | small_town | 10% |
| Great Dunmow | 600 | small_town | 35% |
| Wye | 500 | small_town | 15% |
| Witham | 500 | small_town | 30% |
| Brentwood | 500 | small_town | 25% |
| Chipping Barnet | 500 | small_town | 15% |
| Bletchingley | 500 | small_town | 15% |
| Westerham | 400 | small_town | 30% |
| Charing | 400 | small_town | 20% |
| Wrotham | 400 | small_town | 35% |
| Lenham | 400 | small_town | 30% |
| Elham | 400 | small_town | 35% |
| Wingham | 400 | small_town | 30% |
| Bromley | 400 | small_town | 30% |
| Robertsbridge | 400 | small_town | 40% |
| Hailsham | 400 | small_town | 55% |
| Mayfield | 400 | small_town | 0% |
| Cuckfield | 400 | small_town | 20% |
| Crawley | 400 | small_town | 5% |
| Epping | 400 | small_town | 30% |
| Billericay | 400 | small_town | 30% |
| Rayleigh | 400 | small_town | 30% |
| Rochford | 400 | small_town | 65% |
| Harlow | 400 | small_town | 45% |
| Chipping Ongar | 400 | small_town | 50% |
| Hoddesdon | 400 | small_town | 25% |
| Sawbridgeworth | 400 | small_town | 25% |
| Hatfield | 400 | small_town | 15% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 107 | 6 | 75% | Colchester | 25 | 8 |
| 3 | 47 | 75% | London (City) | 11 | 7 |
| 116 | 57 | 55% | Sandwich | 14 | 5 |
| 83 | 27 | 50% | Rochford | 17 | 1 |
