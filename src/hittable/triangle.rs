use super::{AABB, HitRecord, Hittable};

use crate::interval::Interval;
use crate::material::Material;
use crate::prelude::{random_f64, EPSILON};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

use std::sync::Arc;

#[derive(Clone)]
pub struct Triangle {
    a: Point3,
    b: Point3,
    c: Point3,
    material: Arc<Material>,
    bounding_box: AABB,
    normal: Vec3,  // flat unit normal
    area: f64,     // Geometric area
    // Future: allowing for mesh textures with the following:
    // uv: Option<[(f64, f64)]>, // UV coordinates for each vertex
}

impl Triangle {
    /// Constructor for Triangle given three vertices and material.
    pub fn new(a: &Point3, b: &Point3, c: &Point3, material: Arc<Material>) -> Self {
        let e1 = *b - *a;
        let e2 = *c - *a;
        let n = Vec3::cross(&e1, &e2);

        let area = 0.5 * n.length(); // Half of the parallelogram area
        let normal = Vec3::unit_vector(&n);

        let bounding_box = AABB::from_point_triplet(a, b, c);
        
        Self {
            a: *a,
            b: *b,
            c: *c,
            material,
            bounding_box,
            normal,
            area,
        }
    }

    pub fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }

    /// Hit method for the triangle, using the Möller-Trumbore ray-triangle intersection algorithm.
    #[inline]
    pub fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let e1 = self.b - self.a;
        let e2 = self.c - self.a;

        let ray_cross_e2 = Vec3::cross(&r.direction, &e2);
        let det = Vec3::dot(&e1, &ray_cross_e2);

        if det.abs() < EPSILON {
            return false; // Ray is parallel to triangle plane
        }

        let inv_det = 1.0 / det;
        let s = r.origin - self.a;
        let u = inv_det * Vec3::dot(&s, &ray_cross_e2);
        if u < 0.0 || u > 1.0 {
            return false; // Intersection outside triangle
        }

        let s_cross_e1 = Vec3::cross(&s, &e1);
        let v = inv_det * Vec3::dot(&r.direction, &s_cross_e1);
        if v < 0.0 || u + v > 1.0 {
            return false; // Intersection outside triangle
        }

        // Computer t to find intersection point on ray
        let t = inv_det * Vec3::dot(&e2, &s_cross_e1);
        if !ray_t.contains(t) {
            return false; // Intersection outside ray bounds
        }

        // Update hit record
        rec.t = t;
        rec.point = r.at(t);
        rec.material = Arc::clone(&self.material);
        rec.set_face_normal(r, &self.normal);

        true
    }

    /// Get the PDF value for a ray hitting the triangle from a given origin in a given direction.
    pub fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        let mut rec = HitRecord::new();
        if !self.hit(&Ray::new(*origin, *direction), &Interval::new(0.001, f64::INFINITY), &mut rec) {
            return 0.0;
        }

        let distance_squared = rec.t * rec.t * direction.length_squared();
        let cosine = (Vec3::dot(direction, &rec.normal) / direction.length()).abs();

        distance_squared / (cosine * self.area)
    }

    /// Generate a random direction from the given origin towards the triangle.
    pub fn random(&self, origin: &Point3) -> Vec3 {
        // Generate a random point on the triangle using barycentric coordinates (no square root version, reflection)
        let mut u = random_f64();
        let mut v = random_f64();

        if u + v > 1.0 { // Reflect back into triangle if outside (from full unit square)
            u = 1.0 - u;
            v = 1.0 - v;
        }

        let point = self.a + u * (self.b - self.a) + v * (self.c - self.a);
        point - *origin
    }
}

impl From<Triangle> for Hittable {
    fn from(triangle: Triangle) -> Self {
        Hittable::Triangle(triangle)
    }
}