"""v7 M0: the marketplaces diagnostic.

usage: python3 -I tools/market/m0.py OUT1.txt [OUT2.txt ...]
Each input is the output of `hinterlands marketplaces`. For each run and year:
  - turnover under seller routing against today's market_in: Spearman rank
    correlation over all places, rank-size slope of each (OLS of log value on
    log rank over places with value >= 1), and the overlap of the top tens;
  - for places over 500 and over 2,000: the toll on turnover at which income
    (toll + rents + fees + goods) meets the food bill, and the share that
    break even at tolls of 2%, 5% and 10%.
"""
import collections, math, statistics as st, sys


def ranks(xs):
    order = sorted(range(len(xs)), key=lambda i: xs[i])
    r = [0.0] * len(xs)
    i = 0
    while i < len(order):
        j = i
        while j + 1 < len(order) and xs[order[j + 1]] == xs[order[i]]:
            j += 1
        for k in range(i, j + 1):
            r[order[k]] = (i + j) / 2 + 1
        i = j + 1
    return r


def spearman(a, b):
    ra, rb = ranks(a), ranks(b)
    ma, mb = st.fmean(ra), st.fmean(rb)
    num = sum((x - ma) * (y - mb) for x, y in zip(ra, rb))
    den = math.sqrt(sum((x - ma) ** 2 for x in ra) * sum((y - mb) ** 2 for y in rb))
    return num / den if den else float("nan")


def slope(vals):
    v = sorted((x for x in vals if x >= 1), reverse=True)
    if len(v) < 3:
        return float("nan")
    xs = [math.log(i + 1) for i in range(len(v))]
    ys = [math.log(x) for x in v]
    mx, my = st.fmean(xs), st.fmean(ys)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)


def q(xs, f):
    xs = sorted(xs)
    return xs[min(len(xs) - 1, int(f * len(xs)))] if xs else float("nan")


rows = collections.defaultdict(list)
for path in sys.argv[1:]:
    for line in open(path):
        f = line.split()
        if f[0] != "mkt":
            continue
        key = (int(f[1]), int(f[2]), int(f[3]))
        rows[key].append(
            dict(id=int(f[4]), pop=float(f[5]), mi=float(f[6]), to=float(f[7]),
                 rent=float(f[8]), fees=float(f[9]), goods=float(f[10]), bill=float(f[11]))
        )

be = {500: collections.defaultdict(list), 2000: collections.defaultdict(list)}
print("map seed year  spearman  slope(turnover)  slope(market_in)  top10 overlap")
for (mp, sd, yr), rs in sorted(rows.items()):
    to = [r["to"] for r in rs]
    mi = [r["mi"] for r in rs]
    top = lambda key: {r["id"] for r in sorted(rs, key=lambda r: -r[key])[:10]}
    print(f"{mp:>3} {sd:>4} {yr:>4}  {spearman(to, mi):8.3f}  {slope(to):15.2f}  {slope(mi):16.2f}  {len(top('to') & top('mi')):>5}/10")
    for r in rs:
        need = r["bill"] - r["rent"] - r["fees"] - r["goods"]
        tau = max(0.0, need / r["to"]) if r["to"] > 0 else (0.0 if need <= 0 else float("inf"))
        for size in (500, 2000):
            if r["pop"] > size:
                be[size][yr].append(tau)

for size in (500, 2000):
    for yr, taus in sorted(be[size].items()):
        fin = [t for t in taus if math.isfinite(t)]
        share = lambda c: sum(1 for t in taus if t <= c) / len(taus)
        print(
            f"places over {size:,} in year {yr} ({len(taus)} over all runs): break-even toll "
            f"p25 {q(fin, .25):.3f} / median {q(fin, .5):.3f} / p75 {q(fin, .75):.3f}; "
            f"break even at 2% {share(.02):.2f}, 5% {share(.05):.2f}, 10% {share(.10):.2f}; "
            f"with no toll {share(0.0):.2f}"
        )
