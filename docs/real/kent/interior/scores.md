# Real-terrain scores: kent

Interior only: everything within 10 km of the map edge left out.

## All 58 real towns against the model's top 58

20 seeds, final year; random baselines drop the same 58 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.34 | 0.38 | 0.32 | 0% |
| Real towns with a model place within 10 km | 0.77 | 0.83 | 0.69 | 0% |
| Model places with a real town within 5 km | 0.41 | 0.42 | 0.40 | 0% |
| Model places with a real town within 10 km | 0.88 | 0.91 | 0.88 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.43 | 0.32 | 0.41 | 5% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.13 | -0.00 | 0.20 | 30% |

## The 23 real towns of 1,000 or more against the model's top 23

20 seeds, final year; random baselines drop the same 23 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.18 | 0.17 | 0.20 | 5% |
| Real towns with a model place within 10 km | 0.52 | 0.47 | 0.48 | 5% |
| Model places with a real town within 5 km | 0.18 | 0.17 | 0.20 | 10% |
| Model places with a real town within 10 km | 0.53 | 0.49 | 0.54 | 5% |
| Real top 10 with a model top-15 place within 10 km | 0.43 | 0.33 | 0.42 | 5% |
| Rank agreement (Spearman, real size vs model size within 5 km) | -0.00 | -0.01 | 0.11 | 5% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (58) | 0.53 | 0.45 | 0.17 | 0.16 |
| 1,000 or more (23) | 0.66 | 0.58 | 0.24 | 0.16 |
| Under 1,000 (35) | 0.44 | 0.36 | 0.13 | 0.16 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 0; any top-58 place within 10 km: 19. Median distance from the model's largest place to London (City): 41 km. Largest place's share of the region's people (median): 0.30.

Nearest real town to the model's largest place, by seeds: Gravesend 15, Faversham 3, Fordwich 2.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 17 | 29% | 21.2 | 37% | 40% |
| coast | 4 | 7% | 6.0 | 10% | 38% |
| inland | 37 | 64% | 30.7 | 53% | 32% |

Inland places over 500 people, nearest other place over 500 (median): real 10.6 km (22 towns); model 9.0 km (median over seeds; 16 such places per seed).

Mean nearest-neighbour spacing of the 58 places: real 8.5 km, model 6.8 km, random land 6.7 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.33 | 0.69 | 0.40 | 0.84 | 0.10 | 0.05 | Iselhaven 11779 |
| 2 | 0.41 | 0.79 | 0.43 | 0.79 | 0.30 | 0.22 | Malden 33981 |
| 3 | 0.31 | 0.81 | 0.40 | 0.88 | 0.50 | 0.05 | Belham 40110 |
| 4 | 0.36 | 0.74 | 0.38 | 0.91 | 0.50 | 0.03 | Talelholt 38906 |
| 5 | 0.34 | 0.76 | 0.50 | 0.90 | 0.40 | 0.08 | Buresholt 31398 |
| 6 | 0.24 | 0.71 | 0.22 | 0.76 | 0.50 | 0.06 | Seleley 38736 |
| 7 | 0.36 | 0.78 | 0.38 | 0.84 | 0.50 | -0.08 | Stanenwood 16652 |
| 8 | 0.33 | 0.71 | 0.40 | 0.95 | 0.70 | 0.35 | Grimerford 29479 |
| 9 | 0.28 | 0.76 | 0.29 | 0.84 | 0.50 | 0.22 | Elmelcote 37927 |
| 10 | 0.33 | 0.60 | 0.41 | 0.86 | 0.50 | 0.08 | Walenwood 35950 |
| 11 | 0.38 | 0.76 | 0.48 | 0.90 | 0.60 | 0.15 | Uperworth 32864 |
| 12 | 0.43 | 0.86 | 0.47 | 0.95 | 0.60 | -0.06 | Everingford 50741 |
| 13 | 0.40 | 0.91 | 0.43 | 0.95 | 0.10 | 0.27 | Upenbourne 38987 |
| 14 | 0.34 | 0.76 | 0.36 | 0.88 | 0.20 | 0.27 | Roenhurst 34598 |
| 15 | 0.33 | 0.88 | 0.43 | 0.86 | 0.50 | 0.06 | Wyowood 26668 |
| 16 | 0.34 | 0.90 | 0.38 | 0.86 | 0.30 | 0.03 | Fairenhurst 42253 |
| 17 | 0.36 | 0.78 | 0.45 | 0.95 | 0.60 | 0.29 | Upenbourne 36579 |
| 18 | 0.28 | 0.71 | 0.40 | 0.84 | 0.30 | 0.20 | Holesey 40270 |
| 19 | 0.40 | 0.74 | 0.52 | 0.91 | 0.40 | 0.12 | Chedewood 31643 |
| 20 | 0.34 | 0.83 | 0.43 | 0.90 | 0.60 | 0.19 | Upenbourne 38018 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 50% |
| Canterbury | 10000 | city | 30% |
| Sandwich | 4000 | large_town | 30% |
| Winchelsea | 3500 | large_town | 40% |
| Dover | 3000 | large_town | 40% |
| New Romney | 2500 | town | 90% |
| Rye | 2500 | town | 30% |
| Lewes | 2500 | town | 5% |
| Faversham | 2000 | town | 45% |
| Hythe | 2000 | town | 35% |
| Hastings | 2000 | town | 30% |
| Maidstone | 2000 | town | 50% |
| Maldon | 2000 | town | 80% |
| Rochester | 1900 | town | 45% |
| Lydd | 1500 | town | 70% |
| Chelmsford | 1500 | town | 40% |
| Battle | 1200 | town | 40% |
| Tonbridge | 1200 | town | 50% |
| Dartford | 1200 | town | 45% |
| Hertford | 1200 | town | 15% |
| Ware | 1200 | town | 5% |
| Croydon | 1000 | town | 60% |
| Folkestone | 1000 | town | 45% |
| East Grinstead | 800 | small_town | 15% |
| Milton Regis | 800 | small_town | 5% |
| Barking | 800 | small_town | 55% |
| Waltham Abbey | 800 | small_town | 45% |
| Fordwich | 600 | small_town | 35% |
| Tenterden | 600 | small_town | 20% |
| Ashford | 600 | small_town | 65% |
| Sevenoaks | 600 | small_town | 30% |
| West Malling | 600 | small_town | 55% |
| Cranbrook | 600 | small_town | 25% |
| Gravesend | 600 | small_town | 15% |
| Romford | 600 | small_town | 10% |
| Wye | 500 | small_town | 30% |
| Witham | 500 | small_town | 55% |
| Brentwood | 500 | small_town | 35% |
| Bletchingley | 500 | small_town | 5% |
| Westerham | 400 | small_town | 30% |
| Charing | 400 | small_town | 25% |
| Wrotham | 400 | small_town | 45% |
| Lenham | 400 | small_town | 15% |
| Elham | 400 | small_town | 20% |
| Wingham | 400 | small_town | 15% |
| Bromley | 400 | small_town | 30% |
| Robertsbridge | 400 | small_town | 35% |
| Hailsham | 400 | small_town | 30% |
| Mayfield | 400 | small_town | 35% |
| Cuckfield | 400 | small_town | 10% |
| Epping | 400 | small_town | 35% |
| Billericay | 400 | small_town | 25% |
| Rayleigh | 400 | small_town | 60% |
| Rochford | 400 | small_town | 40% |
| Harlow | 400 | small_town | 20% |
| Chipping Ongar | 400 | small_town | 35% |
| Hoddesdon | 400 | small_town | 10% |
| Sawbridgeworth | 400 | small_town | 15% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 115 | 59 | 55% | Sandwich | 11 | 19 |
| 81 | 31 | 55% | Rochford | 13 | 0 |
