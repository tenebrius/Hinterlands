//! Procedural names. A name is built from a hash of the place, so naming never
//! draws on the run's random generator and cannot change the simulation.
//! Settlement suffixes follow the site (-ford at a ford, -mouth at a river
//! mouth, -port on a harbour, -wick on a navigable river, -hurst in woodland,
//! -don on high ground); natural features get English-style forms
//! ("River Selwe", "the Marrow Fells", "Wenley Bay").

use crate::terrain::{Resource, Terrain};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

const STARTS: &[&str] = &[
    "Ash", "Brad", "Wick", "Thorn", "Ald", "Hal", "Mar", "Sel", "Wen", "Elm", "Brom", "Ox", "Kin",
    "Lang", "Nor", "Stan", "Dun", "Gil", "Hart", "Lin", "Mel", "Ro", "Sut", "Tal", "Wal", "Wy",
    "Ched", "Cal", "Bar", "Bel", "Cro", "Ew", "Fen", "Grim", "Hol", "Ick", "Kel", "Lud", "Mor",
    "Pen", "Ren", "Shap", "Tor", "Ul", "Ver", "Wex", "Yar", "Ast", "Bur", "Chal", "Dar", "Ever",
    "Fair", "Glen", "Hay", "Ley", "Mal", "Ned", "Oak", "Pick", "Ray", "Stow", "Thet", "Up", "Wins",
    "Wool", "Brack", "Cran", "Dew", "Fal", "Harn", "Isel", "Kirk", "Lox", "Mere", "Ord",
];
const MIDS: &[&str] = &["", "", "", "", "er", "en", "ing", "e", "an", "el", "es"];

const PLAIN: &[&str] = &[
    "ton", "by", "ham", "stead", "worth", "thorpe", "cote", "field", "well", "ington",
];
const WOODED: &[&str] = &["hurst", "ley", "wood", "den", "holt"];
const HIGH: &[&str] = &["don", "hope", "thwaite", "low", "combe"];
const RIVER: &[&str] = &["wick", "bourne", "ey", "ham", "ford"];
const COAST: &[&str] = &["port", "haven", "sea", "wick"];
const MOUTH: &[&str] = &["mouth", "mouth", "port"];
const FORD: &[&str] = &["ford"];
const MINE: &[&str] = &["dale", "pit", "stone"];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FeatureKind {
    Sea,
    Bay,
    River,
    Range,
    Forest,
}

/// A named natural feature, found at map generation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Feature {
    pub kind: FeatureKind,
    pub name: String,
    /// Where the label goes.
    pub label_cell: u32,
    /// Rank for label size: drainage km² for rivers, cells otherwise.
    pub size: f32,
    /// Rivers: the main stem from mouth upward. Others: the feature's cells.
    pub cells: Vec<u32>,
}

fn mix(mut h: u64) -> u64 {
    h ^= h >> 33;
    h = h.wrapping_mul(0xff51_afd7_ed55_8ccd);
    h ^= h >> 33;
    h = h.wrapping_mul(0xc4ce_b9fe_1a85_ec53);
    h ^ (h >> 33)
}

fn pick<'a>(list: &[&'a str], h: u64) -> &'a str {
    list[(h % list.len() as u64) as usize]
}

/// A root such as "Selwe" or "Marrow", from a hash.
pub fn root(h: u64) -> String {
    let a = pick(STARTS, mix(h));
    let b = pick(MIDS, mix(h ^ 0x9e37));
    let mut s = format!("{a}{b}");
    if s.len() < 3 {
        s.push_str(pick(&["e", "a", "ow", "en"], mix(h ^ 0x51)));
    }
    s
}

fn suffixes_for(t: &Terrain, cell: usize) -> &'static [&'static str] {
    let g = t.grid;
    if t.ford[cell] {
        return FORD;
    }
    let nav = t.touches(cell, |j| t.is_nav(j));
    let coastal = t.is_coastal(cell);
    if nav && coastal {
        return MOUTH;
    }
    if coastal {
        return COAST;
    }
    if nav {
        return RIVER;
    }
    if t.touches(cell, |j| t.resource[j] != Resource::None) {
        return MINE;
    }
    if t.elevation[cell] > 450.0 || t.slope[cell] > 0.06 {
        return HIGH;
    }
    let wood: f32 = g.disk(cell, 2.0).map(|c| t.forest0[c]).sum::<f32>()
        / g.disk(cell, 2.0).count().max(1) as f32;
    if wood > 0.75 {
        return WOODED;
    }
    PLAIN
}

/// A settlement name for `cell`, distinct from `taken`.
pub fn settlement_name(t: &Terrain, cell: usize, salt: u64, taken: &BTreeSet<String>) -> String {
    let suffixes = suffixes_for(t, cell);
    for k in 0..32u64 {
        let h = mix(cell as u64 ^ salt.wrapping_mul(0x2545_f491_4f6c_dd1d) ^ (k << 40));
        let mut r = root(h);
        let suf = pick(suffixes, mix(h ^ 0xabc));
        // Avoid doubled letters at the join, as in "Wickwick".
        if r.to_lowercase().ends_with(&suf[..1]) && suf.len() > 2 {
            r.pop();
        }
        let name = format!("{r}{suf}");
        if !taken.contains(&name) && !name.to_lowercase().contains(&suf.repeat(2)) {
            return name;
        }
    }
    format!("{} {}", root(mix(cell as u64 ^ salt)), cell)
}

/// A name for a natural feature, distinct from `taken`.
pub fn feature_name(kind: FeatureKind, key: u64, size: f32, taken: &BTreeSet<String>) -> String {
    for k in 0..32u64 {
        let r = root(mix(key ^ (kind as u64) << 56 ^ (k << 32)));
        let name = match kind {
            FeatureKind::River => format!("River {r}"),
            FeatureKind::Sea => format!("The {r} Sea"),
            FeatureKind::Bay => format!("{r} Bay"),
            FeatureKind::Range => {
                if size > 120.0 {
                    format!("The {r} Mountains")
                } else if size > 50.0 {
                    format!("The {r} Fells")
                } else {
                    format!("The {r} Hills")
                }
            }
            FeatureKind::Forest => {
                if size > 200.0 {
                    format!("{r} Forest")
                } else {
                    format!("{r} Wood")
                }
            }
        };
        if !taken.contains(&name) {
            return name;
        }
    }
    format!("{} {key}", root(key))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_stable_and_distinct() {
        let t = Terrain::flat(20, 20, 1.0);
        let mut taken = BTreeSet::new();
        for c in 0..50 {
            let n = settlement_name(&t, c, 7, &taken);
            assert_eq!(n, settlement_name(&t, c, 7, &taken));
            assert!(taken.insert(n));
        }
        let mut t2 = Terrain::flat(20, 20, 1.0);
        t2.ford[45] = true;
        t2.water[45] = crate::terrain::Water::Stream;
        assert!(settlement_name(&t2, 45, 7, &BTreeSet::new()).ends_with("ford"));
    }
}
