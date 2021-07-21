use crate::hittable::*;
use crate::material::*;
use crate::ray::*;
use std::sync::Arc;

pub struct XyRect {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}
impl XyRect {
    pub fn new0() -> Self {
        Self {
            mp: Arc::new(Lambertian::new_by_color(color::new0())),
            x0: 0.,
            x1: 0.,
            y0: 0.,
            y1: 0.,
            k: 0.,
        }
    }
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            mp: mat,
        }
    }
}
impl Hittable for XyRect {
    fn hit(&self, ray: crate::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - ray.orign().z) / ray.direction().z;
        if t < t_min || t > t_max {
            return false;
        }
        let x = ray.orign().x + t * ray.direction().x;
        let y = ray.orign().y + t * ray.direction().y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(&ray, &outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = ray.at(t);
        return true;
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        true
    }
}
