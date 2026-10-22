"""D0 (D118, D120): aggregate `hinterlands demand` output over runs.

usage: python3 -I tools/demand/d0.py FILE [FILE...]
Prints medians over runs, by decade: each good's current and stock-based
demand, cloth in wool against wool produced; iron 1300-1340 against
1400-1440 (total and per arable cell); growing places' share of stone against
their share of W; and demand by place type at 1300, 1400 and 1500.
"""
import collections, statistics as st, sys

rows = collections.defaultdict(list)
types = collections.defaultdict(lambda: collections.defaultdict(list))
stone = collections.defaultdict(list)
for path in sys.argv[1:]:
    for line in open(path):
        f = line.split()
        if not f:
            continue
        if f[0] == "d0" and f[1] != "seed":
            y = int(f[2])
            rows[y].append(dict(pop=float(f[3]), fields=float(f[4]), salt=(float(f[7]), float(f[8])),
                                iron=(float(f[11]), float(f[12])), stone=(float(f[15]), float(f[16])),
                                imports=float(f[19]), cloth=float(f[21]), wool=float(f[22])))
        elif f[0] == "d0type":
            y, ty = int(f[2]), f[3]
            types[y][ty].append([float(x) for x in (f[5], f[6], f[8], f[9], f[11], f[12])])
        elif f[0] == "d0stone":
            stone[int(f[2])].append((float(f[4]), float(f[6])))

med = lambda xs: st.median(xs) if xs else float("nan")
print("| Year | People | Arable cells | Salt cur / stock | Iron cur / stock | Stone cur / stock | Cloth wool lb / wool produced lb | Growing places: stone share / W share |")
print("| --- | --- | --- | --- | --- | --- | --- | --- |")
for y in sorted(rows):
    r = rows[y]
    g = lambda k, i: med([x[k][i] for x in r])
    s = stone.get(y, [])
    print(f"| {1100 + y} | {med([x['pop'] for x in r]):,.0f} | {med([x['fields'] for x in r]):.0f} | {g('salt',0):.0f} / {g('salt',1):.0f} | "
          f"{g('iron',0):.0f} / {g('iron',1):.0f} | {g('stone',0):.0f} / {g('stone',1):.0f} | "
          f"{med([x['cloth'] for x in r]):,.0f} / {med([x['wool'] for x in r]):,.0f} | "
          f"{med([a for a, b in s]):.2f} / {med([b for a, b in s]):.2f} |")

def window(lo, hi, key):
    return [x for y in rows if lo <= y <= hi for x in rows[y]]
for lo, hi, name in ((200, 240, "1300-1340"), (300, 340, "1400-1440")):
    w = window(lo, hi, None)
    tot = med([x["iron"][1] for x in w]); cur = med([x["iron"][0] for x in w])
    per = med([x["iron"][1] / max(x["fields"], 1) for x in w])
    print(f"iron {name}: stock-based {tot:.0f} (current {cur:.1f}), per arable cell {per:.2f}, arable cells {med([x['fields'] for x in w]):.0f}")
grow = [a > b for y in stone for a, b in stone[y] if y >= 100]
print(f"decades (from 1200) in which growing places' stone share exceeds their W share: {sum(grow)}/{len(grow)}")
for y in (200, 300, 400):
    print(f"by type, {1100 + y} (median; salt cur/stock, iron cur/stock, stone cur/stock):")
    for ty, v in sorted(types[y].items()):
        m = [med([x[i] for x in v]) for i in range(6)]
        print(f"  {ty:<12} salt {m[0]:.0f}/{m[1]:.0f}  iron {m[2]:.1f}/{m[3]:.0f}  stone {m[4]:.0f}/{m[5]:.0f}")
