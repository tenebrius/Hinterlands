"""Clark (1999) checks on shadow grain prices (v6 S0, D69).

usage: python3 -I tools/market/clark.py OUT1.txt [OUT2.txt ...]
Each input is the output of `hinterlands prices`. For each run:
  1. local price response to the regional price: b_P in
     ln p_it = a_i + b_P ln P_t + b_Y ln Y_it  (settlement fixed effects);
  2. response to local yield with the regional price controlled for: b_Y;
  3. share of local price variation the regional price explains: R^2 of
     ln p_it on ln P_t alone (pooled, with a constant).
P_t is the population-weighted mean grain price of year t; Y_it is the
settlement's harvest per cultivated cell. Also the grain arcs' price gap
against transport (median gap/transport), and the solver residuals.
"""
import collections, math, statistics as st, sys


def ols2(rows):
    """Within (fixed-effect) OLS of y on x1, x2; rows are (group, y, x1, x2)."""
    g = collections.defaultdict(list)
    for k, y, a, b in rows:
        g[k].append((y, a, b))
    sxx = [[0.0, 0.0], [0.0, 0.0]]
    sxy = [0.0, 0.0]
    for vals in g.values():
        if len(vals) < 3:
            continue
        my = sum(v[0] for v in vals) / len(vals)
        ma = sum(v[1] for v in vals) / len(vals)
        mb = sum(v[2] for v in vals) / len(vals)
        for y, a, b in vals:
            dy, da, db = y - my, a - ma, b - mb
            sxx[0][0] += da * da
            sxx[0][1] += da * db
            sxx[1][1] += db * db
            sxy[0] += da * dy
            sxy[1] += db * dy
    sxx[1][0] = sxx[0][1]
    det = sxx[0][0] * sxx[1][1] - sxx[0][1] ** 2
    if abs(det) < 1e-12:
        return float("nan"), float("nan")
    bp = (sxy[0] * sxx[1][1] - sxy[1] * sxx[0][1]) / det
    by = (sxx[0][0] * sxy[1] - sxx[1][0] * sxy[0]) / det
    return bp, by


def r2(xs, ys):
    mx, my = st.fmean(xs), st.fmean(ys)
    sxy = sum((x - mx) * (y - my) for x, y in zip(xs, ys))
    sxx = sum((x - mx) ** 2 for x in xs)
    syy = sum((y - my) ** 2 for y in ys)
    return sxy * sxy / (sxx * syy) if sxx > 0 and syy > 0 else float("nan")


for path in sys.argv[1:]:
    prices = collections.defaultdict(list)  # year -> (id, p, yield, pop)
    gaps, resid = [], collections.defaultdict(list)
    for line in open(path):
        f = line.split()
        if f[0] == "price":
            y, i, p, yl, pop = int(f[2]), int(f[3]), float(f[6]), float(f[11]), float(f[12])
            if p > 0 and yl > 0 and pop > 0:
                prices[y].append((i, p, yl, pop))
        elif f[0] == "arc":
            gap, t = float(f[6]), float(f[7])
            if t > 0:
                gaps.append(gap / t)
        elif f[0] == "market":
            for k in range(5):
                resid[k].append(float(f[3 + k]))
    rows, lpt, lp = [], [], []
    for y, v in sorted(prices.items()):
        P = sum(p * pop for _, p, _, pop in v) / sum(pop for *_, pop in v)
        for i, p, yl, _ in v:
            rows.append((i, math.log(p), math.log(P), math.log(yl)))
            lpt.append(math.log(P))
            lp.append(math.log(p))
    bp, by = ols2(rows)
    print(
        f"{path.split('/')[-1]}: b_P {bp:.3f} (0.8-1.2), b_Y {by:.3f} (-0.15-0), "
        f"R2 regional {r2(lpt, lp):.2f} (>=0.60); gap/transport median "
        f"{st.median(gaps) if gaps else float('nan'):.2f} over {len(gaps)} arcs; "
        f"residual median grain {st.median(resid[0]):.3f} salt {st.median(resid[1]):.3f} "
        f"iron {st.median(resid[2]):.3f} stone {st.median(resid[3]):.3f} imports {st.median(resid[4]):.3f}"
    )
