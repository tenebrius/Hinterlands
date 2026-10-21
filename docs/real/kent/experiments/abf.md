# Real-terrain scores: kent

## All 70 real towns against the model's top 70

20 seeds, final year; random baselines drop the same 70 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.34 | 0.37 | 0.30 | 0% |
| Real towns with a model place within 10 km | 0.75 | 0.82 | 0.65 | 0% |
| Model places with a real town within 5 km | 0.43 | 0.41 | 0.38 | 10% |
| Model places with a real town within 10 km | 0.83 | 0.90 | 0.85 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.33 | 0.29 | 0.38 | 0% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.15 | -0.03 | 0.20 | 45% |

## The 27 real towns of 1,000 or more against the model's top 27

20 seeds, final year; random baselines drop the same 27 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.22 | 0.15 | 0.17 | 5% |
| Real towns with a model place within 10 km | 0.51 | 0.45 | 0.46 | 5% |
| Model places with a real town within 5 km | 0.22 | 0.15 | 0.18 | 5% |
| Model places with a real town within 10 km | 0.52 | 0.48 | 0.52 | 10% |
| Real top 10 with a model top-15 place within 10 km | 0.33 | 0.29 | 0.39 | 0% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.07 | 0.01 | 0.14 | 5% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (70) | 0.56 | 0.47 | 0.19 | 0.14 |
| 1,000 or more (27) | 0.65 | 0.56 | 0.23 | 0.14 |
| Under 1,000 (43) | 0.50 | 0.42 | 0.16 | 0.14 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 0; any top-70 place within 10 km: 17. Median distance from the model's largest place to London (City): 39 km. Largest place's share of the region's people (median): 0.26.

Nearest real town to the model's largest place, by seeds: Gravesend 12, Rayleigh 2, Barking 1, Faversham 1, Croydon 1, Sandwich 1, Brentwood 1, Rochford 1.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 18 | 26% | 29.4 | 42% | 49% |
| coast | 6 | 9% | 15.6 | 22% | 34% |
| inland | 46 | 66% | 25.0 | 36% | 28% |

Inland places over 500 people, nearest other place over 500 (median): real 10.6 km (29 towns); model 10.4 km (median over seeds; 12 such places per seed).

Mean nearest-neighbour spacing of the 70 places: real 8.6 km, model 6.4 km, random land 6.9 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.34 | 0.74 | 0.41 | 0.80 | 0.50 | 0.06 | Melham 15448 |
| 2 | 0.34 | 0.79 | 0.40 | 0.84 | 0.30 | 0.19 | Nedeport 28652 |
| 3 | 0.34 | 0.80 | 0.37 | 0.86 | 0.30 | 0.25 | Wenesham 25245 |
| 4 | 0.27 | 0.67 | 0.34 | 0.80 | 0.50 | 0.36 | Torerey 22850 |
| 5 | 0.34 | 0.83 | 0.40 | 0.87 | 0.30 | 0.07 | Belanport 30751 |
| 6 | 0.36 | 0.64 | 0.51 | 0.84 | 0.50 | 0.28 | Linerhaven 13254 |
| 7 | 0.39 | 0.86 | 0.46 | 0.86 | 0.10 | -0.00 | Seley 30785 |
| 8 | 0.31 | 0.81 | 0.43 | 0.89 | 0.30 | 0.10 | Ashanport 18224 |
| 9 | 0.34 | 0.74 | 0.36 | 0.87 | 0.30 | 0.14 | Dewingmouth 24541 |
| 10 | 0.31 | 0.64 | 0.36 | 0.74 | 0.40 | 0.22 | Norehaven 25236 |
| 11 | 0.34 | 0.74 | 0.44 | 0.76 | 0.10 | 0.01 | Dewbourne 23228 |
| 12 | 0.37 | 0.83 | 0.47 | 0.86 | 0.30 | 0.26 | Darwood 7170 |
| 13 | 0.30 | 0.73 | 0.47 | 0.81 | 0.30 | 0.03 | Wickanhaven 10577 |
| 14 | 0.26 | 0.59 | 0.33 | 0.80 | 0.20 | 0.35 | Wenerholt 13611 |
| 15 | 0.31 | 0.80 | 0.40 | 0.87 | 0.40 | 0.21 | Ewenhaven 23807 |
| 16 | 0.39 | 0.70 | 0.49 | 0.73 | 0.40 | 0.13 | Yarsea 19377 |
| 17 | 0.37 | 0.73 | 0.54 | 0.87 | 0.20 | 0.04 | Winserhaven 15801 |
| 18 | 0.33 | 0.76 | 0.44 | 0.93 | 0.10 | 0.22 | Chalelford 9721 |
| 19 | 0.33 | 0.76 | 0.43 | 0.86 | 0.50 | 0.03 | Wenesford 29855 |
| 20 | 0.44 | 0.80 | 0.47 | 0.80 | 0.50 | 0.13 | Ordhaven 21254 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 40% |
| Canterbury | 10000 | city | 50% |
| Colchester | 4000 | large_town | 5% |
| Sandwich | 4000 | large_town | 30% |
| Winchelsea | 3500 | large_town | 65% |
| Dover | 3000 | large_town | 20% |
| New Romney | 2500 | town | 75% |
| Rye | 2500 | town | 65% |
| Lewes | 2500 | town | 30% |
| Maidstone | 2000 | town | 60% |
| Hythe | 2000 | town | 20% |
| Hastings | 2000 | town | 45% |
| Maldon | 2000 | town | 70% |
| Faversham | 2000 | town | 55% |
| Rochester | 1900 | town | 80% |
| Lydd | 1500 | town | 55% |
| Chelmsford | 1500 | town | 55% |
| New Shoreham | 1500 | town | 30% |
| Battle | 1200 | town | 0% |
| Tonbridge | 1200 | town | 50% |
| Dartford | 1200 | town | 25% |
| Hertford | 1200 | town | 30% |
| Ware | 1200 | town | 35% |
| Croydon | 1000 | town | 50% |
| Reigate | 1000 | town | 10% |
| Folkestone | 1000 | town | 30% |
| Bishop's Stortford | 1000 | town | 35% |
| Seaford | 800 | small_town | 15% |
| East Grinstead | 800 | small_town | 0% |
| Milton Regis | 800 | small_town | 40% |
| Barking | 800 | small_town | 70% |
| Waltham Abbey | 800 | small_town | 55% |
| Pevensey | 600 | small_town | 70% |
| Fordwich | 600 | small_town | 60% |
| Tenterden | 600 | small_town | 0% |
| Ashford | 600 | small_town | 65% |
| Sevenoaks | 600 | small_town | 80% |
| West Malling | 600 | small_town | 25% |
| Cranbrook | 600 | small_town | 0% |
| Gravesend | 600 | small_town | 20% |
| Romford | 600 | small_town | 0% |
| Coggeshall | 600 | small_town | 70% |
| Braintree | 600 | small_town | 0% |
| Great Dunmow | 600 | small_town | 55% |
| Wye | 500 | small_town | 25% |
| Witham | 500 | small_town | 45% |
| Brentwood | 500 | small_town | 0% |
| Chipping Barnet | 500 | small_town | 0% |
| Bletchingley | 500 | small_town | 0% |
| Westerham | 400 | small_town | 0% |
| Charing | 400 | small_town | 0% |
| Wrotham | 400 | small_town | 0% |
| Lenham | 400 | small_town | 0% |
| Elham | 400 | small_town | 0% |
| Wingham | 400 | small_town | 25% |
| Bromley | 400 | small_town | 50% |
| Robertsbridge | 400 | small_town | 45% |
| Hailsham | 400 | small_town | 30% |
| Mayfield | 400 | small_town | 10% |
| Cuckfield | 400 | small_town | 30% |
| Crawley | 400 | small_town | 15% |
| Epping | 400 | small_town | 10% |
| Billericay | 400 | small_town | 50% |
| Rayleigh | 400 | small_town | 20% |
| Rochford | 400 | small_town | 0% |
| Harlow | 400 | small_town | 55% |
| Chipping Ongar | 400 | small_town | 50% |
| Hoddesdon | 400 | small_town | 40% |
| Sawbridgeworth | 400 | small_town | 70% |
| Hatfield | 400 | small_town | 70% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 107 | 2 | 75% | Colchester | 25 | -2 |
| 2 | 47 | 75% | London (City) | 12 | 8 |
| 30 | 88 | 60% | East Grinstead | 11 | 54 |
| 2 | 59 | 60% | Croydon | 10 | 34 |
| 118 | 59 | 55% | Sandwich | 12 | 41 |
| 102 | 12 | 55% | Colchester | 22 | 10 |
| 89 | 11 | 50% | Colchester | 11 | 1 |
