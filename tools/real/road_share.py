"""Share of model settlements on or next to an initial road (a cell of the
road grid or one of its 8 neighbours), all and 1,000 or more, against the share
of land that is.

usage: python3 -I tools/real/road_share.py data/real/kent target/real/kent-res [cell_km]
"""
import csv, glob, os, sys

region, runs = sys.argv[1], sys.argv[2]
cell = float(sys.argv[3]) if len(sys.argv) > 3 else 1.0
name = os.path.basename(region.rstrip("/"))


def grid(path):
    rows = [l.split() for l in open(path) if l[:1].isdigit() or l[:1] == "-"]
    return [[float(v) for v in r] for r in rows]


road = grid(os.path.join(region, "roads.asc"))
elev = grid(os.path.join(region, name + ".asc"))
h, w = len(road), len(road[0])
near = [[any(road[j][i] > 0.5 for j in range(max(0, y - 1), min(h, y + 2))
             for i in range(max(0, x - 1), min(w, x + 2)))
         for x in range(w)] for y in range(h)]
land = [(x, y) for y in range(h) for x in range(w) if elev[y][x] > 0]
land_share = sum(near[y][x] for x, y in land) / len(land)
all_n = all_on = big_n = big_on = 0
for f in sorted(glob.glob(os.path.join(runs, "seed*.csv"))):
    for r in csv.DictReader(open(f)):
        x = min(w - 1, int(float(r["x_km"]) / cell))
        y = min(h - 1, int(float(r["y_km"]) / cell))
        on = near[y][x]
        all_n += 1
        all_on += on
        if float(r["pop"]) >= 1000:
            big_n += 1
            big_on += on
print(f"on or next to a road: all {all_on / all_n:.0%} ({all_n}), "
      f"1,000 or more {big_on / max(big_n, 1):.0%} ({big_n}); land {land_share:.0%}")
