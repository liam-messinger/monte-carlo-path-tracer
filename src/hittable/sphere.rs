use super::core::{HitRecord, HittableObject};
use crate::ray::Ray;
use crate::interval::Interval;
use crate::vec3::{Point3, Vec3};
use crate::material::Material;
use std::sync::Arc;

#[derive(Clone)]
pub struct Sphere {
    pub center: Ray,
    pub radius: f64,
    pub material: Arc<Material>,
}

impl Sphere {
    // Constructor for Stationary Sphere
    pub fn new(center: Point3, radius: f64, material: impl Into<Material>) -> Self {
        Self {
            center: Ray::new(center, Vec3::zero()),
            radius: radius.max(0.0),
            material: Arc::new(material.into()),
        }
    }

    // Constructor for Moving Sphere
    pub fn new_moving(center1: Point3, center2: Point3, radius: f64, material: impl Into<Material>) -> Self {
        Self {
            center: Ray::new(center1, center2 - center1),
            radius: radius.max(0.0),
            material: Arc::new(material.into()),
        }
    }

    // Check for ray-sphere intersection
    #[inline]
    pub fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let current_center: Point3 = self.center.at(r.time);
        // Calculate the discriminant of the quadratic equation for ray-sphere intersection
        let oc: Vec3 = current_center - r.origin;
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
        let outward_normal = (rec.point - current_center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.material = Arc::clone(&self.material);

        true
    }
}

// From Sphere to HittableObject implementation
impl From<Sphere> for HittableObject {
    fn from(sphere: Sphere) -> Self {
        HittableObject::Sphere(sphere)
    }
}