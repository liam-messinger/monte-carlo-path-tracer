use super::{Hittable, HitRecord, AABB};

use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::prelude::degrees_to_radians;

use std::sync::Arc;

/// An instance of a Hittable object rotated around the Y axis.
#[derive(Clone)]
pub struct RotateY {
    object: Arc<Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bounding_box: AABB,
}

impl RotateY {
    /// Constructor for RotateY given an object and rotation angle in degrees.
    pub fn new(object: impl Into<Hittable>, angle: f64) -> Self {
        let object = Arc::new(object.into());

        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let bbox = object.bounding_box();

        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        // Rotate each corner of the bounding box to find the new AABB
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    min = Point3::new(
                        min.x().min(tester.x()),
                        min.y().min(tester.y()),
                        min.z().min(tester.z()),
                    );
                    max = Point3::new(
                        max.x().max(tester.x()),
                        max.y().max(tester.y()),
                        max.z().max(tester.z()),
                    );
                }
            }
        }

        let bounding_box = AABB::from_points(&min, &max);

        Self {
            object,
            sin_theta,
            cos_theta,
            bounding_box,
        }
    }  

    /// Get the bounding box of the rotated object.
    pub fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }

    /// Check if a ray hits the rotated object.
    #[inline]
    pub fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        // Transform the ray origin from world space to object space (rotate by -theta)
        let origin = Point3::new(
            self.cos_theta * r.origin.x() - self.sin_theta * r.origin.z(),
            r.origin.y(),
            self.sin_theta * r.origin.x() + self.cos_theta * r.origin.z(),
        );

        // Transform the direction from world space to object space (rotate by -theta)
        let direction = Vec3::new(
            self.cos_theta * r.direction.x() - self.sin_theta * r.direction.z(),
            r.direction.y(),
            self.sin_theta * r.direction.x() + self.cos_theta * r.direction.z(),
        );

        let rotated_r = Ray::new_with_time(origin, direction, r.time);

        // Determine whether an intersection exists in object space
        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }

        // Transform the intersection from object space back to world space (rotate by +theta)
        rec.point = Point3::new(
            self.cos_theta * rec.point.x() + self.sin_theta * rec.point.z(),
            rec.point.y(),
            -self.sin_theta * rec.point.x() + self.cos_theta * rec.point.z(),
        );

        rec.normal = Vec3::new(
            self.cos_theta * rec.normal.x() + self.sin_theta * rec.normal.z(),
            rec.normal.y(),
            -self.sin_theta * rec.normal.x() + self.cos_theta * rec.normal.z(),
        );

        true
    }
}

// From RotateY to Hittable implementation
impl From<RotateY> for Hittable {
    fn from(rotate_y: RotateY) -> Self {
        Hittable::RotateY(rotate_y)
    }
}