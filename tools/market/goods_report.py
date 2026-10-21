"""Goods prices under the equilibrium (v6 S1): per run, at one year, the
median delivered price of each good among buyers that some supplier reaches
(price not exactly 1), the supply prices at holders, unmet demand by good,
and specialist sizes.

usage: python3 -I tools/market/goods_report.py OUT1.txt [...]
"""
import statistics as st, sys

names = ["salt", "iron", "stone", "imports"]
for path in sys.argv[1:]:
    rows = [l.split() for l in open(path) if l.startswith("price")]
    mk = [l.split() for l in open(path) if l.startswith("market")][-1]
    unmet = [float(v) for v in mk[mk.index("unmet") + 1:]]
    out = [path.split("/")[-1]]
    for k, n in enumerate(names):
        v = sorted(float(r[7 + k]) for r in rows if abs(float(r[7 + k]) - 1.0) > 1e-4)
        out.append(f"{n} towns {st.median(v):.2f} ({len(v)})" if v else f"{n} towns -")
    for k, n in enumerate(names[:3]):
        v = sorted(float(r[14 + k]) for r in rows if float(r[14 + k]) > 0 and abs(float(r[14 + k]) - 1.0) > 1e-4)
        out.append(f"{n} sites {', '.join(f'{x:.2f}' for x in v) or '-'}")
    out.append("unmet " + " ".join(f"{n} {u:.0f}" for n, u in zip(names, unmet)))
    spec = sorted(float(r[12]) for r in rows if r[-1] != "Agrarian")
    out.append(f"specialists {len(spec)} sizes {', '.join(f'{x:.0f}' for x in spec)}")
    print("; ".join(out))
