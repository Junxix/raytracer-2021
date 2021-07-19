pub use crate::hittable::*;
pub use crate::material::*;
pub use crate::vec3::Vec3;
use crate::Ray;
//use std::rc::Rc;
use std::sync::Arc;
pub use Vec3 as point3;
pub use Vec3 as color;
pub struct Sphere {
    pub center: point3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(cen: point3, rad: f64, mp: Arc<dyn Material>) -> Self {
        return Self {
            center: cen,
            radius: rad,
            mat_ptr: mp,
        };
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}
impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.orign() - self.center;
        let a: f64 = Vec3::squared_length(&r.direction());
        let half_b: f64 = r.direction() * oc;
        let c: f64 = Vec3::squared_length(&oc) - self.radius * self.radius;
        let discriminant = half_b.powi(2) - a * c;
        if discriminant > 0.0 {
            let root: f64 = discriminant.sqrt();
            let t = (-half_b - root) / a;
            if t < t_max && t > t_min {
                rec.t = t; //??????????
                rec.p = Ray::at(&r, t);
                let mut outward_normal: Vec3 = (rec.p - self.center()) / self.radius();
                rec.set_face_normal(&r, &mut outward_normal);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
            let t = (-half_b + root) / a;
            if t < t_max && t > t_min {
                rec.t = t;
                rec.p = Ray::at(&r, t);
                let mut outward_normal: Vec3 = (rec.p - self.center()) / self.radius();
                rec.set_face_normal(&r, &mut outward_normal);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
        }
        return false;
    }
}
