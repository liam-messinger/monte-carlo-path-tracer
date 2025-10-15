// Common imports
pub use crate::color::Color;
pub use crate::ray::Ray;
pub use crate::vec3::{Point3, Vec3};
pub use crate::interval::Interval;

// Constants
pub const PI: f64 = std::f64::consts::PI;

// Utility Functions
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn radians_to_degrees(radians: f64) -> f64 {
    radians * 180.0 / PI
}

pub fn random_f64() -> f64 {
    // Returns a random real in [0,1).
    rand::random::<f64>()
}

pub fn random_f64_range(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    min + (max - min) * random_f64()
}