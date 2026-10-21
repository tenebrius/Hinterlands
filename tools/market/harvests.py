"""Local harvests report (v6 W1).

usage: python3 -I tools/market/harvests.py OUT1.txt [OUT2.txt ...]
Each input is the output of `hinterlands prices` (years 100-500). For each run:
  - the coefficient of variation of each settlement's harvest over the years
    it lived (settlements alive 50 years or more), as p10 / median / p90, both
    of the whole harvest and of the fields' mean yield multiplier;
  - the depth of local dearths: the worst place's multiplier each year, as
    p10 / median / min over the years, outside the famine (years 215-217).
"""
import collections, statistics as st, sys


def cv(xs):
    m = st.fmean(xs)
    return st.pstdev(xs) / m if m > 0 else float("nan")


def q(xs, f):
    xs = sorted(xs)
    return xs[min(len(xs) - 1, int(f * len(xs)))]


for path in sys.argv[1:]:
    harvest = collections.defaultdict(list)
    mult = collections.defaultdict(list)
    worst = {}
    for line in open(path):
        f = line.split()
        if f[0] != "price" or "harvest" not in f:
            continue
        y, i = int(f[2]), int(f[3])
        h, m = float(f[f.index("harvest") + 1]), float(f[f.index("m") + 1])
        if h <= 0:
            continue
        harvest[i].append(h)
        mult[i].append(m)
        worst[y] = min(worst.get(y, 9.0), m)
    hc = [cv(v) for v in harvest.values() if len(v) >= 50]
    mc = [cv(v) for v in mult.values() if len(v) >= 50]
    ws = [m for y, m in worst.items() if not 215 <= y <= 217]
    print(
        f"{path.split('/')[-1]}: {len(hc)} places; harvest CV "
        f"{q(hc, .1):.3f} / {st.median(hc):.3f} / {q(hc, .9):.3f}; multiplier CV "
        f"{q(mc, .1):.3f} / {st.median(mc):.3f} / {q(mc, .9):.3f}; worst place a year "
        f"{q(ws, .1):.3f} / {st.median(ws):.3f} / min {min(ws):.3f}"
    )
