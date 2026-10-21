//! Scoring a run against a ground-truth file of historical settlements.
//!
//! Format: CSV with a header, columns `name,x_km,y_km,population,year`, where
//! coordinates are km from the top-left corner of the terrain grid.

use hinterlands_core::{Snapshot, Terrain};

pub struct Place {
    pub name: String,
    pub x_km: f32,
    pub y_km: f32,
    pub pop: f32,
}

pub fn parse(text: &str) -> Result<Vec<Place>, String> {
    let mut out = Vec::new();
    for (n, line) in text.lines().enumerate().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        if f.len() < 4 {
            return Err(format!(
                "truth line {}: expected name,x_km,y_km,population,year",
                n + 1
            ));
        }
        let num = |s: &str| {
            s.parse::<f32>()
                .map_err(|_| format!("truth line {}: bad number `{s}`", n + 1))
        };
        out.push(Place {
            name: f[0].to_string(),
            x_km: num(f[1])?,
            y_km: num(f[2])?,
            pop: num(f[3])?,
        });
    }
    Ok(out)
}

/// For each historical place, the distance to the nearest simulated settlement of
/// at least a fifth of its size; prints the hit rate within 5 and 10 km.
pub fn score(path: &str, t: &Terrain, snap: &Snapshot) {
    let text =
        std::fs::read_to_string(path).unwrap_or_else(|e| crate::die(&format!("{path}: {e}")));
    let places = parse(&text).unwrap_or_else(|e| crate::die(&e));
    let g = t.grid;
    let (mut hit5, mut hit10) = (0, 0);
    println!(
        "{:<24} {:>8} {:>10} {:>8}",
        "place", "pop", "nearest", "sim pop"
    );
    for p in &places {
        let best = snap
            .settlements
            .iter()
            .filter(|s| s.pop >= p.pop * 0.2)
            .map(|s| {
                let (x, y) = g.xy(s.anchor as usize);
                let dx = (x as f32 + 0.5) * g.cell_km - p.x_km;
                let dy = (y as f32 + 0.5) * g.cell_km - p.y_km;
                ((dx * dx + dy * dy).sqrt(), s.pop)
            })
            .min_by(|a, b| a.0.total_cmp(&b.0));
        let (d, sp) = best.unwrap_or((f32::INFINITY, 0.0));
        hit5 += (d <= 5.0) as u32;
        hit10 += (d <= 10.0) as u32;
        println!("{:<24} {:>8.0} {:>8.1}km {:>8.0}", p.name, p.pop, d, sp);
    }
    let n = places.len().max(1) as f32;
    println!(
        "within 5 km: {:.0}%   within 10 km: {:.0}%",
        100.0 * hit5 as f32 / n,
        100.0 * hit10 as f32 / n
    );
}

/// Write every settlement as CSV, `name,x_km,y_km,pop,founded,site`, with
/// coordinates of the anchor cell's centre in the same frame as the truth file.
pub fn dump(path: &str, t: &Terrain, snap: &Snapshot) {
    let g = t.grid;
    let mut out = String::from("name,x_km,y_km,pop,founded,site\n");
    for s in &snap.settlements {
        let i = s.anchor as usize;
        let (x, y) = g.xy(i);
        let site = if t.is_nav(i) || t.touches(i, |j| t.is_nav(j)) {
            "river"
        } else if t.is_coastal(i) {
            "coast"
        } else {
            "inland"
        };
        out += &format!(
            "{},{:.1},{:.1},{:.0},{},{}\n",
            s.name,
            (x as f32 + 0.5) * g.cell_km,
            (y as f32 + 0.5) * g.cell_km,
            s.pop,
            s.founded,
            site
        );
    }
    std::fs::write(path, out).unwrap_or_else(|e| crate::die(&format!("{path}: {e}")));
}

/// Write each cell's site class as CSV, `x,y,class,water,bridge`, with class one
/// of sea, river (on or beside a navigable river), coast or inland.
pub fn dump_cells(path: &str, t: &Terrain) {
    let g = t.grid;
    let mut out = String::from("x,y,class,water,bridge\n");
    for i in 0..g.n() {
        let (x, y) = g.xy(i);
        let class = if t.is_sea(i) {
            "sea"
        } else if t.is_nav(i) || t.touches(i, |j| t.is_nav(j)) {
            "river"
        } else if t.is_coastal(i) {
            "coast"
        } else {
            "inland"
        };
        let water = match t.water[i] {
            hinterlands_core::terrain::Water::None => "none",
            hinterlands_core::terrain::Water::Stream => "stream",
            hinterlands_core::terrain::Water::NavigableRiver => "navigable",
            hinterlands_core::terrain::Water::Sea => "sea",
        };
        out += &format!("{x},{y},{class},{water},{}\n", t.free_crossing(i) && !t.ford[i]);
    }
    std::fs::write(path, out).unwrap_or_else(|e| crate::die(&format!("{path}: {e}")));
}

#[cfg(test)]
mod tests {
    #[test]
    fn parses_rows() {
        let p = super::parse("name,x_km,y_km,population,year\nAshford, 10, 20.5, 3000, 1300\n")
            .unwrap();
        assert_eq!(p.len(), 1);
        assert_eq!(p[0].name, "Ashford");
        assert_eq!(p[0].y_km, 20.5);
    }
}
