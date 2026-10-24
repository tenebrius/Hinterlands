# D148: attribution of map 1's rank-size under putting-out (exploration, 20 seeds)

An exploration run under D146: map 1 only, seeds 1–20. **It is not a gate.**
- Binary as D145.
- Outputs: `target/d2/a148_*.txt`.

| Run | Params | Rank-size in band | Median slope | Largest share | Over 5,000 (median) | Exports / output | Runs passing every check |
| --- | --- | --- | --- | --- | --- | --- | --- |
| D144 (default) | — | 19/20 | −0.936 | 0.109 | 2 | 0.084 | 12/20 |
| D145 | `putting_out` | **3/20** | −0.745 | 0.092 | 1 | 0.186 | 3/20 |
| f = 1.0: all of the making cost stays in the mill town | `putting_out`, `finishing_share` 1.0 | **6/20** | −0.731 | 0.082 | 1 | 0.180 | 3/20 |
| Capacity frozen at D144's rule, making cost paid to the farms (f = 0.3) | `putting_out`, `by_employment_lb` 0 | 19/20 | −0.906 | 0.102 | 2 | 0.086 | 14/20 |

**How the frozen run works:**
- With s = 0 the stock's bound is 0, so each year the capacity falls back to `cloth_lb_per_w` × W. That is D144's rule.
- The farm adults in reach are unchanged, so 0.7 of the making cost is still paid to their farm purses.

**Read:**
- **The cloth boom flattens map 1's hierarchy, not the payment split.**
  - Moving the making cost to the farms with D144's capacity keeps the slope (19/20, −0.906).
  - Keeping all of it in the towns with the growing stock does not save it (6/20, −0.731).
- **Reading, not traced:** a whole-clip cloth trade spreads coin to about 100 mill places at once, so the middle ranks grow against the top. Over 5,000 falls to a median of 1, and the largest share to 0.08–0.09.
