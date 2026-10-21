# Real-terrain scores: kent

## All 70 real towns against the model's top 70

20 seeds, final year; random baselines drop the same 70 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.34 | 0.37 | 0.30 | 0% |
| Real towns with a model place within 10 km | 0.71 | 0.82 | 0.65 | 0% |
| Model places with a real town within 5 km | 0.43 | 0.41 | 0.38 | 5% |
| Model places with a real town within 10 km | 0.81 | 0.90 | 0.85 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.40 | 0.29 | 0.38 | 10% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.17 | -0.03 | 0.20 | 45% |

## The 27 real towns of 1,000 or more against the model's top 27

20 seeds, final year; random baselines drop the same 27 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.21 | 0.15 | 0.17 | 10% |
| Real towns with a model place within 10 km | 0.51 | 0.45 | 0.46 | 5% |
| Model places with a real town within 5 km | 0.21 | 0.15 | 0.18 | 5% |
| Model places with a real town within 10 km | 0.50 | 0.48 | 0.52 | 10% |
| Real top 10 with a model top-15 place within 10 km | 0.40 | 0.29 | 0.39 | 10% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.07 | 0.01 | 0.14 | 5% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (70) | 0.58 | 0.49 | 0.19 | 0.13 |
| 1,000 or more (27) | 0.72 | 0.64 | 0.29 | 0.13 |
| Under 1,000 (43) | 0.49 | 0.39 | 0.13 | 0.13 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 0; any top-70 place within 10 km: 16. Median distance from the model's largest place to London (City): 54 km.

Nearest real town to the model's largest place, by seeds: Gravesend 5, Fordwich 3, Rochford 2, Rayleigh 2, Colchester 2, Rochester 1, Maldon 1, Barking 1, Canterbury 1, Dartford 1, Milton Regis 1.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 18 | 26% | 28.8 | 41% | 50% |
| coast | 6 | 9% | 18.1 | 26% | 44% |
| inland | 46 | 66% | 23.1 | 33% | 26% |

Inland places over 500 people, nearest other place over 500 (median): real 10.6 km (29 towns); model 11.6 km (median over seeds; 10 such places per seed).

Mean nearest-neighbour spacing of the 70 places: real 8.6 km, model 6.3 km, random land 6.9 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.39 | 0.76 | 0.47 | 0.83 | 0.50 | 0.22 | Wexbourne 12708 |
| 2 | 0.37 | 0.76 | 0.43 | 0.86 | 0.40 | 0.04 | Veresmouth 6826 |
| 3 | 0.36 | 0.76 | 0.44 | 0.89 | 0.40 | -0.12 | Reneswick 6850 |
| 4 | 0.29 | 0.74 | 0.39 | 0.77 | 0.30 | 0.10 | Thornmouth 6950 |
| 5 | 0.39 | 0.74 | 0.49 | 0.84 | 0.30 | 0.24 | Veranford 6168 |
| 6 | 0.37 | 0.70 | 0.50 | 0.86 | 0.40 | 0.19 | Torwood 5505 |
| 7 | 0.37 | 0.73 | 0.41 | 0.77 | 0.40 | -0.12 | Stanenmouth 8194 |
| 8 | 0.31 | 0.76 | 0.34 | 0.80 | 0.40 | 0.29 | Oxelwick 6937 |
| 9 | 0.30 | 0.63 | 0.39 | 0.83 | 0.20 | 0.36 | Leyelmouth 7650 |
| 10 | 0.30 | 0.64 | 0.37 | 0.76 | 0.40 | 0.30 | Ordewick 14257 |
| 11 | 0.34 | 0.67 | 0.51 | 0.86 | 0.20 | 0.09 | Elmingmouth 6440 |
| 12 | 0.31 | 0.70 | 0.37 | 0.73 | 0.70 | 0.34 | Ickelbourne 5544 |
| 13 | 0.34 | 0.76 | 0.41 | 0.80 | 0.50 | 0.14 | Walden 4711 |
| 14 | 0.29 | 0.56 | 0.39 | 0.81 | 0.50 | 0.15 | Gilenmouth 8732 |
| 15 | 0.37 | 0.70 | 0.47 | 0.80 | 0.50 | 0.39 | Belenhurst 7384 |
| 16 | 0.36 | 0.76 | 0.47 | 0.76 | 0.10 | 0.10 | Dunmouth 9923 |
| 17 | 0.40 | 0.73 | 0.46 | 0.79 | 0.40 | 0.32 | Maranmouth 7170 |
| 18 | 0.30 | 0.66 | 0.40 | 0.73 | 0.60 | 0.04 | Falmouth 13406 |
| 19 | 0.31 | 0.74 | 0.43 | 0.83 | 0.30 | 0.17 | Barmouth 13743 |
| 20 | 0.30 | 0.63 | 0.49 | 0.80 | 0.40 | 0.12 | Hartsea 8042 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 25% |
| Canterbury | 10000 | city | 50% |
| Colchester | 4000 | large_town | 10% |
| Sandwich | 4000 | large_town | 55% |
| Winchelsea | 3500 | large_town | 50% |
| Dover | 3000 | large_town | 60% |
| New Romney | 2500 | town | 75% |
| Rye | 2500 | town | 35% |
| Lewes | 2500 | town | 25% |
| Maidstone | 2000 | town | 55% |
| Hythe | 2000 | town | 50% |
| Hastings | 2000 | town | 45% |
| Maldon | 2000 | town | 80% |
| Faversham | 2000 | town | 35% |
| Rochester | 1900 | town | 75% |
| Lydd | 1500 | town | 65% |
| Chelmsford | 1500 | town | 45% |
| New Shoreham | 1500 | town | 60% |
| Battle | 1200 | town | 0% |
| Tonbridge | 1200 | town | 55% |
| Dartford | 1200 | town | 45% |
| Hertford | 1200 | town | 25% |
| Ware | 1200 | town | 25% |
| Croydon | 1000 | town | 60% |
| Reigate | 1000 | town | 15% |
| Folkestone | 1000 | town | 45% |
| Bishop's Stortford | 1000 | town | 30% |
| Seaford | 800 | small_town | 15% |
| East Grinstead | 800 | small_town | 0% |
| Milton Regis | 800 | small_town | 55% |
| Barking | 800 | small_town | 65% |
| Waltham Abbey | 800 | small_town | 30% |
| Pevensey | 600 | small_town | 65% |
| Fordwich | 600 | small_town | 30% |
| Tenterden | 600 | small_town | 0% |
| Ashford | 600 | small_town | 45% |
| Sevenoaks | 600 | small_town | 50% |
| West Malling | 600 | small_town | 50% |
| Cranbrook | 600 | small_town | 0% |
| Gravesend | 600 | small_town | 50% |
| Romford | 600 | small_town | 0% |
| Coggeshall | 600 | small_town | 50% |
| Braintree | 600 | small_town | 0% |
| Great Dunmow | 600 | small_town | 55% |
| Wye | 500 | small_town | 20% |
| Witham | 500 | small_town | 45% |
| Brentwood | 500 | small_town | 0% |
| Chipping Barnet | 500 | small_town | 0% |
| Bletchingley | 500 | small_town | 0% |
| Westerham | 400 | small_town | 0% |
| Charing | 400 | small_town | 5% |
| Wrotham | 400 | small_town | 0% |
| Lenham | 400 | small_town | 0% |
| Elham | 400 | small_town | 0% |
| Wingham | 400 | small_town | 20% |
| Bromley | 400 | small_town | 40% |
| Robertsbridge | 400 | small_town | 40% |
| Hailsham | 400 | small_town | 70% |
| Mayfield | 400 | small_town | 25% |
| Cuckfield | 400 | small_town | 30% |
| Crawley | 400 | small_town | 15% |
| Epping | 400 | small_town | 15% |
| Billericay | 400 | small_town | 45% |
| Rayleigh | 400 | small_town | 40% |
| Rochford | 400 | small_town | 20% |
| Harlow | 400 | small_town | 45% |
| Chipping Ongar | 400 | small_town | 10% |
| Hoddesdon | 400 | small_town | 35% |
| Sawbridgeworth | 400 | small_town | 55% |
| Hatfield | 400 | small_town | 40% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 108 | 3 | 100% | Colchester | 26 | 0 |
| 102 | 0 | 100% | Colchester | 20 | 17 |
| 105 | 10 | 90% | Colchester | 24 | 0 |
| 98 | 15 | 75% | Colchester | 20 | 1 |
| 118 | 57 | 70% | Sandwich | 14 | 10 |
| 120 | 63 | 65% | Sandwich | 10 | 6 |
| 80 | 35 | 55% | Rochford | 11 | -3 |
| 3 | 48 | 55% | London (City) | 11 | 7 |
| 60 | 47 | 50% | Rayleigh | 12 | 1 |
| 32 | 86 | 50% | Tonbridge | 10 | 97 |
