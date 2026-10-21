//! Naming pass: find the rivers, mountain ranges, seas, bays and large forests,
//! and give each a name and a label position.

use crate::MapParams;
use hinterlands_core::names::{feature_name, Feature, FeatureKind};
use hinterlands_core::{Rng, Terrain};
use std::collections::{BTreeSet, VecDeque};

/// Connected groups (8-neighbour) of cells where `inside` holds.
fn components(t: &Terrain, inside: &dyn Fn(usize) -> bool) -> Vec<Vec<u32>> {
    let g = t.grid;
    let mut seen = vec![false; g.n()];
    let mut out = Vec::new();
    for start in 0..g.n() {
        if seen[start] || !inside(start) {
            continue;
        }
        let mut comp = Vec::new();
        let mut q = VecDeque::from([start]);
        seen[start] = true;
        while let Some(c) = q.pop_front() {
            comp.push(c as u32);
            for d in 0..8 {
                if let Some(j) = g.neighbor(c, d) {
                    if !seen[j] && inside(j) {
                        seen[j] = true;
                        q.push_back(j);
                    }
                }
            }
        }
        out.push(comp);
    }
    out
}

/// The member cell nearest the group's centroid.
fn centre(t: &Terrain, cells: &[u32]) -> u32 {
    let g = t.grid;
    let (sx, sy) = cells.iter().fold((0.0f32, 0.0f32), |a, &c| {
        let (x, y) = g.xy(c as usize);
        (a.0 + x as f32, a.1 + y as f32)
    });
    let (cx, cy) = (sx / cells.len() as f32, sy / cells.len() as f32);
    *cells
        .iter()
        .min_by(|&&a, &&b| {
            let d = |c: u32| {
                let (x, y) = g.xy(c as usize);
                (x as f32 - cx).powi(2) + (y as f32 - cy).powi(2)
            };
            d(a).total_cmp(&d(b)).then(a.cmp(&b))
        })
        .unwrap()
}

pub fn name_features(t: &mut Terrain, mp: &MapParams, _rng: &mut Rng) {
    let g = t.grid;
    let n = g.n();
    let key = |c: u32| c as u64 ^ (t.elevation[c as usize].to_bits() as u64) << 20;
    let mut taken: BTreeSet<String> = BTreeSet::new();
    let mut feats: Vec<Feature> = Vec::new();
    let add = |feats: &mut Vec<Feature>,
               taken: &mut BTreeSet<String>,
               kind,
               label: u32,
               size: f32,
               cells: Vec<u32>,
               k: u64| {
        let name = feature_name(kind, k, size, taken);
        taken.insert(name.clone());
        feats.push(Feature {
            kind,
            name,
            label_cell: label,
            size,
            cells,
        });
    };

    // Seas: large bodies of sea.
    for comp in components(t, &|i| t.is_sea(i)) {
        if comp.len() >= 300 {
            let label = centre(t, &comp);
            add(
                &mut feats,
                &mut taken,
                FeatureKind::Sea,
                label,
                comp.len() as f32,
                comp,
                key(label),
            );
        }
    }
    // Bays: sea cells mostly surrounded by land within 5 km.
    let enclosed: Vec<bool> = (0..n)
        .map(|i| {
            if !t.is_sea(i) {
                return false;
            }
            let (land, all) = g
                .disk(i, 5.0)
                .fold((0, 0), |a, c| (a.0 + t.is_land(c) as u32, a.1 + 1));
            land as f32 / all as f32 >= 0.5
        })
        .collect();
    for comp in components(t, &|i| enclosed[i]) {
        if comp.len() >= 8 {
            let label = centre(t, &comp);
            add(
                &mut feats,
                &mut taken,
                FeatureKind::Bay,
                label,
                comp.len() as f32,
                comp,
                key(label),
            );
        }
    }
    // Rivers: every stream reaching the sea with enough drainage; the main stem
    // is traced upstream along the largest tributary.
    let mut mouths: Vec<usize> = (0..n)
        .filter(|&i| {
            t.is_river(i)
                && t.downstream(i).is_some_and(|j| t.is_sea(j))
                && t.flux[i] >= mp.nav_area
        })
        .collect();
    mouths.sort_by(|&a, &b| t.flux[b].total_cmp(&t.flux[a]).then(a.cmp(&b)));
    for mouth in mouths {
        let mut stem = vec![mouth as u32];
        let mut c = mouth;
        loop {
            let up = (0..8)
                .filter_map(|d| g.neighbor(c, d))
                .filter(|&j| t.downstream(j) == Some(c) && t.flux[j] >= mp.stream_area)
                .max_by(|&a, &b| t.flux[a].total_cmp(&t.flux[b]).then(b.cmp(&a)));
            match up {
                Some(j) => {
                    stem.push(j as u32);
                    c = j;
                }
                None => break,
            }
        }
        if stem.len() >= 8 {
            let label = stem[stem.len() / 3];
            add(
                &mut feats,
                &mut taken,
                FeatureKind::River,
                label,
                t.flux[mouth],
                stem,
                key(mouth as u32),
            );
        }
    }
    // Ranges: high ground.
    for comp in components(t, &|i| t.is_land(i) && t.elevation[i] >= 650.0) {
        if comp.len() >= 25 {
            let label = centre(t, &comp);
            add(
                &mut feats,
                &mut taken,
                FeatureKind::Range,
                label,
                comp.len() as f32,
                comp,
                key(label),
            );
        }
    }
    // Forests: large stretches of dense natural woodland.
    for comp in components(t, &|i| t.is_land(i) && t.forest0[i] >= 0.9) {
        if comp.len() >= 60 {
            let label = centre(t, &comp);
            add(
                &mut feats,
                &mut taken,
                FeatureKind::Forest,
                label,
                comp.len() as f32,
                comp,
                key(label),
            );
        }
    }
    t.features = feats;
}
