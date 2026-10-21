# Real-terrain scores: kent

## All 70 real towns against the model's top 70

20 seeds, final year; random baselines drop the same 70 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.33 | 0.37 | 0.30 | 0% |
| Real towns with a model place within 10 km | 0.76 | 0.82 | 0.65 | 0% |
| Model places with a real town within 5 km | 0.39 | 0.41 | 0.38 | 0% |
| Model places with a real town within 10 km | 0.84 | 0.90 | 0.85 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.35 | 0.29 | 0.38 | 5% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.11 | -0.03 | 0.20 | 40% |

## The 27 real towns of 1,000 or more against the model's top 27

20 seeds, final year; random baselines drop the same 27 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.16 | 0.15 | 0.17 | 5% |
| Real towns with a model place within 10 km | 0.47 | 0.45 | 0.46 | 5% |
| Model places with a real town within 5 km | 0.16 | 0.15 | 0.18 | 5% |
| Model places with a real town within 10 km | 0.49 | 0.48 | 0.52 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.35 | 0.29 | 0.39 | 5% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.01 | 0.01 | 0.14 | 5% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (70) | 0.53 | 0.44 | 0.16 | 0.15 |
| 1,000 or more (27) | 0.64 | 0.54 | 0.22 | 0.15 |
| Under 1,000 (43) | 0.46 | 0.37 | 0.13 | 0.15 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 0; any top-70 place within 10 km: 19. Median distance from the model's largest place to London (City): 41 km. Largest place's share of the region's people (median): 0.30.

Nearest real town to the model's largest place, by seeds: Gravesend 15, Faversham 3, Fordwich 2.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 18 | 26% | 22.8 | 32% | 41% |
| coast | 6 | 9% | 8.4 | 12% | 32% |
| inland | 46 | 66% | 38.8 | 55% | 31% |

Inland places over 500 people, nearest other place over 500 (median): real 10.6 km (29 towns); model 9.1 km (median over seeds; 20 such places per seed).

Mean nearest-neighbour spacing of the 70 places: real 8.6 km, model 6.9 km, random land 6.9 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.33 | 0.73 | 0.37 | 0.87 | 0.30 | 0.05 | Iselhaven 11779 |
| 2 | 0.39 | 0.76 | 0.40 | 0.81 | 0.30 | 0.18 | Malden 33981 |
| 3 | 0.31 | 0.83 | 0.39 | 0.87 | 0.30 | -0.00 | Belham 40110 |
| 4 | 0.36 | 0.77 | 0.36 | 0.84 | 0.40 | 0.03 | Talelholt 38906 |
| 5 | 0.33 | 0.70 | 0.47 | 0.86 | 0.30 | 0.09 | Buresholt 31398 |
| 6 | 0.23 | 0.67 | 0.21 | 0.73 | 0.40 | 0.02 | Seleley 38736 |
| 7 | 0.33 | 0.74 | 0.36 | 0.79 | 0.20 | -0.06 | Stanenwood 16652 |
| 8 | 0.31 | 0.73 | 0.37 | 0.91 | 0.50 | 0.36 | Grimerford 29479 |
| 9 | 0.34 | 0.81 | 0.36 | 0.84 | 0.40 | 0.19 | Elmelcote 37927 |
| 10 | 0.29 | 0.54 | 0.39 | 0.81 | 0.30 | 0.13 | Walenwood 35950 |
| 11 | 0.36 | 0.74 | 0.44 | 0.90 | 0.30 | 0.06 | Uperworth 32864 |
| 12 | 0.40 | 0.83 | 0.43 | 0.89 | 0.50 | -0.12 | Everingford 50741 |
| 13 | 0.39 | 0.87 | 0.40 | 0.89 | 0.30 | 0.18 | Upenbourne 38987 |
| 14 | 0.31 | 0.70 | 0.34 | 0.81 | 0.10 | 0.22 | Roenhurst 34598 |
| 15 | 0.34 | 0.81 | 0.44 | 0.84 | 0.50 | 0.07 | Wyowood 26668 |
| 16 | 0.33 | 0.77 | 0.34 | 0.79 | 0.20 | 0.01 | Fairenhurst 42253 |
| 17 | 0.34 | 0.76 | 0.41 | 0.86 | 0.60 | 0.24 | Upenbourne 36579 |
| 18 | 0.26 | 0.73 | 0.33 | 0.79 | 0.20 | 0.19 | Holesey 40270 |
| 19 | 0.36 | 0.77 | 0.47 | 0.86 | 0.50 | 0.12 | Chedewood 31643 |
| 20 | 0.37 | 0.83 | 0.43 | 0.84 | 0.40 | 0.18 | Upenbourne 38018 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 50% |
| Canterbury | 10000 | city | 30% |
| Colchester | 4000 | large_town | 45% |
| Sandwich | 4000 | large_town | 30% |
| Winchelsea | 3500 | large_town | 40% |
| Dover | 3000 | large_town | 35% |
| New Romney | 2500 | town | 85% |
| Rye | 2500 | town | 30% |
| Lewes | 2500 | town | 10% |
| Maidstone | 2000 | town | 50% |
| Hythe | 2000 | town | 35% |
| Hastings | 2000 | town | 35% |
| Maldon | 2000 | town | 80% |
| Faversham | 2000 | town | 45% |
| Rochester | 1900 | town | 35% |
| Lydd | 1500 | town | 70% |
| Chelmsford | 1500 | town | 40% |
| New Shoreham | 1500 | town | 10% |
| Battle | 1200 | town | 40% |
| Tonbridge | 1200 | town | 50% |
| Dartford | 1200 | town | 45% |
| Hertford | 1200 | town | 15% |
| Ware | 1200 | town | 5% |
| Croydon | 1000 | town | 65% |
| Reigate | 1000 | town | 20% |
| Folkestone | 1000 | town | 45% |
| Bishop's Stortford | 1000 | town | 25% |
| Seaford | 800 | small_town | 25% |
| East Grinstead | 800 | small_town | 15% |
| Milton Regis | 800 | small_town | 5% |
| Barking | 800 | small_town | 55% |
| Waltham Abbey | 800 | small_town | 45% |
| Pevensey | 600 | small_town | 50% |
| Fordwich | 600 | small_town | 35% |
| Tenterden | 600 | small_town | 20% |
| Ashford | 600 | small_town | 65% |
| Sevenoaks | 600 | small_town | 30% |
| West Malling | 600 | small_town | 55% |
| Cranbrook | 600 | small_town | 25% |
| Gravesend | 600 | small_town | 15% |
| Romford | 600 | small_town | 10% |
| Coggeshall | 600 | small_town | 30% |
| Braintree | 600 | small_town | 45% |
| Great Dunmow | 600 | small_town | 30% |
| Wye | 500 | small_town | 30% |
| Witham | 500 | small_town | 55% |
| Brentwood | 500 | small_town | 35% |
| Chipping Barnet | 500 | small_town | 10% |
| Bletchingley | 500 | small_town | 5% |
| Westerham | 400 | small_town | 30% |
| Charing | 400 | small_town | 25% |
| Wrotham | 400 | small_town | 35% |
| Lenham | 400 | small_town | 15% |
| Elham | 400 | small_town | 20% |
| Wingham | 400 | small_town | 15% |
| Bromley | 400 | small_town | 30% |
| Robertsbridge | 400 | small_town | 30% |
| Hailsham | 400 | small_town | 55% |
| Mayfield | 400 | small_town | 30% |
| Cuckfield | 400 | small_town | 10% |
| Crawley | 400 | small_town | 0% |
| Epping | 400 | small_town | 35% |
| Billericay | 400 | small_town | 25% |
| Rayleigh | 400 | small_town | 60% |
| Rochford | 400 | small_town | 40% |
| Harlow | 400 | small_town | 20% |
| Chipping Ongar | 400 | small_town | 35% |
| Hoddesdon | 400 | small_town | 10% |
| Sawbridgeworth | 400 | small_town | 25% |
| Hatfield | 400 | small_town | 35% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 104 | 2 | 70% | Colchester | 22 | 0 |
| 105 | 8 | 65% | Colchester | 23 | 15 |
| 115 | 59 | 55% | Sandwich | 11 | 19 |
| 3 | 47 | 55% | London (City) | 11 | 7 |
| 95 | 10 | 50% | Colchester | 15 | 18 |
| 81 | 31 | 50% | Rochford | 13 | 0 |
