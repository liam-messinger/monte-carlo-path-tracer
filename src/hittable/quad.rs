use super::{Hittable, HitRecord, AABB};

use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::prelude::{EPSILON, random_f64};

use std::sync::Arc;

/// A quadrilateral defined by a point and two edge vectors.
#[derive(Clone)]
pub struct Quad {
    Q: Point3,
    edge_u: Vec3,
    edge_v: Vec3,
    material: Arc<Material>,
    bounding_box: AABB,
    normal: Vec3,
    D: f64, // Normal dot Q
    w: Vec3, // n / (n dot n)
    area: f64,
}

impl Quad {
    /// Constructor for Quad given point Q, edge vectors u and v, and material.
    pub fn new(Q: &Point3, u: &Vec3, v: &Vec3, material: Arc<Material>) -> Self {
        let n = Vec3::cross(u, v);
        let normal = Vec3::unit_vector(&n);
        let D = Vec3::dot(&normal, Q);
        
        // Compute the bounding box by considering the two diagonals of the quad
        let bbox_diagonal1 = AABB::from_corners(Q, &(*Q + *u + *v));
        let bbox_diagonal2 = AABB::from_corners(&(*Q + *u), &(*Q + *v));
        Self {
            Q: *Q,
            edge_u: *u,
            edge_v: *v,
            material,
            bounding_box: AABB::merge(&bbox_diagonal1, &bbox_diagonal2),
            normal,
            D,
            w: n / Vec3::dot(&n, &n),
            area: n.length(),
        }
    }

    /// Get the bounding box of the quad.
    pub fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }

    // TODO: Use a more efficient algorithm for hit detection
    /// Hit method for the quad.
    #[inline]
    pub fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let denom = Vec3::dot(&self.normal, &r.direction);
        
        // No hit if the ray is parallel to the plane, multiply epsilon by direction length to allow for longer rays to be treated as parallel
        if denom.abs() < EPSILON * r.direction.length() {
            return false;
        }
        
        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.D - Vec3::dot(&self.normal, &r.origin)) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let intersection: Vec3 = r.at(t);
        let planar_hitpt_vector: Vec3 = intersection - self.Q;
        let alpha = Vec3::dot(&self.w, &Vec3::cross(&planar_hitpt_vector, &self.edge_v));
        let beta = Vec3::dot(&self.w, &Vec3::cross(&self.edge_u, &planar_hitpt_vector));

        // Check if hit point is outside the quad, otherwise set u, v coordinates
        if !Self::is_interior(alpha, beta) {
            return false;
        }
        rec.u = alpha;
        rec.v = beta;

        // Ray hits the 2D shape; set the rest of the hit record and return true.
        // Reconstruct the hit point in 3D space to ensure numerical stability.
        rec.t = t;
        rec.point = self.Q + (alpha * self.edge_u) + (beta * self.edge_v);
        rec.material = Arc::clone(&self.material);
        rec.set_face_normal(r, &self.normal);

        true
    }

    /// Check if the point with plane coordinates (a, b) is inside the quad.
    pub fn is_interior(a: f64, b: f64) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        // Given the hit point in plane coordinates, return false if it is outside the primitive
        return unit_interval.contains(a) && unit_interval.contains(b);
    }

    /// Get the PDF value for a ray hitting the quad from a given origin in a given direction.
    pub fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        let mut rec = HitRecord::new();
        if !self.hit(&Ray::new(*origin, *direction), &Interval::new(0.001, f64::INFINITY), &mut rec) {
            return 0.0;
        }

        let distance_squared = rec.t * rec.t * direction.length_squared();
        let cosine = (Vec3::dot(direction, &rec.normal) / direction.length()).abs();

        distance_squared / (cosine * self.area)
    }

    /// Generate a random direction from the given origin towards the quad.
    pub fn random(&self, origin: &Point3) -> Vec3 {
        let p = self.Q + (random_f64() * self.edge_u) + (random_f64() * self.edge_v);
        p - *origin
    }

    /// Getter for the area of the quad.
    pub fn area(&self) -> f64 {
        self.area
    }
}

// From Quad to Hittable implementation
impl From<Quad> for Hittable {
    fn from(quad: Quad) -> Self {
        Hittable::Quad(quad)
    }
}