"""Fetch BGS 1:625k geology for a lat/lon box and write a class grid (D116).

Source: BGS Geology 625k (DiGMapGB-625), ArcGIS REST service
https://map.bgs.ac.uk/arcgis/rest/services/SDDS/Geology_625k/MapServer,
layers 3 (bedrock) and 2 (superficial deposits). Open Government Licence:
"Contains NERC materials (c) NERC 2026."

Each cell centre of the grid (the same lon/lat box and cells as
`fetch_dem.py`) is classed by the polygons containing it:
  1 chalk: bedrock in the Chalk Group with no superficial deposit over it, or
    only clay-with-flints (the downland cap);
  2 marsh: marine, tidal-flat, estuarine or alluvial deposits, or peat;
  0 other.

usage: python3 -I fetch_geology.py WEST SOUTH EAST NORTH CELLS_X CELLS_Y out.asc [cache_dir]
"""
import json, os, sys, urllib.parse, urllib.request

URL = "https://map.bgs.ac.uk/arcgis/rest/services/SDDS/Geology_625k/MapServer/{layer}/query"


def fetch(layer, fields, box, cache):
    path = os.path.join(cache, f"bgs625k_layer{layer}.json")
    if os.path.exists(path):
        return json.load(open(path))
    q = urllib.parse.urlencode({
        "where": "1=1",
        "geometry": ",".join(str(v) for v in box),
        "geometryType": "esriGeometryEnvelope",
        "inSR": 4326,
        "spatialRel": "esriSpatialRelIntersects",
        "outFields": fields,
        "outSR": 4326,
        "returnGeometry": "true",
        "f": "json",
    })
    with urllib.request.urlopen(URL.format(layer=layer) + "?" + q, timeout=120) as r:
        data = json.load(r)
    os.makedirs(cache, exist_ok=True)
    json.dump(data, open(path, "w"))
    return data


def polys(data):
    out = []
    for f in data.get("features", []):
        rings = f["geometry"]["rings"]
        xs = [p[0] for r in rings for p in r]
        ys = [p[1] for r in rings for p in r]
        out.append(((min(xs), min(ys), max(xs), max(ys)), rings, f["attributes"]))
    return out


def inside(x, y, rings):
    # Even-odd rule over all rings (holes included).
    c = False
    for r in rings:
        n = len(r)
        j = n - 1
        for i in range(n):
            xi, yi = r[i]
            xj, yj = r[j]
            if (yi > y) != (yj > y) and x < (xj - xi) * (y - yi) / (yj - yi) + xi:
                c = not c
            j = i
    return c


def at(x, y, ps):
    for (x0, y0, x1, y1), rings, a in ps:
        if x0 <= x <= x1 and y0 <= y <= y1 and inside(x, y, rings):
            return a
    return None


MARSH = ("TIDAL", "MARINE", "ESTUARINE", "ALLUVIUM", "PEAT", "STORM BEACH", "SALTMARSH")


def main():
    west, south, east, north = map(float, sys.argv[1:5])
    nx, ny = int(sys.argv[5]), int(sys.argv[6])
    out = sys.argv[7]
    cache = sys.argv[8] if len(sys.argv) > 8 else os.path.join(os.path.dirname(out), "geology")
    box = (west, south, east, north)
    bed = polys(fetch(3, "LEX_D,RCS_D,GP_EQ_D", box, cache))
    sup = polys(fetch(2, "LEX_D,ROCK_D,RCS_D", box, cache))
    grid = []
    counts = [0, 0, 0]
    for yy in range(ny):
        row = []
        lat = north - (yy + 0.5) / ny * (north - south)
        for xx in range(nx):
            lon = west + (xx + 0.5) / nx * (east - west)
            s = at(lon, lat, sup)
            b = at(lon, lat, bed)
            sd = " ".join(str(s.get(k) or "") for k in ("LEX_D", "ROCK_D", "RCS_D")).upper() if s else ""
            bd = " ".join(str(b.get(k) or "") for k in ("LEX_D", "RCS_D", "GP_EQ_D")).upper() if b else ""
            if s and any(m in sd for m in MARSH):
                k = 2
            elif "CHALK" in bd and (not s or "CLAY-WITH-FLINTS" in sd):
                k = 1
            else:
                k = 0
            counts[k] += 1
            row.append(k)
        grid.append(row)
    with open(out, "w") as f:
        f.write(f"ncols {nx}\nnrows {ny}\nxllcorner 0\nyllcorner 0\ncellsize 1000.0\nnodata_value -9999\n")
        for row in grid:
            f.write(" ".join(str(v) for v in row) + "\n")
    print(f"cells: other {counts[0]}, chalk {counts[1]}, marsh {counts[2]}")


if __name__ == "__main__":
    main()
