pub use crate::aabb::*;
pub use crate::material::*;
pub use crate::sphere::Sphere;
pub use crate::vec3::Vec3;
use crate::Ray;
//use std::rc::Rc;
use std::sync::Arc;
pub use Vec3 as point3;
pub use Vec3 as color;

#[derive(Clone)]
pub struct HitRecord {
    pub p: point3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat_ptr: Arc<dyn Material>, //材料
}

impl HitRecord {
    pub fn new0() -> Self {
        Self {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
            mat_ptr: Arc::new(Metal::new1(color::new(0.0, 0.0, 0.0))),
            u: 0.0,
            v: 0.0,
        }
    }
    pub fn new(mp: Arc<dyn Material>) -> Self {
        Self {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
            mat_ptr: mp,
            u: 0.0,
            v: 0.0,
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = (r.direction() * (*outward_normal)) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool;
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        //let spheres_list: Vec<Sphere> = Vec::new();
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, sphere: Arc<dyn Hittable>) {
        self.objects.push(sphere);
    }

    pub fn size(&self) -> usize {
        self.objects.len()
    }
}
impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::clone(&rec);
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = t_max;
        for i in 0..self.size() {
            if self.objects[i].hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        if self.objects.is_empty() {
            return false;
        }
        let mut temp_box = AABB::new0();
        let mut first_box = true;
        for object in self.objects.iter() {
            if !object.bounding_box(time0, time1, &mut temp_box) {
                return false;
            }
            *output_box = if first_box {
                temp_box
            } else {
                AABB::surrounding_box(&output_box, &temp_box)
            };
            first_box = false;
        }
        true
    }
}
