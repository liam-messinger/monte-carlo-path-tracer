use super::utils::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3, dot};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    // Constructor for Sphere
    pub fn new(center: Point3, radius: f64) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // Calculate the discriminant of the quadratic equation for ray-sphere intersection
        let oc: Vec3 = self.center - *r.origin();
        let a = r.direction().length_squared();
        let h = dot(r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            // No real roots, ray does not hit the sphere
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let root = (h - sqrtd) / a;
        if root <= t_min || root >= t_max {
            let root = (h + sqrtd) / a;
            if root <= t_min || root >= t_max {
                return false;
            }
        }

        rec.t = root;
        rec.point = r.at(rec.t);
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        true
    }
}