"""D2 report tables (D126): medians over runs, by 50 years, from the
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
    print("| Year | People | Clip lb | Cloth share of clip | Wool exported lb | Cloth home lb | Cloth exported lb | Cloth made at W 100-2,000 | Cloth towns | Over 2,000 | Median / largest cloth town |")
    print("| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |")
    for y in years:
        print(f"| {1100 + y} | {med(c, y, 'pop'):,.0f} | {med(c, y, 'clip_lb'):,.0f} | {med(c, y, 'cloth_share'):.2f} | {med(c, y, 'wool_out_lb'):,.0f} | {med(c, y, 'cloth_home_lb'):,.0f} | {med(c, y, 'cloth_out_lb'):,.0f} | {med(c, y, 'mid_w_share'):.2f} | {med(c, y, 'cloth_towns'):.0f} | {med(c, y, 'cloth_towns_over2k'):.0f} | {med(c, y, 'cloth_town_median_pop'):,.0f} / {med(c, y, 'largest_cloth_town'):,.0f} |")
    print()
    print("| Year | Coin stock | Price level | Grain to ports | Wool in | Cloth in | Duties out | Imports out | Luxuries out | Wear |")
    print("| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |")
    for y in years:
        print(f"| {1100 + y} | {med(m, y, 'stock'):,.0f} | {med(m, y, 'price'):.2f} | {med(m, y, 'port_in'):,.0f} | {med(m, y, 'wool_in'):,.0f} | {med(m, y, 'cloth_in'):,.0f} | {med(m, y, 'duties_out'):,.0f} | {med(m, y, 'imports'):,.0f} | {med(m, y, 'luxury'):,.0f} | {med(m, y, 'wear'):,.0f} |")


if __name__ == "__main__":
    main()
