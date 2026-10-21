# Real-terrain scores: kent

## All 70 real towns against the model's top 70

20 seeds, final year; random baselines drop the same 70 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.39 | 0.37 | 0.30 | 10% |
| Real towns with a model place within 10 km | 0.87 | 0.82 | 0.65 | 25% |
| Model places with a real town within 5 km | 0.43 | 0.41 | 0.38 | 5% |
| Model places with a real town within 10 km | 0.88 | 0.90 | 0.85 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.55 | 0.29 | 0.38 | 55% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.15 | -0.03 | 0.20 | 40% |

## The 27 real towns of 1,000 or more against the model's top 27

20 seeds, final year; random baselines drop the same 27 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.25 | 0.15 | 0.17 | 15% |
| Real towns with a model place within 10 km | 0.60 | 0.45 | 0.46 | 50% |
| Model places with a real town within 5 km | 0.23 | 0.15 | 0.18 | 15% |
| Model places with a real town within 10 km | 0.54 | 0.48 | 0.52 | 5% |
| Real top 10 with a model top-15 place within 10 km | 0.55 | 0.29 | 0.39 | 55% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.29 | 0.01 | 0.14 | 35% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (70) | 0.54 | 0.47 | 0.24 | 0.16 |
| 1,000 or more (27) | 0.60 | 0.54 | 0.30 | 0.16 |
| Under 1,000 (43) | 0.50 | 0.43 | 0.20 | 0.16 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 5; any top-70 place within 10 km: 20. Median distance from the model's largest place to London (City): 43 km. Largest place's share of the region's people (median): 0.28.

Nearest real town to the model's largest place, by seeds: Gravesend 10, Rochford 8, Rayleigh 1, Faversham 1.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 18 | 26% | 29.4 | 42% | 43% |
| coast | 6 | 9% | 10.4 | 15% | 38% |
| inland | 46 | 66% | 30.1 | 43% | 37% |

Inland places over 500 people, nearest other place over 500 (median): real 10.6 km (29 towns); model 6.0 km (median over seeds; 26 such places per seed).

Mean nearest-neighbour spacing of the 70 places: real 8.6 km, model 7.5 km, random land 6.9 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.49 | 0.90 | 0.46 | 0.87 | 0.50 | 0.08 | Bradesea 36233 |
| 2 | 0.39 | 0.89 | 0.47 | 0.89 | 0.60 | 0.11 | Ickingport 39420 |
| 3 | 0.40 | 0.89 | 0.43 | 0.94 | 0.50 | 0.25 | Iselerholt 52977 |
| 4 | 0.47 | 0.87 | 0.51 | 0.89 | 0.60 | 0.17 | Bareshaven 43982 |
| 5 | 0.36 | 0.89 | 0.40 | 0.87 | 0.60 | 0.15 | Wexehaven 43410 |
| 6 | 0.41 | 0.86 | 0.47 | 0.90 | 0.50 | 0.06 | Langerford 37866 |
| 7 | 0.40 | 0.91 | 0.44 | 0.86 | 0.60 | 0.21 | Mereinghurst 42082 |
| 8 | 0.39 | 0.87 | 0.47 | 0.90 | 0.60 | 0.09 | Pickelden 47158 |
| 9 | 0.34 | 0.87 | 0.37 | 0.87 | 0.50 | 0.25 | Thetwick 46243 |
| 10 | 0.36 | 0.87 | 0.37 | 0.87 | 0.60 | 0.13 | Marhurst 50696 |
| 11 | 0.43 | 0.87 | 0.44 | 0.89 | 0.70 | 0.03 | Selingholt 52681 |
| 12 | 0.43 | 0.84 | 0.49 | 0.83 | 0.50 | 0.01 | Thornham 48482 |
| 13 | 0.30 | 0.83 | 0.40 | 0.83 | 0.60 | 0.32 | Ickingport 33280 |
| 14 | 0.36 | 0.93 | 0.37 | 0.93 | 0.60 | 0.05 | Bareport 43731 |
| 15 | 0.34 | 0.93 | 0.41 | 0.86 | 0.50 | 0.06 | Wickesea 38035 |
| 16 | 0.43 | 0.83 | 0.47 | 0.89 | 0.50 | 0.26 | Gilden 52766 |
| 17 | 0.31 | 0.91 | 0.30 | 0.93 | 0.40 | 0.20 | Rayton 32277 |
| 18 | 0.37 | 0.76 | 0.46 | 0.84 | 0.50 | 0.12 | Leyingholt 50852 |
| 19 | 0.41 | 0.77 | 0.44 | 0.84 | 0.60 | 0.19 | Kinanhurst 53317 |
| 20 | 0.37 | 0.89 | 0.40 | 0.89 | 0.60 | 0.18 | Malefield 40284 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 25% |
| Canterbury | 10000 | city | 100% |
| Colchester | 4000 | large_town | 0% |
| Sandwich | 4000 | large_town | 100% |
| Winchelsea | 3500 | large_town | 95% |
| Dover | 3000 | large_town | 25% |
| New Romney | 2500 | town | 90% |
| Rye | 2500 | town | 100% |
| Lewes | 2500 | town | 0% |
| Maidstone | 2000 | town | 70% |
| Hythe | 2000 | town | 65% |
| Hastings | 2000 | town | 0% |
| Maldon | 2000 | town | 40% |
| Faversham | 2000 | town | 75% |
| Rochester | 1900 | town | 15% |
| Lydd | 1500 | town | 30% |
| Chelmsford | 1500 | town | 100% |
| New Shoreham | 1500 | town | 0% |
| Battle | 1200 | town | 0% |
| Tonbridge | 1200 | town | 45% |
| Dartford | 1200 | town | 75% |
| Hertford | 1200 | town | 0% |
| Ware | 1200 | town | 0% |
| Croydon | 1000 | town | 25% |
| Reigate | 1000 | town | 20% |
| Folkestone | 1000 | town | 70% |
| Bishop's Stortford | 1000 | town | 10% |
| Seaford | 800 | small_town | 0% |
| East Grinstead | 800 | small_town | 5% |
| Milton Regis | 800 | small_town | 70% |
| Barking | 800 | small_town | 5% |
| Waltham Abbey | 800 | small_town | 95% |
| Pevensey | 600 | small_town | 15% |
| Fordwich | 600 | small_town | 95% |
| Tenterden | 600 | small_town | 0% |
| Ashford | 600 | small_town | 95% |
| Sevenoaks | 600 | small_town | 50% |
| West Malling | 600 | small_town | 40% |
| Cranbrook | 600 | small_town | 10% |
| Gravesend | 600 | small_town | 30% |
| Romford | 600 | small_town | 15% |
| Coggeshall | 600 | small_town | 100% |
| Braintree | 600 | small_town | 5% |
| Great Dunmow | 600 | small_town | 75% |
| Wye | 500 | small_town | 30% |
| Witham | 500 | small_town | 5% |
| Brentwood | 500 | small_town | 0% |
| Chipping Barnet | 500 | small_town | 10% |
| Bletchingley | 500 | small_town | 0% |
| Westerham | 400 | small_town | 0% |
| Charing | 400 | small_town | 40% |
| Wrotham | 400 | small_town | 0% |
| Lenham | 400 | small_town | 5% |
| Elham | 400 | small_town | 10% |
| Wingham | 400 | small_town | 0% |
| Bromley | 400 | small_town | 65% |
| Robertsbridge | 400 | small_town | 65% |
| Hailsham | 400 | small_town | 35% |
| Mayfield | 400 | small_town | 15% |
| Cuckfield | 400 | small_town | 85% |
| Crawley | 400 | small_town | 35% |
| Epping | 400 | small_town | 60% |
| Billericay | 400 | small_town | 25% |
| Rayleigh | 400 | small_town | 10% |
| Rochford | 400 | small_town | 5% |
| Harlow | 400 | small_town | 70% |
| Chipping Ongar | 400 | small_town | 50% |
| Hoddesdon | 400 | small_town | 90% |
| Sawbridgeworth | 400 | small_town | 75% |
| Hatfield | 400 | small_town | 50% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 90 | 9 | 95% | Colchester | 11 | 20 |
| 2 | 46 | 95% | London (City) | 11 | 9 |
| 15 | 124 | 85% | Lewes | 11 | -5 |
| 41 | 42 | 75% | Brentwood | 10 | 19 |
| 104 | 11 | 70% | Colchester | 23 | 2 |
| 81 | 34 | 65% | Rochford | 12 | 0 |
| 29 | 89 | 60% | East Grinstead | 11 | 50 |
| 104 | 5 | 55% | Colchester | 22 | 10 |
| 26 | 103 | 50% | Mayfield | 13 | 44 |
