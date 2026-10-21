//! Map generation: a pipeline of passes over a `Terrain`, each
//! `fn(&mut Terrain, &MapParams, &mut Rng)`. v1 has heightmap, thermal erosion,
//! hydrology and land cover (yield, forest, resources, harbours). Later passes
//! slot into `PIPELINE` without touching anything else.

mod dem;
mod features;
mod noise;

pub use dem::{import_asc, import_dem};

use hinterlands_core::grid::{opposite, Grid, NO_DIR};
use hinterlands_core::terrain::{Resource, Terrain, Water};
use hinterlands_core::{rng_from_seed, Rng};
use noise::Perlin;
use rand::Rng as _;
use serde::{Deserialize, Serialize};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MapParams {
    pub w: u32,
    pub h: u32,
    pub cell_km: f32,
    /// Highest inland relief, metres.
    pub max_elev: f32,
    /// Share of the map's depth, from the sea edge, that is sea.
    pub coast_frac: f32,
    pub erosion_iters: u32,
    /// Rounds of stream-power incision.
    pub incision_iters: u32,
    /// Incision rate: share of a cell's drop removed per round per sqrt(km²) of drainage.
    pub incision_k: f32,
    /// Steepest stable slope for thermal erosion, m/m.
    pub talus: f32,
    /// Drainage area (km²) above which a cell carries a stream.
    pub stream_area: f32,
    /// Drainage area (km²) above which a river can be navigable.
    pub nav_area: f32,
    /// Steepest river gradient that is navigable, m/m.
    pub nav_max_slope: f32,
    /// Drainage area (km²) above which a river cannot be forded.
    pub ford_max_area: f32,
    /// Steepest bank slope at a ford, m/m.
    pub ford_max_slope: f32,
    pub n_ore: u32,
    pub n_salt: u32,
    pub n_stone: u32,
    /// Amplitude of the noise in soil yield (0 gives uniform soil, as on an
    /// imported DEM with no soil map).
    pub soil_noise: f32,
    /// Amplitude of the noise in natural forest cover.
    pub forest_noise: f32,
    /// Drainage entering across the map edge: `[x, y, km²]` added at that cell,
    /// for rivers whose catchment lies mostly off an imported map.
    pub inflows: Vec<[f32; 3]>,
    /// An ESRI ASCII grid of soil quality on the map's grid (e.g. from
    /// `tools/real/fetch_soil.py`), read by the DEM importer. When set, it
    /// replaces the soil noise, rescaled to the same mean yield.
    pub soil_file: Option<String>,
    /// The soil grid itself, filled from `soil_file` by the importer.
    #[serde(skip)]
    pub soil_grid: Vec<f32>,
    /// Weight of valley position in soil (a topographic position index: height
    /// above the mean within `tpi_radius_km`). Valley floors, standing in for
    /// alluvial soils, gain yield and spurs lose it; 0 turns it off.
    pub soil_tpi: f32,
    pub tpi_radius_km: f32,
    /// Height difference, m, at which the valley-position effect is full.
    pub tpi_scale_m: f32,
    /// An ESRI ASCII 0/1 grid of roads present at the start (e.g. Roman roads
    /// from `tools/real/fetch_roads.py`), read by the DEM importer (D32).
    pub roads_file: Option<String>,
}

impl Default for MapParams {
    fn default() -> Self {
        Self {
            w: 128,
            h: 128,
            cell_km: 1.0,
            max_elev: 1300.0,
            coast_frac: 0.16,
            erosion_iters: 30,
            incision_iters: 25,
            incision_k: 0.01,
            talus: 0.08,
            stream_area: 60.0,
            nav_area: 200.0,
            nav_max_slope: 0.01,
            ford_max_area: 500.0,
            ford_max_slope: 0.02,
            n_ore: 6,
            n_salt: 4,
            n_stone: 5,
            soil_noise: 0.45,
            forest_noise: 0.5,
            inflows: Vec::new(),
            soil_file: None,
            soil_grid: Vec::new(),
            soil_tpi: 0.0,
            tpi_radius_km: 3.0,
            tpi_scale_m: 25.0,
            roads_file: None,
        }
    }
}

pub type Pass = fn(&mut Terrain, &MapParams, &mut Rng);

/// The v1 pipeline, in order.
pub const PIPELINE: &[(&str, Pass)] = &[
    ("heightmap", heightmap),
    ("thermal_erosion", thermal_erosion),
    ("fluvial_incision", fluvial_incision),
    ("hydrology", hydrology),
    ("land_cover", land_cover),
    ("features", features::name_features),
];

pub fn blank(mp: &MapParams) -> Terrain {
    let grid = Grid::new(mp.w, mp.h, mp.cell_km);
    let n = grid.n();
    Terrain {
        grid,
        elevation: vec![0.0; n],
        slope: vec![0.0; n],
        water: vec![Water::None; n],
        flux: vec![0.0; n],
        flow_dir: vec![NO_DIR; n],
        food_yield: vec![0.0; n],
        forest0: vec![0.0; n],
        resource: vec![Resource::None; n],
        harbour: vec![false; n],
        ford: vec![false; n],
        river_mouth: None,
        features: Vec::new(),
        initial_road: Vec::new(),
        bridge: Vec::new(),
    }
}

pub fn generate(seed: u64, mp: &MapParams) -> Terrain {
    let mut rng = rng_from_seed(seed ^ 0x6d61_7067_656e);
    let mut t = blank(mp);
    for (_, pass) in PIPELINE {
        pass(&mut t, mp, &mut rng);
    }
    t
}

fn smoothstep(a: f32, b: f32, x: f32) -> f32 {
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Fractal noise relief rising away from one sea edge, with raised borders on
/// the other three so drainage runs to the sea.
pub fn heightmap(t: &mut Terrain, mp: &MapParams, rng: &mut Rng) {
    let g = t.grid;
    let sea_side: u32 = rng.gen_range(0..4);
    let relief = Perlin::new(rng);
    let warp = Perlin::new(rng);
    let ridge = Perlin::new(rng);
    let (w, h) = (g.w as f32, g.h as f32);
    for i in 0..g.n() {
        let (x, y) = g.xy(i);
        let (u, v) = ((x as f32 + 0.5) / w, (y as f32 + 0.5) / h);
        // Distance inland from the sea edge, 0..1, and along the coast.
        let (inland, along) = match sea_side {
            0 => (u, v),
            1 => (1.0 - u, v),
            2 => (v, u),
            _ => (1.0 - v, u),
        };
        let wx = warp.fbm(u * 2.0, v * 2.0, 3) * 0.2;
        let coast = inland + wx + 0.1 * warp.fbm(along * 4.0, 0.5, 4);
        let land = coast - mp.coast_frac;
        if land < 0.0 {
            t.elevation[i] = (land * 400.0).min(-1.0);
            continue;
        }
        let inl = land / (1.0 - mp.coast_frac);
        // A gentle rise inland, broad basins and ridges from low-frequency noise
        // (so drainage converges into a few large valleys), hills on top, and a
        // mountain belt toward the far side.
        let base = libm::powf(inl, 1.3) * 0.2;
        let basins = (0.5 + 0.8 * relief.fbm(u * 2.2 + 3.1, v * 2.2 - 1.7, 4)).max(0.0);
        let hills = 0.5 + 0.5 * relief.fbm(u * 6.0 + wx, v * 6.0 - wx, 4);
        let mount = 1.0 - libm::fabsf(ridge.fbm(u * 3.0, v * 3.0, 5));
        let mount = mount * mount * smoothstep(0.5, 0.95, inland);
        // Borders other than the sea side rise into hills.
        let edge = match sea_side {
            0 | 1 => v.min(1.0 - v),
            _ => u.min(1.0 - u),
        };
        let border = (1.0 - smoothstep(0.0, 0.08, edge)) * 0.3;
        let lowland = smoothstep(0.0, 0.12, land);
        let e = base + 0.3 * basins * (0.25 + inl) + 0.08 * hills + 0.5 * mount + border * lowland;
        t.elevation[i] = (e * mp.max_elev * lowland.max(0.08)).max(1.0);
    }
}

/// Material slides from cells steeper than the talus slope to their lowest neighbour.
pub fn thermal_erosion(t: &mut Terrain, mp: &MapParams, _rng: &mut Rng) {
    let g = t.grid;
    let n = g.n();
    let mut delta = vec![0f32; n];
    for _ in 0..mp.erosion_iters {
        delta.iter_mut().for_each(|d| *d = 0.0);
        for i in 0..n {
            if t.elevation[i] <= 0.0 {
                continue;
            }
            let mut best = None;
            let mut best_s = mp.talus;
            for d in 0..8 {
                let Some(j) = g.neighbor(i, d) else { continue };
                let s = (t.elevation[i] - t.elevation[j]) / (g.step_km(d) * 1000.0);
                if s > best_s {
                    best_s = s;
                    best = Some((j, d));
                }
            }
            if let Some((j, d)) = best {
                let moved = 0.25 * (best_s - mp.talus) * g.step_km(d) * 1000.0;
                delta[i] -= moved;
                delta[j] += moved;
            }
        }
        for i in 0..n {
            if t.elevation[i] > 0.0 {
                t.elevation[i] = (t.elevation[i] + delta[i]).max(1.0);
            }
        }
    }
}

/// Stream-power incision (a simple form of the hydraulic erosion on the map
/// track, landed early): rivers cut down in proportion to sqrt(drainage) times
/// their drop, and a slide pass slopes the valley sides, so tributaries are
/// captured into a few main valleys instead of running parallel to the sea.
pub fn fluvial_incision(t: &mut Terrain, mp: &MapParams, rng: &mut Rng) {
    let g = t.grid;
    let n = g.n();
    let slide = MapParams {
        erosion_iters: 2,
        ..mp.clone()
    };
    for _ in 0..mp.incision_iters {
        fill_and_route(t, &[]);
        let mut cut = vec![0f32; n];
        for i in 0..n {
            let Some(j) = t.downstream(i) else { continue };
            let drop = t.elevation[i] - t.elevation[j].max(0.0);
            if drop > 0.0 {
                cut[i] = (mp.incision_k * libm::sqrtf(t.flux[i]) * drop).min(0.9 * drop);
            }
        }
        for i in 0..n {
            if t.elevation[i] > 0.0 {
                t.elevation[i] = (t.elevation[i] - cut[i]).max(1.0);
            }
        }
        thermal_erosion(t, &slide, rng);
    }
}

/// Fill depressions, set flow directions and accumulate drainage area.
fn fill_and_route(t: &mut Terrain, inflows: &[[f32; 3]]) {
    let g = t.grid;
    let n = g.n();
    for i in 0..n {
        t.water[i] = if t.elevation[i] <= 0.0 {
            Water::Sea
        } else {
            Water::None
        };
        t.flux[i] = 0.0;
    }
    fill_depressions(t);
    flow_directions(t);
    let area = g.cell_km * g.cell_km;
    let mut order: Vec<usize> = (0..n).filter(|&i| t.water[i] != Water::Sea).collect();
    order.sort_by(|&a, &b| t.elevation[b].total_cmp(&t.elevation[a]).then(a.cmp(&b)));
    for &i in &order {
        t.flux[i] += area * (1.0 + t.elevation[i] / 1500.0);
    }
    for &[x, y, km2] in inflows {
        if g.in_bounds(x as i32, y as i32) {
            t.flux[g.idx(x as i32, y as i32)] += km2;
        }
    }
    for &i in &order {
        if let Some(j) = t.downstream(i) {
            if t.water[j] != Water::Sea {
                t.flux[j] += t.flux[i];
            }
        }
    }
}

/// Priority-flood depression filling (Barnes et al. 2014) so every land cell
/// drains to the sea, D8 flow directions, drainage accumulation, and the water
/// classes: streams, navigable rivers, fords and the main river mouth.
pub fn hydrology(t: &mut Terrain, mp: &MapParams, _rng: &mut Rng) {
    let g = t.grid;
    let n = g.n();
    let cell_m = g.cell_km * 1000.0;
    fill_and_route(t, &mp.inflows);
    let mut order: Vec<usize> = (0..n).filter(|&i| t.water[i] != Water::Sea).collect();
    order.sort_by(|&a, &b| t.elevation[b].total_cmp(&t.elevation[a]).then(a.cmp(&b)));

    // Slope by central differences, m/m.
    for i in 0..n {
        let (x, y) = g.xy(i);
        let e = |xx: i32, yy: i32| {
            let xx = xx.clamp(0, g.w as i32 - 1);
            let yy = yy.clamp(0, g.h as i32 - 1);
            t.elevation[g.idx(xx, yy)].max(0.0)
        };
        let gx = (e(x + 1, y) - e(x - 1, y)) / (2.0 * cell_m);
        let gy = (e(x, y + 1) - e(x, y - 1)) / (2.0 * cell_m);
        t.slope[i] = if t.water[i] == Water::Sea {
            0.0
        } else {
            libm::sqrtf(gx * gx + gy * gy)
        };
    }

    // Streams, then navigable reaches grown upstream from the sea.
    for &i in &order {
        if t.flux[i] >= mp.stream_area {
            t.water[i] = Water::Stream;
        }
    }
    for &i in order.iter().rev() {
        if t.flux[i] < mp.nav_area {
            continue;
        }
        let Some(j) = t.downstream(i) else { continue };
        let d = t.flow_dir[i] as usize;
        let gradient = (t.elevation[i] - t.elevation[j].max(0.0)) / (g.step_km(d) * 1000.0);
        if matches!(t.water[j], Water::Sea | Water::NavigableRiver) && gradient <= mp.nav_max_slope
        {
            t.water[i] = Water::NavigableRiver;
        }
    }

    // Fords: fordable rivers whose banks are low on both sides.
    for i in 0..n {
        t.ford[i] = false;
        if !t.is_river(i) || t.flux[i] > mp.ford_max_area || t.flow_dir[i] == NO_DIR {
            continue;
        }
        let d = t.flow_dir[i] as usize;
        let banks = [(d + 2) % 8, opposite((d + 2) % 8)];
        t.ford[i] = banks.iter().all(|&b| {
            g.neighbor(i, b)
                .is_some_and(|j| t.is_land(j) && !t.is_river(j) && t.slope[j] <= mp.ford_max_slope)
        });
    }

    t.river_mouth = (0..n)
        .filter(|&i| t.is_nav(i) && t.downstream(i).is_some_and(|j| t.is_sea(j)))
        .max_by(|&a, &b| t.flux[a].total_cmp(&t.flux[b]).then(b.cmp(&a)))
        .map(|i| i as u32);
}

fn fill_depressions(t: &mut Terrain) {
    let g = t.grid;
    let n = g.n();
    // Fill from the sea inwards; every land cell ends up just above its outlet.
    let mut done = vec![false; n];
    let mut heap: BinaryHeap<Reverse<(u32, u32)>> = BinaryHeap::new();
    let key = |e: f32| e.max(0.0).to_bits();
    for i in 0..n {
        if t.water[i] == Water::Sea {
            done[i] = true;
            heap.push(Reverse((0, i as u32)));
        }
    }
    if heap.is_empty() {
        // No sea: drain off the lowest border cell.
        let lowest = (0..n)
            .filter(|&i| {
                let (x, y) = g.xy(i);
                x == 0 || y == 0 || x as u32 == g.w - 1 || y as u32 == g.h - 1
            })
            .min_by(|&a, &b| t.elevation[a].total_cmp(&t.elevation[b]).then(a.cmp(&b)))
            .unwrap_or(0);
        done[lowest] = true;
        heap.push(Reverse((key(t.elevation[lowest]), lowest as u32)));
    }
    while let Some(Reverse((_, u))) = heap.pop() {
        let u = u as usize;
        for d in 0..8 {
            let Some(v) = g.neighbor(u, d) else { continue };
            if done[v] {
                continue;
            }
            done[v] = true;
            // A jittered epsilon keeps filled flats from draining in straight lines.
            let jitter = (hinterlands_core::fnv1a(&(v as u32).to_le_bytes()) % 100) as f32 / 100.0;
            let floor = t.elevation[u].max(0.0) + 0.01 * (0.5 + jitter) * g.step_km(d);
            if t.elevation[v] < floor {
                t.elevation[v] = floor;
            }
            heap.push(Reverse((key(t.elevation[v]), v as u32)));
        }
    }
}

fn flow_directions(t: &mut Terrain) {
    let g = t.grid;
    let n = g.n();
    // Steepest descent; ties go to the lowest direction index.
    for i in 0..n {
        t.flow_dir[i] = NO_DIR;
        if t.water[i] == Water::Sea {
            continue;
        }
        let mut best = 0.0;
        for d in 0..8 {
            let Some(j) = g.neighbor(i, d) else { continue };
            let drop = (t.elevation[i] - t.elevation[j].max(0.0)) / g.step_km(d);
            if drop > best {
                best = drop;
                t.flow_dir[i] = d as u8;
            }
        }
    }
}

/// Yield, natural forest, point resources and harbours.
pub fn land_cover(t: &mut Terrain, mp: &MapParams, rng: &mut Rng) {
    let g = t.grid;
    let n = g.n();
    let soil = Perlin::new(rng);
    let wood = Perlin::new(rng);
    let (w, h) = (g.w as f32, g.h as f32);
    // Soil before slope and height: noise, or a soil grid, optionally shaded by
    // valley position; a grid or valley term is rescaled to the noise-free mean.
    let mut base: Vec<f32> = (0..n)
        .map(|i| {
            let (x, y) = g.xy(i);
            let (u, v) = (x as f32 / w, y as f32 / h);
            match mp.soil_grid.get(i) {
                Some(&q) if q > 0.0 => q,
                _ => (0.6 + mp.soil_noise * soil.fbm(u * 5.0, v * 5.0, 4)).clamp(0.15, 1.0),
            }
        })
        .collect();
    if mp.soil_tpi > 0.0 {
        let tpi: Vec<f32> = (0..n)
            .map(|i| {
                let (sum, cnt) = g.disk(i, mp.tpi_radius_km).fold((0.0, 0.0), |(s, c), j| {
                    (s + t.elevation[j].max(0.0), c + 1.0)
                });
                t.elevation[i].max(0.0) - sum / cnt
            })
            .collect();
        for i in 0..n {
            base[i] *= 1.0 + mp.soil_tpi * (-tpi[i] / mp.tpi_scale_m).clamp(-1.0, 1.0);
        }
    }
    if !mp.soil_grid.is_empty() || mp.soil_tpi > 0.0 {
        let land: Vec<usize> = (0..n).filter(|&i| !t.is_sea(i)).collect();
        let mean = land.iter().map(|&i| base[i]).sum::<f32>() / land.len().max(1) as f32;
        for &i in &land {
            base[i] = (base[i] * 0.6 / mean.max(1e-6)).clamp(0.05, 1.0);
        }
    }
    for i in 0..n {
        if t.is_sea(i) {
            t.food_yield[i] = 0.0;
            t.forest0[i] = 0.0;
            continue;
        }
        let (x, y) = g.xy(i);
        let (u, v) = (x as f32 / w, y as f32 / h);
        let e = t.elevation[i];
        let s = t.slope[i];
        let base = base[i];
        let slope_f = (1.0 - s / 0.15).clamp(0.0, 1.0);
        let elev_f = libm::powf((1.0 - e / 1100.0).clamp(0.0, 1.0), 0.8);
        let near_river = t.touches(i, |j| t.is_river(j));
        let flood = if near_river && s < 0.03 { 0.25 } else { 0.0 };
        t.food_yield[i] = (base * slope_f * elev_f + flood * slope_f).clamp(0.0, 1.0);
        let cover = 0.8 + mp.forest_noise * wood.fbm(u * 4.0 + 7.0, v * 4.0, 4);
        let alpine = 1.0 - smoothstep(800.0, 1300.0, e) * 0.8;
        t.forest0[i] = (cover * alpine).clamp(0.1, 1.0);
    }

    for i in 0..n {
        t.harbour[i] = false;
        t.resource[i] = Resource::None;
    }
    for i in 0..n {
        if t.is_land(i) {
            let seas = (0..8)
                .filter(|&d| g.neighbor(i, d).is_some_and(|j| t.is_sea(j)))
                .count();
            t.harbour[i] = (1..=2).contains(&seas);
        }
    }

    let mut place =
        |t: &mut Terrain, kind: Resource, count: u32, ok: &dyn Fn(&Terrain, usize) -> bool| {
            let cells: Vec<usize> = (0..n)
                .filter(|&i| t.resource[i] == Resource::None && ok(t, i))
                .collect();
            for _ in 0..count {
                if cells.is_empty() {
                    return;
                }
                let c = cells[rng.gen_range(0..cells.len() as u32) as usize];
                t.resource[c] = kind;
            }
        };
    place(t, Resource::Ore, mp.n_ore, &|t, i| {
        t.is_land(i) && t.elevation[i] > 350.0 && t.slope[i] > 0.03
    });
    place(t, Resource::Salt, mp.n_salt, &|t, i| t.is_coastal(i));
    place(t, Resource::Stone, mp.n_stone, &|t, i| {
        t.is_land(i) && t.slope[i] > 0.02 && (150.0..700.0).contains(&t.elevation[i])
    });
}
