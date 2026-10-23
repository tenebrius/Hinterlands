//! The map's style close in (R12): how large roads, rivers, trees and buildings
//! are drawn against their true size, and how many people a house holds (so how
//! many houses a place has). True scale is the record of R9–R11; the storybook
//! preset draws them larger and fewer, in the manner of hand-drawn village maps.
//! Drawing only: nothing feeds back.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Look {
    /// Roads, tracks, streets and footpaths: width against true width.
    pub roads: f32,
    /// Rivers: width against true width.
    pub rivers: f32,
    /// Trees in woods, hedges and orchards: crown size (and spacing).
    pub trees: f32,
    /// Buildings, plots, churches, market places and walls.
    pub buildings: f32,
    /// People to a house (a household is 4.5).
    pub per_house: f32,
}

/// One slider: label, range, and the field it sets.
pub type Field = (&'static str, f32, f32, fn(&mut Look) -> &mut f32);

impl Look {
    pub const TRUE_SCALE: Look = Look {
        roads: 1.0,
        rivers: 1.0,
        trees: 1.0,
        buildings: 1.0,
        per_house: 4.5,
    };
    pub const STORYBOOK: Look = Look {
        roads: 2.5,
        rivers: 1.6,
        trees: 1.8,
        buildings: 2.0,
        per_house: 12.0,
    };

    pub const FIELDS: [Field; 5] = [
        ("Road width ×", 0.5, 5.0, |l| &mut l.roads),
        ("River width ×", 0.5, 4.0, |l| &mut l.rivers),
        ("Tree size ×", 0.5, 4.0, |l| &mut l.trees),
        ("Building size ×", 0.5, 4.0, |l| &mut l.buildings),
        ("People per house", 1.0, 40.0, |l| &mut l.per_house),
    ];

    /// Kept within the sliders' ranges.
    pub fn clamped(mut self) -> Self {
        for (_, lo, hi, f) in Self::FIELDS {
            let x = f(&mut self);
            *x = if x.is_finite() { x.clamp(lo, hi) } else { lo };
        }
        self
    }

    /// From `?look=`: `true`, `storybook`, or five numbers in the order of
    /// the sliders (roads, rivers, trees, buildings, people per house).
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "true" => Some(Self::TRUE_SCALE),
            "storybook" => Some(Self::STORYBOOK),
            _ => {
                let n: Vec<f32> = s.split(',').filter_map(|x| x.parse().ok()).collect();
                match n[..] {
                    [roads, rivers, trees, buildings, per_house] => Some(
                        Self {
                            roads,
                            rivers,
                            trees,
                            buildings,
                            per_house,
                        }
                        .clamped(),
                    ),
                    _ => None,
                }
            }
        }
    }

    /// A key that changes whenever the look does, for caches.
    pub fn key(&self) -> [u32; 5] {
        [
            self.roads.to_bits(),
            self.rivers.to_bits(),
            self.trees.to_bits(),
            self.buildings.to_bits(),
            self.per_house.to_bits(),
        ]
    }
}

impl Default for Look {
    fn default() -> Self {
        Self::TRUE_SCALE
    }
}
