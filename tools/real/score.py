"""Score model runs on a real region against the historical towns.

usage: python3 -I tools/real/score.py data/real/kent target/real/kent docs/real/kent [border_km]

With `border_km`, real towns, model places and random cells within that distance
of the map edge are left out (a check on edge effects: catchments cut off by the
box).

Reads `<region>/region.json` (the lat/lon box and grid size), `<region>/<name>.asc`
(for the land mask and the base map), `<region>/towns.csv` (name,lat,lon,
population,tier,basis) and `<runs>/seed*.csv` (from `hinterlands run --towns`).
Writes `<out>/scores.md`, `<out>/truth.csv` (towns in grid km) and figures.

The model's top K places are compared with the K real towns, K = the number of
real towns, so the test does not depend on the model's absolute population
scale. Every number is set against chance: the same K places (same sizes)
dropped on random land cells, and on random lowland cells (below 50 m, where
towns usually stand).
"""
import csv, glob, json, os, re, sys

import numpy as np
from PIL import Image, ImageDraw

DRAWS = 200


def load_asc(path):
    lines = open(path).read().split("\n")
    rows = [l for l in lines[6:] if l.strip()]
    return np.array([[float(v) for v in r.split()] for r in rows])


def rankdata(a):
    a = np.asarray(a, dtype=float)
    order = a.argsort(kind="mergesort")
    ranks = np.empty(len(a))
    ranks[order] = np.arange(len(a))
    # Average ties.
    for v in np.unique(a):
        m = a == v
        if m.sum() > 1:
            ranks[m] = ranks[m].mean()
    return ranks


def spearman(a, b):
    ra, rb = rankdata(a), rankdata(b)
    if ra.std() == 0 or rb.std() == 0:
        return 0.0
    return float(np.corrcoef(ra, rb)[0, 1])


def metrics(real_xy, real_pop, sim_xy, sim_pop, top_real=10):
    """Recall, precision, big-town recall and rank agreement for one placement."""
    d = np.sqrt(((real_xy[:, None, :] - sim_xy[None, :, :]) ** 2).sum(-1))
    out = {}
    for r in (5, 10):
        out[f"recall{r}"] = float((d.min(1) <= r).mean())
        out[f"precision{r}"] = float((d.min(0) <= r).mean())
    # The real top 10: is one of the model's top 15 within 10 km?
    big_real = np.argsort(-real_pop)[:top_real]
    big_sim = np.argsort(-sim_pop)[:15]
    out["big10"] = float((d[np.ix_(big_real, big_sim)].min(1) <= 10).mean())
    # Rank agreement: each real town against the largest model place within 5 km.
    near = np.where(d <= 5, sim_pop[None, :], 0).max(1)
    out["spearman"] = spearman(real_pop, near)
    return out


def nn_mean(xy):
    d = np.sqrt(((xy[:, None, :] - xy[None, :, :]) ** 2).sum(-1))
    np.fill_diagonal(d, np.inf)
    return float(d.min(1).mean())


def auc(pos, neg):
    """P(a real town's cell scores above a random land cell), ties counted half."""
    neg = np.sort(neg)
    lo = np.searchsorted(neg, pos, "left")
    hi = np.searchsorted(neg, pos, "right")
    return float(((lo + hi) / 2).mean() / len(neg))


def main():
    region, runs, out = sys.argv[1:4]
    border = float(sys.argv[4]) if len(sys.argv) > 4 else 0.0
    os.makedirs(out, exist_ok=True)
    global TERRAIN
    if os.path.exists(os.path.join(runs, "terrain.png")):
        TERRAIN = os.path.join(runs, "terrain.png")
    reg = json.load(open(os.path.join(region, "region.json")))
    name = os.path.basename(region.rstrip("/"))
    elev = load_asc(os.path.join(region, f"{name}.asc"))
    ny, nx = elev.shape
    w, s, e, n = reg["west"], reg["south"], reg["east"], reg["north"]

    towns = list(csv.DictReader(open(os.path.join(region, "towns.csv"))))
    towns = [t for t in towns if w <= float(t["lon"]) <= e and s <= float(t["lat"]) <= n]
    real_xy = np.array(
        [[(float(t["lon"]) - w) / (e - w) * nx, (n - float(t["lat"])) / (n - s) * ny] for t in towns]
    )
    inside = lambda x, y: border <= x <= nx - border and border <= y <= ny - border
    keep = [i for i, (x, y) in enumerate(real_xy) if inside(x, y)]
    towns = [towns[i] for i in keep]
    real_xy = real_xy[keep]
    real_pop = np.array([float(t["population"]) for t in towns])
    k = len(towns)
    with open(os.path.join(out, "truth.csv"), "w") as f:
        f.write("name,x_km,y_km,population,year\n")
        for t, (x, y) in zip(towns, real_xy):
            f.write(f"{t['name']},{x:.1f},{y:.1f},{float(t['population']):.0f},1300\n")

    land = np.array([p for p in np.argwhere(elev > 0)[:, ::-1] + 0.5 if inside(*p)])
    lowland = np.array([p for p in np.argwhere((elev > 0) & (elev < 50))[:, ::-1] + 0.5 if inside(*p)])
    rng = np.random.default_rng(12345)

    files = sorted(glob.glob(os.path.join(runs, "seed*.csv")), key=lambda p: int(re.findall(r"\d+", os.path.basename(p))[0]))
    rows, base_land, base_low = [], [], []
    big = real_pop >= 1000
    kb = int(big.sum())
    rows_b, land_b, low_b = [], [], []
    spacing = {"model": [], "land": []}
    freq = np.zeros((ny, nx))
    found = np.zeros(k)
    yy, xx = np.mgrid[0:ny, 0:nx] + 0.5
    sims = []
    for path in files:
        sim = [r for r in csv.DictReader(open(path)) if inside(float(r["x_km"]), float(r["y_km"]))]
        sim.sort(key=lambda r: -float(r["pop"]))
        top = sim[:k]
        sxy = np.array([[float(r["x_km"]), float(r["y_km"])] for r in top])
        spop = np.array([float(r["pop"]) for r in top])
        sims.append((sxy, spop, top))
        m = metrics(real_xy, real_pop, sxy, spop)
        m["seed"] = int(re.findall(r"\d+", os.path.basename(path))[0])
        m["largest"] = f"{top[0]['name']} {float(top[0]['pop']):.0f}"
        rows.append(m)
        rows_b.append(metrics(real_xy[big], real_pop[big], sxy[:kb], spop[:kb]))
        spacing["model"].append(nn_mean(sxy))
        d = np.sqrt(((real_xy[:, None, :] - sxy[None, :, :]) ** 2).sum(-1))
        found += d.min(1) <= 5
        hit = np.zeros((ny, nx), bool)
        for x, y in sxy:
            hit |= (xx - x) ** 2 + (yy - y) ** 2 <= 9
        freq += hit
        for pool, acc in ((land, base_land), (lowland, base_low)):
            for _ in range(DRAWS // 10):
                pick = pool[rng.choice(len(pool), k, replace=False)]
                acc.append(metrics(real_xy, real_pop, pick, rng.permutation(spop)))
        for pool, acc in ((land, land_b), (lowland, low_b)):
            for _ in range(DRAWS // 10):
                pick = pool[rng.choice(len(pool), kb, replace=False)]
                acc.append(metrics(real_xy[big], real_pop[big], pick, rng.permutation(spop[:kb])))
        spacing["land"].append(nn_mean(land[rng.choice(len(land), k, replace=False)]))
    freq /= max(len(files), 1)

    keys = ["recall5", "recall10", "precision5", "precision10", "big10", "spearman"]
    labels = {
        "recall5": "Real towns with a model place within 5 km",
        "recall10": "Real towns with a model place within 10 km",
        "precision5": "Model places with a real town within 5 km",
        "precision10": "Model places with a real town within 10 km",
        "big10": "Real top 10 with a model top-15 place within 10 km",
        "spearman": "Rank agreement (Spearman, real size vs model size within 5 km)",
    }

    def mean(rs, key):
        return float(np.mean([r[key] for r in rs]))

    def beat(rs, base, key):
        # Share of seeds whose score exceeds the 95th percentile of random land.
        p95 = np.percentile([r[key] for r in base], 95)
        return float(np.mean([r[key] > p95 for r in rs]))

    md = [f"# Real-terrain scores: {name}", ""]
    if border:
        md += [f"Interior only: everything within {border:.0f} km of the map edge left out.", ""]
    for title, rs, bl, bw, kk in (
        (f"All {k} real towns against the model's top {k}", rows, base_land, base_low, k),
        (f"The {kb} real towns of 1,000 or more against the model's top {kb}", rows_b, land_b, low_b, kb),
    ):
        md.append(f"## {title}")
        md.append("")
        md.append(f"{len(files)} seeds, final year; random baselines drop the same {kk} places on random cells.")
        md.append("")
        md.append("| Measure | Model | Random land | Random lowland | Seeds beating random land (95th pct) |")
        md.append("| --- | ---: | ---: | ---: | ---: |")
        for key in keys:
            md.append(
                f"| {labels[key]} | {mean(rs, key):.2f} | {mean(bl, key):.2f} | {mean(bw, key):.2f} | {beat(rs, bl, key):.0%} |"
            )
        md.append("")

    # Site prediction over all seeds: does the frequency map rate real town
    # sites above land in general?
    land_f = np.array([freq[int(y), int(x)] for x, y in land])
    low_f = np.array([freq[int(y), int(x)] for x, y in lowland])
    cells = lambda sel: freq[np.clip(real_xy[sel, 1].astype(int), 0, ny - 1), np.clip(real_xy[sel, 0].astype(int), 0, nx - 1)]
    md.append("## Site prediction over all seeds")
    md.append("")
    md.append("Frequency map: share of seeds with a top-K model place within 3 km of a cell. AUC is the chance that a real town's cell scores above a random land cell (0.5 = no skill).")
    md.append("")
    md.append("| Towns | AUC against land | AUC against lowland | Mean frequency at real towns | Mean frequency on land |")
    md.append("| --- | ---: | ---: | ---: | ---: |")
    for lab, sel in (("All", np.ones(k, bool)), ("1,000 or more", big), ("Under 1,000", ~big)):
        md.append(f"| {lab} ({int(sel.sum())}) | {auc(cells(sel), land_f):.2f} | {auc(cells(sel), low_f):.2f} | {cells(sel).mean():.2f} | {land_f.mean():.2f} |")
    md.append("")
    # The largest real town: does the model put its largest places there?
    top_i = int(np.argmax(real_pop))
    near_top = []
    largest_at = []
    shares = []
    for path in files:
        pops = [float(r["pop"]) for r in csv.DictReader(open(path))]
        shares.append(max(pops) / sum(pops) if pops else 0.0)
    for sxy, spop, top in sims:
        dl = np.sqrt(((sxy - real_xy[top_i]) ** 2).sum(1))
        within = np.where(dl <= 10)[0]
        near_top.append(int(within.min()) + 1 if len(within) else None)
        d0 = np.sqrt(((real_xy - sxy[0]) ** 2).sum(1))
        largest_at.append((towns[int(d0.argmin())]["name"], float(d0.min()), float(dl[0])))
    md.append(f"## Largest city: {towns[top_i]['name']}")
    md.append("")
    ranks = [r for r in near_top if r is not None]
    md.append(
        f"Seeds whose largest model place is within 10 km of {towns[top_i]['name']}: "
        f"{sum(r == 1 for r in ranks)} of {len(sims)}; a top-3 place within 10 km: "
        f"{sum(r <= 3 for r in ranks)}; any top-{k} place within 10 km: {len(ranks)}. "
        f"Median distance from the model's largest place to {towns[top_i]['name']}: "
        f"{np.median([x[2] for x in largest_at]):.0f} km. "
        f"Largest place's share of the region's people (median): {np.median(shares):.2f}."
    )
    md.append("")
    from collections import Counter
    near = Counter(n_ for n_, _, _ in largest_at)
    md.append(
        "Nearest real town to the model's largest place, by seeds: "
        + ", ".join(f"{n_} {c}" for n_, c in near.most_common())
        + "."
    )
    md.append("")

    cpath = os.path.join(runs, "cells.csv")
    if os.path.exists(cpath):
        cls = np.full((ny, nx), "", dtype=object)
        for r in csv.DictReader(open(cpath)):
            cls[int(r["y"]), int(r["x"])] = r["class"]

        def site(x, y):
            # The best site within a cell of the point: a town on the bank or shore
            # counts as river or coast even if its centre falls a cell inland.
            x, y = int(x), int(y)
            near = set(cls[max(0, y - 1) : y + 2, max(0, x - 1) : x + 2].ravel())
            return next(c for c in ("river", "coast", "inland", "sea") if c in near)

        rsite = [site(x, y) for x, y in real_xy]
        msite = [site(x, y) for sxy, _, _ in sims for x, y in sxy]
        md.append("## By site")
        md.append("")
        md.append("Site of the real towns and of the model's top places (within a cell of a navigable river, else of the coast, else inland).")
        md.append("")
        md.append("| Site | Real towns | Share | Model top places | Share | Real towns found (5 km) |")
        md.append("| --- | ---: | ---: | ---: | ---: | ---: |")
        for c in ("river", "coast", "inland"):
            sel = np.array([x == c for x in rsite])
            nr, nm = int(sel.sum()), sum(x == c for x in msite)
            fr = found[sel].mean() / len(files) if nr else 0.0
            md.append(f"| {c} | {nr} | {nr / k:.0%} | {nm / len(files):.1f} | {nm / len(msite):.0%} | {fr:.0%} |")
        md.append("")

        # Spacing of inland markets: each inland place over 500 people to the
        # nearest other place over 500 (any site); medians.
        def inland_spacing(xy, pop, sites):
            big = pop >= 500
            pts = xy[big]
            out = []
            for q, (x, y) in enumerate(xy):
                if not big[q] or sites[q] != "inland":
                    continue
                d = np.sqrt(((pts - [x, y]) ** 2).sum(1))
                d = d[d > 0]
                if len(d):
                    out.append(d.min())
            return out

        real_sp = inland_spacing(real_xy, real_pop, rsite)
        model_sp, counts = [], []
        for path in files:
            allp = [r for r in csv.DictReader(open(path)) if inside(float(r["x_km"]), float(r["y_km"]))]
            xy = np.array([[float(r["x_km"]), float(r["y_km"])] for r in allp])
            pp = np.array([float(r["pop"]) for r in allp])
            sp = inland_spacing(xy, pp, [site(x, y) for x, y in xy])
            counts.append(len(sp))
            if sp:
                model_sp.append(float(np.median(sp)))
        md.append(
            f"Inland places over 500 people, nearest other place over 500 (median): real {np.median(real_sp):.1f} km "
            f"({len(real_sp)} towns); model {np.median(model_sp) if model_sp else float('nan'):.1f} km "
            f"(median over seeds; {np.median(counts):.0f} such places per seed)."
        )
        md.append("")

    md.append(
        f"Mean nearest-neighbour spacing of the {k} places: real {nn_mean(real_xy):.1f} km, "
        f"model {np.mean(spacing['model']):.1f} km, random land {np.mean(spacing['land']):.1f} km."
    )
    md.append("")
    md.append("## Per seed")
    md.append("")
    md.append("| Seed | " + " | ".join(keys) + " | Largest |")
    md.append("| ---: |" + " ---: |" * len(keys) + " --- |")
    for r in rows:
        md.append(f"| {r['seed']} | " + " | ".join(f"{r[k_]:.2f}" for k_ in keys) + f" | {r['largest']} |")
    md.append("")
    md.append("## Real towns by how often the model finds them (place within 5 km, share of seeds)")
    md.append("")
    md.append("| Town | Pop. c.1300 | Tier | Found |")
    md.append("| --- | ---: | --- | ---: |")
    order = np.argsort(-real_pop)
    for i in order:
        md.append(f"| {towns[i]['name']} | {real_pop[i]:.0f} | {towns[i]['tier']} | {found[i] / len(files):.0%} |")
    md.append("")

    # Persistent inventions: local maxima of the frequency map, held in at least
    # half the seeds, with no real town within 10 km.
    md.append("## Persistent model places with no real town within 10 km")
    md.append("")
    md.append("Cells where at least half the seeds put a top-K place within 3 km (local maxima).")
    md.append("")
    md.append("| x km | y km | Share of seeds | Nearest real town | km | Elevation m |")
    md.append("| ---: | ---: | ---: | --- | ---: | ---: |")
    inv = []
    for y in range(ny):
        for x in range(nx):
            f = freq[y, x]
            if f < 0.5:
                continue
            win = freq[max(0, y - 3) : y + 4, max(0, x - 3) : x + 4]
            if f < win.max():
                continue
            d = np.sqrt(((real_xy - [x + 0.5, y + 0.5]) ** 2).sum(1))
            if d.min() > 10:
                inv.append((f, x, y, towns[d.argmin()]["name"], d.min()))
    taken = []
    for f, x, y, t, dd in sorted(inv, reverse=True):
        if any((x - a) ** 2 + (y - b) ** 2 < 36 for a, b in taken):
            continue
        taken.append((x, y))
        md.append(f"| {x} | {y} | {f:.0%} | {t} | {dd:.0f} | {elev[y, x]:.0f} |")
    open(os.path.join(out, "scores.md"), "w").write("\n".join(md) + "\n")
    print("\n".join(md[: md.index("## Per seed")]))

    figures(out, elev, real_xy, real_pop, towns, sims, freq)


TERRAIN = None


def base(elev, scale):
    if TERRAIN is not None:
        img = Image.open(TERRAIN).convert("RGB").resize((elev.shape[1] * scale, elev.shape[0] * scale))
        return Image.blend(img, Image.new("RGB", img.size, (255, 255, 255)), 0.45)
    t = np.clip(elev / 200.0, 0, 1)
    img = np.zeros(elev.shape + (3,), np.uint8)
    img[..., 0] = 150 + 70 * t
    img[..., 1] = 170 + 50 * t
    img[..., 2] = 130 + 40 * t
    img[elev <= 0] = [170, 195, 215]
    return Image.fromarray(img).resize((elev.shape[1] * scale, elev.shape[0] * scale), Image.NEAREST)


def dots(draw, xy, pop, scale, fill, outline=(40, 30, 30)):
    for (x, y), p in zip(xy, pop):
        r = max(2.0, 1.2 * np.sqrt(p / 150.0)) * scale / 4
        r = min(r, 6 * scale)
        draw.ellipse([x * scale - r, y * scale - r, x * scale + r, y * scale + r], fill=fill, outline=outline)


def figures(out, elev, real_xy, real_pop, towns, sims, freq):
    scale = 4
    # Real towns.
    a = base(elev, scale)
    da = ImageDraw.Draw(a)
    dots(da, real_xy, real_pop, scale, (200, 40, 40))
    for i in np.argsort(-real_pop)[:12]:
        x, y = real_xy[i]
        da.text((x * scale + 6, y * scale - 6), towns[i]["name"], fill=(20, 20, 20))
    a.save(os.path.join(out, "real.png"))
    # Model, seeds 1-3 (top K, same symbol scale with sizes rescaled to the real total).
    for idx in range(min(3, len(sims))):
        sxy, spop, top = sims[idx]
        b = base(elev, scale)
        db = ImageDraw.Draw(b)
        dots(db, real_xy, real_pop, scale, None, (200, 40, 40))
        dots(db, sxy, spop * real_pop.sum() / spop.sum(), scale, (40, 70, 200))
        b.save(os.path.join(out, f"model_seed{idx + 1}.png"))
    # Frequency over all seeds with the real towns on top.
    c = base(elev, scale).convert("RGBA")
    heat = np.zeros(freq.shape + (4,), np.uint8)
    heat[..., 0] = 220
    heat[..., 1] = 80
    heat[..., 2] = 10
    heat[..., 3] = (np.clip(freq, 0, 1) * 220).astype(np.uint8)
    c = Image.alpha_composite(c, Image.fromarray(heat).resize(c.size, Image.NEAREST))
    dc = ImageDraw.Draw(c)
    dots(dc, real_xy, real_pop, scale, None, (20, 20, 20))
    c.convert("RGB").save(os.path.join(out, "frequency.png"))
    # Side by side.
    imgs = [Image.open(os.path.join(out, f)) for f in ("real.png", "model_seed1.png", "frequency.png")]
    titles = (
        "Real towns c.1300 (size = population)",
        "Model seed 1, top places (blue); real towns (red rings)",
        "Model, all seeds: share with a top place within 3 km",
    )
    W = sum(i.width for i in imgs) + 20
    side = Image.new("RGB", (W, imgs[0].height + 22), (255, 255, 255))
    ds = ImageDraw.Draw(side)
    x = 0
    for i, title in zip(imgs, titles):
        side.paste(i, (x, 22))
        ds.text((x + 4, 5), title, fill=(20, 20, 20))
        x += i.width + 10
    side.save(os.path.join(out, "side_by_side.png"))


if __name__ == "__main__":
    main()
