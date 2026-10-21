"""Rasterise Roman roads from Itiner-e onto a region's grid as an ESRI ASCII 0/1 grid.

Source: Itiner-e static version 2024 (de Soto, Pažout, Brughmans & Bjerregaard
Vahlstrup 2025, Scientific Data; Zenodo 10.5281/zenodo.17122148; CC BY 4.0),
in World Mercator (EPSG:3395). Segments marked Certain or Conjectured are kept
and Hypothetical ones dropped. Each line is sampled every 0.2 km, and every cell
it passes through is marked.

usage: python3 -I fetch_roads.py WEST SOUTH EAST NORTH CELLS_X CELLS_Y out.asc [cache_dir]
"""
import json, math, os, sys, urllib.request

import numpy as np

URL = "https://zenodo.org/api/records/17122148/files/itinere_roads.geojson/content"
A = 6378137.0
E = 0.0818191908426  # WGS84 eccentricity
KEEP = {"Certain", "Conjectured"}


def lonlat(x, y):
    # Inverse ellipsoidal Mercator (EPSG:3395).
    lon = math.degrees(x / A)
    t = math.exp(-y / A)
    phi = math.pi / 2 - 2 * math.atan(t)
    for _ in range(8):
        es = E * math.sin(phi)
        phi = math.pi / 2 - 2 * math.atan(t * ((1 - es) / (1 + es)) ** (E / 2))
    return lon, math.degrees(phi)


def main():
    west, south, east, north = map(float, sys.argv[1:5])
    nx, ny = int(sys.argv[5]), int(sys.argv[6])
    out = sys.argv[7]
    cache = sys.argv[8] if len(sys.argv) > 8 else os.path.join(os.path.dirname(out), "roads_cache")
    os.makedirs(cache, exist_ok=True)
    path = os.path.join(cache, "itinere_roads.geojson")
    if not os.path.exists(path):
        urllib.request.urlretrieve(URL, path)
    data = json.load(open(path))
    grid = np.zeros((ny, nx), dtype=np.uint8)
    kept = {}
    for f in data["features"]:
        p = f["properties"]
        if p.get("Segment_s") not in KEEP:
            continue
        g = f["geometry"]
        lines = g["coordinates"] if g["type"] == "MultiLineString" else [g["coordinates"]]
        hit = False
        for line in lines:
            pts = [lonlat(c[0], c[1]) for c in line]
            cells = [((lo - west) / (east - west) * nx, (north - la) / (north - south) * ny) for lo, la in pts]
            for (x0, y0), (x1, y1) in zip(cells, cells[1:]):
                steps = max(1, int(math.hypot(x1 - x0, y1 - y0) / 0.2))
                for k in range(steps + 1):
                    x = x0 + (x1 - x0) * k / steps
                    y = y0 + (y1 - y0) * k / steps
                    if 0 <= x < nx and 0 <= y < ny:
                        grid[int(y), int(x)] = 1
                        hit = True
        if hit:
            key = (p.get("Name") or "unnamed", p.get("Type"), p.get("Segment_s"))
            kept[key] = kept.get(key, 0) + 1
    for (name, typ, cert), n in sorted(kept.items()):
        print(f"  {name} ({typ}, {cert})")
    print(f"{len(kept)} named segments; {int(grid.sum())} road cells ({100 * grid.mean():.1f}% of the grid)")
    with open(out, "w") as fo:
        fo.write(f"ncols {nx}\nnrows {ny}\nxllcorner 0\nyllcorner 0\ncellsize 1000\nnodata_value -9999\n")
        for row in grid:
            fo.write(" ".join(str(int(v)) for v in row) + "\n")


if __name__ == "__main__":
    main()
