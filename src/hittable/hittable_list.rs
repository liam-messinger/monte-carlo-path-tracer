use super::core::{HitRecord, HittableObject};
use crate::ray::Ray;
use crate::interval::Interval;
use std::rc::Rc;

pub struct HittableList {
    pub objects: Vec<HittableObject>,
}

impl HittableList {
    // Constructor for HittableList
    pub fn new() -> Self {
        Self { 
            objects: Vec::new()
        }
    }

    // Clear all objects from the list
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    // Add an object to the list
    pub fn add(&mut self, object: HittableObject) {
        self.objects.push(object);
    }

    // Check for ray intersections with all objects in the list
    pub fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}