use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    // Create a new ray with zero origin and zero direction
    pub fn zero() -> Self {
        Ray {
            origin: Point3::zero(),
            direction: Vec3::zero(),
        }
    }

    // Create a new ray with default values (origin at zero, direction at zero)
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray {
            origin,
            direction,
        }
    }

    // Implements P(t) = origin + t * direction
    #[inline]
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}

impl Default for Ray {
    fn default() -> Self {
        Ray::zero()
    }
}