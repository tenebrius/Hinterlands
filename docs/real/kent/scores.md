# Real-terrain scores: kent

## All 70 real towns against the model's top 70

20 seeds, final year; random baselines drop the same 70 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.37 | 0.37 | 0.30 | 5% |
| Real towns with a model place within 10 km | 0.82 | 0.82 | 0.65 | 5% |
| Model places with a real town within 5 km | 0.43 | 0.41 | 0.38 | 5% |
| Model places with a real town within 10 km | 0.87 | 0.90 | 0.85 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.43 | 0.29 | 0.38 | 25% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.08 | -0.03 | 0.20 | 15% |

## The 27 real towns of 1,000 or more against the model's top 27

20 seeds, final year; random baselines drop the same 27 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.20 | 0.15 | 0.17 | 0% |
| Real towns with a model place within 10 km | 0.54 | 0.45 | 0.46 | 10% |
| Model places with a real town within 5 km | 0.20 | 0.15 | 0.18 | 0% |
| Model places with a real town within 10 km | 0.52 | 0.48 | 0.52 | 5% |
| Real top 10 with a model top-15 place within 10 km | 0.43 | 0.29 | 0.39 | 25% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.12 | 0.01 | 0.14 | 10% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (70) | 0.57 | 0.48 | 0.21 | 0.15 |
| 1,000 or more (27) | 0.63 | 0.54 | 0.25 | 0.15 |
| Under 1,000 (43) | 0.53 | 0.44 | 0.19 | 0.15 |

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 18 | 26% | 34.1 | 49% | 63% |
| coast | 6 | 9% | 9.1 | 13% | 22% |
| inland | 46 | 66% | 26.8 | 38% | 29% |

Mean nearest-neighbour spacing of the 70 places: real 8.6 km, model 7.1 km, random land 6.9 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.34 | 0.81 | 0.39 | 0.89 | 0.60 | 0.16 | Wexanhaven 19317 |
| 2 | 0.36 | 0.77 | 0.40 | 0.80 | 0.50 | 0.08 | Ulowmouth 19566 |
| 3 | 0.37 | 0.89 | 0.44 | 0.91 | 0.30 | 0.03 | Nedenmouth 16998 |
| 4 | 0.43 | 0.79 | 0.44 | 0.83 | 0.60 | 0.22 | Oakesmouth 27465 |
| 5 | 0.41 | 0.79 | 0.46 | 0.90 | 0.50 | 0.01 | Hartsea 17519 |
| 6 | 0.33 | 0.81 | 0.40 | 0.81 | 0.30 | 0.21 | Yarelsea 23207 |
| 7 | 0.33 | 0.91 | 0.37 | 0.93 | 0.30 | -0.07 | Falwick 19183 |
| 8 | 0.36 | 0.89 | 0.37 | 0.84 | 0.40 | 0.16 | Torelport 27309 |
| 9 | 0.34 | 0.87 | 0.43 | 0.91 | 0.50 | 0.06 | Walanport 17027 |
| 10 | 0.33 | 0.74 | 0.39 | 0.76 | 0.60 | 0.18 | Everey 14429 |
| 11 | 0.30 | 0.81 | 0.39 | 0.87 | 0.40 | -0.18 | Wickhaven 20469 |
| 12 | 0.36 | 0.84 | 0.41 | 0.89 | 0.60 | 0.10 | Hartsea 12017 |
| 13 | 0.41 | 0.86 | 0.44 | 0.91 | 0.30 | 0.12 | Pickinghaven 26261 |
| 14 | 0.46 | 0.81 | 0.47 | 0.84 | 0.40 | -0.11 | Winsesford 16430 |
| 15 | 0.36 | 0.84 | 0.41 | 0.93 | 0.40 | 0.15 | Thorningport 18753 |
| 16 | 0.40 | 0.83 | 0.57 | 0.93 | 0.20 | 0.06 | Pickerwick 24342 |
| 17 | 0.41 | 0.83 | 0.44 | 0.87 | 0.60 | 0.08 | Calensea 22991 |
| 18 | 0.34 | 0.76 | 0.40 | 0.79 | 0.50 | 0.07 | Aldeley 27345 |
| 19 | 0.39 | 0.74 | 0.43 | 0.84 | 0.20 | 0.08 | Asherford 17440 |
| 20 | 0.43 | 0.83 | 0.46 | 0.86 | 0.50 | 0.16 | Ordansea 20454 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 20% |
| Canterbury | 10000 | city | 75% |
| Colchester | 4000 | large_town | 0% |
| Sandwich | 4000 | large_town | 35% |
| Winchelsea | 3500 | large_town | 50% |
| Dover | 3000 | large_town | 40% |
| New Romney | 2500 | town | 95% |
| Rye | 2500 | town | 30% |
| Lewes | 2500 | town | 40% |
| Maidstone | 2000 | town | 65% |
| Hythe | 2000 | town | 5% |
| Hastings | 2000 | town | 30% |
| Maldon | 2000 | town | 60% |
| Faversham | 2000 | town | 65% |
| Rochester | 1900 | town | 75% |
| Lydd | 1500 | town | 35% |
| Chelmsford | 1500 | town | 65% |
| New Shoreham | 1500 | town | 25% |
| Battle | 1200 | town | 0% |
| Tonbridge | 1200 | town | 45% |
| Dartford | 1200 | town | 40% |
| Hertford | 1200 | town | 60% |
| Ware | 1200 | town | 55% |
| Croydon | 1000 | town | 60% |
| Reigate | 1000 | town | 20% |
| Folkestone | 1000 | town | 5% |
| Bishop's Stortford | 1000 | town | 15% |
| Seaford | 800 | small_town | 20% |
| East Grinstead | 800 | small_town | 0% |
| Milton Regis | 800 | small_town | 70% |
| Barking | 800 | small_town | 55% |
| Waltham Abbey | 800 | small_town | 50% |
| Pevensey | 600 | small_town | 75% |
| Fordwich | 600 | small_town | 65% |
| Tenterden | 600 | small_town | 0% |
| Ashford | 600 | small_town | 55% |
| Sevenoaks | 600 | small_town | 55% |
| West Malling | 600 | small_town | 45% |
| Cranbrook | 600 | small_town | 0% |
| Gravesend | 600 | small_town | 80% |
| Romford | 600 | small_town | 0% |
| Coggeshall | 600 | small_town | 75% |
| Braintree | 600 | small_town | 0% |
| Great Dunmow | 600 | small_town | 15% |
| Wye | 500 | small_town | 10% |
| Witham | 500 | small_town | 55% |
| Brentwood | 500 | small_town | 0% |
| Chipping Barnet | 500 | small_town | 0% |
| Bletchingley | 500 | small_town | 0% |
| Westerham | 400 | small_town | 0% |
| Charing | 400 | small_town | 10% |
| Wrotham | 400 | small_town | 0% |
| Lenham | 400 | small_town | 0% |
| Elham | 400 | small_town | 0% |
| Wingham | 400 | small_town | 45% |
| Bromley | 400 | small_town | 50% |
| Robertsbridge | 400 | small_town | 65% |
| Hailsham | 400 | small_town | 90% |
| Mayfield | 400 | small_town | 20% |
| Cuckfield | 400 | small_town | 50% |
| Crawley | 400 | small_town | 25% |
| Epping | 400 | small_town | 20% |
| Billericay | 400 | small_town | 55% |
| Rayleigh | 400 | small_town | 35% |
| Rochford | 400 | small_town | 10% |
| Harlow | 400 | small_town | 45% |
| Chipping Ongar | 400 | small_town | 65% |
| Hoddesdon | 400 | small_town | 55% |
| Sawbridgeworth | 400 | small_town | 55% |
| Hatfield | 400 | small_town | 80% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 105 | 60 | 75% | Wingham | 11 | 2 |
| 106 | 9 | 70% | Colchester | 25 | 6 |
| 32 | 87 | 70% | Tonbridge | 11 | 100 |
| 64 | 55 | 65% | Rochester | 10 | -2 |
| 4 | 48 | 55% | London (City) | 10 | 5 |
| 1 | 58 | 55% | Croydon | 12 | 24 |
| 81 | 14 | 50% | Colchester | 12 | 1 |
| 50 | 79 | 50% | Maidstone | 11 | 14 |
