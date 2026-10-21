# Real-terrain scores: kent

## All 70 real towns against the model's top 70

20 seeds, final year; random baselines drop the same 70 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.42 | 0.37 | 0.30 | 20% |
| Real towns with a model place within 10 km | 0.89 | 0.82 | 0.65 | 55% |
| Model places with a real town within 5 km | 0.43 | 0.41 | 0.38 | 5% |
| Model places with a real town within 10 km | 0.88 | 0.90 | 0.85 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.36 | 0.29 | 0.38 | 10% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.11 | -0.03 | 0.20 | 25% |

## The 27 real towns of 1,000 or more against the model's top 27

20 seeds, final year; random baselines drop the same 27 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.25 | 0.15 | 0.17 | 10% |
| Real towns with a model place within 10 km | 0.57 | 0.45 | 0.46 | 40% |
| Model places with a real town within 5 km | 0.21 | 0.15 | 0.18 | 0% |
| Model places with a real town within 10 km | 0.55 | 0.48 | 0.52 | 10% |
| Real top 10 with a model top-15 place within 10 km | 0.36 | 0.29 | 0.39 | 10% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.12 | 0.01 | 0.14 | 10% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (70) | 0.54 | 0.45 | 0.22 | 0.16 |
| 1,000 or more (27) | 0.58 | 0.49 | 0.24 | 0.16 |
| Under 1,000 (43) | 0.51 | 0.42 | 0.20 | 0.16 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 0; any top-70 place within 10 km: 20. Median distance from the model's largest place to London (City): 41 km. Largest place's share of the region's people (median): 0.33.

Nearest real town to the model's largest place, by seeds: Gravesend 16, Faversham 3, Milton Regis 1.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 18 | 26% | 34.2 | 49% | 59% |
| coast | 6 | 9% | 7.9 | 11% | 54% |
| inland | 46 | 66% | 27.9 | 40% | 34% |

Inland places over 500 people, nearest other place over 500 (median): real 10.6 km (29 towns); model 7.2 km (median over seeds; 27 such places per seed).

Mean nearest-neighbour spacing of the 70 places: real 8.6 km, model 7.8 km, random land 6.9 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.40 | 0.90 | 0.37 | 0.87 | 0.30 | 0.23 | Ickholt 55428 |
| 2 | 0.29 | 0.84 | 0.30 | 0.89 | 0.30 | -0.07 | Malden 55951 |
| 3 | 0.47 | 0.91 | 0.47 | 0.90 | 0.30 | 0.07 | Belham 57305 |
| 4 | 0.54 | 0.90 | 0.50 | 0.90 | 0.50 | 0.10 | Talelholt 54099 |
| 5 | 0.49 | 0.89 | 0.54 | 0.94 | 0.40 | 0.02 | Buresholt 50321 |
| 6 | 0.39 | 0.94 | 0.40 | 0.83 | 0.40 | 0.03 | Seleley 57340 |
| 7 | 0.44 | 0.93 | 0.47 | 0.91 | 0.50 | 0.27 | Upenbourne 49659 |
| 8 | 0.41 | 0.84 | 0.43 | 0.84 | 0.20 | 0.11 | Grimerford 40146 |
| 9 | 0.39 | 0.90 | 0.40 | 0.87 | 0.30 | 0.23 | Elmelcote 46970 |
| 10 | 0.44 | 0.93 | 0.43 | 0.94 | 0.40 | 0.12 | Walenwood 61307 |
| 11 | 0.40 | 0.87 | 0.49 | 0.91 | 0.60 | 0.14 | Uperworth 46862 |
| 12 | 0.41 | 0.89 | 0.41 | 0.86 | 0.40 | 0.17 | Everingford 58326 |
| 13 | 0.40 | 0.89 | 0.40 | 0.87 | 0.20 | -0.01 | Upenbourne 58374 |
| 14 | 0.46 | 0.96 | 0.46 | 0.90 | 0.40 | 0.08 | Roenhurst 51236 |
| 15 | 0.43 | 0.84 | 0.43 | 0.87 | 0.40 | 0.11 | Reningsea 50447 |
| 16 | 0.39 | 0.84 | 0.41 | 0.86 | 0.20 | 0.13 | Fairenhurst 56721 |
| 17 | 0.43 | 0.86 | 0.44 | 0.87 | 0.60 | -0.01 | Upenbourne 57089 |
| 18 | 0.44 | 0.91 | 0.41 | 0.83 | 0.30 | 0.19 | Holesey 56449 |
| 19 | 0.43 | 0.90 | 0.44 | 0.86 | 0.10 | 0.15 | Chedewood 54362 |
| 20 | 0.40 | 0.90 | 0.41 | 0.91 | 0.40 | 0.13 | Upenbourne 58706 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 45% |
| Canterbury | 10000 | city | 65% |
| Colchester | 4000 | large_town | 0% |
| Sandwich | 4000 | large_town | 85% |
| Winchelsea | 3500 | large_town | 100% |
| Dover | 3000 | large_town | 10% |
| New Romney | 2500 | town | 60% |
| Rye | 2500 | town | 95% |
| Lewes | 2500 | town | 10% |
| Maidstone | 2000 | town | 50% |
| Hythe | 2000 | town | 80% |
| Hastings | 2000 | town | 30% |
| Maldon | 2000 | town | 55% |
| Faversham | 2000 | town | 85% |
| Rochester | 1900 | town | 25% |
| Lydd | 1500 | town | 30% |
| Chelmsford | 1500 | town | 70% |
| New Shoreham | 1500 | town | 5% |
| Battle | 1200 | town | 0% |
| Tonbridge | 1200 | town | 35% |
| Dartford | 1200 | town | 50% |
| Hertford | 1200 | town | 75% |
| Ware | 1200 | town | 25% |
| Croydon | 1000 | town | 70% |
| Reigate | 1000 | town | 15% |
| Folkestone | 1000 | town | 60% |
| Bishop's Stortford | 1000 | town | 15% |
| Seaford | 800 | small_town | 50% |
| East Grinstead | 800 | small_town | 0% |
| Milton Regis | 800 | small_town | 15% |
| Barking | 800 | small_town | 80% |
| Waltham Abbey | 800 | small_town | 40% |
| Pevensey | 600 | small_town | 95% |
| Fordwich | 600 | small_town | 85% |
| Tenterden | 600 | small_town | 10% |
| Ashford | 600 | small_town | 95% |
| Sevenoaks | 600 | small_town | 70% |
| West Malling | 600 | small_town | 35% |
| Cranbrook | 600 | small_town | 0% |
| Gravesend | 600 | small_town | 10% |
| Romford | 600 | small_town | 20% |
| Coggeshall | 600 | small_town | 90% |
| Braintree | 600 | small_town | 0% |
| Great Dunmow | 600 | small_town | 50% |
| Wye | 500 | small_town | 25% |
| Witham | 500 | small_town | 90% |
| Brentwood | 500 | small_town | 0% |
| Chipping Barnet | 500 | small_town | 0% |
| Bletchingley | 500 | small_town | 0% |
| Westerham | 400 | small_town | 0% |
| Charing | 400 | small_town | 5% |
| Wrotham | 400 | small_town | 0% |
| Lenham | 400 | small_town | 0% |
| Elham | 400 | small_town | 0% |
| Wingham | 400 | small_town | 70% |
| Bromley | 400 | small_town | 70% |
| Robertsbridge | 400 | small_town | 75% |
| Hailsham | 400 | small_town | 55% |
| Mayfield | 400 | small_town | 25% |
| Cuckfield | 400 | small_town | 35% |
| Crawley | 400 | small_town | 15% |
| Epping | 400 | small_town | 20% |
| Billericay | 400 | small_town | 35% |
| Rayleigh | 400 | small_town | 45% |
| Rochford | 400 | small_town | 35% |
| Harlow | 400 | small_town | 35% |
| Chipping Ongar | 400 | small_town | 75% |
| Hoddesdon | 400 | small_town | 95% |
| Sawbridgeworth | 400 | small_town | 90% |
| Hatfield | 400 | small_town | 70% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 104 | 60 | 95% | Fordwich | 10 | 2 |
| 3 | 47 | 90% | London (City) | 11 | 7 |
| 102 | 10 | 85% | Colchester | 21 | 10 |
| 82 | 14 | 80% | Colchester | 12 | 2 |
| 89 | 10 | 65% | Colchester | 11 | 3 |
| 24 | 105 | 50% | Lewes | 10 | 32 |
