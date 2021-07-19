pub use crate::material::*;
pub use crate::sphere::Sphere;
pub use crate::vec3::Vec3;
use crate::Ray;
use std::rc::Rc;
use std::sync::Arc;
pub use Vec3 as point3;
pub use Vec3 as color;

#[derive(Clone)]
pub struct HitRecord {
    pub p: point3,
    pub normal: Vec3,
    pub t: f64,
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
        }
    }
    pub fn new(mp: Arc<dyn Material>) -> Self {
        Self {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
            mat_ptr: mp,
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
}

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        //let spheres_list: Vec<Sphere> = Vec::new();
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, sphere: Rc<dyn Hittable>) {
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
                closest_so_far = temp_rec.t.clone();
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
}
