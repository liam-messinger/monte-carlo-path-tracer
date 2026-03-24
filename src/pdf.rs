use std::f64::consts::PI;
use std::sync::Arc;

use crate::hittable::Hittable;
use crate::onb::ONB;
use crate::vec3::{Point3, Vec3};
use crate::prelude::*;

// ----- Enum for different PDF types -----

/// Pdf enum to represent different probability density function types.
#[derive(Clone)]
pub enum Pdf {
    Sphere(SpherePdf),
    Cosine(CosinePdf),
    Hittable(HittablePdf),
    Mixture(MixturePdf),
}

impl Pdf {
    /// Evaluates the PDF value for a given direction.
    /// "How much does the PDF say that this direction is likely to be chosen?”
    pub fn value(&self, direction: &Vec3) -> f64 {
        match self {
            Pdf::Sphere(pdf) => pdf.value(direction),
            Pdf::Cosine(pdf) => pdf.value(direction),
            Pdf::Hittable(pdf) => pdf.value(direction),
            Pdf::Mixture(pdf) => pdf.value(direction),
        }
    }

    /// Generates a random direction according to the PDF.
    /// Effectively "What direction does the PDF say I should scatter in?"
    pub fn generate(&self) -> Vec3 {
        match self {
            Pdf::Sphere(pdf) => pdf.generate(),
            Pdf::Cosine(pdf) => pdf.generate(),
            Pdf::Hittable(pdf) => pdf.generate(),
            Pdf::Mixture(pdf) => pdf.generate(),
        }
    }

    // Convenience Arc constructors

    /// Create an Arc<Pdf> for a SpherePdf.
    pub fn sphere() -> Arc<Self> {
        Arc::new(Self::Sphere(SpherePdf::new()))
    }
    /// Create an Arc<Pdf> for a CosinePdf with the given normal direction.
    pub fn cosine(w: &Vec3) -> Arc<Self> {
        Arc::new(Self::Cosine(CosinePdf::new(w)))
    }
    /// Create an Arc<Pdf> for a HittablePdf with the given hittable objects and origin point.
    pub fn hittable(objects: Arc<Hittable>, origin: Point3) -> Arc<Self> {
        Arc::new(Self::Hittable(HittablePdf::new(objects, origin)))
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

// ----- Hittable PDF -----
#[derive(Clone)]
pub struct HittablePdf {
    objects: Arc<Hittable>,
    origin: Point3,
}

impl HittablePdf {
    /// Creates a new HittablePdf instance for the given hittable objects and origin point.
    pub fn new(objects: Arc<Hittable>, origin: Point3) -> Self {
        Self { objects, origin }
    }

    #[inline]
    fn value(&self, direction: &Vec3) -> f64 {
        self.objects.pdf_value(&self.origin, direction)
    }

    #[inline]
    fn generate(&self) -> Vec3 {
        self.objects.random(&self.origin)
    }
}

// ----- Mixture PDF -----
#[derive(Clone)]
pub struct MixturePdf {
    pdfs: [Arc<Pdf>; 2],
}

impl MixturePdf {
    /// Creates a new MixturePdf instance with the given PDFs.
    pub fn new(pdf1: Arc<Pdf>, pdf2: Arc<Pdf>) -> Self {
        Self { pdfs: [pdf1, pdf2] }
    }

    #[inline]
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * self.pdfs[0].value(direction) + 0.5 * self.pdfs[1].value(direction)
    }

    #[inline]
    fn generate(&self) -> Vec3 {
        if random_f64() < 0.5 {
            self.pdfs[0].generate()
        } else {
            self.pdfs[1].generate()
        }
    }
}