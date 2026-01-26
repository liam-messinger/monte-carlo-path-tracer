// Re-exports
pub mod bvh_node;
pub mod hit_record;
pub mod hittable_list;
pub mod sphere;
pub mod quad;
pub mod aabb;
pub mod cuboid;
pub mod translate;
pub mod rotate_y;

pub use bvh_node::BVHNode;
pub use hit_record::HitRecord;
pub use hittable_list::HittableList;
pub use sphere::Sphere;
pub use quad::Quad;
pub use aabb::AABB;
pub use cuboid::Cuboid;
pub use translate::Translate;
pub use rotate_y::RotateY;

use crate::ray::Ray;
use crate::interval::Interval;

/// Enum representing different types of Hittable objects.
#[derive(Clone)]
pub enum Hittable {
    HittableList(HittableList),
    BVHNode(BVHNode),
    Sphere(Sphere),
    Quad(Quad),
    Cuboid(Cuboid),
    Translate(Translate),
    RotateY(RotateY),
    // Etc.
}

impl Hittable {
    /// Check if a ray hits the Hittable object.
    #[inline]
    pub fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        match self {
            Hittable::HittableList(list) => list.hit(r, ray_t, rec),
            Hittable::BVHNode(node) => node.hit(r, ray_t, rec),
            Hittable::Sphere(sphere) => sphere.hit(r, ray_t, rec),
            Hittable::Quad(quad) => quad.hit(r, ray_t, rec),
            Hittable::Cuboid(cuboid) => cuboid.hit(r, ray_t, rec),
            Hittable::Translate(translate) => translate.hit(r, ray_t, rec),
            Hittable::RotateY(rotate_y) => rotate_y.hit(r, ray_t, rec),
            // Etc.
        }
    }

    /// Get the bounding box of the Hittable object.
    pub fn bounding_box(&self) -> &AABB {
        match self {
            Hittable::HittableList(list) => list.bounding_box(),
            Hittable::BVHNode(node) => node.bounding_box(),
            Hittable::Sphere(sphere) => sphere.bounding_box(),
            Hittable::Quad(quad) => quad.bounding_box(),
            Hittable::Cuboid(cuboid) => cuboid.bounding_box(),
            Hittable::Translate(translate) => translate.bounding_box(),
            Hittable::RotateY(rotate_y) => rotate_y.bounding_box(),
            // Etc.
        }
    }
}