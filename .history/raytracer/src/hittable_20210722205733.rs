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

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool;
}
#[derive(Clone)]
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
impl Default for HittableList {
    fn default() -> Self {
        Self::new()
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

pub struct Translate {
    ptr: Arc<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(p: Arc<dyn Hittable>, displacement: &Vec3) -> Self {
        Self {
            ptr: p,
            offset: *displacement,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let moved_r = Ray {
            orig: r.orig - self.offset,
            dir: r.dir,
            tm: r.tm,
        };
        if !self.ptr.hit(moved_r, t_min, t_max, rec) {
            return false;
        }

        rec.p += self.offset;
        let normal = rec.normal;
        rec.set_face_normal(&moved_r, &normal);

        true
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        if !self.ptr.bounding_box(t0, t1, output_box) {
            return false;
        }
        *output_box = AABB::new(
            output_box.min() + self.offset,
            output_box.max() + self.offset,
        );
        true
    }
}

pub struct RotateY {
    pub ptr: Arc<dyn Hittable>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub hasbox: bool,
    pub bbox: AABB,
}

impl RotateY {
    pub fn new(p: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut min = point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = point3::new(-INFINITY, -INFINITY, -INFINITY);
        let mut bbox = AABB::new(point3::ones(), point3::ones());
        let hasbox = p.bounding_box(0.0, 1.0, &mut bbox);
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.max().x + (1 - i) as f64 * bbox.min().x;
                    let y = j as f64 * bbox.max().y + (1 - j) as f64 * bbox.min().y;
                    let z = k as f64 * bbox.max().z + (1 - k) as f64 * bbox.min().z;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    min.x = min.x.min(tester.x);
                    max.x = max.x.max(tester.x);

                    min.y = min.y.min(tester.y);
                    max.y = max.y.max(tester.y);

                    min.z = min.z.min(tester.z);
                    max.z = max.z.max(tester.z);
                }
            }
        }
        let bbox = AABB::new(min, max);
        Self {
            ptr: p,
            sin_theta,
            cos_theta,
            hasbox,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut orig = r.orig;
        let mut dir = r.dir;

        orig.x = self.cos_theta * r.orig.x - self.sin_theta * r.orig.z;
        orig.z = self.sin_theta * r.orig.x + self.cos_theta * r.orig.z;

        dir.x = self.cos_theta * r.dir.x - self.sin_theta * r.dir.z;
        dir.z = self.sin_theta * r.dir.x + self.cos_theta * r.dir.z;

        let rotated_r = Ray {
            orig,
            dir,
            tm: r.tm,
        };

        if !self.ptr.hit(rotated_r, t_min, t_max, rec) {
            return false;
        }

        let mut p = rec.p;
        let mut normal = rec.normal;

        p.x = self.cos_theta * rec.p.x + self.sin_theta * rec.p.z;
        p.z = -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z;

        normal.x = self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z;
        normal.z = -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z;

        rec.p = p;
        rec.set_face_normal(&rotated_r, &normal);

        true
    }
    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox;
        self.hasbox
    }
}
