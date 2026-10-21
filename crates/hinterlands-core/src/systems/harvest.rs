//! Local harvests (v6 W1). Each year every cell's yield is multiplied by
//! `m = clamp(1 + σ z, harvest_min, harvest_max)`, where `z` is a smooth field:
//! standard normals drawn from the run's `Rng` (Box–Muller with `libm`) on a
//! grid of nodes `harvest_km` apart, interpolated bilinearly to cell centres.
//! Each year is drawn fresh (Clark's yield autocorrelation is about 0.16).
//! Cultivation applies the multiplier after the fields are chosen, on top of the
//! famine factor. Writes: harvest.

use crate::params::Params;
use crate::state::State;
use crate::terrain::Terrain;
use crate::Rng;
use rand::Rng as _;

/// Nodes along one side covering `cells` cells of `cell_km`, `step_km` apart.
fn nodes(cells: usize, cell_km: f32, step_km: f32) -> usize {
    (libm::ceilf(cells as f32 * cell_km / step_km) as usize) + 1
}

/// A uniform draw in (0, 1], from a `u32` (D6).
fn unit(rng: &mut Rng) -> f64 {
    (rng.gen::<u32>() as f64 + 1.0) / 4_294_967_296.0
}

pub fn run(state: &mut State, t: &Terrain, p: &Params, rng: &mut Rng) {
    if !p.local_harvests {
        state.harvest.clear();
        return;
    }
    let g = &t.grid;
    let step = p.harvest_km.max(g.cell_km);
    let (nx, ny) = (
        nodes(g.w as usize, g.cell_km, step),
        nodes(g.h as usize, g.cell_km, step),
    );
    let mut z = vec![0f32; nx * ny];
    let mut i = 0;
    while i < z.len() {
        let r = libm::sqrt(-2.0 * libm::log(unit(rng)));
        let a = 2.0 * core::f64::consts::PI * unit(rng);
        z[i] = (r * libm::cos(a)) as f32;
        if i + 1 < z.len() {
            z[i + 1] = (r * libm::sin(a)) as f32;
        }
        i += 2;
    }
    state.harvest.resize(t.n(), 1.0);
    let (w, h) = (g.w as usize, g.h as usize);
    for y in 0..h {
        let v = (y as f32 + 0.5) * g.cell_km / step;
        let (j, fy) = (v as usize, v - libm::floorf(v));
        for x in 0..w {
            let u = (x as f32 + 0.5) * g.cell_km / step;
            let (k, fx) = (u as usize, u - libm::floorf(u));
            let at = |a: usize, b: usize| z[b.min(ny - 1) * nx + a.min(nx - 1)];
            let top = at(k, j) * (1.0 - fx) + at(k + 1, j) * fx;
            let bot = at(k, j + 1) * (1.0 - fx) + at(k + 1, j + 1) * fx;
            let zz = top * (1.0 - fy) + bot * fy;
            state.harvest[y * w + x] =
                (1.0 + p.harvest_sd * zz).clamp(p.harvest_min, p.harvest_max);
        }
    }
}
