# Real-terrain scores: kent

## All 70 real towns against the model's top 70

20 seeds, final year; random baselines drop the same 70 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.40 | 0.37 | 0.30 | 10% |
| Real towns with a model place within 10 km | 0.86 | 0.82 | 0.65 | 20% |
| Model places with a real town within 5 km | 0.43 | 0.41 | 0.38 | 0% |
| Model places with a real town within 10 km | 0.89 | 0.90 | 0.85 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.53 | 0.29 | 0.38 | 35% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.12 | -0.03 | 0.20 | 20% |

## The 27 real towns of 1,000 or more against the model's top 27

20 seeds, final year; random baselines drop the same 27 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.27 | 0.15 | 0.17 | 10% |
| Real towns with a model place within 10 km | 0.59 | 0.45 | 0.46 | 45% |
| Model places with a real town within 5 km | 0.24 | 0.15 | 0.18 | 5% |
| Model places with a real town within 10 km | 0.52 | 0.48 | 0.52 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.53 | 0.29 | 0.39 | 35% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.26 | 0.01 | 0.14 | 25% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (70) | 0.57 | 0.49 | 0.23 | 0.16 |
| 1,000 or more (27) | 0.61 | 0.53 | 0.27 | 0.16 |
| Under 1,000 (43) | 0.54 | 0.46 | 0.21 | 0.16 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 6; any top-70 place within 10 km: 20. Median distance from the model's largest place to London (City): 58 km. Largest place's share of the region's people (median): 0.25.

Nearest real town to the model's largest place, by seeds: Faversham 10, Gravesend 10.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 18 | 26% | 31.5 | 45% | 54% |
| coast | 6 | 9% | 10.1 | 14% | 39% |
| inland | 46 | 66% | 28.4 | 41% | 34% |

Inland places over 500 people, nearest other place over 500 (median): real 10.6 km (29 towns); model 6.0 km (median over seeds; 22 such places per seed).

Mean nearest-neighbour spacing of the 70 places: real 8.6 km, model 7.3 km, random land 6.9 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.41 | 0.86 | 0.47 | 0.90 | 0.60 | 0.21 | Dunelpit 32254 |
| 2 | 0.43 | 0.89 | 0.41 | 0.94 | 0.50 | 0.14 | Kinerstone 39269 |
| 3 | 0.39 | 0.83 | 0.40 | 0.87 | 0.50 | 0.22 | Iselerholt 43280 |
| 4 | 0.43 | 0.89 | 0.50 | 0.89 | 0.50 | 0.15 | Yarenhaven 38505 |
| 5 | 0.39 | 0.86 | 0.41 | 0.93 | 0.50 | 0.11 | Thetwick 42639 |
| 6 | 0.37 | 0.94 | 0.39 | 0.91 | 0.40 | 0.11 | Barport 34050 |
| 7 | 0.36 | 0.89 | 0.40 | 0.89 | 0.30 | 0.15 | Daringden 33435 |
| 8 | 0.39 | 0.83 | 0.43 | 0.89 | 0.50 | 0.11 | Pickelden 44854 |
| 9 | 0.27 | 0.77 | 0.39 | 0.86 | 0.50 | 0.25 | Ewelsea 36930 |
| 10 | 0.41 | 0.89 | 0.46 | 0.84 | 0.60 | 0.06 | Weneford 46275 |
| 11 | 0.40 | 0.87 | 0.46 | 0.89 | 0.60 | 0.16 | Leyerwick 37608 |
| 12 | 0.36 | 0.89 | 0.46 | 0.84 | 0.50 | -0.02 | Stowanden 39101 |
| 13 | 0.46 | 0.91 | 0.49 | 0.86 | 0.50 | 0.16 | Kirkenbourne 37983 |
| 14 | 0.40 | 0.84 | 0.46 | 0.91 | 0.70 | 0.04 | Oxingwick 41240 |
| 15 | 0.41 | 0.86 | 0.41 | 0.89 | 0.60 | 0.03 | Melelhaven 34716 |
| 16 | 0.44 | 0.93 | 0.46 | 0.93 | 0.60 | 0.13 | Gilden 36744 |
| 17 | 0.39 | 0.90 | 0.40 | 0.90 | 0.60 | 0.03 | Yarenhaven 40254 |
| 18 | 0.46 | 0.81 | 0.41 | 0.84 | 0.50 | -0.03 | Croden 48183 |
| 19 | 0.37 | 0.80 | 0.43 | 0.86 | 0.50 | 0.23 | Reneswood 40218 |
| 20 | 0.44 | 0.86 | 0.44 | 0.91 | 0.50 | 0.11 | Kelpit 35193 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 25% |
| Canterbury | 10000 | city | 0% |
| Colchester | 4000 | large_town | 10% |
| Sandwich | 4000 | large_town | 100% |
| Winchelsea | 3500 | large_town | 100% |
| Dover | 3000 | large_town | 50% |
| New Romney | 2500 | town | 95% |
| Rye | 2500 | town | 100% |
| Lewes | 2500 | town | 10% |
| Maidstone | 2000 | town | 70% |
| Hythe | 2000 | town | 85% |
| Hastings | 2000 | town | 0% |
| Maldon | 2000 | town | 15% |
| Faversham | 2000 | town | 50% |
| Rochester | 1900 | town | 30% |
| Lydd | 1500 | town | 40% |
| Chelmsford | 1500 | town | 100% |
| New Shoreham | 1500 | town | 5% |
| Battle | 1200 | town | 10% |
| Tonbridge | 1200 | town | 70% |
| Dartford | 1200 | town | 80% |
| Hertford | 1200 | town | 0% |
| Ware | 1200 | town | 5% |
| Croydon | 1000 | town | 45% |
| Reigate | 1000 | town | 35% |
| Folkestone | 1000 | town | 45% |
| Bishop's Stortford | 1000 | town | 15% |
| Seaford | 800 | small_town | 0% |
| East Grinstead | 800 | small_town | 0% |
| Milton Regis | 800 | small_town | 65% |
| Barking | 800 | small_town | 25% |
| Waltham Abbey | 800 | small_town | 100% |
| Pevensey | 600 | small_town | 100% |
| Fordwich | 600 | small_town | 15% |
| Tenterden | 600 | small_town | 10% |
| Ashford | 600 | small_town | 70% |
| Sevenoaks | 600 | small_town | 45% |
| West Malling | 600 | small_town | 35% |
| Cranbrook | 600 | small_town | 10% |
| Gravesend | 600 | small_town | 45% |
| Romford | 600 | small_town | 15% |
| Coggeshall | 600 | small_town | 95% |
| Braintree | 600 | small_town | 5% |
| Great Dunmow | 600 | small_town | 95% |
| Wye | 500 | small_town | 25% |
| Witham | 500 | small_town | 10% |
| Brentwood | 500 | small_town | 5% |
| Chipping Barnet | 500 | small_town | 0% |
| Bletchingley | 500 | small_town | 10% |
| Westerham | 400 | small_town | 0% |
| Charing | 400 | small_town | 5% |
| Wrotham | 400 | small_town | 0% |
| Lenham | 400 | small_town | 0% |
| Elham | 400 | small_town | 0% |
| Wingham | 400 | small_town | 40% |
| Bromley | 400 | small_town | 40% |
| Robertsbridge | 400 | small_town | 60% |
| Hailsham | 400 | small_town | 40% |
| Mayfield | 400 | small_town | 25% |
| Cuckfield | 400 | small_town | 80% |
| Crawley | 400 | small_town | 40% |
| Epping | 400 | small_town | 45% |
| Billericay | 400 | small_town | 0% |
| Rayleigh | 400 | small_town | 50% |
| Rochford | 400 | small_town | 35% |
| Harlow | 400 | small_town | 50% |
| Chipping Ongar | 400 | small_town | 75% |
| Hoddesdon | 400 | small_town | 95% |
| Sawbridgeworth | 400 | small_town | 70% |
| Hatfield | 400 | small_town | 70% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 81 | 16 | 100% | Colchester | 14 | 1 |
| 46 | 89 | 95% | Cranbrook | 11 | 78 |
| 3 | 47 | 95% | London (City) | 11 | 7 |
| 104 | 11 | 75% | Colchester | 23 | 2 |
| 26 | 103 | 75% | Mayfield | 13 | 44 |
| 22 | 1 | 65% | Bishop's Stortford | 10 | 98 |
| 90 | 9 | 60% | Colchester | 11 | 20 |
| 104 | 4 | 55% | Colchester | 22 | 0 |
