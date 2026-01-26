use super::{Hittable, HitRecord, AABB};

use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;

use std::sync::Arc;

/// An instance of a Hittable object translated by a given offset.
#[derive(Clone)]
pub struct Translate {
    object: Arc<Hittable>,
    offset: Vec3,
    bounding_box: AABB,
}

impl Translate {
    /// Constructor for a translated Hittable object.
    /// The bounding box is adjusted by the offset.
    pub fn new(object: Arc<Hittable>, offset: Vec3) -> Self {
        let bounding_box = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bounding_box,
        }
    }

    /// Get the bounding box of the translated object.
    pub fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }

    /// Check if a ray hits the translated object.
    #[inline]
    pub fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        // Move the ray backwards by the offset
        let offset_r = Ray::new_with_time(r.origin - self.offset, r.direction, r.time);

        // Determine whether an intersection exists along the offset ray
        if !self.object.hit(&offset_r, ray_t, rec) {
            return false;
        }

        // Move the intersection point forward by the offset
        rec.point += self.offset;

        true
    }
}

// From Translate to Hittable implementation
impl From<Translate> for Hittable {
    fn from(translate: Translate) -> Self {
        Hittable::Translate(translate)
    }
}