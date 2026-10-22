"""R (D119): one row per validate output: each map's runs passing every
check, the minimum and total, gated checks below 15/20, and each map's
most-failed check.

usage: python3 -I tools/retune/summary.py FILE [FILE...]
"""
import re, sys

rows = []
for path in sys.argv[1:]:
    text = open(path).read()
    if "validation" not in text:
        continue
    passing = [int(x) for x in re.findall(r"runs passing every check: (\d+)/", text)]
    below = len(re.findall(r"FAIL\s*$", text, re.M))
    worst = []
    for line in re.findall(r"failing checks \(runs failing / of which the only failure\): (.*)", text):
        first = line.split(";")[0].strip()
        worst.append(first or "-")
    name = path.rsplit("/", 1)[-1][:-4]
    rows.append((min(passing), sum(passing), name, passing, below, worst))
rows.sort(key=lambda r: (-r[0], -r[1], r[2]))
print("| Setting | Map 1 | Map 2 | Map 3 | Min | Total | Gated checks < 15/20 | Most-failed check (map 1 / 2 / 3) |")
print("| --- | --- | --- | --- | --- | --- | --- | --- |")
for mn, tot, name, p, below, worst in rows:
    print(f"| {name} | {p[0]} | {p[1]} | {p[2]} | {mn} | {tot} | {below} | {' / '.join(worst)} |")
