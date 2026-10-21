"""Map of shadow grain prices for one year (v6 S0).

usage: python3 -I tools/market/price_map.py PRICES.txt YEAR BACKGROUND.png OUT.png [LABEL]
PRICES.txt is `hinterlands prices` output (places founded after the market step that year, with no price yet, are left out); BACKGROUND.png a bare terrain
render (`hinterlands render --bare 1`). Each settlement is a dot sized by
population and coloured by its grain price on a fixed scale, cheap (blue)
through dear (red), 0.4 to 2.0 grain units.
"""
import math, sys
from PIL import Image, ImageDraw

path, year, bg, out = sys.argv[1], int(sys.argv[2]), sys.argv[3], sys.argv[4]
label = sys.argv[5] if len(sys.argv) > 5 else f"year {year}"
img = Image.open(bg).convert("RGB")
img = Image.blend(img, Image.new("RGB", img.size, (255, 255, 255)), 0.45)
d = ImageDraw.Draw(img)
scale = img.size[0] / 128.0
lo, hi = math.log(0.4), math.log(2.0)


def colour(p):
    t = min(1.0, max(0.0, (math.log(p) - lo) / (hi - lo)))
    stops = [(40, 80, 200), (240, 240, 200), (200, 30, 30)]
    if t < 0.5:
        a, b, u = stops[0], stops[1], t / 0.5
    else:
        a, b, u = stops[1], stops[2], (t - 0.5) / 0.5
    return tuple(int(a[k] + (b[k] - a[k]) * u) for k in range(3))


prices = []
for line in open(path):
    f = line.split()
    if f[0] == "price" and int(f[2]) == year and float(f[6]) > 0:
        x, y, p, pop = int(f[4]), int(f[5]), float(f[6]), float(f[12])
        prices.append(p)
        r = max(1.5, math.sqrt(pop) / 12) * scale / 4
        cx, cy = (x + 0.5) * scale, (y + 0.5) * scale
        d.ellipse([cx - r, cy - r, cx + r, cy + r], fill=colour(p), outline=(30, 30, 30))
prices.sort()
med = prices[len(prices) // 2] if prices else float("nan")
for k in range(100):
    p = math.exp(lo + (hi - lo) * k / 99)
    d.rectangle([10 + k * 2, img.size[1] - 22, 12 + k * 2, img.size[1] - 10], fill=colour(p))
d.text((10, img.size[1] - 38), f"{label}: grain 0.4 to 2.0, median {med:.2f}", fill=(0, 0, 0))
img.save(out)
print(f"wrote {out} ({len(prices)} places, median {med:.3f})")
