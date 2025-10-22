use super::{HitRecord, Hittable, BVHNode, AABB};

use crate::ray::Ray;
use crate::interval::Interval;

use std::sync::Arc;

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<Hittable>>,
    bounding_box: AABB,
}

impl HittableList {
    // Constructor for HittableList
    pub fn new() -> Self {
        Self { 
            objects: Vec::new(),
            bounding_box: AABB::empty(),
        }
    }

    pub fn from_hittable(object: impl Into<Hittable>) -> Self {
        let hittable_object = object.into();
        let bounding_box = hittable_object.bounding_box().clone();
        Self {
            objects: vec![Arc::new(hittable_object)],
            bounding_box,
        }
    }

    pub fn to_bvh(&mut self) -> BVHNode {
        BVHNode::build_from_list(self)
    }

    // Clear all objects from the list
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    // Add an object to the list
    pub fn add(&mut self, object: impl Into<Hittable>) {
        let hittable_object = object.into();
        self.bounding_box = AABB::merge(&self.bounding_box, hittable_object.bounding_box());
        self.objects.push(Arc::new(hittable_object));
    }

    // Check for ray intersections with all objects in the list
    #[inline]
    pub fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        // Reusable interval
        let mut range = Interval::new(ray_t.min, closest_so_far);

        for object in &self.objects {
            range.max = closest_so_far;

            if object.hit(r, &range, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }

    // Get bounding box of the hittable list
    pub fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}



// From HittableList to HittableObject conversion
impl From<HittableList> for Hittable {
    fn from(list: HittableList) -> Self {
        Hittable::HittableList(list)
    }
}