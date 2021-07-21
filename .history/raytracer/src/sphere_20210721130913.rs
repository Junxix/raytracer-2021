pub use crate::aabb::*;
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
        Self {
            center: cen,
            radius: rad,
            mat_ptr: mp,
        }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn get_sphere_uv(p: &point3, u: &mut f64, v: &mut f64) {
        let theta = p.y.acos();
        let phi = p.z.atan2(p.x) + PI;

        *u = phi / (2. * PI);
        *v = theta / PI;
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
                let outward_normal: Vec3 = (rec.p - self.center()) / self.radius();
                rec.set_face_normal(&r, &outward_normal);
                Sphere::get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
            let t = (-half_b + root) / a;
            if t < t_max && t > t_min {
                rec.t = t;
                rec.p = Ray::at(&r, t);
                let outward_normal: Vec3 = (rec.p - self.center()) / self.radius();
                rec.set_face_normal(&r, &outward_normal);
                Sphere::get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
        }
        false
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        true
    }
}
