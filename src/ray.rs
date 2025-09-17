use crate::vec3::{Vec3, Point3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
  orig: Point3,
  dir: Vec3,
}

impl Ray {
  // Create a new ray with zero origin and zero direction
  pub fn zero() -> Self {
    Ray {
      orig: Point3::zero(),
      dir: Vec3::zero(),
    }
  }
  
  // Create a new ray with default values (origin at zero, direction at zero)
  pub fn new(origin: Point3, direction: Vec3) -> Self {
    Ray {
      orig: origin,
      dir: direction,
    }
  }

  // Get the origin of the ray (immutable reference)
  pub fn origin(&self) -> &Point3 {
    &self.orig
  }

  // Get the direction of the ray (immutable reference)
  pub fn direction(&self) -> &Vec3 {
    &self.dir
  }

  // Implements P(t) = origin + t * direction
  pub fn at(&self, t: f64) -> Point3 {
    self.orig + t * self.dir
  }
}

impl Default for Ray {
  fn default() -> Self {
    Ray::zero()
  }
}