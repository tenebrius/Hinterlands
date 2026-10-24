//! Local harvests (v6 W1). Each year every cell's yield is multiplied by
//! `m = clamp(1 + σ z, harvest_min, harvest_max)`, where `z` is a smooth field:
//! standard normals drawn from the run's `Rng` (Box–Muller with `libm`) on a
//! grid of nodes `harvest_km` apart, interpolated bilinearly to cell centres
//! and rescaled so that `z` has unit variance at every cell (D75). Each year is
//! independent (Clark's yield autocorrelation is about 0.16). The node values
//! for `harvest_years` years are drawn at initialisation, before anything else
//! draws, so a seed has the same weather whatever else the run does (D75);
//! later years are drawn when reached. Cultivation applies the multiplier after
//! the fields are chosen, on top of the famine factor. Writes: harvest,
//! harvest_nodes.

use crate::params::Params;
use crate::state::State;
use crate::terrain::Terrain;
use crate::Rng;
use rand::Rng as _;

/// Nodes along one side covering `cells` cells of `cell_km`, `step_km` apart.
fn nodes(cells: u32, cell_km: f32, step_km: f32) -> usize {
    (libm::ceilf(cells as f32 * cell_km / step_km) as usize) + 1
}

fn step_km(t: &Terrain, p: &Params) -> f32 {
    p.harvest_km.max(t.grid.cell_km)
}

/// Node values for one year.
pub fn node_count(t: &Terrain, p: &Params) -> usize {
    let s = step_km(t, p);
    nodes(t.grid.w, t.grid.cell_km, s) * nodes(t.grid.h, t.grid.cell_km, s)
}

/// A uniform draw in (0, 1], from a `u32` (D6).
fn unit(rng: &mut Rng) -> f64 {
    (rng.gen::<u32>() as f64 + 1.0) / 4_294_967_296.0
}

/// Append standard normals until `nodes` holds `years` years.
fn draw(nodes: &mut Vec<f32>, per_year: usize, years: usize, rng: &mut Rng) {
    let want = per_year * years;
    while nodes.len() < want {
        let r = libm::sqrt(-2.0 * libm::log(unit(rng)));
        let a = 2.0 * core::f64::consts::PI * unit(rng);
        nodes.push((r * libm::cos(a)) as f32);
        nodes.push((r * libm::sin(a)) as f32);
    }
    nodes.truncate(want);
}

/// Draw the run's weather (D75). Called once, with the run's fresh `Rng`.
pub fn init(state: &mut State, t: &Terrain, p: &Params, rng: &mut Rng) {
    if p.local_harvests {
        let per_year = node_count(t, p);
        draw(
            &mut state.harvest_nodes,
            per_year,
            p.harvest_years as usize,
            rng,
        );
    }
}

pub fn run(state: &mut State, t: &Terrain, p: &Params, rng: &mut Rng) {
    if !p.local_harvests {
        state.harvest.clear();
        return;
    }
    let g = &t.grid;
    let step = step_km(t, p);
    let (nx, ny) = (nodes(g.w, g.cell_km, step), nodes(g.h, g.cell_km, step));
    let per_year = nx * ny;
    let year = state.year as usize;
    draw(&mut state.harvest_nodes, per_year, year + 1, rng);
    let z = &state.harvest_nodes[year * per_year..(year + 1) * per_year];
    state.harvest.resize(t.n(), 1.0);
    let (w, h) = (g.w as usize, g.h as usize);
    for y in 0..h {
        let v = (y as f32 + 0.5) * g.cell_km / step;
        let (j, fy) = (v as usize, v - libm::floorf(v));
        let vy = fy * fy + (1.0 - fy) * (1.0 - fy);
        for x in 0..w {
            let u = (x as f32 + 0.5) * g.cell_km / step;
            let (k, fx) = (u as usize, u - libm::floorf(u));
            let at = |a: usize, b: usize| z[b.min(ny - 1) * nx + a.min(nx - 1)];
            let top = at(k, j) * (1.0 - fx) + at(k + 1, j) * fx;
            let bot = at(k, j + 1) * (1.0 - fx) + at(k + 1, j + 1) * fx;
            // Unit variance at every cell (D75): divide by the interpolation's sd.
            let sd = libm::sqrtf((fx * fx + (1.0 - fx) * (1.0 - fx)) * vy);
            let zz = (top * (1.0 - fy) + bot * fy) / sd;
            state.harvest[y * w + x] =
                (1.0 + p.harvest_sd * zz).clamp(p.harvest_min, p.harvest_max);
        }
    }
}
