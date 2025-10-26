use super::{HitRecord, HittableList, BVHNode, AABB, Sphere, Quad};

use crate::ray::Ray;
use crate::interval::Interval;

// ----- Enum for Hittable Object Types -----
#[derive(Clone)]
pub enum Hittable {
    HittableList(HittableList),
    BVHNode(BVHNode),
    Sphere(Sphere),
    Quad(Quad),
    // Etc.
}

impl Hittable {
    #[inline]
    pub fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        match self {
            Hittable::HittableList(list) => list.hit(r, ray_t, rec),
            Hittable::BVHNode(node) => node.hit(r, ray_t, rec),
            Hittable::Sphere(sphere) => sphere.hit(r, ray_t, rec),
            Hittable::Quad(quad) => quad.hit(r, ray_t, rec),
            // Etc.
        }
    }

    pub fn bounding_box(&self) -> &AABB {
        match self {
            Hittable::HittableList(list) => list.bounding_box(),
            Hittable::BVHNode(node) => node.bounding_box(),
            Hittable::Sphere(sphere) => sphere.bounding_box(),
            Hittable::Quad(quad) => quad.bounding_box(),
            // Etc.
        }
    }
}