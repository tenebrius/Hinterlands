//! Smoothing for lines drawn through cell centres (rivers, roads, routes and
//! the trade moving on them), so they do not zig-zag on the grid.

use egui::Vec2;
use std::collections::{BTreeMap, BTreeSet};

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

/// Splits undirected edges into chains of nodes that run from pin to pin. A node
/// keeps its corner (ends a chain) if it is pinned or has other than two edges;
/// closed loops with no such node come back whole.
pub fn chains(edges: &BTreeSet<(u32, u32)>, pins: &BTreeSet<u32>) -> Vec<Vec<u32>> {
    let mut adj: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    for &(a, b) in edges {
        adj.entry(a).or_default().push(b);
        adj.entry(b).or_default().push(a);
    }
    let stop = |n: u32| pins.contains(&n) || adj[&n].len() != 2;
    let key = |a: u32, b: u32| (a.min(b), a.max(b));
    let mut seen: BTreeSet<(u32, u32)> = BTreeSet::new();
    let mut out = Vec::new();
    let mut walk = |from: u32, to: u32, seen: &mut BTreeSet<(u32, u32)>| {
        let mut nodes = vec![from, to];
        seen.insert(key(from, to));
        let (mut prev, mut cur) = (from, to);
        while !stop(cur) {
            let Some(&next) = adj[&cur].iter().find(|&&x| x != prev) else {
                break;
            };
            if !seen.insert(key(cur, next)) {
                break;
            }
            nodes.push(next);
            (prev, cur) = (cur, next);
        }
        out.push(nodes);
    };
    for (&node, next) in &adj {
        if stop(node) {
            for &to in next {
                if !seen.contains(&key(node, to)) {
                    walk(node, to, &mut seen);
                }
            }
        }
    }
    for &(a, b) in edges {
        if !seen.contains(&(a, b)) {
            walk(a, b, &mut seen);
        }
    }
    out
}
