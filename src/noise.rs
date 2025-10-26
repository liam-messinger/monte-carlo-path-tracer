use crate::prelude::*;

const POINT_COUNT: usize = 256;

#[derive(Clone)]
pub struct Noise {
    randvec: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Noise {
    pub fn perlin() -> Self {
        let mut n = Noise {
            randvec: [Vec3::zero(); POINT_COUNT],
            perm_x: [0; POINT_COUNT],
            perm_y: [0; POINT_COUNT],
            perm_z: [0; POINT_COUNT],
        };
        for i in 0..POINT_COUNT {
            n.randvec[i] = Vec3::unit_vector(Vec3::random_range(-1.0, 1.0));
        }
        n.perm_x = Noise::perlin_generate_perm();
        n.perm_y = Noise::perlin_generate_perm();
        n.perm_z = Noise::perlin_generate_perm();
        n
    }

    pub fn value(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = (p.x()).floor() as i32;
        let j = (p.y()).floor() as i32;
        let k = (p.z()).floor() as i32;
        let mut c = [[[Vec3::zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randvec[
                        self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]
                    ];
                }
            }
        }

        Noise::perlin_interp(&c, u, v, w)
    }

    pub fn turbulence(&self, p: &Point3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.value(&temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }

        accum.abs()
    }

    fn perlin_generate_perm() -> [usize; POINT_COUNT] {
        let mut p: [usize; POINT_COUNT] = [0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            p[i] = i;
        }
        Noise::permute(&mut p, POINT_COUNT);
        p
    }

    fn permute(p: &mut [usize; POINT_COUNT], n: usize) {
        for i in (1..n).rev() {
            let target = random_usize_range(0, i);
            p.swap(i, target);
        }
    }

    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    let dot = Vec3::dot(&c[i][j][k], &weight_v);
                    accum += ((i as f64) * uu + ((1 - i) as f64) * (1.0 - uu)) *
                             ((j as f64) * vv + ((1 - j) as f64) * (1.0 - vv)) *
                             ((k as f64) * ww + ((1 - k) as f64) * (1.0 - ww)) * dot;
                }
            }
        }
        accum
    }
}