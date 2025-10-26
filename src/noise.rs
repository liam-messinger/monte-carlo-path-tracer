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
        let i = (4.0 * p.x()).floor() as i32;
        let j = (4.0 * p.y()).floor() as i32;
        let k = (4.0 * p.z()).floor() as i32;

        self.randfloat[
            self.perm_x[(i & 255) as usize] ^
            self.perm_y[(j & 255) as usize] ^
            self.perm_z[(k & 255) as usize]
        ]
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
}