# Real-terrain scores: kent

## All 70 real towns against the model's top 70

20 seeds, final year; random baselines drop the same 70 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.41 | 0.37 | 0.30 | 20% |
| Real towns with a model place within 10 km | 0.86 | 0.82 | 0.65 | 15% |
| Model places with a real town within 5 km | 0.45 | 0.41 | 0.38 | 10% |
| Model places with a real town within 10 km | 0.88 | 0.90 | 0.85 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.54 | 0.29 | 0.38 | 45% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.09 | -0.03 | 0.20 | 15% |

## The 27 real towns of 1,000 or more against the model's top 27

20 seeds, final year; random baselines drop the same 27 places on random cells.

| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |
| --- | ---: | ---: | ---: | ---: |
| Real towns with a model place within 5 km | 0.23 | 0.15 | 0.17 | 10% |
| Real towns with a model place within 10 km | 0.53 | 0.45 | 0.46 | 10% |
| Model places with a real town within 5 km | 0.21 | 0.15 | 0.18 | 0% |
| Model places with a real town within 10 km | 0.51 | 0.48 | 0.52 | 0% |
| Real top 10 with a model top-15 place within 10 km | 0.54 | 0.29 | 0.39 | 45% |
| Rank agreement (Spearman, real size vs model size within 5 km) | 0.30 | 0.01 | 0.14 | 35% |

## Site prediction over all seeds

Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).

| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |
| --- | ---: | ---: | ---: | ---: |
| All (70) | 0.58 | 0.49 | 0.24 | 0.16 |
| 1,000 or more (27) | 0.62 | 0.53 | 0.26 | 0.16 |
| Under 1,000 (43) | 0.55 | 0.46 | 0.22 | 0.16 |

## Largest city: London (City)

Seeds whose largest model place is within 10 km of London (City): 0 of 20; a top-3 place within 10 km: 5; any top-70 place within 10 km: 20. Median distance from the model's largest place to London (City): 41 km. Largest place's share of the region's people (median): 0.30.

Nearest real town to the model's largest place, by seeds: Gravesend 10, Faversham 4, Rayleigh 3, Milton Regis 2, Wingham 1.

## By site

Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).

| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |
| --- | ---: | ---: | ---: | ---: | ---: |
| river | 18 | 26% | 30.6 | 44% | 47% |
| coast | 6 | 9% | 9.0 | 13% | 44% |
| inland | 46 | 66% | 30.4 | 44% | 39% |

Inland places over 500 people, nearest other place over 500 (median): real 10.6 km (29 towns); model 6.2 km (median over seeds; 32 such places per seed).

Mean nearest-neighbour spacing of the 70 places: real 8.6 km, model 7.5 km, random land 6.9 km.

## Per seed

| Seed | recall5 | recall10 | precision5 | precision10 | big10 | spearman | Largest |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 0.46 | 0.91 | 0.46 | 0.87 | 0.60 | 0.11 | Dunholt 52456 |
| 2 | 0.41 | 0.80 | 0.44 | 0.89 | 0.50 | 0.04 | Croeford 50366 |
| 3 | 0.40 | 0.86 | 0.41 | 0.91 | 0.50 | -0.02 | Fairesholt 55459 |
| 4 | 0.43 | 0.84 | 0.51 | 0.89 | 0.50 | 0.10 | Faleshaven 54227 |
| 5 | 0.44 | 0.89 | 0.49 | 0.89 | 0.50 | 0.26 | Penwick 46548 |
| 6 | 0.34 | 0.83 | 0.37 | 0.89 | 0.70 | 0.07 | Torport 59460 |
| 7 | 0.46 | 0.89 | 0.47 | 0.83 | 0.60 | 0.16 | Halhaven 43455 |
| 8 | 0.47 | 0.87 | 0.51 | 0.87 | 0.60 | 0.11 | Noreington 45849 |
| 9 | 0.43 | 0.90 | 0.49 | 0.89 | 0.50 | 0.00 | Roasea 41543 |
| 10 | 0.41 | 0.86 | 0.49 | 0.90 | 0.70 | 0.13 | Oakhurst 53772 |
| 11 | 0.41 | 0.81 | 0.40 | 0.91 | 0.30 | 0.19 | Craneley 52494 |
| 12 | 0.39 | 0.87 | 0.39 | 0.91 | 0.50 | 0.03 | Falden 47189 |
| 13 | 0.36 | 0.81 | 0.46 | 0.91 | 0.60 | 0.24 | Roingley 61687 |
| 14 | 0.39 | 0.87 | 0.41 | 0.84 | 0.50 | 0.05 | Barford 25690 |
| 15 | 0.41 | 0.86 | 0.43 | 0.84 | 0.60 | 0.09 | Dewesfield 39708 |
| 16 | 0.41 | 0.90 | 0.43 | 0.90 | 0.50 | 0.08 | Verholt 55883 |
| 17 | 0.46 | 0.87 | 0.49 | 0.89 | 0.40 | 0.16 | Iselstead 47725 |
| 18 | 0.44 | 0.86 | 0.44 | 0.90 | 0.50 | 0.11 | Holstead 42742 |
| 19 | 0.36 | 0.83 | 0.39 | 0.86 | 0.60 | -0.10 | Fenesham 45030 |
| 20 | 0.37 | 0.86 | 0.43 | 0.89 | 0.60 | 0.00 | Verby 47563 |

## Real towns by how often the model finds them (place within 5 km, share of seeds)

| Town | Pop. c.1300 | Tier | Found |
| --- | ---: | --- | ---: |
| London (City) | 80000 | city | 40% |
| Canterbury | 10000 | city | 90% |
| Colchester | 4000 | large_town | 5% |
| Sandwich | 4000 | large_town | 90% |
| Winchelsea | 3500 | large_town | 100% |
| Dover | 3000 | large_town | 25% |
| New Romney | 2500 | town | 90% |
| Rye | 2500 | town | 100% |
| Lewes | 2500 | town | 0% |
| Maidstone | 2000 | town | 35% |
| Hythe | 2000 | town | 65% |
| Hastings | 2000 | town | 20% |
| Maldon | 2000 | town | 20% |
| Faversham | 2000 | town | 60% |
| Rochester | 1900 | town | 30% |
| Lydd | 1500 | town | 10% |
| Chelmsford | 1500 | town | 85% |
| New Shoreham | 1500 | town | 10% |
| Battle | 1200 | town | 10% |
| Tonbridge | 1200 | town | 50% |
| Dartford | 1200 | town | 80% |
| Hertford | 1200 | town | 0% |
| Ware | 1200 | town | 5% |
| Croydon | 1000 | town | 50% |
| Reigate | 1000 | town | 15% |
| Folkestone | 1000 | town | 70% |
| Bishop's Stortford | 1000 | town | 5% |
| Seaford | 800 | small_town | 0% |
| East Grinstead | 800 | small_town | 0% |
| Milton Regis | 800 | small_town | 50% |
| Barking | 800 | small_town | 20% |
| Waltham Abbey | 800 | small_town | 100% |
| Pevensey | 600 | small_town | 100% |
| Fordwich | 600 | small_town | 90% |
| Tenterden | 600 | small_town | 10% |
| Ashford | 600 | small_town | 70% |
| Sevenoaks | 600 | small_town | 50% |
| West Malling | 600 | small_town | 55% |
| Cranbrook | 600 | small_town | 15% |
| Gravesend | 600 | small_town | 35% |
| Romford | 600 | small_town | 5% |
| Coggeshall | 600 | small_town | 100% |
| Braintree | 600 | small_town | 5% |
| Great Dunmow | 600 | small_town | 90% |
| Wye | 500 | small_town | 35% |
| Witham | 500 | small_town | 0% |
| Brentwood | 500 | small_town | 5% |
| Chipping Barnet | 500 | small_town | 0% |
| Bletchingley | 500 | small_town | 0% |
| Westerham | 400 | small_town | 0% |
| Charing | 400 | small_town | 0% |
| Wrotham | 400 | small_town | 0% |
| Lenham | 400 | small_town | 0% |
| Elham | 400 | small_town | 5% |
| Wingham | 400 | small_town | 25% |
| Bromley | 400 | small_town | 65% |
| Robertsbridge | 400 | small_town | 80% |
| Hailsham | 400 | small_town | 25% |
| Mayfield | 400 | small_town | 25% |
| Cuckfield | 400 | small_town | 100% |
| Crawley | 400 | small_town | 50% |
| Epping | 400 | small_town | 65% |
| Billericay | 400 | small_town | 25% |
| Rayleigh | 400 | small_town | 40% |
| Rochford | 400 | small_town | 35% |
| Harlow | 400 | small_town | 80% |
| Chipping Ongar | 400 | small_town | 50% |
| Hoddesdon | 400 | small_town | 65% |
| Sawbridgeworth | 400 | small_town | 90% |
| Hatfield | 400 | small_town | 70% |

## Persistent model places with no real town within 10 km

Cells where at least half the seeds put a top-K place within 3 km (local maxima).

| x km | y km | Share of seeds | Nearest real town | km | Elevation m |
| ---: | ---: | ---: | --- | ---: | ---: |
| 3 | 47 | 95% | London (City) | 11 | 7 |
| 89 | 11 | 90% | Colchester | 11 | 1 |
| 104 | 12 | 85% | Colchester | 24 | 0 |
| 29 | 89 | 80% | East Grinstead | 11 | 50 |
| 22 | 1 | 80% | Bishop's Stortford | 10 | 98 |
| 61 | 80 | 70% | Lenham | 10 | 24 |
| 27 | 104 | 55% | Mayfield | 12 | 32 |
