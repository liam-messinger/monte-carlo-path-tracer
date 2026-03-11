use std::f64::consts::PI;

use crate::onb::ONB;
use crate::vec3::Vec3;

/// Pdf enum to represent different probability density function types.
#[derive(Clone)]
pub enum Pdf {
    Sphere(SpherePdf),
    Cosine(CosinePdf),
}

impl Pdf {
    /// Evaluates the PDF value for a given direction.
    pub fn value(&self, direction: &Vec3) -> f64 {
        match self {
            Pdf::Sphere(pdf) => pdf.value(direction),
            Pdf::Cosine(pdf) => pdf.value(direction),
        }
    }

    /// Generates a random direction according to the PDF.
    pub fn generate(&self) -> Vec3 {
        match self {
            Pdf::Sphere(pdf) => pdf.generate(),
            Pdf::Cosine(pdf) => pdf.generate(),
        }
    }
}