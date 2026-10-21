# Real-terrain scores: kent

## All 70 real towns against the model's top 70

20 seeds, final year; random baselines drop the same 70 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.40 | 0.37 | 0.30 | 0% |
| Real towns with a model place within 10 km | 0.87 | 0.82 | 0.65 | 15% |
| Model places with a real town within 5 km | 0.43 | 0.41 | 0.38 | 0% |
| Model places with a real town within 10 km | 0.88 | 0.90 | 0.85 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.49 | 0.29 | 0.38 | 20% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.10 | -0.03 | 0.20 | 15% |

## The 27 real towns of 1,000 or more against the model's top 27

20 seeds, final year; random baselines drop the same 27 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.27 | 0.15 | 0.17 | 15% |
| Real towns with a model place within 10 km | 0.58 | 0.45 | 0.46 | 35% |
| Model places with a real town within 5 km | 0.23 | 0.15 | 0.18 | 5% |
| Model places with a real town within 10 km | 0.51 | 0.48 | 0.52 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.49 | 0.29 | 0.39 | 20% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.25 | 0.01 | 0.14 | 35% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (70) | 0.55 | 0.48 | 0.23 | 0.16 |
| 1,000 or more (27) | 0.58 | 0.51 | 0.25 | 0.16 |
| Under 1,000 (43) | 0.54 | 0.47 | 0.22 | 0.16 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 6; any top-70 place within 10 km: 20. Median distance from the model's largest place to London (City): 41 km. Largest place's share of the region's people (median): 0.25.

Nearest real town to the model's largest place, by seeds: Gravesend 11, Faversham 9.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 18 | 26% | 31.4 | 45% | 52% |
| coast | 6 | 9% | 9.3 | 13% | 43% |
| inland | 46 | 66% | 29.3 | 42% | 34% |

Inland places over 500 people, nearest other place over 500 (median): real 10.6 km (29 towns); model 5.8 km (median over seeds; 25 such places per seed).

Mean nearest-neighbour spacing of the 70 places: real 8.6 km, model 7.3 km, random land 6.9 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.39 | 0.87 | 0.41 | 0.86 | 0.60 | 0.15 | Glenhaven 41354 |
| 2 | 0.34 | 0.79 | 0.36 | 0.89 | 0.40 | 0.05 | Kinerstone 35811 |
| 3 | 0.37 | 0.89 | 0.39 | 0.86 | 0.50 | 0.14 | Iselerholt 37132 |
| 4 | 0.44 | 0.90 | 0.47 | 0.89 | 0.30 | -0.00 | Yarenhaven 41785 |
| 5 | 0.39 | 0.87 | 0.40 | 0.93 | 0.50 | 0.20 | Thetwick 46188 |
| 6 | 0.44 | 0.87 | 0.41 | 0.90 | 0.60 | 0.08 | Barport 35144 |
| 7 | 0.39 | 0.89 | 0.43 | 0.87 | 0.60 | 0.12 | Maringhurst 33777 |
| 8 | 0.39 | 0.79 | 0.43 | 0.93 | 0.50 | 0.08 | Pickelden 48941 |
| 9 | 0.37 | 0.86 | 0.46 | 0.89 | 0.40 | 0.17 | Shapwood 44466 |
| 10 | 0.41 | 0.86 | 0.47 | 0.83 | 0.50 | 0.09 | Weneford 46787 |
| 11 | 0.40 | 0.89 | 0.43 | 0.89 | 0.50 | 0.06 | Wyowsea 32617 |
| 12 | 0.37 | 0.89 | 0.49 | 0.84 | 0.40 | -0.00 | Stowanden 34185 |
| 13 | 0.40 | 0.89 | 0.47 | 0.86 | 0.50 | 0.11 | Kirkenbourne 36811 |
| 14 | 0.40 | 0.87 | 0.49 | 0.90 | 0.60 | -0.01 | Oxingwick 44834 |
| 15 | 0.41 | 0.83 | 0.44 | 0.90 | 0.50 | 0.06 | Melelhaven 38212 |
| 16 | 0.40 | 0.84 | 0.43 | 0.93 | 0.50 | 0.16 | Gilden 41469 |
| 17 | 0.37 | 0.91 | 0.39 | 0.87 | 0.50 | 0.07 | Yarenhaven 38494 |
| 18 | 0.43 | 0.89 | 0.43 | 0.87 | 0.50 | 0.10 | Croden 45584 |
| 19 | 0.36 | 0.86 | 0.43 | 0.84 | 0.50 | 0.26 | Calingport 29858 |
| 20 | 0.44 | 0.90 | 0.43 | 0.89 | 0.50 | 0.08 | Kelpit 31114 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 15% |
| Canterbury | 10000 | city | 5% |
| Colchester | 4000 | large_town | 5% |
| Sandwich | 4000 | large_town | 100% |
| Winchelsea | 3500 | large_town | 100% |
| Dover | 3000 | large_town | 40% |
| New Romney | 2500 | town | 95% |
| Rye | 2500 | town | 100% |
| Lewes | 2500 | town | 5% |
| Maidstone | 2000 | town | 70% |
| Hythe | 2000 | town | 90% |
| Hastings | 2000 | town | 0% |
| Maldon | 2000 | town | 10% |
| Faversham | 2000 | town | 50% |
| Rochester | 1900 | town | 25% |
| Lydd | 1500 | town | 50% |
| Chelmsford | 1500 | town | 100% |
| New Shoreham | 1500 | town | 5% |
| Battle | 1200 | town | 5% |
| Tonbridge | 1200 | town | 65% |
| Dartford | 1200 | town | 75% |
| Hertford | 1200 | town | 0% |
| Ware | 1200 | town | 0% |
| Croydon | 1000 | town | 30% |
| Reigate | 1000 | town | 50% |
| Folkestone | 1000 | town | 65% |
| Bishop's Stortford | 1000 | town | 20% |
| Seaford | 800 | small_town | 0% |
| East Grinstead | 800 | small_town | 0% |
| Milton Regis | 800 | small_town | 65% |
| Barking | 800 | small_town | 10% |
| Waltham Abbey | 800 | small_town | 100% |
| Pevensey | 600 | small_town | 100% |
| Fordwich | 600 | small_town | 15% |
| Tenterden | 600 | small_town | 10% |
| Ashford | 600 | small_town | 75% |
| Sevenoaks | 600 | small_town | 40% |
| West Malling | 600 | small_town | 45% |
| Cranbrook | 600 | small_town | 5% |
| Gravesend | 600 | small_town | 40% |
| Romford | 600 | small_town | 5% |
| Coggeshall | 600 | small_town | 90% |
| Braintree | 600 | small_town | 5% |
| Great Dunmow | 600 | small_town | 90% |
| Wye | 500 | small_town | 25% |
| Witham | 500 | small_town | 10% |
| Brentwood | 500 | small_town | 5% |
| Chipping Barnet | 500 | small_town | 20% |
| Bletchingley | 500 | small_town | 10% |
| Westerham | 400 | small_town | 0% |
| Charing | 400 | small_town | 0% |
| Wrotham | 400 | small_town | 0% |
| Lenham | 400 | small_town | 0% |
| Elham | 400 | small_town | 0% |
| Wingham | 400 | small_town | 35% |
| Bromley | 400 | small_town | 45% |
| Robertsbridge | 400 | small_town | 70% |
| Hailsham | 400 | small_town | 45% |
| Mayfield | 400 | small_town | 25% |
| Cuckfield | 400 | small_town | 85% |
| Crawley | 400 | small_town | 50% |
| Epping | 400 | small_town | 40% |
| Billericay | 400 | small_town | 5% |
| Rayleigh | 400 | small_town | 25% |
| Rochford | 400 | small_town | 45% |
| Harlow | 400 | small_town | 50% |
| Chipping Ongar | 400 | small_town | 70% |
| Hoddesdon | 400 | small_town | 85% |
| Sawbridgeworth | 400 | small_town | 75% |
| Hatfield | 400 | small_town | 80% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 81 | 16 | 100% | Colchester | 14 | 1 |
| 46 | 89 | 100% | Cranbrook | 11 | 78 |
| 3 | 47 | 100% | London (City) | 11 | 7 |
| 104 | 11 | 75% | Colchester | 23 | 2 |
| 104 | 4 | 70% | Colchester | 22 | 0 |
| 26 | 103 | 65% | Mayfield | 13 | 44 |
| 22 | 1 | 65% | Bishop's Stortford | 10 | 98 |
| 91 | 9 | 60% | Colchester | 11 | 12 |
| 31 | 91 | 50% | Mayfield | 10 | 100 |
