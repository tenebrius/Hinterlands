//! Real terrain: import an ESRI ASCII grid (e.g. SRTM resampled in GIS), resample
//! it to the chosen cell size, and run the hydrology and land-cover passes on it.
//! `MapParams` supplies the hydrology thresholds, edge inflows and the soil and
//! forest noise (zero for a fair test on real ground with no soil map); its
//! size and cell fields are taken from the grid.

use crate::{blank, hydrology, land_cover, MapParams};
use hinterlands_core::grid::Grid;
use hinterlands_core::{rng_from_seed, Terrain};

pub fn import_dem(path: &str, cell_km: f32, mp: &MapParams) -> Result<Terrain, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    let mut mp = mp.clone();
    if let Some(soil) = &mp.soil_file {
        let s = std::fs::read_to_string(soil).map_err(|e| format!("{soil}: {e}"))?;
        mp.soil_grid = read_grid(&s)?;
    }
    let mut t = import_asc(&text, cell_km, &mp)?;
    if let Some(roads) = &mp.roads_file {
        let s = std::fs::read_to_string(roads).map_err(|e| format!("{roads}: {e}"))?;
        let r = read_grid(&s)?;
        if r.len() != t.n() {
            return Err(format!(
                "road grid has {} cells, the map {}",
                r.len(),
                t.n()
            ));
        }
        t.initial_road = (0..t.n()).map(|i| r[i] > 0.5 && t.is_land(i)).collect();
        if mp.road_bridges {
            t.bridge = road_bridges(&t);
        }
    }
    if let Some(geo) = &mp.geology_file {
        let s = std::fs::read_to_string(geo).map_err(|e| format!("{geo}: {e}"))?;
        let g = read_grid(&s)?;
        if g.len() != t.n() {
            return Err(format!(
                "geology grid has {} cells, the map {}",
                g.len(),
                t.n()
            ));
        }
        for i in 0..t.n() {
            let k = g[i].round() as i32;
            let m = match k {
                1 => mp.geology_chalk,
                2 => mp.geology_marsh,
                _ => 1.0,
            };
            t.food_yield[i] *= m;
        }
    }
    if let Some(f) = &mp.outside_file {
        let s = std::fs::read_to_string(f).map_err(|e| format!("{f}: {e}"))?;
        t.outside_ports = read_outside(&t, &s)?;
    }
    if let Some(res) = &mp.resources_file {
        let s = std::fs::read_to_string(res).map_err(|e| format!("{res}: {e}"))?;
        place_resources(&mut t, &s)?;
    }
    Ok(t)
}

/// Resource sites from a CSV (`name,x_km,y_km,good`) in place of the random
/// ones (D50). A site that falls in the sea moves to the nearest land cell
/// within two cells; a site with none is dropped.
fn place_resources(t: &mut Terrain, text: &str) -> Result<(), String> {
    use hinterlands_core::terrain::Resource;
    let g = t.grid;
    t.resource = vec![Resource::None; t.n()];
    for line in text.lines().skip(1).filter(|l| !l.trim().is_empty()) {
        let f: Vec<&str> = line.split(',').map(str::trim).collect();
        let [_, x, y, good] = f[..] else {
            return Err(format!("resources: bad line {line}"));
        };
        let num = |v: &str| {
            v.parse::<f32>()
                .map_err(|_| format!("resources: bad number {v}"))
        };
        let kind = match good {
            "iron" => Resource::Ore,
            "salt" => Resource::Salt,
            "stone" => Resource::Stone,
            _ => return Err(format!("resources: unknown good {good}")),
        };
        let (cx, cy) = (
            (num(x)? / g.cell_km).floor() as i32,
            (num(y)? / g.cell_km).floor() as i32,
        );
        if cx < 0 || cy < 0 || cx >= g.w as i32 || cy >= g.h as i32 {
            continue;
        }
        let c = g.idx(cx, cy);
        let cell = if t.is_land(c) {
            Some(c)
        } else {
            g.disk(c, 2.0 * g.cell_km)
                .filter(|&j| t.is_land(j))
                .min_by(|&a, &b| g.dist_km(c, a).total_cmp(&g.dist_km(c, b)).then(a.cmp(&b)))
        };
        if let Some(j) = cell {
            t.resource[j] = kind;
        }
    }
    Ok(())
}

/// Outside ports from a CSV (`name,x_km,y_km`): each at the sea cell on the map
/// edge nearest the point (D64). Ties go to the lower cell.
fn read_outside(t: &Terrain, text: &str) -> Result<Vec<u32>, String> {
    let g = t.grid;
    let edge: Vec<usize> = (0..t.n())
        .filter(|&i| {
            let (x, y) = g.xy(i);
            t.is_sea(i) && (x == 0 || y == 0 || x as u32 == g.w - 1 || y as u32 == g.h - 1)
        })
        .collect();
    let mut out = Vec::new();
    for line in text.lines().skip(1).filter(|l| !l.trim().is_empty()) {
        let f: Vec<&str> = line.split(',').map(str::trim).collect();
        let [_, x, y] = f[..] else {
            return Err(format!("outside ports: bad line {line}"));
        };
        let num = |v: &str| {
            v.parse::<f32>()
                .map_err(|_| format!("outside ports: bad number {v}"))
        };
        let (px, py) = (num(x)? / g.cell_km, num(y)? / g.cell_km);
        let best = edge.iter().copied().min_by(|&a, &b| {
            let d = |i: usize| {
                let (cx, cy) = g.xy(i);
                (cx as f32 + 0.5 - px).powi(2) + (cy as f32 + 0.5 - py).powi(2)
            };
            d(a).total_cmp(&d(b)).then(a.cmp(&b))
        });
        if let Some(c) = best {
            if !out.contains(&(c as u32)) {
                out.push(c as u32);
            }
        }
    }
    Ok(out)
}

/// Bridges where an initial road crosses a river or stream (D33): every river
/// cell on the road, and, where the road steps diagonally between two linked
/// river cells without touching either, both of those cells.
fn road_bridges(t: &Terrain) -> Vec<bool> {
    let g = t.grid;
    let mut bridge: Vec<bool> = (0..t.n())
        .map(|i| t.initial_road[i] && t.is_river(i))
        .collect();
    for i in 0..t.n() {
        if !t.initial_road[i] {
            continue;
        }
        for d in [1usize, 3, 5, 7] {
            let Some(j) = g.neighbor(i, d) else { continue };
            if !t.initial_road[j] {
                continue;
            }
            let (dx, dy) = hinterlands_core::grid::DIRS[d];
            let (x, y) = g.xy(i);
            let (a, b) = (g.idx(x + dx, y), g.idx(x, y + dy));
            if t.is_river(a) && t.is_river(b) && t.flow_linked(a, b) {
                bridge[a] = true;
                bridge[b] = true;
            }
        }
    }
    bridge
}

/// The values of an ESRI ASCII grid, row by row from the north.
fn read_grid(text: &str) -> Result<Vec<f32>, String> {
    text.lines()
        .filter(|l| {
            l.trim_start()
                .starts_with(|c: char| c.is_ascii_digit() || c == '-')
        })
        .flat_map(|l| l.split_whitespace())
        .map(|v| {
            v.parse::<f32>()
                .map_err(|_| format!("soil grid: bad value {v}"))
        })
        .collect()
}

/// Parse an `.asc` grid. `cellsize` is read in metres; cells at or below zero,
/// and NODATA cells, become sea.
pub fn import_asc(text: &str, cell_km: f32, mp: &MapParams) -> Result<Terrain, String> {
    let mut lines = text.lines();
    let mut header = std::collections::BTreeMap::new();
    let mut first_data = None;
    for line in lines.by_ref() {
        let mut parts = line.split_whitespace();
        let Some(k) = parts.next() else { continue };
        if k.chars().next().is_some_and(|c| c.is_ascii_alphabetic()) {
            let v: f64 = parts
                .next()
                .ok_or("asc: header without value")?
                .parse()
                .map_err(|_| "asc: bad header")?;
            header.insert(k.to_ascii_lowercase(), v);
        } else {
            first_data = Some(line);
            break;
        }
    }
    let get = |k: &str| header.get(k).copied().ok_or(format!("asc: missing {k}"));
    let (ncols, nrows) = (get("ncols")? as usize, get("nrows")? as usize);
    let src_km = get("cellsize")? as f32 / 1000.0;
    let nodata = header.get("nodata_value").copied().unwrap_or(-9999.0) as f32;
    let mut values = Vec::with_capacity(ncols * nrows);
    for line in first_data.into_iter().chain(lines) {
        for tok in line.split_whitespace() {
            values.push(tok.parse::<f32>().map_err(|_| "asc: bad value")?);
        }
    }
    if values.len() != ncols * nrows {
        return Err(format!(
            "asc: expected {} values, found {}",
            ncols * nrows,
            values.len()
        ));
    }

    // Average the source cells that fall in each target cell.
    let scale = cell_km / src_km;
    // A whole number of target cells, allowing for a source cell size given
    // to the nearest decimetre (999.8 m is 1 km).
    let fit = |v: f32| {
        let r = v.round();
        if (v - r).abs() < 0.05 {
            r
        } else {
            v.floor()
        }
    };
    let w = (fit(ncols as f32 / scale) as u32).max(1);
    let h = (fit(nrows as f32 / scale) as u32).max(1);
    let mp = MapParams {
        w,
        h,
        cell_km,
        ..mp.clone()
    };
    if !mp.soil_grid.is_empty() && mp.soil_grid.len() != (w * h) as usize {
        return Err(format!(
            "soil grid has {} cells, the map {w} x {h}",
            mp.soil_grid.len()
        ));
    }
    let mut t = blank(&mp);
    t.grid = Grid::new(w, h, cell_km);
    for ty in 0..h {
        for tx in 0..w {
            let (x0, x1) = (
                (tx as f32 * scale) as usize,
                (((tx + 1) as f32 * scale) as usize).min(ncols),
            );
            let (y0, y1) = (
                (ty as f32 * scale) as usize,
                (((ty + 1) as f32 * scale) as usize).min(nrows),
            );
            let (mut sum, mut cnt) = (0.0, 0);
            for y in y0..y1.max(y0 + 1) {
                for x in x0..x1.max(x0 + 1) {
                    let v = values[y.min(nrows - 1) * ncols + x.min(ncols - 1)];
                    sum += if v == nodata { -1.0 } else { v };
                    cnt += 1;
                }
            }
            let e = sum / cnt as f32;
            t.elevation[t.grid.idx(tx as i32, ty as i32)] = if e <= 0.0 { e.min(-1.0) } else { e };
        }
    }
    // Sea is what joins the open sea at the map edge; low inland pockets (a
    // tidal river averaged over a cell, a marsh) are land just above sea level.
    let g = t.grid;
    let mut open = vec![false; g.n()];
    let mut stack: Vec<usize> = (0..g.n())
        .filter(|&i| {
            let (x, y) = g.xy(i);
            let edge = x == 0 || y == 0 || x as u32 == w - 1 || y as u32 == h - 1;
            edge && t.elevation[i] <= 0.0
        })
        .collect();
    while let Some(i) = stack.pop() {
        if open[i] {
            continue;
        }
        open[i] = true;
        stack.extend(
            (0..8)
                .filter_map(|d| g.neighbor(i, d))
                .filter(|&j| !open[j] && t.elevation[j] <= 0.0),
        );
    }
    for i in 0..g.n() {
        if !open[i] && t.elevation[i] <= 0.0 {
            t.elevation[i] = 0.5;
        }
    }
    let mut rng = rng_from_seed(0);
    hydrology(&mut t, &mp, &mut rng);
    land_cover(&mut t, &mp, &mut rng);
    crate::features::name_features(&mut t, &mp, &mut rng);
    Ok(t)
}
