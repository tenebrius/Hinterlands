# Real-terrain scores: kent

Interior only: everything within 10 km of the map edge left out.

## All 58 real towns against the model's top 58

20 seeds, final year; random baselines drop the same 58 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.42 | 0.38 | 0.32 | 10% |
| Real towns with a model place within 10 km | 0.88 | 0.83 | 0.69 | 15% |
| Model places with a real town within 5 km | 0.44 | 0.42 | 0.40 | 10% |
| Model places with a real town within 10 km | 0.90 | 0.91 | 0.88 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.68 | 0.32 | 0.41 | 60% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.21 | -0.00 | 0.20 | 45% |

## The 23 real towns of 1,000 or more against the model's top 23

20 seeds, final year; random baselines drop the same 23 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.30 | 0.17 | 0.20 | 30% |
| Real towns with a model place within 10 km | 0.59 | 0.47 | 0.48 | 15% |
| Model places with a real town within 5 km | 0.27 | 0.17 | 0.20 | 20% |
| Model places with a real town within 10 km | 0.55 | 0.49 | 0.54 | 5% |
| Real top 10 with a model top-15 place within 10 km | 0.68 | 0.33 | 0.42 | 60% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.31 | -0.01 | 0.11 | 45% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (58) | 0.55 | 0.49 | 0.26 | 0.16 |
| 1,000 or more (23) | 0.66 | 0.60 | 0.36 | 0.16 |
| Under 1,000 (35) | 0.48 | 0.41 | 0.19 | 0.16 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 5; any top-58 place within 10 km: 20. Median distance from the model's largest place to London (City): 43 km. Largest place's share of the region's people (median): 0.28.

Nearest real town to the model's largest place, by seeds: Gravesend 10, Rochford 8, Rayleigh 1, Faversham 1.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 17 | 29% | 27.9 | 48% | 45% |
| coast | 4 | 7% | 7.9 | 14% | 57% |
| inland | 37 | 64% | 22.1 | 38% | 39% |

Inland places over 500 people, nearest other place over 500 (median): real 10.6 km (22 towns); model 5.9 km (median over seeds; 18 such places per seed).

Mean nearest-neighbour spacing of the 58 places: real 8.5 km, model 7.3 km, random land 6.7 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.52 | 0.91 | 0.45 | 0.88 | 0.70 | 0.15 | Bradesea 36233 |
| 2 | 0.43 | 0.90 | 0.53 | 0.95 | 0.90 | 0.17 | Ickingport 39420 |
| 3 | 0.41 | 0.90 | 0.43 | 0.93 | 0.60 | 0.34 | Iselerholt 52977 |
| 4 | 0.53 | 0.91 | 0.57 | 0.91 | 0.70 | 0.21 | Bareshaven 43982 |
| 5 | 0.41 | 0.93 | 0.43 | 0.88 | 0.80 | 0.18 | Wexehaven 43410 |
| 6 | 0.45 | 0.88 | 0.52 | 0.91 | 0.50 | 0.12 | Langerford 37866 |
| 7 | 0.41 | 0.90 | 0.45 | 0.86 | 0.70 | 0.32 | Mereinghurst 42082 |
| 8 | 0.41 | 0.90 | 0.47 | 0.93 | 0.80 | 0.16 | Pickelden 47158 |
| 9 | 0.38 | 0.93 | 0.38 | 0.90 | 0.70 | 0.34 | Thetwick 46243 |
| 10 | 0.38 | 0.90 | 0.38 | 0.88 | 0.70 | 0.22 | Marhurst 50696 |
| 11 | 0.45 | 0.90 | 0.45 | 0.91 | 0.80 | 0.11 | Selingholt 52681 |
| 12 | 0.47 | 0.90 | 0.50 | 0.88 | 0.60 | 0.10 | Thornham 48482 |
| 13 | 0.31 | 0.83 | 0.40 | 0.84 | 0.70 | 0.35 | Ickingport 33280 |
| 14 | 0.40 | 0.93 | 0.40 | 0.97 | 0.50 | 0.07 | Bareport 43731 |
| 15 | 0.36 | 0.88 | 0.41 | 0.84 | 0.60 | 0.07 | Wickesea 38035 |
| 16 | 0.47 | 0.79 | 0.48 | 0.91 | 0.60 | 0.35 | Gilden 52766 |
| 17 | 0.34 | 0.90 | 0.29 | 0.95 | 0.60 | 0.23 | Rayton 32277 |
| 18 | 0.36 | 0.78 | 0.47 | 0.88 | 0.60 | 0.22 | Leyingholt 50852 |
| 19 | 0.41 | 0.81 | 0.43 | 0.86 | 0.70 | 0.32 | Kinanhurst 53317 |
| 20 | 0.43 | 0.90 | 0.45 | 0.91 | 0.80 | 0.19 | Malefield 40284 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 25% |
| Canterbury | 10000 | city | 100% |
| Sandwich | 4000 | large_town | 100% |
| Winchelsea | 3500 | large_town | 95% |
| Dover | 3000 | large_town | 25% |
| New Romney | 2500 | town | 90% |
| Rye | 2500 | town | 100% |
| Lewes | 2500 | town | 0% |
| Faversham | 2000 | town | 80% |
| Hythe | 2000 | town | 65% |
| Hastings | 2000 | town | 0% |
| Maidstone | 2000 | town | 70% |
| Maldon | 2000 | town | 40% |
| Rochester | 1900 | town | 15% |
| Lydd | 1500 | town | 35% |
| Chelmsford | 1500 | town | 100% |
| Battle | 1200 | town | 0% |
| Tonbridge | 1200 | town | 45% |
| Dartford | 1200 | town | 75% |
| Hertford | 1200 | town | 0% |
| Ware | 1200 | town | 0% |
| Croydon | 1000 | town | 25% |
| Folkestone | 1000 | town | 70% |
| East Grinstead | 800 | small_town | 10% |
| Milton Regis | 800 | small_town | 70% |
| Barking | 800 | small_town | 10% |
| Waltham Abbey | 800 | small_town | 95% |
| Fordwich | 600 | small_town | 95% |
| Tenterden | 600 | small_town | 0% |
| Ashford | 600 | small_town | 95% |
| Sevenoaks | 600 | small_town | 50% |
| West Malling | 600 | small_town | 40% |
| Cranbrook | 600 | small_town | 10% |
| Gravesend | 600 | small_town | 30% |
| Romford | 600 | small_town | 15% |
| Wye | 500 | small_town | 30% |
| Witham | 500 | small_town | 5% |
| Brentwood | 500 | small_town | 0% |
| Bletchingley | 500 | small_town | 0% |
| Westerham | 400 | small_town | 0% |
| Charing | 400 | small_town | 45% |
| Wrotham | 400 | small_town | 0% |
| Lenham | 400 | small_town | 5% |
| Elham | 400 | small_town | 10% |
| Wingham | 400 | small_town | 0% |
| Bromley | 400 | small_town | 65% |
| Robertsbridge | 400 | small_town | 70% |
| Hailsham | 400 | small_town | 30% |
| Mayfield | 400 | small_town | 15% |
| Cuckfield | 400 | small_town | 85% |
| Epping | 400 | small_town | 60% |
| Billericay | 400 | small_town | 25% |
| Rayleigh | 400 | small_town | 10% |
| Rochford | 400 | small_town | 5% |
| Harlow | 400 | small_town | 70% |
| Chipping Ongar | 400 | small_town | 55% |
| Hoddesdon | 400 | small_town | 90% |
| Sawbridgeworth | 400 | small_town | 70% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 41 | 42 | 75% | Brentwood | 10 | 19 |
| 104 | 12 | 65% | Maldon | 38 | 0 |
| 91 | 10 | 65% | Maldon | 26 | 21 |
| 81 | 34 | 65% | Rochford | 12 | 0 |
| 29 | 89 | 55% | East Grinstead | 11 | 50 |
| 26 | 103 | 55% | Mayfield | 13 | 44 |
