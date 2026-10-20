//! 2D gradient noise (Perlin 2002) with a seeded permutation table.

use hinterlands_core::Rng;
use rand::Rng as _;

pub struct Perlin {
    perm: [u8; 512],
}

const GRAD: [(f32, f32); 8] = [
    (1.0, 0.0),
    (-1.0, 0.0),
    (0.0, 1.0),
    (0.0, -1.0),
    (0.70710677, 0.70710677),
    (-0.70710677, 0.70710677),
    (0.70710677, -0.70710677),
    (-0.70710677, -0.70710677),
];

impl Perlin {
    pub fn new(rng: &mut Rng) -> Self {
        let mut p: Vec<u8> = (0..=255).collect();
        for i in (1..256).rev() {
            let j = rng.gen_range(0..=i as u32) as usize;
            p.swap(i, j);
        }
        let mut perm = [0u8; 512];
        for i in 0..512 {
            perm[i] = p[i & 255];
        }
        Self { perm }
    }

    fn grad(&self, xi: i32, yi: i32, x: f32, y: f32) -> f32 {
        let h = self.perm[(self.perm[(xi & 255) as usize] as i32 + (yi & 255)) as usize] & 7;
        let (gx, gy) = GRAD[h as usize];
        gx * x + gy * y
    }

    /// Roughly in [-1, 1].
    pub fn noise(&self, x: f32, y: f32) -> f32 {
        let xf = libm::floorf(x);
        let yf = libm::floorf(y);
        let (xi, yi) = (xf as i32, yf as i32);
        let (x, y) = (x - xf, y - yf);
        let fade = |t: f32| t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
        let (u, v) = (fade(x), fade(y));
        let a = self.grad(xi, yi, x, y);
        let b = self.grad(xi + 1, yi, x - 1.0, y);
        let c = self.grad(xi, yi + 1, x, y - 1.0);
        let d = self.grad(xi + 1, yi + 1, x - 1.0, y - 1.0);
        let ab = a + u * (b - a);
        let cd = c + u * (d - c);
        (ab + v * (cd - ab)) * 1.4
    }

    /// Fractal sum of `octaves` layers, halving amplitude and doubling frequency.
    pub fn fbm(&self, x: f32, y: f32, octaves: u32) -> f32 {
        let (mut sum, mut amp, mut freq, mut norm) = (0.0, 1.0, 1.0, 0.0);
        for _ in 0..octaves {
            sum += amp * self.noise(x * freq, y * freq);
            norm += amp;
            amp *= 0.5;
            freq *= 2.0;
        }
        sum / norm
    }
}
