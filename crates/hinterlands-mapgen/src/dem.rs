//! Real terrain: import an ESRI ASCII grid (e.g. SRTM resampled in GIS), resample
//! it to the chosen cell size, and run the hydrology and land-cover passes on it.

use crate::{blank, hydrology, land_cover, MapParams};
use hinterlands_core::grid::Grid;
use hinterlands_core::{rng_from_seed, Terrain};

pub fn import_dem(path: &str, cell_km: f32) -> Result<Terrain, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    import_asc(&text, cell_km)
}

/// Parse an `.asc` grid. `cellsize` is read in metres; cells at or below zero,
/// and NODATA cells, become sea.
pub fn import_asc(text: &str, cell_km: f32) -> Result<Terrain, String> {
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
    let w = ((ncols as f32 / scale).floor() as u32).max(1);
    let h = ((nrows as f32 / scale).floor() as u32).max(1);
    let mp = MapParams {
        w,
        h,
        cell_km,
        ..MapParams::default()
    };
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
    let mut rng = rng_from_seed(0);
    hydrology(&mut t, &mp, &mut rng);
    land_cover(&mut t, &mp, &mut rng);
    Ok(t)
}
