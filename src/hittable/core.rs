use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::interval::Interval;
use crate::material::{Material};
use crate::hittable::Sphere;
use crate::aabb::AABB;

use super::hittable_list::HittableList;
use super::bvh_node::BVHNode;

use std::sync::Arc;

// Record of a ray-object intersection
#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Arc<Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    // Constructor for HitRecord
    pub fn new() -> Self {
        Self {
            point: Point3::zero(),
            normal: Vec3::zero(),
            material: Arc::new(Material::default()),
            t: 0.0,
            front_face: false,
        }
    }

    // Sets the hit record normal vector
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        // NOTE: the parameter outward_normal is assumed to have unit length
        self.front_face = Vec3::dot(&r.direction, outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

// TODO: Move Hittable enum to its own file
// ----- Enum for Hittable Object Types -----
#[derive(Clone)]
pub enum Hittable {
    HittableList(HittableList),
    BVHNode(BVHNode),
    Sphere(Sphere),
    // Etc.
}

impl Hittable {
    #[inline]
    pub fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        match self {
            Hittable::HittableList(list) => list.hit(r, ray_t, rec),
            Hittable::BVHNode(node) => node.hit(r, ray_t, rec),
            Hittable::Sphere(sphere) => sphere.hit(r, ray_t, rec),
            // Etc.
        }
    }

    pub fn bounding_box(&self) -> &AABB {
        match self {
            Hittable::HittableList(list) => list.bounding_box(),
            Hittable::BVHNode(node) => node.bounding_box(),
            Hittable::Sphere(sphere) => sphere.bounding_box(),
            // Etc.
        }
    }
}