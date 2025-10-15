use super::core::{HitRecord};
use crate::ray::Ray;
use crate::interval::Interval;
use crate::vec3::{Point3, Vec3};
use crate::material::Material;

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    // Constructor for Sphere
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
            material,
        }
    }

    // Check for ray-sphere intersection
    pub fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        // Calculate the discriminant of the quadratic equation for ray-sphere intersection
        let oc: Vec3 = self.center - r.origin;
        let a = r.direction.length_squared();
        let h = Vec3::dot(&r.direction, &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 { // No real roots, ray does not hit the sphere
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.contains(root) {
            root = (h + sqrtd) / a;
            if !ray_t.contains(root) {
                return false;
            }
        }

        rec.t = root;
        rec.point = r.at(rec.t);
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.material = self.material.clone();

        true
    }
}