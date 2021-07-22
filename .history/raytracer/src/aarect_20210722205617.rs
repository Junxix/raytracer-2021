use crate::aabb::*;
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
    /*pub fn new0() -> Self {
        Self {
            mp: Arc::new(Lambertian::new_by_color(color::new0())),
            x0: 0.,
            x1: 0.,
            y0: 0.,
            y1: 0.,
            k: 0.,
        }
    }*/
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
        true
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        let _x = time0;
        let _y = time1;
        *output_box = AABB::new(
            point3::new(self.x0, self.y0, self.k - 0.0001),
            point3::new(self.x1, self.y1, self.k + 0.0001),
        );
        true
    }
}

pub struct XzRect {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl XzRect {
    /*pub fn new0() -> Self {
        Self {
            mp: Arc::new(Lambertian::new_by_color(color::new0())),
            x0: 0.,
            x1: 0.,
            z0: 0.,
            z1: 0.,
            k: 0.,
        }
    }*/
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            z0,
            z1,
            k,
            mp: mat,
        }
    }
}

impl Hittable for XzRect {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - ray.orign().y) / ray.direction().y;
        if t < t_min || t > t_max {
            return false;
        }
        let x = ray.orign().x + t * ray.direction().x;
        let z = ray.orign().z + t * ray.direction().z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }

        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);

        rec.t = t;

        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        rec.set_face_normal(&ray, &outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = ray.at(t);

        true
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        let _x = time0;
        let _y = time1;
        *output_box = AABB::new(
            point3::new(self.x0, self.k - 0.0001, self.z0),
            point3::new(self.x1, self.k + 0.0001, self.z1),
        );
        true
    }
}

pub struct YzRect {
    mp: Arc<dyn Material>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl YzRect {
    /* pub fn new0() -> Self {
        Self {
            mp: Arc::new(Lambertian::new_by_color(color::new0())),
            y0: 0.,
            y1: 0.,
            z0: 0.,
            z1: 0.,
            k: 0.,
        }
    }*/
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            y0,
            y1,
            z0,
            z1,
            k,
            mp: mat,
        }
    }
}

impl Hittable for YzRect {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - ray.orign().x) / ray.direction().x;
        if t < t_min || t > t_max {
            return false;
        }
        let y = ray.orign().y + t * ray.direction().y;
        let z = ray.orign().z + t * ray.direction().z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return false;
        }

        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);

        rec.t = t;

        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        rec.set_face_normal(&ray, &outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = ray.at(t);
        true
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        let _x = time0;
        let _y = time1;
        *output_box = AABB::new(
            point3::new(self.k - 0.0001, self.y0, self.z0),
            point3::new(self.k + 0.0001, self.y1, self.z1),
        );
        true
    }
}

/*use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::vec3::point3;
use crate::{Hittable, Ray, Vec3};
use std::rc::Rc;
use std::sync::Arc;

pub struct XYRect {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl XYRect {
    pub fn new(x0_: f64, x1_: f64, y0_: f64, y1_: f64, k_: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            mp: mat,
            x0: x0_,
            x1: x1_,
            y0: y0_,
            y1: y1_,
            k: k_,
        }
    }
}

impl Hittable for XYRect {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.orign().z) / r.direction().z;
        if t < t_min || t > t_max {
            return false;
        }
        let x = r.orign().x + t * r.direction().x;
        let y = r.orign().y + t * r.direction().y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        let mut outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(&r, &mut outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            point3::new(self.x0, self.y0, self.k - 0.0001),
            point3::new(self.x1, self.y1, self.k + 0.0001),
        );
        true
    }
}

pub struct XZRect {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl XZRect {
    pub fn new(x0_: f64, x1_: f64, z0_: f64, z1_: f64, k_: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            mp: mat,
            x0: x0_,
            x1: x1_,
            z0: z0_,
            z1: z1_,
            k: k_,
        }
    }
}

impl Hittable for XZRect {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.orign().y) / r.direction().y;
        if t < t_min || t > t_max {
            return false;
        }
        let x = r.orign().x + t * r.direction().x;
        let z = r.orign().z + t * r.direction().z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let mut outward_normal = Vec3::new(0.0, 1.0, 0.0);
        rec.set_face_normal(&r, &mut outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            point3::new(self.x0, self.k - 0.0001, self.z0),
            point3::new(self.x1, self.k + 0.0001, self.z1),
        );
        true
    }
}

pub struct YZRect {
    mp: Arc<dyn Material>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl YZRect {
    pub fn new(y0_: f64, y1_: f64, z0_: f64, z1_: f64, k_: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            mp: mat,
            y0: y0_,
            y1: y1_,
            z0: z0_,
            z1: z1_,
            k: k_,
        }
    }
}

impl Hittable for YZRect {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.orign().x) / r.direction().x;
        if t < t_min || t > t_max {
            return false;
        }
        let y = r.orign().y + t * r.direction().y;
        let z = r.orign().z + t * r.direction().z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return false;
        }
        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let mut outward_normal = Vec3::new(1.0, 0.0, 0.0);
        rec.set_face_normal(&r, &mut outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            point3::new(self.k - 0.0001, self.y0, self.z0),
            point3::new(self.k + 0.0001, self.y1, self.z1),
        );
        true
    }
}
*/
