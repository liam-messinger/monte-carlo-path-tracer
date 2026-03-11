use std::f64::consts::PI;

use crate::onb::ONB;
use crate::vec3::Vec3;

// ----- Enum for different PDF types -----

/// Pdf enum to represent different probability density function types.
#[derive(Clone)]
pub enum Pdf {
    Sphere(SpherePdf),
    Cosine(CosinePdf),
}

impl Pdf {
    /// Evaluates the PDF value for a given direction.
    /// "How much does the PDF say that this direction is likely to be chosen?”
    pub fn value(&self, direction: &Vec3) -> f64 {
        match self {
            Pdf::Sphere(pdf) => pdf.value(direction),
            Pdf::Cosine(pdf) => pdf.value(direction),
        }
    }

    /// Generates a random direction according to the PDF.
    /// Effectively "What direction does the PDF say I should scatter in?"
    pub fn generate(&self) -> Vec3 {
        match self {
            Pdf::Sphere(pdf) => pdf.generate(),
            Pdf::Cosine(pdf) => pdf.generate(),
        }
    }
}

// ----- Sphere PDF -----

/// A uniform PDF over the unit sphere.
#[derive(Clone)]
pub struct SpherePdf;

impl SpherePdf {
    /// Creates a new SpherePdf instance.
    pub fn new() -> Self { SpherePdf }
    
    #[inline]
    fn value(&self, _direction: &Vec3) -> f64 {
        1.0 / (4.0 * PI)
    }

    #[inline]
    fn generate(&self) -> Vec3 {
        Vec3::random_unit_vector()
    }
}

// ----- Cosine PDF -----

/// A cosine-weighted PDF oriented around a given normal direction.
#[derive(Clone)]
pub struct CosinePdf {
    uvw: ONB,
}

impl CosinePdf {
    /// Creates a new CosinePdf instance with the given normal direction.
    pub fn new(w: &Vec3) -> Self {
        Self { uvw: ONB::new(w) }
    }

    #[inline]
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine_theta = Vec3::dot(&Vec3::unit_vector(direction), &self.uvw.w());
        f64::max(0.0, cosine_theta) / PI
    }

    #[inline]
    fn generate(&self) -> Vec3 {
        self.uvw.transform(&Vec3::random_cosine_direction())
    }
}