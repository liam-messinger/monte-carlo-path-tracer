use super::{Hittable, HitRecord, AABB};

use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

use std::sync::Arc;

#[derive(Clone)]
pub struct Quad {
    Q: Point3,
    u: Vec3,
    v: Vec3,
    material: Arc<Material>,
    bounding_box: AABB,
    normal: Vec3,
    D: f64, // Normal dot Q
}

impl Quad {
    // Constructor
    pub fn new(Q: &Point3, u: &Vec3, v: &Vec3, material: Arc<Material>) -> Self {
        let normal = Vec3::unit_vector(&Vec3::cross(u, v));
        let D = Vec3::dot(&normal, Q);
        
        // Compute the bounding box by considering the two diagonals of the quad
        let bbox_diagonal1 = AABB::from_points(Q, &(*Q + *u + *v));
        let bbox_diagonal2 = AABB::from_points(&(*Q + *u), &(*Q + *v));
        Self {
            Q: *Q,
            u: *u,
            v: *v,
            material,
            bounding_box: AABB::merge(&bbox_diagonal1, &bbox_diagonal2),
            normal,
            D,
        }
    }

    // Get the bounding box of the quad
    pub fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }

    // Hit method
    #[inline]
    pub fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let denom = Vec3::dot(&self.normal, &r.direction);
        
        // No hit if the ray is parallel to the plane.
        if denom.abs() < 1e-8 {
            return false;
        }
        
        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.D - Vec3::dot(&self.normal, &r.origin)) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        let intersection = r.at(t);

        rec.t = t;
        rec.point = intersection;
        rec.material = Arc::clone(&self.material);
        rec.set_face_normal(r, &self.normal);

        true
    }
}

// From Quad to Hittable implementation
impl From<Quad> for Hittable {
    fn from(quad: Quad) -> Self {
        Hittable::Quad(quad)
    }
}