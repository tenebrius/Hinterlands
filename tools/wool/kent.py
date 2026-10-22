"""L1 (D107): Kent's pasture share by region, from `hinterlands landuse` dumps.

usage: python3 -I tools/wool/kent.py FILE [FILE...]
Regions (approximate, on the 128 x 128 km grid for lon -0.29..1.55,
lat 50.76..51.911, row 0 north):
  Romney Marsh: below 10 m within lat 50.93-51.05, lon 0.75-1.02;
  North Downs: 90 m and above, north of lat 51.18 or east of lon 1.0;
  Weald: other land south of lat 51.2 and west of lon 0.9;
  rest: everything else.
Prints, per region and file, the share of land that is pasture and the
pasture share of used land (arable + pasture), then the medians.
"""
import statistics as st, sys

W, S, E, N, CELLS = -0.29, 50.76, 1.55, 51.911, 128


def region(x, y, elev):
    lon = W + (x + 0.5) / CELLS * (E - W)
    lat = N - (y + 0.5) / CELLS * (N - S)
    if 50.93 <= lat <= 51.05 and 0.75 <= lon <= 1.02 and elev < 10:
        return "Romney Marsh"
    if elev >= 90 and (lat >= 51.18 or lon >= 1.0):
        return "North Downs"
    if lat < 51.2 and lon < 0.9:
        return "Weald"
    return "rest"


names = ["North Downs", "Romney Marsh", "Weald", "rest"]
res = {n: [] for n in names}
for path in sys.argv[1:]:
    cnt = {n: [0, 0, 0] for n in names}
    for line in open(path):
        f = line.split()
        if not f or f[0] == "x":
            continue
        x, y, elev, use = int(f[0]), int(f[1]), float(f[2]), int(f[4])
        cnt[region(x, y, elev)][use] += 1
    for n in names:
        free, arable, past = cnt[n]
        land = free + arable + past
        res[n].append((past / max(land, 1), past / max(arable + past, 1), land))
for n in names:
    v = res[n]
    print(f"{n:<13} cells {v[0][2]:>5}  pasture/land {st.median(x[0] for x in v):.3f}  pasture/used {st.median(x[1] for x in v):.3f}")
