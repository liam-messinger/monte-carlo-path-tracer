use crate::prelude::*;

/// The number of random gradient vectors and permutation entries.
const POINT_COUNT: usize = 256;

/// A struct representing Perlin noise.
#[derive(Clone)]
pub struct Noise {
    randvec: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Noise {
    /// Constructor for Perlin noise.
    pub fn perlin() -> Self {
        let mut n = Noise {
            randvec: [Vec3::zero(); POINT_COUNT],
            perm_x: [0; POINT_COUNT],
            perm_y: [0; POINT_COUNT],
            perm_z: [0; POINT_COUNT],
        };
        for rv in n.randvec.iter_mut() {
            *rv = Vec3::unit_vector(&Vec3::random_range(-1.0, 1.0));
        }
        n.perm_x = Noise::perlin_generate_perm();
        n.perm_y = Noise::perlin_generate_perm();
        n.perm_z = Noise::perlin_generate_perm();
        n
    }

    /// Get the Perlin noise value at point p.
    pub fn value(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = (p.x()).floor() as i32;
        let j = (p.y()).floor() as i32;
        let k = (p.z()).floor() as i32;
        let mut c = [[[Vec3::zero(); 2]; 2]; 2];

        for (di, plane) in c.iter_mut().enumerate() {
            for (dj, row) in plane.iter_mut().enumerate() {
                for (dk, cell) in row.iter_mut().enumerate() {
                    *cell = self.randvec[
                        self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]
                    ];
                }
            }
        }

        Noise::perlin_interp(&c, u, v, w)
    }

    /// Get the turbulence value at point p with given depth.
    pub fn turbulence(&self, p: &Point3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.value(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }

    /// Generate a permutation array for Perlin noise.
    fn perlin_generate_perm() -> [usize; POINT_COUNT] {
        let mut p: [usize; POINT_COUNT] = [0; POINT_COUNT];
        for (i, slot) in p.iter_mut().enumerate() {
            *slot = i;
        }
        Noise::permute(&mut p, POINT_COUNT);
        p
    }

    /// Permute the given array in place.
    fn permute(p: &mut [usize; POINT_COUNT], n: usize) {
        for i in (1..n).rev() {
            let target = random_usize_range(0, i);
            p.swap(i, target);
        }
    }

    /// Trilinear interpolation for Perlin noise.
    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for (i, plane) in c.iter().enumerate() {
            let wu = if i == 1 { uu } else { 1.0 - uu };
            for (j, row) in plane.iter().enumerate() {
                let wv = if j == 1 { vv } else { 1.0 - vv };
                for (k, cell) in row.iter().enumerate() {
                    let ww_factor = if k == 1 { ww } else { 1.0 - ww };
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    let dot = Vec3::dot(cell, &weight_v);
                    accum += wu * wv * ww_factor * dot;
                }
            }
        }
        accum
    }
}