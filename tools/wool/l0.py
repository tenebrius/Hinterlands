"""L0 (D105): aggregate `hinterlands wool` output.

usage: python3 -I tools/wool/l0.py GROUP:FILE[,FILE...] [GROUP:FILE...]
Prints, per group, year and trial wool price, the medians over runs of:
A's pasture cells and wool coin against grain coin; B's converted share, grain
lost (of harvest, of grain arriving at markets) and coin; pasture share by
travel-cost quintile and by yield band; and the break-even sack prices.
"""
import collections, statistics as st, sys


def med(xs):
    xs = [x for x in xs if x == x]
    return st.median(xs) if xs else float("nan")


def bands(tokens):
    return [float("nan") if t == "-" else float(t) for t in tokens]


for arg in sys.argv[1:]:
    group, files = arg.split(":", 1)
    a = collections.defaultdict(list)
    b = collections.defaultdict(list)
    be = collections.defaultdict(list)
    head = collections.defaultdict(list)
    for path in files.split(","):
        for line in open(path):
            f = line.split()
            if not f:
                continue
            if f[0] == "wool":
                y = int(f[2])
                head[y].append(dict(pop=float(f[4]), labour=float(f[6]), grain_coin=float(f[15]), p=float(f[13])))
            elif f[0] == "wool_be":
                y = int(f[2])
                be[y].append([float(f[i]) for i in (5, 7, 11, 13, 15, 17)])
            elif f[0] in ("wool_A", "wool_B"):
                y, pr = int(f[2]), float(f[4])
                d = dict(zip(f[5::2], f[6::2]))
                i = f.index("by_cost")
                j = f.index("by_yield")
                rec = dict(by_cost=bands(f[i + 1 : j]), by_yield=bands(f[j + 1 : j + 5]))
                if f[0] == "wool_A":
                    rec.update(cells=float(f[f.index("pasture") + 1]), vs=float(d["vs_grain"]))
                    a[(y, pr)].append(rec)
                else:
                    rec.update(share=float(d["share"]), vs=float(d["vs_grain"]),
                               harvest=float(d["of_harvest"]), arrived=float(d["of_arrived"]))
                    b[(y, pr)].append(rec)
    print(f"== {group}")
    for y in sorted(head):
        h = head[y]
        e = be[y]
        print(f"year {y}: pop {med([x['pop'] for x in h]):.0f}, regional price {med([x['p'] for x in h]):.2f}, "
              f"grain coin a year {med([x['grain_coin'] for x in h]):.0f}, arable labour {med([x['labour'] for x in h]):.1f}/cell; "
              f"break-even £/sack: A min {med([r[0] for r in e]):.2f} med {med([r[1] for r in e]):.2f}; "
              f"B min {med([r[2] for r in e]):.2f} p10 {med([r[3] for r in e]):.2f} p25 {med([r[4] for r in e]):.2f} med {med([r[5] for r in e]):.2f}")
        for pr in sorted({k[1] for k in a if k[0] == y}):
            ra, rb = a[(y, pr)], b[(y, pr)]
            mb = lambda key, k: med([r[key][k] for r in rb])
            ma = lambda key, k: med([r[key][k] for r in ra])
            print(f"  £{pr:g}: A cells {med([r['cells'] for r in ra]):.0f}, wool/grain coin {med([r['vs'] for r in ra]):.2f}, "
                  f"by cost {' '.join(f'{ma('by_cost', k):.2f}' for k in range(5))}, by yield {' '.join(f'{ma('by_yield', k):.2f}' for k in range(4))}")
            print(f"        B share {med([r['share'] for r in rb]):.2f}, grain lost {med([r['harvest'] for r in rb]):.2f} of harvest / "
                  f"{med([r['arrived'] for r in rb]):.2f} of arrivals, wool/grain coin {med([r['vs'] for r in rb]):.1f}, "
                  f"by cost {' '.join(f'{mb('by_cost', k):.2f}' for k in range(5))}, by yield {' '.join(f'{mb('by_yield', k):.2f}' for k in range(4))}")
