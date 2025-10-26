use crate::prelude::*;

const POINT_COUNT: usize = 256;

#[derive(Clone)]
pub struct Noise {
    randfloat: [f64; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Noise {
    pub fn perlin() -> Self {
        let mut n = Noise {
            randfloat: [0.0; POINT_COUNT],
            perm_x: [0; POINT_COUNT],
            perm_y: [0; POINT_COUNT],
            perm_z: [0; POINT_COUNT],
        };
        for i in 0..POINT_COUNT {
            n.randfloat[i] = random_f64();
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
        let mut c = [[[0.0; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randfloat[
                        self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]
                    ];
                }
            }
        }

        Noise::trilinear_interp(&c, u, v, w)
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

    fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f64 * u + (1.0 - i as f64) * (1.0 - u))
                        * (j as f64 * v + (1.0 - j as f64) * (1.0 - v))
                        * (k as f64 * w + (1.0 - k as f64) * (1.0 - w))
                        * c[i][j][k];
                }
            }
        }
        accum
    }
}