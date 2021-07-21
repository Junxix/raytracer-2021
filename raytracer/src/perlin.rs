use std::usize;

pub use crate::ray::*;
pub use crate::rtweekend::*;
pub use crate::vec3::*;

pub struct Perlin {
    ranvec: Vec<Vec3>,
    pub perm_x: Vec<i32>,
    pub perm_y: Vec<i32>,
    pub perm_z: Vec<i32>,
}
impl Perlin {
    const POINTCOUNT: i32 = 256;
    pub fn new() -> Self {
        let mut ranvec: Vec<Vec3> = Vec::with_capacity(Perlin::POINTCOUNT as usize);
        for _i in 0..Perlin::POINTCOUNT {
            ranvec.push(Vec3::unit(&Vec3::random2(-1.0, 1.0)));
        }
        Self {
            ranvec,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }
    pub fn perlin_generate_perm() -> Vec<i32> {
        let mut _p = Vec::new();
        for i in 0..Perlin::POINTCOUNT {
            _p.push(i);
        }
        Self::permute(&mut _p, Self::POINTCOUNT as usize);
        _p
    }

    fn permute(p: &mut Vec<i32>, n: usize) {
        for i in (n - 1..0).rev() {
            let target = Vec3::random_int(0, i as i64);
            let tmp = p[i];
            p[i] = p[target as usize];
            p[target as usize] = tmp;
        }
    }
    /*pub fn permute(p: &mut Vec<i32>, n: i32) {
            for i in (n - 1..0).rev() {
                let target = Vec3::random_int(0, i as i64);
                let tmp = p[i];
                p[i] = p[target];
                p[target] = tmp;
            }
        }
    */
    pub fn noise(&self, p: &Vec3) -> f64 {
        let mut u = p.x - p.x.floor();
        let mut v = p.y - p.y.floor();
        let mut w = p.z - p.z.floor();

        let _i = p.x.floor() as i32;
        let _j = p.y.floor() as i32;
        let _k = p.z.floor() as i32;

        let mut c = [[[Vec3::new0(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[(self.perm_x[(_i + di as i32) as usize & 255]
                        ^ self.perm_y[(_j + dj as i32) as usize & 255]
                        ^ self.perm_z[(_k + dk as i32) as usize & 255])
                        as usize];
                }
            }
        }

        Self::perlin_interp(c, u, v, w)
    }
    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3. - 2. * u);
        let vv = v * v * (3. - 2. * v);
        let ww = w * w * (3. - 2. * w);

        let mut accum = 0.;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * u as f64 + (1.0 - i as f64) * (1.0 - u))
                        * (j as f64 * v as f64 + (1.0 - j as f64) * (1.0 - v))
                        * (k as f64 * w as f64 + (1.0 - k as f64) * (1. - w))
                        * (c[i][j][k] * weight_v);
                }
            }
        }
        accum
    }
    pub fn turb(&self, p: &point3) -> f64 {
        let depth = 7;
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _i in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }
        accum.abs()
    }
}
