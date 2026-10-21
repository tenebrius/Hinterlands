"""Fetch SRTM-derived elevation for a lat/lon box and write a 1 km ESRI ASCII grid.

Source: Mapzen/AWS Terrarium tiles (https://registry.opendata.aws/terrain-tiles/),
which in Britain are SRTM 3" over land and ETOPO1/GEBCO bathymetry at sea.
Tiles are sampled at zoom 10 (~95 m/pixel here), projected to a local
equirectangular grid in km about the box's mid-latitude, and averaged into
`cell_m` cells.

usage: python3 -I fetch_dem.py WEST SOUTH EAST NORTH CELLS_X CELLS_Y out.asc [cache_dir]
"""
import io, math, os, sys, urllib.request

import numpy as np
from PIL import Image

Z = 10
URL = "https://s3.amazonaws.com/elevation-tiles-prod/terrarium/{z}/{x}/{y}.png"


def tile_xy(lon, lat):
    n = 2 ** Z
    x = (lon + 180.0) / 360.0 * n
    y = (1.0 - math.asinh(math.tan(math.radians(lat))) / math.pi) / 2.0 * n
    return x, y


def tile(x, y, cache):
    path = os.path.join(cache, f"{Z}_{x}_{y}.png")
    if not os.path.exists(path):
        with urllib.request.urlopen(URL.format(z=Z, x=x, y=y), timeout=60) as r:
            data = r.read()
        with open(path, "wb") as f:
            f.write(data)
    a = np.asarray(Image.open(path).convert("RGB"), dtype=np.float64)
    return a[:, :, 0] * 256.0 + a[:, :, 1] + a[:, :, 2] / 256.0 - 32768.0


def main():
    west, south, east, north = map(float, sys.argv[1:5])
    nx, ny = int(sys.argv[5]), int(sys.argv[6])
    out = sys.argv[7]
    cache = sys.argv[8] if len(sys.argv) > 8 else os.path.join(os.path.dirname(out), "tiles")
    os.makedirs(cache, exist_ok=True)

    # Mosaic the tiles covering the box.
    x0, y0 = tile_xy(west, north)
    x1, y1 = tile_xy(east, south)
    tx = range(int(x0), int(x1) + 1)
    ty = range(int(y0), int(y1) + 1)
    mosaic = np.vstack([np.hstack([tile(x, y, cache) for x in tx]) for y in ty])

    # Sample 8 x 8 points per output cell and average them.
    sub = 8
    us = (np.arange(nx * sub) + 0.5) / (nx * sub)
    vs = (np.arange(ny * sub) + 0.5) / (ny * sub)
    lon = west + us * (east - west)
    lat = north - vs * (north - south)
    px = np.array([(tile_xy(l, 0)[0] - tx[0]) * 256 for l in lon])
    py = np.array([(tile_xy(0, l)[1] - ty[0]) * 256 for l in lat])
    pxi = np.clip(px.astype(int), 0, mosaic.shape[1] - 1)
    pyi = np.clip(py.astype(int), 0, mosaic.shape[0] - 1)
    fine = mosaic[np.ix_(pyi, pxi)]
    grid = fine.reshape(ny, sub, nx, sub).mean(axis=(1, 3))

    mid = math.radians((north + south) / 2)
    cell_x = (east - west) * 111.32 * math.cos(mid) / nx
    cell_y = (north - south) * 111.2 / ny
    print(f"cells {nx}x{ny}, cell {cell_x:.3f} x {cell_y:.3f} km, "
          f"elev {grid.min():.0f}..{grid.max():.0f} m, land {100 * (grid > 0).mean():.0f}%")
    with open(out, "w") as f:
        f.write(f"ncols {nx}\nnrows {ny}\nxllcorner 0\nyllcorner 0\n")
        f.write(f"cellsize {1000.0 * (cell_x + cell_y) / 2:.1f}\nnodata_value -9999\n")
        for row in grid:
            f.write(" ".join(f"{v:.1f}" for v in row) + "\n")


if __name__ == "__main__":
    main()
