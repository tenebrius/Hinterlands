"""D2 report tables (D128): medians over runs, by 50 years, from the
`hinterlands cloth` and `hinterlands money` probes.

usage: python3 -I d2.py DIR   (DIR holds c*.txt from cloth, m*.txt from money)
"""
import glob, os, statistics, sys


def load(pattern, tag):
    runs = []
    for f in sorted(glob.glob(pattern)):
        hdr, rows = None, {}
        for line in open(f):
            t = line.split()
            if not t or t[0] != tag:
                continue
            if hdr is None and not t[1].lstrip('-').isdigit():
                hdr = t[1:]
                continue
            r = dict(zip(hdr, t[1:]))
            rows[int(r["year"])] = r
        runs.append(rows)
    return runs


def med(runs, y, k):
    v = []
    for r in runs:
        if y in r:
            try:
                x = float(r[y][k])
            except ValueError:
                continue
            if x == x:
                v.append(x)
    return statistics.median(v) if v else float("nan")


def main():
    d = sys.argv[1]
    c = load(os.path.join(d, "c*.txt"), "cloth")
    m = load(os.path.join(d, "m*.txt"), "money")
    years = range(50, 501, 50)
    print("| Year | People | Clip lb | Home use lb (need) | To mills lb | To ports lb | Cloth share of wool leaving | Cloth made at W 100-2,000 | Cloth towns | Over 2,000 (median / max of runs) | Median / largest cloth town |")
    print("| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |")
    for y in years:
        mx = max((float(r[y]["cloth_towns_over2k"]) for r in c if y in r), default=float("nan"))
        print(f"| {1100 + y} | {med(c, y, 'pop'):,.0f} | {med(c, y, 'clip_lb'):,.0f} | {med(c, y, 'home_use_lb'):,.0f} ({med(c, y, 'home_need_lb'):,.0f}) | {med(c, y, 'to_mills_lb'):,.0f} | {med(c, y, 'to_ports_lb'):,.0f} | {med(c, y, 'cloth_share_out'):.2f} | {med(c, y, 'mid_w_share'):.2f} | {med(c, y, 'cloth_towns'):.0f} | {med(c, y, 'cloth_towns_over2k'):.0f} / {mx:.0f} | {med(c, y, 'cloth_town_median_pop'):,.0f} / {med(c, y, 'largest_cloth_town'):,.0f} |")
    print()
    print("Cloth share of wool leaving, by decade 1300-1600 (median of runs): " + ", ".join(f"{1100 + y} {med(c, y, 'cloth_share_out'):.2f}" for y in range(200, 501, 10)))
    print()
    print("Cloth-making places over 2,000, by decade 1250-1340 (median / max of runs): " + ", ".join(f"{1100 + y} {med(c, y, 'cloth_towns_over2k'):.0f}/{max((float(r[y]['cloth_towns_over2k']) for r in c if y in r), default=float('nan')):.0f}" for y in range(150, 241, 10)))
    print()
    print("| Year | Coin stock | Price level | Grain to ports | Wool in | Cloth in | Imports out | Luxuries out | Wear |")
    print("| --- | --- | --- | --- | --- | --- | --- | --- | --- |")
    for y in years:
        print(f"| {1100 + y} | {med(m, y, 'stock'):,.0f} | {med(m, y, 'price'):.2f} | {med(m, y, 'port_in'):,.0f} | {med(m, y, 'wool_in'):,.0f} | {med(m, y, 'cloth_in'):,.0f} | {med(m, y, 'imports'):,.0f} | {med(m, y, 'luxury'):,.0f} | {med(m, y, 'wear'):,.0f} |")


if __name__ == "__main__":
    main()
