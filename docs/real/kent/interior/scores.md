# Real-terrain scores: kent

Interior only: everything within 10 km of the map edge left out.

## All 58 real towns against the model's top 58

20 seeds, final year; random baselines drop the same 58 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.42 | 0.38 | 0.32 | 5% |
| Real towns with a model place within 10 km | 0.88 | 0.83 | 0.69 | 15% |
| Model places with a real town within 5 km | 0.43 | 0.42 | 0.40 | 0% |
| Model places with a real town within 10 km | 0.89 | 0.91 | 0.88 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.60 | 0.32 | 0.41 | 30% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.16 | -0.00 | 0.20 | 20% |

## The 23 real towns of 1,000 or more against the model's top 23

20 seeds, final year; random baselines drop the same 23 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.27 | 0.17 | 0.20 | 10% |
| Real towns with a model place within 10 km | 0.58 | 0.47 | 0.48 | 10% |
| Model places with a real town within 5 km | 0.24 | 0.17 | 0.20 | 10% |
| Model places with a real town within 10 km | 0.52 | 0.49 | 0.54 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.60 | 0.33 | 0.42 | 30% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.29 | -0.01 | 0.11 | 40% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (58) | 0.58 | 0.50 | 0.25 | 0.17 |
| 1,000 or more (23) | 0.65 | 0.58 | 0.31 | 0.17 |
| Under 1,000 (35) | 0.53 | 0.45 | 0.21 | 0.17 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 5; any top-58 place within 10 km: 20. Median distance from the model's largest place to London (City): 41 km. Largest place's share of the region's people (median): 0.30.

Nearest real town to the model's largest place, by seeds: Gravesend 10, Faversham 4, Rayleigh 3, Milton Regis 2, Wingham 1.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 17 | 29% | 27.9 | 48% | 44% |
| coast | 4 | 7% | 7.6 | 13% | 61% |
| inland | 37 | 64% | 22.4 | 39% | 38% |

Inland places over 500 people, nearest other place over 500 (median): real 10.6 km (22 towns); model 6.4 km (median over seeds; 22 such places per seed).

Mean nearest-neighbour spacing of the 58 places: real 8.5 km, model 7.7 km, random land 6.7 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.45 | 0.93 | 0.45 | 0.86 | 0.80 | 0.21 | Dunholt 52456 |
| 2 | 0.41 | 0.84 | 0.43 | 0.88 | 0.50 | 0.15 | Croeford 50366 |
| 3 | 0.38 | 0.90 | 0.38 | 0.93 | 0.60 | 0.10 | Fairesholt 55459 |
| 4 | 0.41 | 0.81 | 0.47 | 0.88 | 0.60 | 0.19 | Faleshaven 54227 |
| 5 | 0.43 | 0.91 | 0.45 | 0.88 | 0.70 | 0.29 | Penwick 46548 |
| 6 | 0.34 | 0.83 | 0.36 | 0.88 | 0.70 | 0.14 | Torport 59460 |
| 7 | 0.45 | 0.90 | 0.45 | 0.90 | 0.80 | 0.18 | Halhaven 43455 |
| 8 | 0.50 | 0.90 | 0.52 | 0.88 | 0.50 | 0.18 | Noreington 45849 |
| 9 | 0.41 | 0.93 | 0.45 | 0.84 | 0.50 | 0.08 | Roasea 41543 |
| 10 | 0.41 | 0.86 | 0.48 | 0.91 | 0.60 | 0.20 | Oakhurst 53772 |
| 11 | 0.43 | 0.84 | 0.38 | 0.91 | 0.30 | 0.23 | Craneley 52494 |
| 12 | 0.41 | 0.90 | 0.41 | 0.95 | 0.60 | 0.05 | Falden 47189 |
| 13 | 0.38 | 0.86 | 0.45 | 0.95 | 0.60 | 0.29 | Roingley 61687 |
| 14 | 0.40 | 0.90 | 0.41 | 0.84 | 0.60 | 0.11 | Barford 25690 |
| 15 | 0.41 | 0.86 | 0.41 | 0.84 | 0.60 | 0.14 | Dewesfield 39708 |
| 16 | 0.43 | 0.88 | 0.43 | 0.91 | 0.60 | 0.16 | Verholt 55883 |
| 17 | 0.47 | 0.93 | 0.48 | 0.91 | 0.50 | 0.28 | Iselstead 47725 |
| 18 | 0.47 | 0.91 | 0.47 | 0.91 | 0.70 | 0.13 | Holstead 42742 |
| 19 | 0.34 | 0.86 | 0.33 | 0.86 | 0.50 | -0.07 | Fenesham 45030 |
| 20 | 0.38 | 0.90 | 0.41 | 0.88 | 0.70 | 0.07 | Verby 47563 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 40% |
| Canterbury | 10000 | city | 90% |
| Sandwich | 4000 | large_town | 90% |
| Winchelsea | 3500 | large_town | 100% |
| Dover | 3000 | large_town | 25% |
| New Romney | 2500 | town | 90% |
| Rye | 2500 | town | 100% |
| Lewes | 2500 | town | 0% |
| Faversham | 2000 | town | 60% |
| Hythe | 2000 | town | 65% |
| Hastings | 2000 | town | 10% |
| Maidstone | 2000 | town | 35% |
| Maldon | 2000 | town | 20% |
| Rochester | 1900 | town | 30% |
| Lydd | 1500 | town | 10% |
| Chelmsford | 1500 | town | 85% |
| Battle | 1200 | town | 10% |
| Tonbridge | 1200 | town | 50% |
| Dartford | 1200 | town | 80% |
| Hertford | 1200 | town | 0% |
| Ware | 1200 | town | 0% |
| Croydon | 1000 | town | 35% |
| Folkestone | 1000 | town | 70% |
| East Grinstead | 800 | small_town | 0% |
| Milton Regis | 800 | small_town | 50% |
| Barking | 800 | small_town | 20% |
| Waltham Abbey | 800 | small_town | 100% |
| Fordwich | 600 | small_town | 90% |
| Tenterden | 600 | small_town | 10% |
| Ashford | 600 | small_town | 70% |
| Sevenoaks | 600 | small_town | 50% |
| West Malling | 600 | small_town | 55% |
| Cranbrook | 600 | small_town | 15% |
| Gravesend | 600 | small_town | 35% |
| Romford | 600 | small_town | 5% |
| Wye | 500 | small_town | 35% |
| Witham | 500 | small_town | 0% |
| Brentwood | 500 | small_town | 5% |
| Bletchingley | 500 | small_town | 0% |
| Westerham | 400 | small_town | 0% |
| Charing | 400 | small_town | 0% |
| Wrotham | 400 | small_town | 0% |
| Lenham | 400 | small_town | 0% |
| Elham | 400 | small_town | 5% |
| Wingham | 400 | small_town | 25% |
| Bromley | 400 | small_town | 65% |
| Robertsbridge | 400 | small_town | 85% |
| Hailsham | 400 | small_town | 30% |
| Mayfield | 400 | small_town | 30% |
| Cuckfield | 400 | small_town | 100% |
| Epping | 400 | small_town | 65% |
| Billericay | 400 | small_town | 25% |
| Rayleigh | 400 | small_town | 40% |
| Rochford | 400 | small_town | 35% |
| Harlow | 400 | small_town | 80% |
| Chipping Ongar | 400 | small_town | 50% |
| Hoddesdon | 400 | small_town | 65% |
| Sawbridgeworth | 400 | small_town | 80% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 104 | 12 | 85% | Maldon | 38 | 0 |
| 29 | 89 | 80% | East Grinstead | 11 | 50 |
| 61 | 80 | 70% | Lenham | 10 | 24 |
| 88 | 11 | 60% | Maldon | 23 | 6 |
| 26 | 103 | 60% | Mayfield | 13 | 44 |
