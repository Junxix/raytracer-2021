pub use crate::aabb::*;
pub use crate::hittable::*;
pub use crate::material::*;
pub use crate::ray::*;
pub use crate::vec3::*;
//use std::rc::Rc;
use std::sync::Arc;
pub use Vec3 as point3;
pub use Vec3 as color;

pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new0() -> Self {
        Self {
            center0: Vec3::zero(),
            center1: Vec3::zero(),
            time0: 0.0,
            time1: 0.0,
            radius: 0.0,
            mat_ptr: Arc::new(Metal::new1(color::new(0.0, 0.0, 0.0))),
        }
    }

    pub fn new(
        cen0: point3,
        cen1: point3,
        _time0: f64,
        _time1: f64,
        r: f64,
        mp: Arc<dyn Material>,
    ) -> Self {
        Self {
            center0: cen0,
            center1: cen1,
            time0: _time0,
            time1: _time1,
            radius: r,
            mat_ptr: mp,
        }
    }

    pub fn center(&self, time: f64) -> point3 {
        self.center0
            + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0))
    }
}
///这里可能有错误
impl Hittable for MovingSphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.orign() - self.center(r.time());
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
                let outward_normal = (rec.p - self.center(r.time())) / self.radius;
                rec.set_face_normal(&r, &outward_normal);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
            let t = (-half_b + root) / a;
            if t < t_max && t > t_min {
                rec.t = t;
                rec.p = Ray::at(&r, t);
                let outward_normal: Vec3 = (rec.p - self.center(r.time())) / self.radius;
                rec.set_face_normal(&r, &outward_normal);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
        }
        false
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        let box0 = AABB::new(
            MovingSphere::center(&self, time0) - Vec3::new(self.radius, self.radius, self.radius),
            MovingSphere::center(&self, time0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = AABB::new(
            MovingSphere::center(&self, time1) - Vec3::new(self.radius, self.radius, self.radius),
            MovingSphere::center(&self, time1) + Vec3::new(self.radius, self.radius, self.radius),
        );
        *output_box = AABB::surrounding_box(&box0, &box1);
        true
    }
}
