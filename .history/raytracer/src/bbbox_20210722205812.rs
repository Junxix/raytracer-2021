use crate::aabb::*;
use crate::aarect::*;
use crate::hittable::*;
use crate::material::*;
use crate::ray::*;
use std::sync::Arc;

pub struct Box6 {
    pub box_min: point3,
    pub box_max: point3,
    pub sides: HittableList,
}

impl Box6 {
    pub fn new(p0: &point3, p1: &point3, ptr: Arc<dyn Material>) -> Self {
        let mut tmp = Self {
            box_min: *p0,
            box_max: *p1,
            sides: HittableList::new(),
        };
        tmp.sides.add(Arc::new(XyRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p1.z,
            ptr.clone(),
        )));
        tmp.sides.add(Arc::new(XyRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p0.z,
            ptr.clone(),
        )));

        tmp.sides.add(Arc::new(XzRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p1.y,
            ptr.clone(),
        )));
        tmp.sides.add(Arc::new(XzRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p0.y,
            ptr.clone(),
        )));

        tmp.sides.add(Arc::new(YzRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p1.x,
            ptr.clone(),
        )));
        tmp.sides.add(Arc::new(YzRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p0.x,
            ptr.clone(),
        )));

        tmp
    }
}

impl Hittable for Box6 {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        self.sides.hit(r, t_min, t_max, rec)
    }
    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(self.box_min, self.box_max);
        true
    }
}
