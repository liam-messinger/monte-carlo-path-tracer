// Common imports
pub use crate::color::Color;
pub use crate::ray::Ray;
pub use crate::vec3::{Point3, Vec3};
pub use crate::interval::Interval;

use rand::Rng;

// Utility Functions
#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[inline]
pub fn radians_to_degrees(radians: f64) -> f64 {
    radians * 180.0 / PI
}

#[inline]
pub fn random_f64() -> f64 {
    // Returns a random real in [0,1).
    rand::random::<f64>()
}

#[inline]
pub fn random_f64_range(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    min + (max - min) * random_f64()
}

#[inline]
pub fn random_i32(min: i32, max: i32) -> i32 {
    // Returns a random integer in [min,max].
    rand::rng().random_range(min..=max)
}