use crate::ray::*;
use crate::vec3::*;

#[derive(Clone, Copy, PartialEq)]
pub struct AABB {
    pub minimum: Vec3,
    pub maximum: Vec3,
}
impl AABB {
    pub fn new0() -> Self {
        Self {
            minimum: Vec3::zero(),
            maximum: Vec3::zero(),
        }
    }
    pub fn new(a: point3, b: point3) -> Self {
        Self {
            minimum: a,
            maximum: b,
        }
    }
    pub fn min(&self) -> point3 {
        self.minimum
    }
    pub fn max(&self) -> point3 {
        self.maximum
    }
    pub fn hit(&self, ray: &Ray, mut tmin: f64, mut tmax: f64) -> bool {
        for idx in 0..3 {
            let t0 = Vec3::fmin(
                (self.minimum.get(idx) - ray.orig.get(idx)) / ray.dir.get(idx),
                (self.maximum.get(idx) - ray.orig.get(idx)) / ray.dir.get(idx),
            );
            let t1 = Vec3::fmax(
                (self.minimum.get(idx) - ray.orig.get(idx)) / ray.dir.get(idx),
                (self.maximum.get(idx) - ray.orig.get(idx)) / ray.dir.get(idx),
            );
            tmin = Vec3::fmax(t0, tmin);
            tmax = Vec3::fmin(t1, tmax);
            if tmax <= tmin {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
        let small = point3::new(
            Vec3::fmin(box0.min().x, box1.min().x),
            Vec3::fmin(box0.min().y, box1.min().y),
            Vec3::fmin(box0.min().z, box1.min().z),
        );

        let big = point3::new(
            Vec3::fmax(box0.max().x, box1.max().x),
            Vec3::fmax(box0.max().y, box1.max().y),
            Vec3::fmax(box0.max().z, box1.max().z),
        );

        AABB::new(small, big)
    }
}
