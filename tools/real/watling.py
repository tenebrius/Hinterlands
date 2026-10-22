"""Watling Street mask for Kent: the Roman road cells (roads.asc) within 2 km
of the line London - Rochester - Canterbury - Dover. Writes watling.asc (0/1)
in the region folder.

usage: python3 -I tools/real/watling.py data/real/kent
"""
import json, math, os, sys

region = sys.argv[1]
reg = json.load(open(os.path.join(region, "region.json")))
w, s, e, n = reg["west"], reg["south"], reg["east"], reg["north"]
nx, ny = reg["cells"]
lines = open(os.path.join(region, "roads.asc")).read().splitlines()
header = [l for l in lines if l[:1].isalpha()]
rows = [[float(v) for v in l.split()] for l in lines if l[:1].isdigit() or l[:1] == "-"]
towns = [(51.512, -0.090), (51.388, 0.506), (51.280, 1.080), (51.127, 1.313)]
pts = [((lo - w) / (e - w) * nx, (n - la) / (n - s) * ny) for la, lo in towns]


def dist(px, py):
    best = 1e9
    for (ax, ay), (bx, by) in zip(pts, pts[1:]):
        dx, dy = bx - ax, by - ay
        t = max(0.0, min(1.0, ((px - ax) * dx + (py - ay) * dy) / (dx * dx + dy * dy)))
        best = min(best, math.hypot(px - ax - t * dx, py - ay - t * dy))
    return best


mask = [[1 if rows[y][x] > 0.5 and dist(x + 0.5, y + 0.5) <= 2.0 else 0 for x in range(nx)] for y in range(ny)]
with open(os.path.join(region, "watling.asc"), "w") as f:
    f.write("\n".join(header) + "\n")
    for r in mask:
        f.write(" ".join(str(v) for v in r) + "\n")
print(f"Watling Street: {sum(map(sum, mask))} road cells")
