// Re-exports
pub mod bvh_node;
pub mod hit_record;
pub mod hittable_list;
pub mod sphere;
pub mod quad;
pub mod aabb;

pub use bvh_node::BVHNode;
pub use hit_record::HitRecord;
pub use hittable_list::HittableList;
pub use sphere::Sphere;
pub use quad::Quad;
pub use aabb::AABB;

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