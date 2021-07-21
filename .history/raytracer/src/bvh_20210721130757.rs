use crate::aabb::*;
use crate::hittable::*;
use crate::ray::*;
use crate::vec3::*;
use std::ops::Deref;
use std::sync::Arc;
pub struct BvhNode {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub bvh_box: AABB,
}
impl BvhNode {
    /*pub fn new0()->Self{
        Self{
           left:Arc::new(HitRecord::new0()),
           right:Arc::new(HitRecord::new0()),
           bvh_box: AABB::new0(),
        }
    }*/
    pub fn from(list: &mut HittableList, time0: f64, time1: f64) -> Self {
        let len = &mut list.objects.len();
        Self::fromvec(&mut list.objects, 0, *len, time0, time1)
    }

    pub fn fromvec(
        src_objects: &mut Vec<Arc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let objects = src_objects;
        let axis = Vec3::random_int(0, 2);

        let mut left: Arc<dyn Hittable> = objects[start].clone();
        let mut right: Arc<dyn Hittable> = objects[start].clone();

        let comparator = if axis == 0 {
            BvhNode::box_x_compare
        } else if axis == 1 {
            BvhNode::box_y_compare
        } else {
            BvhNode::box_z_compare
        };
        let object_span = end - start;

        if object_span == 1 {
        } else if object_span == 2 {
            if comparator(objects[start].deref(), objects[start + 1].deref()) {
                left = objects[start].clone();
                right = objects[start + 1].clone();
            } else {
                right = objects[start].clone();
                left = objects[start + 1].clone();
            }
        } else {
            &objects[start..end].sort_unstable_by(|a, b| {
                if comparator(&**a, &**b) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            });

            let mid = start + object_span / 2;
            left =
                Arc::new(BvhNode::fromvec(objects, start, mid, time0, time1)) as Arc<dyn Hittable>;
            right =
                Arc::new(BvhNode::fromvec(objects, mid, end, time0, time1)) as Arc<dyn Hittable>;
        }
        let mut box_left = AABB::new0();
        let mut box_right = AABB::new0();

        if !left.bounding_box(time0, time1, &mut box_left)
            || !right.bounding_box(time0, time1, &mut box_right)
        {
            eprintln!("No bounding box in bvh_node constructor");
        }

        let boxx = AABB::surrounding_box(&box_left, &box_right);

        return Self {
            left: left.clone(),
            right: right.clone(),
            bvh_box: boxx.clone(),
        };
    }
    pub fn box_compare(a: &dyn Hittable, b: &dyn Hittable, axis: usize) -> bool {
        let mut box_a = AABB::new0();
        let mut box_b = AABB::new0();

        if !a.bounding_box(0., 0., &mut box_a) || !b.bounding_box(0., 0., &mut box_b) {
            eprintln!("No bounding box in BvhNode constructor");
        }
        let idx = axis as i32;
        box_a.min().get(idx) < box_b.min().get(idx)
    }

    pub fn box_x_compare(a: &dyn Hittable, b: &dyn Hittable) -> bool {
        BvhNode::box_compare(a, b, 0)
    }
    pub fn box_y_compare(a: &dyn Hittable, b: &dyn Hittable) -> bool {
        BvhNode::box_compare(a, b, 1)
    }
    pub fn box_z_compare(a: &dyn Hittable, b: &dyn Hittable) -> bool {
        BvhNode::box_compare(a, b, 2)
    }
}

impl Hittable for BvhNode {
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bvh_box;
        true
    }
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.bvh_box.hit(&ray, t_min, t_max) {
            return false;
        } else {
            let hit_left = self.left.hit(ray, t_min, t_max, rec);
            let hit_right = self.right.hit(
                ray,
                t_min,
                {
                    if hit_left {
                        rec.t
                    } else {
                        t_max
                    }
                },
                rec,
            );

            hit_left || hit_right
        }
    }
}
