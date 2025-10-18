use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    // Create a new ray with zero origin and zero direction
    pub fn zero() -> Self {
        Ray {
            origin: Point3::zero(),
            direction: Vec3::zero(),
            time: 0.0,
        }
    }

    // Create a new ray with default values (origin at zero, direction at zero, time at 0.0)
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray {
            origin,
            direction,
            time: 0.0,
        }
    }

    // Create a new ray with specified origin, direction, and time
    pub fn new_with_time(origin: Point3, direction: Vec3, time: f64) -> Self {
        Ray {
            origin,
            direction,
            time,
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