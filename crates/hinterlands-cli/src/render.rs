//! Headless map rendering to PNG, for milestone notes and eyeballing runs.

use crate::png;
use hinterlands_core::cost::{CostGrid, Dijkstra};
use hinterlands_core::state::NONE;
use hinterlands_core::terrain::{Terrain, Water};
use hinterlands_core::Snapshot;

type Rgb = [f32; 3];

fn mix(a: Rgb, b: Rgb, t: f32) -> Rgb {
    let t = t.clamp(0.0, 1.0);
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

fn ramp(v: f32) -> Rgb {
    // Dark blue -> yellow heat ramp.
    let v = v.clamp(0.0, 1.0);
    if v < 0.5 {
        mix([0.1, 0.1, 0.35], [0.8, 0.2, 0.3], v * 2.0)
    } else {
        mix([0.8, 0.2, 0.3], [1.0, 0.95, 0.4], (v - 0.5) * 2.0)
    }
}

fn shade(t: &Terrain, i: usize) -> f32 {
    let g = t.grid;
    let (x, y) = g.xy(i);
    let e = |xx: i32, yy: i32| {
        let xx = xx.clamp(0, g.w as i32 - 1);
        let yy = yy.clamp(0, g.h as i32 - 1);
        t.elevation[g.idx(xx, yy)].max(0.0)
    };
    // Light from the north-west.
    let dz = (e(x - 1, y - 1) - e(x + 1, y + 1)) / (g.cell_km * 1000.0);
    (1.0 + dz * 4.0).clamp(0.6, 1.3)
}

pub fn base_color(t: &Terrain, i: usize, forest: f32) -> Rgb {
    match t.water[i] {
        Water::Sea => mix(
            [0.16, 0.32, 0.55],
            [0.05, 0.12, 0.3],
            (-t.elevation[i] / 300.0).clamp(0.0, 1.0),
        ),
        Water::NavigableRiver => [0.1, 0.3, 0.75],
        _ => {
            let poor = [0.72, 0.66, 0.5];
            let good = [0.55, 0.72, 0.35];
            let mut c = mix(poor, good, t.food_yield[i]);
            c = mix(
                c,
                [0.86, 0.86, 0.86],
                ((t.elevation[i] - 900.0) / 400.0).clamp(0.0, 1.0),
            );
            c = mix(c, [0.15, 0.38, 0.18], forest * 0.85);
            if t.water[i] == Water::Stream {
                c = mix(c, [0.35, 0.55, 0.85], 0.7);
            }
            let s = shade(t, i);
            [c[0] * s, c[1] * s, c[2] * s]
        }
    }
}

pub fn render(
    t: &Terrain,
    snap: Option<&Snapshot>,
    layer: &str,
    scale: u32,
    from: Option<usize>,
) -> Vec<u8> {
    let g = t.grid;
    let n = g.n();
    let mut cell_rgb: Vec<Rgb> = vec![[0.0; 3]; n];
    let forest = |i: usize| snap.map_or(t.forest0[i], |s| s.forest[i] as f32 / 255.0);

    match layer {
        "yield" => (0..n).for_each(|i| {
            cell_rgb[i] = if t.is_sea(i) {
                [0.1, 0.2, 0.4]
            } else {
                ramp(t.food_yield[i])
            }
        }),
        "forest" => (0..n).for_each(|i| {
            cell_rgb[i] = if t.is_sea(i) {
                [0.1, 0.2, 0.4]
            } else {
                ramp(forest(i))
            }
        }),
        "elevation" => (0..n).for_each(|i| {
            cell_rgb[i] = if t.is_sea(i) {
                [0.1, 0.2, 0.4]
            } else {
                ramp(t.elevation[i] / 1300.0)
            }
        }),
        "score" => {
            let s = snap.expect("score layer needs a snapshot");
            let (lo, hi) = s
                .site_score
                .iter()
                .filter(|v| v.is_finite())
                .fold((f32::MAX, f32::MIN), |a, &v| (a.0.min(v), a.1.max(v)));
            for i in 0..n {
                let v = s.site_score[i];
                cell_rgb[i] = if v.is_finite() {
                    ramp((v - lo) / (hi - lo).max(1e-6))
                } else {
                    mix(base_color(t, i, forest(i)), [0.0, 0.0, 0.0], 0.5)
                };
            }
        }
        "cost" => {
            let road = snap.map_or(vec![false; n], |s| s.road.clone());
            let tr = snap.map(|s| s.transport).unwrap_or_else(|| {
                hinterlands_core::cost::Transport::from_params(&Default::default())
            });
            let cg = CostGrid::build(t, &road, &tr);
            let mut dj = Dijkstra::new(n);
            let src =
                from.unwrap_or_else(|| (0..n).filter(|&i| t.is_land(i)).nth(n / 3).unwrap_or(0));
            dj.run(&cg, src as u32, 200.0, false);
            for i in 0..n {
                let c = dj.cell_cost(n, i);
                cell_rgb[i] = if c.is_finite() {
                    let band = if ((c / 10.0).floor() as i32) % 2 == 0 {
                        1.0
                    } else {
                        0.85
                    };
                    let v = ramp(1.0 - c / 120.0);
                    [v[0] * band, v[1] * band, v[2] * band]
                } else {
                    [0.0, 0.0, 0.0]
                };
            }
            cell_rgb[src] = [1.0, 1.0, 1.0];
        }
        _ => {
            for i in 0..n {
                let mut c = base_color(t, i, forest(i));
                if let Some(s) = snap {
                    if s.owner[i] != NONE {
                        c = mix(c, [0.93, 0.83, 0.4], 0.6);
                    }
                    if s.road[i] && t.is_land(i) {
                        c = mix(c, [0.45, 0.28, 0.12], 0.75);
                    }
                }
                cell_rgb[i] = c;
            }
            if let Some(s) = snap {
                // Water-borne trade lightens the water it crosses.
                let mut load = vec![0f32; n];
                for f in &s.flows {
                    for &node in &f.path {
                        if node as usize >= n {
                            load[node as usize - n] += f.amount;
                        }
                    }
                }
                let max = load.iter().copied().fold(1.0f32, f32::max);
                for i in 0..n {
                    if load[i] > 0.0 && !t.is_land(i) {
                        let v = (load[i].ln_1p() / max.ln_1p()).clamp(0.0, 1.0);
                        cell_rgb[i] = mix(cell_rgb[i], [0.6, 0.95, 1.0], 0.3 + 0.6 * v);
                    }
                }
            }
        }
    }

    let (w, h) = (g.w * scale, g.h * scale);
    let mut px = vec![0u8; (w * h * 3) as usize];
    let mut put = |x: i32, y: i32, c: Rgb| {
        if x >= 0 && y >= 0 && (x as u32) < w && (y as u32) < h {
            let o = ((y as u32 * w + x as u32) * 3) as usize;
            for k in 0..3 {
                px[o + k] = (c[k].clamp(0.0, 1.0) * 255.0) as u8;
            }
        }
    };
    for i in 0..n {
        let (x, y) = g.xy(i);
        for dy in 0..scale as i32 {
            for dx in 0..scale as i32 {
                put(x * scale as i32 + dx, y * scale as i32 + dy, cell_rgb[i]);
            }
        }
    }
    if let Some(s) = snap {
        if layer != "yield" && layer != "forest" && layer != "elevation" {
            for st in &s.settlements {
                let (x, y) = g.xy(st.anchor as usize);
                let r = 1 + (st.pop.max(10.0).log10() * 1.6) as i32;
                let cx = x * scale as i32 + scale as i32 / 2;
                let cy = y * scale as i32 + scale as i32 / 2;
                for dy in -r..=r {
                    for dx in -r..=r {
                        let edge = dx.abs() == r || dy.abs() == r;
                        put(
                            cx + dx,
                            cy + dy,
                            if edge {
                                [0.1, 0.05, 0.05]
                            } else {
                                [0.85, 0.15, 0.1]
                            },
                        );
                    }
                }
            }
        }
    }
    png::encode(w, h, &px)
}
