pub use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn default_new() -> Self {
        Ray {
            orig: Vec3::zero(),
            dir: Vec3::zero(),
        }
    }
    pub fn new0() -> Self {
        Self {
            orig: Vec3::zero(),
            dir: Vec3::zero(),
        }
    }
    pub fn new(o: Vec3, d: Vec3) -> Self {
        Self { orig: o, dir: d }
    }

    pub fn orign(&self) -> Vec3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Vec3 {
       self.orig + self.dir * t
    }
}
