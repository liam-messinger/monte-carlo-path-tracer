// Common imports
pub use crate::color::Color;
pub use crate::ray::Ray;
pub use crate::vec3::{Point3, Vec3};
pub use crate::interval::Interval;

use rand::Rng;
use std::f64::consts::PI;

// Constants
pub const EPSILON: f64 = 1e-8;
pub const AABB_MIN_PADDING: f64 = 0.0001;

// Utility Functions

/// Convert degrees to radians.
#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

/// Convert radians to degrees.
#[inline]
pub fn radians_to_degrees(radians: f64) -> f64 {
    radians * 180.0 / PI
}

/// Generate a random f64 in [0,1).
#[inline]
pub fn random_f64() -> f64 {
    // Returns a random real in [0,1).
    rand::random::<f64>()
}

/// Generate a random f64 in [min,max).
#[inline]
pub fn random_f64_range(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    min + (max - min) * random_f64()
}

/// Generate a random i32 in [min,max].
#[inline]
pub fn random_i32(min: i32, max: i32) -> i32 {
    // Returns a random integer in [min,max].
    rand::rng().random_range(min..=max)
}

/// Generate a random usize in [min,max].
#[inline]
pub fn random_usize_range(min: usize, max: usize) -> usize {
    // Returns a random usize in [min,max].
    rand::rng().random_range(min..=max)
}