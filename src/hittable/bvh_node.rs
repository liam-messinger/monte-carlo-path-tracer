use super::core::{HitRecord, Hittable};
use crate::aabb::AABB;
use crate::hittable::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;

use std::sync::Arc;

#[derive(Clone)]
pub struct BVHNode {
    left: Arc<Hittable>,
    right: Arc<Hittable>,
    bounding_box: AABB,
}

impl BVHNode {
    pub fn build_from_list(list: &mut HittableList) -> Self {
        let len = list.objects.len();
        Self::build_partition(&mut list.objects, 0, len)
    }

    pub fn build_partition(objects: &mut Vec<Arc<Hittable>>, start: usize, end: usize) -> Self {
        // Build the bounding box of the span of source objects.
        let mut bbox = AABB::empty();
        for i in start..end {
            bbox = AABB::merge(&bbox, objects[i].bounding_box());
        }
        let axis_index = bbox.longest_axis();

        let comparator = match axis_index {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };

        let object_span: usize = end - start;

        let left: Arc<Hittable>;
        let right: Arc<Hittable>;

        if object_span == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
        } else if object_span == 2 {
            if comparator(&objects[start], &objects[start + 1]) == std::cmp::Ordering::Less {
                left = objects[start].clone();
                right = objects[start + 1].clone();
            } else {
                left = objects[start + 1].clone();
                right = objects[start].clone();
            }
        } else {
            objects[start..end].sort_by(comparator);
        
            let mid: usize = start + object_span / 2;
            left = Arc::new(BVHNode::build_partition(objects, start, mid).into());
            right = Arc::new(BVHNode::build_partition(objects, mid, end).into());
        }
        
        Self {
            bounding_box: bbox,
            left,
            right,
        }
    }

    pub fn box_compare(a: &Arc<Hittable>, b: &Arc<Hittable>, axis_index: usize) -> std::cmp::Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis_index);
        let b_axis_interval = b.bounding_box().axis_interval(axis_index);
        a_axis_interval.min.partial_cmp(&b_axis_interval.min).unwrap()
    }

    pub fn box_x_compare(a: &Arc<Hittable>, b: &Arc<Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 0)
    }

    pub fn box_y_compare(a: &Arc<Hittable>, b: &Arc<Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 1)
    }

    pub fn box_z_compare(a: &Arc<Hittable>, b: &Arc<Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 2)
    }

    #[inline]
    pub fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        if !self.bounding_box.hit(r, ray_t) {
            return false;
        }

        // Check if leaf with single object
        if Arc::ptr_eq(&self.left, &self.right) {
            return self.left.hit(r, ray_t, rec);
        }

        let hit_left: bool = self.left.hit(r, ray_t, rec);
        let max_t = if hit_left { rec.t } else { ray_t.max };
        let hit_right: bool = self.right.hit(r, &Interval::new(ray_t.min, max_t), rec);

        hit_left || hit_right
    }

    pub fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}

// From BVHNode to Hittable implementation
impl From<BVHNode> for Hittable {
    fn from(node: BVHNode) -> Self {
        Hittable::BVHNode(node)
    }
}