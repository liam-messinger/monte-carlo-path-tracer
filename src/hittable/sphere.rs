use super::{Hittable, HitRecord, AABB};

use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

use std::f64::consts::PI;
use std::sync::Arc;

#[derive(Clone)]
pub struct Sphere {
    center: Ray,
    radius: f64,
    material: Arc<Material>,
    bounding_box: AABB,
}

impl Sphere {
    // Constructor for Stationary Sphere
    // TODO: make center parameters reference to avoid copying
    pub fn new(static_center: Point3, radius: f64, material: Arc<Material>) -> Self {
        Self {
            center: Ray::new(static_center, Vec3::zero()),
            radius: radius.max(0.0),
            material,
            bounding_box: {
                let r_vec = Vec3::new(radius, radius, radius);
                AABB::from_points(&(static_center - r_vec), &(static_center + r_vec))
            },
        }
    }

    // Constructor for Moving Sphere
    pub fn new_moving( center1: Point3, center2: Point3, radius: f64, material: Arc<Material>) -> Self {
        Self {
            center: Ray::new(center1, center2 - center1),
            radius: radius.max(0.0),
            material,
            bounding_box: {
                let r_vec = Vec3::new(radius, radius, radius);
                let box1 = AABB::from_points(&(center1 - r_vec), &(center1 + r_vec));
                let box2 = AABB::from_points(&(center2 - r_vec), &(center2 + r_vec));
                AABB::merge(&box1, &box2)
            },
        }
    }

    // Check for ray-sphere intersection
    #[inline]
    pub fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
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

        rec.t = root; // Assign hit time
        rec.point = r.at(rec.t); // Assign hit point
        let outward_normal = (rec.point - current_center) / self.radius; // Normal at the hit point
        rec.set_face_normal(r, &outward_normal); // Determine if the hit was on the front face
        Self::get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v); // Assign UV coordinates to record
        rec.material = Arc::clone(&self.material); // Assign material

        true
    }

    // Get the bounding box of the sphere
    pub fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }

    // Get the uv coordinates for a point on the sphere
    fn get_sphere_uv(p: &Point3, u: &mut f64, v: &mut f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }
}

// From Sphere to Hittable implementation
impl From<Sphere> for Hittable {
    fn from(sphere: Sphere) -> Self {
        Hittable::Sphere(sphere)
    }
}
