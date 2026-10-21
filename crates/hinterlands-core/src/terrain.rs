//! The static world produced by map generation. The yearly tick never writes it.

use crate::grid::{Grid, NO_DIR};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Water {
    None,
    Stream,
    NavigableRiver,
    Sea,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Resource {
    None,
    Ore,
    Salt,
    Stone,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Terrain {
    pub grid: Grid,
    /// Metres above sea level; negative is sea depth.
    pub elevation: Vec<f32>,
    /// Steepest local gradient, m/m.
    pub slope: Vec<f32>,
    pub water: Vec<Water>,
    /// Accumulated upstream drainage, in cell-rain units.
    pub flux: Vec<f32>,
    /// D8 downstream direction, or `NO_DIR` at sea and outlets.
    pub flow_dir: Vec<u8>,
    /// Food output of a cultivated cell as a share of prime land.
    pub food_yield: Vec<f32>,
    /// Natural (climax) forest cover; the initial forest of every run.
    pub forest0: Vec<f32>,
    pub resource: Vec<Resource>,
    pub harbour: Vec<bool>,
    /// River cells that can be crossed on foot at no extra cost.
    pub ford: Vec<bool>,
    /// The navigable-river cell with the largest flux that discharges to sea.
    pub river_mouth: Option<u32>,
    /// Named rivers, ranges, seas, bays and forests.
    pub features: Vec<crate::names::Feature>,
    /// Roads that exist before the run starts (e.g. Roman roads on an imported
    /// map), as input data like the DEM; empty on generated maps. Left out of
    /// the serialised terrain when empty, so generated maps keep their hash.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub initial_road: Vec<bool>,
    /// River and stream cells with a bridge, crossed at no extra cost as at a
    /// ford: input data (Roman roads' crossings, D33); empty on generated maps
    /// and left out of the serialised terrain when empty.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bridge: Vec<bool>,
}

impl Terrain {
    /// Flat, dry, prime land with no forest: a base for hand-built test maps.
    pub fn flat(w: u32, h: u32, cell_km: f32) -> Self {
        let grid = Grid::new(w, h, cell_km);
        let n = grid.n();
        Self {
            grid,
            elevation: vec![10.0; n],
            slope: vec![0.0; n],
            water: vec![Water::None; n],
            flux: vec![1.0; n],
            flow_dir: vec![NO_DIR; n],
            food_yield: vec![1.0; n],
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

    pub fn n(&self) -> usize {
        self.grid.n()
    }

    #[inline]
    pub fn is_sea(&self, i: usize) -> bool {
        self.water[i] == Water::Sea
    }

    #[inline]
    pub fn is_land(&self, i: usize) -> bool {
        self.water[i] != Water::Sea
    }

    #[inline]
    pub fn is_nav(&self, i: usize) -> bool {
        self.water[i] == Water::NavigableRiver
    }

    #[inline]
    pub fn is_river(&self, i: usize) -> bool {
        matches!(self.water[i], Water::Stream | Water::NavigableRiver)
    }

    /// The downstream neighbour of `i`, if any.
    #[inline]
    pub fn downstream(&self, i: usize) -> Option<usize> {
        let d = self.flow_dir[i];
        if d == NO_DIR {
            None
        } else {
            self.grid.neighbor(i, d as usize)
        }
    }

    /// True if `a` flows into `b` or `b` into `a`.
    #[inline]
    /// A crossing here costs nothing: a ford or a bridge.
    pub fn free_crossing(&self, i: usize) -> bool {
        self.ford[i] || self.bridge.get(i).copied().unwrap_or(false)
    }

    pub fn flow_linked(&self, a: usize, b: usize) -> bool {
        self.downstream(a) == Some(b) || self.downstream(b) == Some(a)
    }

    /// Land cell with a sea neighbour.
    pub fn is_coastal(&self, i: usize) -> bool {
        self.is_land(i) && (0..8).any(|d| self.grid.neighbor(i, d).is_some_and(|j| self.is_sea(j)))
    }

    pub fn touches(&self, i: usize, pred: impl Fn(usize) -> bool) -> bool {
        pred(i) || (0..8).any(|d| self.grid.neighbor(i, d).is_some_and(&pred))
    }

    /// Any river, stream or sea within `km` of `i`.
    pub fn water_within(&self, i: usize, km: f32) -> bool {
        self.grid.disk(i, km).any(|j| self.water[j] != Water::None)
    }

    /// A stable hash of the terrain, used to key snapshots to the terrain they belong to.
    pub fn hash(&self) -> u64 {
        crate::fnv1a(&bincode::serialize(self).expect("terrain serialises"))
    }
}
