"""Convert a region's resources.csv (name,lat,lon,good,basis) to grid km
(name,x_km,y_km,good) for the DEM importer, with the box in region.json.

usage: python3 -I tools/real/resources_km.py data/real/kent
"""
import csv, json, os, sys

region = sys.argv[1]
reg = json.load(open(os.path.join(region, "region.json")))
w, s, e, n = reg["west"], reg["south"], reg["east"], reg["north"]
nx, ny = reg["cells"]
# The source cell size, m, from the DEM header the importer reads.
asc = reg["fetch"].split()[-1]
cell = next(float(l.split()[1]) for l in open(asc) if l.lower().startswith("cellsize")) / 1000
rows = list(csv.DictReader(open(os.path.join(region, "resources.csv"))))
out = os.path.join(region, "resources_km.csv")
with open(out, "w") as f:
    f.write("name,x_km,y_km,good\n")
    for r in rows:
        lat, lon = float(r["lat"]), float(r["lon"])
        if not (w <= lon <= e and s <= lat <= n):
            print(f"outside the box: {r['name']}")
            continue
        x = (lon - w) / (e - w) * nx * cell
        y = (n - lat) / (n - s) * ny * cell
        f.write(f"{r['name']},{x:.1f},{y:.1f},{r['good']}\n")
print(f"wrote {out}")
