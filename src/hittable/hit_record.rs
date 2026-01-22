use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::material::Material;

use std::sync::Arc;

/// Record of a ray-object intersection
#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Arc<Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    /// Constructor for HitRecord
    pub fn new() -> Self {
        Self {
            point: Point3::zero(),
            normal: Vec3::zero(),
            material: Arc::new(Material::default()),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
        }
    }

    /// Sets the hit record normal vector
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

// Default implementation for HitRecord
impl Default for HitRecord {
    fn default() -> Self {
        Self::new()
    }
}