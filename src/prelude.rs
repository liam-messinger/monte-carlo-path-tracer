// Common imports
pub use crate::color::{Color, rgb, rgb_bytes};
pub use crate::ray::Ray;
pub use crate::vec3::{Point3, Vec3, unit_vector, dot, cross};
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