use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::interval::Interval;
use crate::material::{Material, Lambertian};

use std::rc::Rc;

// Record of a ray-object intersection
#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub mat: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    // Constructor for HitRecord
    pub fn new() -> Self {
        Self {
            point: Point3::zero(),
            normal: Vec3::zero(),
            mat: Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))), // Default material
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

// Trait for hittable objects
pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}