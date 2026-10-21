# Real-terrain scores: kent

## All 70 real towns against the model's top 70

20 seeds, final year; random baselines drop the same 70 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.36 | 0.37 | 0.30 | 0% |
| Real towns with a model place within 10 km | 0.79 | 0.82 | 0.65 | 5% |
| Model places with a real town within 5 km | 0.42 | 0.41 | 0.38 | 5% |
| Model places with a real town within 10 km | 0.87 | 0.90 | 0.85 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.40 | 0.29 | 0.38 | 10% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.04 | -0.03 | 0.20 | 20% |

## The 27 real towns of 1,000 or more against the model's top 27

20 seeds, final year; random baselines drop the same 27 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.21 | 0.15 | 0.17 | 5% |
| Real towns with a model place within 10 km | 0.57 | 0.45 | 0.46 | 30% |
| Model places with a real town within 5 km | 0.20 | 0.15 | 0.18 | 5% |
| Model places with a real town within 10 km | 0.53 | 0.48 | 0.52 | 5% |
| Real top 10 with a model top-15 place within 10 km | 0.40 | 0.29 | 0.39 | 10% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.02 | 0.01 | 0.14 | 0% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (70) | 0.55 | 0.46 | 0.18 | 0.15 |
| 1,000 or more (27) | 0.62 | 0.52 | 0.21 | 0.15 |
| Under 1,000 (43) | 0.51 | 0.42 | 0.17 | 0.15 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 6; any top-70 place within 10 km: 18. Median distance from the model's largest place to London (City): 76 km.

Nearest real town to the model's largest place, by seeds: Rye 5, Faversham 4, Maldon 2, Rochford 2, Fordwich 2, East Grinstead 1, Hythe 1, Gravesend 1, Dartford 1, Milton Regis 1.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 18 | 26% | 32.4 | 46% | 59% |
| coast | 6 | 9% | 8.9 | 13% | 18% |
| inland | 46 | 66% | 28.7 | 41% | 29% |

Mean nearest-neighbour spacing of the 70 places: real 8.6 km, model 7.0 km, random land 6.9 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.34 | 0.74 | 0.40 | 0.86 | 0.40 | 0.01 | Penesford 7627 |
| 2 | 0.37 | 0.76 | 0.46 | 0.90 | 0.30 | -0.16 | Ulowmouth 9793 |
| 3 | 0.34 | 0.76 | 0.47 | 0.84 | 0.10 | 0.06 | Thetenport 9234 |
| 4 | 0.39 | 0.81 | 0.44 | 0.87 | 0.50 | 0.04 | Rayport 8365 |
| 5 | 0.40 | 0.80 | 0.50 | 0.86 | 0.40 | 0.00 | Falanhaven 7244 |
| 6 | 0.30 | 0.84 | 0.36 | 0.91 | 0.40 | 0.08 | Glenesea 13356 |
| 7 | 0.34 | 0.76 | 0.43 | 0.86 | 0.50 | 0.21 | Ickingford 14038 |
| 8 | 0.36 | 0.80 | 0.40 | 0.87 | 0.50 | -0.04 | Harteswick 7000 |
| 9 | 0.31 | 0.73 | 0.40 | 0.83 | 0.40 | 0.07 | Chedport 14772 |
| 10 | 0.33 | 0.77 | 0.36 | 0.91 | 0.30 | 0.19 | Woolenmouth 11635 |
| 11 | 0.31 | 0.84 | 0.37 | 0.86 | 0.40 | 0.10 | Thornemouth 11533 |
| 12 | 0.40 | 0.84 | 0.49 | 0.87 | 0.60 | -0.10 | Falanhaven 7748 |
| 13 | 0.40 | 0.84 | 0.39 | 0.86 | 0.50 | -0.14 | Sutford 7291 |
| 14 | 0.33 | 0.73 | 0.46 | 0.83 | 0.30 | 0.20 | Ickport 5661 |
| 15 | 0.33 | 0.90 | 0.41 | 0.90 | 0.50 | 0.23 | Fenenwick 8682 |
| 16 | 0.33 | 0.89 | 0.37 | 0.87 | 0.40 | -0.05 | Veresley 8158 |
| 17 | 0.37 | 0.83 | 0.37 | 0.89 | 0.40 | -0.09 | Belenford 7093 |
| 18 | 0.34 | 0.70 | 0.40 | 0.86 | 0.30 | 0.05 | Yaringhaven 6385 |
| 19 | 0.41 | 0.79 | 0.53 | 0.89 | 0.60 | 0.15 | Hartford 9423 |
| 20 | 0.39 | 0.74 | 0.43 | 0.86 | 0.30 | -0.02 | Elmey 5745 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 40% |
| Canterbury | 10000 | city | 45% |
| Colchester | 4000 | large_town | 5% |
| Sandwich | 4000 | large_town | 50% |
| Winchelsea | 3500 | large_town | 40% |
| Dover | 3000 | large_town | 25% |
| New Romney | 2500 | town | 80% |
| Rye | 2500 | town | 40% |
| Lewes | 2500 | town | 30% |
| Maidstone | 2000 | town | 60% |
| Hythe | 2000 | town | 25% |
| Hastings | 2000 | town | 15% |
| Maldon | 2000 | town | 65% |
| Faversham | 2000 | town | 30% |
| Rochester | 1900 | town | 60% |
| Lydd | 1500 | town | 30% |
| Chelmsford | 1500 | town | 75% |
| New Shoreham | 1500 | town | 15% |
| Battle | 1200 | town | 0% |
| Tonbridge | 1200 | town | 45% |
| Dartford | 1200 | town | 55% |
| Hertford | 1200 | town | 35% |
| Ware | 1200 | town | 55% |
| Croydon | 1000 | town | 65% |
| Reigate | 1000 | town | 35% |
| Folkestone | 1000 | town | 10% |
| Bishop's Stortford | 1000 | town | 35% |
| Seaford | 800 | small_town | 0% |
| East Grinstead | 800 | small_town | 0% |
| Milton Regis | 800 | small_town | 50% |
| Barking | 800 | small_town | 55% |
| Waltham Abbey | 800 | small_town | 40% |
| Pevensey | 600 | small_town | 60% |
| Fordwich | 600 | small_town | 55% |
| Tenterden | 600 | small_town | 0% |
| Ashford | 600 | small_town | 55% |
| Sevenoaks | 600 | small_town | 30% |
| West Malling | 600 | small_town | 30% |
| Cranbrook | 600 | small_town | 0% |
| Gravesend | 600 | small_town | 50% |
| Romford | 600 | small_town | 0% |
| Coggeshall | 600 | small_town | 60% |
| Braintree | 600 | small_town | 0% |
| Great Dunmow | 600 | small_town | 55% |
| Wye | 500 | small_town | 15% |
| Witham | 500 | small_town | 70% |
| Brentwood | 500 | small_town | 0% |
| Chipping Barnet | 500 | small_town | 0% |
| Bletchingley | 500 | small_town | 0% |
| Westerham | 400 | small_town | 0% |
| Charing | 400 | small_town | 5% |
| Wrotham | 400 | small_town | 0% |
| Lenham | 400 | small_town | 0% |
| Elham | 400 | small_town | 0% |
| Wingham | 400 | small_town | 50% |
| Bromley | 400 | small_town | 65% |
| Robertsbridge | 400 | small_town | 80% |
| Hailsham | 400 | small_town | 60% |
| Mayfield | 400 | small_town | 15% |
| Cuckfield | 400 | small_town | 60% |
| Crawley | 400 | small_town | 60% |
| Epping | 400 | small_town | 10% |
| Billericay | 400 | small_town | 50% |
| Rayleigh | 400 | small_town | 45% |
| Rochford | 400 | small_town | 15% |
| Harlow | 400 | small_town | 50% |
| Chipping Ongar | 400 | small_town | 35% |
| Hoddesdon | 400 | small_town | 70% |
| Sawbridgeworth | 400 | small_town | 55% |
| Hatfield | 400 | small_town | 70% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 105 | 10 | 60% | Colchester | 24 | 0 |
| 68 | 52 | 60% | Milton Regis | 11 | 4 |
| 63 | 82 | 60% | Lenham | 10 | 23 |
| 4 | 48 | 60% | London (City) | 10 | 5 |
| 1 | 58 | 60% | Croydon | 12 | 24 |
| 1 | 41 | 60% | London (City) | 12 | 33 |
| 108 | 3 | 55% | Colchester | 26 | 0 |
| 32 | 88 | 50% | Tonbridge | 12 | 58 |
