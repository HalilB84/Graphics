use core::f64;

use crate::{utils::random_int, vec3::*};

const POINTCOUNT: usize = 256;

//ok so perlin noise is not explained at all in the book
//currently black box -> needs explaining

pub struct Perlin {
    randvec: [Vec3; POINTCOUNT],
    perm_x: [i64; POINTCOUNT],
    perm_y: [i64; POINTCOUNT],
    perm_z: [i64; POINTCOUNT],
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut tempvec: [Vec3; POINTCOUNT] = [Vec3::new(0., 0., 0.); POINTCOUNT];
        let mut perm_x: [i64; POINTCOUNT] = [0; POINTCOUNT];
        let mut perm_y: [i64; POINTCOUNT] = [0; POINTCOUNT];
        let mut perm_z: [i64; POINTCOUNT] = [0; POINTCOUNT];

        for i in 0..POINTCOUNT {
            tempvec[i] = Vec3::unit_vector(Vec3::random_range(-1., 1.));
        }

        Perlin::perlin_generate_perm(&mut perm_x);
        Perlin::perlin_generate_perm(&mut perm_y);
        Perlin::perlin_generate_perm(&mut perm_z);

        Perlin {
            randvec: tempvec,
            perm_x: perm_x,
            perm_y: perm_y,
            perm_z: perm_z,
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i64;
        let j = p.y().floor() as i64;
        let k = p.z().floor() as i64;

        let mut c = [[[Vec3::new(0., 0., 0.); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randvec[(self.perm_x[((i + di as i64) & 255) as usize]
                        ^ self.perm_y[((j + dj as i64) & 255) as usize]
                        ^ self.perm_z[((k + dk as i64) & 255) as usize])
                        as usize]
                }
            }
        }

        Perlin::trilinear_interp(&c, u, v, w)
    }

    pub fn turb(&self, p: Point3, depth: i64) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _i in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.;
        }

        accum.abs()
    }

    fn perlin_generate_perm(p: &mut [i64]) -> () {
        for i in 0..POINTCOUNT {
            p[i] = i as i64;
        }

        Perlin::permute(p, POINTCOUNT);
    }

    fn permute(p: &mut [i64], n: usize) {
        for i in (1..n).rev() {
            let target = random_int(0, i as i64) as usize;
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
    }

    fn trilinear_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3. - 2. * u);
        let vv = v * v * (3. - 2. * v);
        let ww = w * w * (3. - 2. * w);
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);

                    accum += (i as f64 * uu + (1. - i as f64) * (1. - uu))
                        * (j as f64 * vv + (1. - j as f64) * (1. - vv))
                        * (k as f64 * ww + (1. - k as f64) * (1. - ww))
                        * Vec3::dot(c[i][j][k], weight_v);
                }
            }
        }

        accum
    }
}
