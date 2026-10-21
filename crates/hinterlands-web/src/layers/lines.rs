//! Smoothing for lines drawn through cell centres (rivers, roads, routes and
//! the trade moving on them), so they do not zig-zag on the grid.

use egui::Vec2;

/// Chaikin corner cutting, `rounds` times, endpoints kept so lines still meet.
/// Each point carries an attribute; `mix(a, b, f)` gives the attribute of a new
/// point a fraction `f` of the way from a point with `a` to the next with `b`.
pub fn chaikin<T: Copy>(
    pts: &[Vec2],
    attr: &[T],
    rounds: usize,
    mix: impl Fn(T, T, f32) -> T,
) -> (Vec<Vec2>, Vec<T>) {
    let (mut p, mut a) = (pts.to_vec(), attr.to_vec());
    for _ in 0..rounds {
        let k = p.len();
        if k < 3 {
            break;
        }
        let mut np = vec![p[0]];
        let mut na = vec![a[0]];
        for s in 0..k - 1 {
            for f in [0.25, 0.75] {
                if (s == 0 && f == 0.25) || (s == k - 2 && f == 0.75) {
                    continue;
                }
                np.push(p[s] + (p[s + 1] - p[s]) * f);
                na.push(mix(a[s], a[s + 1], f));
            }
        }
        np.push(p[k - 1]);
        na.push(a[k - 1]);
        p = np;
        a = na;
    }
    (p, a)
}
