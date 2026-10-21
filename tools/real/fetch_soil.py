"""Fetch SoilGrids 2.0 topsoil properties for a lat/lon box and write a soil
quality grid for medieval arable farming as an ESRI ASCII grid.

Source: ISRIC SoilGrids 2.0 (250 m, CC-BY 4.0), WCS 1.0 with server-side
reprojection to EPSG:4326. Layers at 5-15 cm (the plough layer): clay and sand
(g/kg) and coarse fragments (cm3/dm3).

Quality, 0.1-1: loams are best; heavy clay (over 30%, hard to plough and
waterlogged), sand (over 55%, hungry heath soils) and stony ground (coarse
fragments over 10%: thin chalk and flint soils) are penalised:
    q = 1 - 0.5*heavy - 0.5*light - 0.4*stony
SoilGrids leaves built-up areas blank (London, every modern town). Those holes
are filled from the surrounding cells before scoring, so a town site gets the
soil of the land around it, never a value tied to the town itself. The importer
rescales q to the map's mean yield, so soil redistributes yield without
changing its total; it ignores q on sea cells.

usage: python3 -I fetch_soil.py WEST SOUTH EAST NORTH CELLS_X CELLS_Y out.asc [cache_dir]
"""
import io, os, sys, urllib.parse, urllib.request

import numpy as np
from PIL import Image

URL = "https://maps.isric.org/mapserv"
SUB = 4  # source pixels per output cell along each axis


def layer(prop, bbox, w, h, cache):
    path = os.path.join(cache, f"{prop}_{w}x{h}.tif")
    if not os.path.exists(path):
        q = urllib.parse.urlencode({
            "map": f"/map/{prop}.map", "SERVICE": "WCS", "VERSION": "1.0.0",
            "REQUEST": "GetCoverage", "COVERAGE": f"{prop}_5-15cm_mean",
            "FORMAT": "GEOTIFF_INT16", "CRS": "EPSG:4326", "RESPONSE_CRS": "EPSG:4326",
            "BBOX": ",".join(str(v) for v in bbox), "WIDTH": w, "HEIGHT": h,
        })
        with urllib.request.urlopen(f"{URL}?{q}", timeout=300) as r:
            data = r.read()
        with open(path, "wb") as f:
            f.write(data)
    a = np.asarray(Image.open(path)).astype(np.float64)
    a[a < 0] = np.nan
    return a


def block_mean(a, nx, ny):
    b = a.reshape(ny, a.shape[0] // ny, nx, a.shape[1] // nx)
    with np.errstate(invalid="ignore"):
        return np.nanmean(b, axis=(1, 3))


def fill_holes(a):
    # Each pass gives every blank cell the mean of its filled neighbours.
    a = a.copy()
    for _ in range(200):
        nan = np.isnan(a)
        if not nan.any():
            break
        p = np.pad(a, 1, constant_values=np.nan)
        stack = np.stack([p[1 + dy : 1 + dy + a.shape[0], 1 + dx : 1 + dx + a.shape[1]]
                          for dy in (-1, 0, 1) for dx in (-1, 0, 1) if (dy, dx) != (0, 0)])
        with np.errstate(invalid="ignore"):
            m = np.nanmean(stack, axis=0)
        a[nan] = m[nan]
    return a


def main():
    west, south, east, north = map(float, sys.argv[1:5])
    nx, ny = int(sys.argv[5]), int(sys.argv[6])
    out = sys.argv[7]
    cache = sys.argv[8] if len(sys.argv) > 8 else os.path.join(os.path.dirname(out), "soil_cache")
    os.makedirs(cache, exist_ok=True)
    bbox = (west, south, east, north)
    clay = fill_holes(block_mean(layer("clay", bbox, nx * SUB, ny * SUB, cache), nx, ny)) / 10.0
    sand = fill_holes(block_mean(layer("sand", bbox, nx * SUB, ny * SUB, cache), nx, ny)) / 10.0
    cfvo = fill_holes(block_mean(layer("cfvo", bbox, nx * SUB, ny * SUB, cache), nx, ny)) / 10.0
    heavy = np.clip((clay - 30.0) / 20.0, 0, 1)
    light = np.clip((sand - 55.0) / 30.0, 0, 1)
    stony = np.clip((cfvo - 10.0) / 25.0, 0, 1)
    q = np.clip(1.0 - 0.5 * heavy - 0.5 * light - 0.4 * stony, 0.1, 1.0)
    q[np.isnan(clay) | np.isnan(sand) | np.isnan(cfvo)] = -9999
    land = q > 0
    print(f"clay {np.nanmedian(clay):.0f}%  sand {np.nanmedian(sand):.0f}%  stones {np.nanmedian(cfvo):.0f}%  "
          f"quality mean {q[land].mean():.2f} (p10 {np.percentile(q[land], 10):.2f}, p90 {np.percentile(q[land], 90):.2f})")
    with open(out, "w") as f:
        f.write(f"ncols {nx}\nnrows {ny}\nxllcorner 0\nyllcorner 0\ncellsize 1000\nnodata_value -9999\n")
        for row in q:
            f.write(" ".join(f"{v:.3f}" for v in row) + "\n")


if __name__ == "__main__":
    main()
