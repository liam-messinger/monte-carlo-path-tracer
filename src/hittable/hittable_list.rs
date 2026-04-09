use super::{HitRecord, Hittable, BVHNode, AABB};

use crate::prelude::{random_usize};
use crate::ray::Ray;
use crate::interval::Interval;
use crate::vec3::{Point3, Vec3};

use std::sync::Arc;

/// A list of Hittable objects.
#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<Hittable>>,
    bounding_box: AABB,
}

impl HittableList {
    /// Constructor for HittableList.
    pub fn new() -> Self {
        Self { 
            objects: Vec::new(),
            bounding_box: AABB::empty(),
        }
    }

    /// Constructor from a single Hittable object.
    pub fn from_hittable(object: impl Into<Hittable>) -> Self {
        let hittable_object = object.into();
        let bounding_box = hittable_object.bounding_box().clone();
        Self {
            objects: vec![Arc::new(hittable_object)],
            bounding_box,
        }
    }

    /// Convert the HittableList into a BVH, returning the root BVHNode.
    pub fn into_bvh(mut self) -> BVHNode {
        BVHNode::build_from_list(&mut self)
    }

    /// Clear all objects from the list.
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    /// Add an object to the list and update bounding box.
    pub fn add(&mut self, object: impl Into<Hittable>) {
        let hittable_object = object.into();
        self.bounding_box = AABB::merge(&self.bounding_box, hittable_object.bounding_box());
        self.objects.push(Arc::new(hittable_object));
    }

    /// Check for ray intersections with all objects in the list.
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

    /// Get bounding box of the hittable list.
    pub fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }

    /// Get the PDF value for a given ray direction.
    pub fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        let mut sum = 0.0;

        for object in &self.objects {
            sum += weight * object.pdf_value(origin, direction);
        }
        sum
    }

    /// Generate a random direction from the given origin towards the hittable list.
    pub fn random(&self, origin: &Point3) -> Vec3 {
        let size = self.objects.len();
        let index = random_usize(0, size - 1);
        self.objects[index].random(origin)
    }
}

// From HittableList to HittableObject conversion
impl From<HittableList> for Hittable {
    fn from(list: HittableList) -> Self {
        Hittable::HittableList(list)
    }
}