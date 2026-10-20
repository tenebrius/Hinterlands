//! Cell indexing. Cells are numbered row-major; directions are the eight
//! compass neighbours, starting east and turning clockwise (y grows south).

use serde::{Deserialize, Serialize};

pub const DIRS: [(i32, i32); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

pub const NO_DIR: u8 = u8::MAX;

#[inline]
pub fn opposite(d: usize) -> usize {
    (d + 4) % 8
}

#[inline]
pub fn is_diagonal(d: usize) -> bool {
    d % 2 == 1
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Grid {
    pub w: u32,
    pub h: u32,
    /// Side of one cell in km. Every cost constant is per km and scaled by this.
    pub cell_km: f32,
}

impl Grid {
    pub fn new(w: u32, h: u32, cell_km: f32) -> Self {
        Self { w, h, cell_km }
    }

    #[inline]
    pub fn n(&self) -> usize {
        (self.w * self.h) as usize
    }

    #[inline]
    pub fn idx(&self, x: i32, y: i32) -> usize {
        (y as u32 * self.w + x as u32) as usize
    }

    #[inline]
    pub fn xy(&self, i: usize) -> (i32, i32) {
        ((i as u32 % self.w) as i32, (i as u32 / self.w) as i32)
    }

    #[inline]
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as u32) < self.w && (y as u32) < self.h
    }

    #[inline]
    pub fn neighbor(&self, i: usize, d: usize) -> Option<usize> {
        let (x, y) = self.xy(i);
        let (dx, dy) = DIRS[d];
        let (nx, ny) = (x + dx, y + dy);
        if self.in_bounds(nx, ny) {
            Some(self.idx(nx, ny))
        } else {
            None
        }
    }

    /// Length of a step in direction `d`, in km.
    #[inline]
    pub fn step_km(&self, d: usize) -> f32 {
        if is_diagonal(d) {
            self.cell_km * std::f32::consts::SQRT_2
        } else {
            self.cell_km
        }
    }

    /// Direction from `a` to an adjacent cell `b`, if they are neighbours.
    pub fn dir_between(&self, a: usize, b: usize) -> Option<usize> {
        let (ax, ay) = self.xy(a);
        let (bx, by) = self.xy(b);
        let (dx, dy) = (bx - ax, by - ay);
        DIRS.iter().position(|&d| d == (dx, dy))
    }

    /// Straight-line distance between cell centres in km.
    pub fn dist_km(&self, a: usize, b: usize) -> f32 {
        let (ax, ay) = self.xy(a);
        let (bx, by) = self.xy(b);
        let dx = (ax - bx) as f32;
        let dy = (ay - by) as f32;
        libm::sqrtf(dx * dx + dy * dy) * self.cell_km
    }

    /// Cells within `km` straight-line distance of `i`, including `i`, in row order.
    pub fn disk(&self, i: usize, km: f32) -> impl Iterator<Item = usize> + '_ {
        let r = (km / self.cell_km).floor() as i32;
        let (cx, cy) = self.xy(i);
        let r2 = (km / self.cell_km) * (km / self.cell_km);
        (cy - r..=cy + r).flat_map(move |y| {
            (cx - r..=cx + r).filter_map(move |x| {
                let dx = (x - cx) as f32;
                let dy = (y - cy) as f32;
                if self.in_bounds(x, y) && dx * dx + dy * dy <= r2 + 1e-4 {
                    Some(self.idx(x, y))
                } else {
                    None
                }
            })
        })
    }
}
