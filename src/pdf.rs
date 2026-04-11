use std::f64::consts::PI;
use std::sync::Arc;

use crate::hittable::Hittable;
use crate::onb::ONB;
use crate::vec3::{Point3, Vec3};
use crate::prelude::*;

// ----- Enum for different PDF types -----

/// PDF enum to represent different probability density function types.
#[derive(Clone)]
pub enum PDF {
    Sphere(SpherePDF),
    Cosine(CosinePDF),
    Hittable(HittablePDF),
    Mixture(MixturePDF),
}

impl PDF {
    /// Evaluates the PDF value for a given direction.
    /// "How much does the PDF say that this direction is likely to be chosen?”
    pub fn value(&self, direction: &Vec3) -> f64 {
        match self {
            PDF::Sphere(pdf) => pdf.value(direction),
            PDF::Cosine(pdf) => pdf.value(direction),
            PDF::Hittable(pdf) => pdf.value(direction),
            PDF::Mixture(pdf) => pdf.value(direction),
        }
    }

    /// Generates a random direction according to the PDF.
    /// Effectively "What direction does the PDF say I should scatter in?"
    pub fn generate(&self) -> Vec3 {
        match self {
            PDF::Sphere(pdf) => pdf.generate(),
            PDF::Cosine(pdf) => pdf.generate(),
            PDF::Hittable(pdf) => pdf.generate(),
            PDF::Mixture(pdf) => pdf.generate(),
        }
    }

    // Convenience Arc constructors

    /// Create an Arc<PDF> for a SpherePdf.
    pub fn sphere() -> Arc<Self> {
        Arc::new(Self::Sphere(SpherePDF::new()))
    }
    /// Create an Arc<PDF> for a CosinePdf with the given normal direction.
    pub fn cosine(w: &Vec3) -> Arc<Self> {
        Arc::new(Self::Cosine(CosinePDF::new(w)))
    }
    /// Create an Arc<PDF> for a HittablePdf with the given hittable objects and origin point.
    pub fn hittable(objects: Arc<Hittable>, origin: Point3) -> Arc<Self> {
        Arc::new(Self::Hittable(HittablePDF::new(objects, origin)))
    }
    /// Create an Arc<PDF> for a MixturePdf with the given PDFs.
    pub fn mixture(pdf1: Arc<Self>, pdf2: Arc<Self>) -> Arc<Self> {
        Arc::new(Self::Mixture(MixturePDF::new(pdf1, pdf2)))
    }
}

// ----- Sphere PDF -----

/// A uniform PDF over the unit sphere.
#[derive(Clone)]
pub struct SpherePDF;

impl SpherePDF {
    /// Creates a new SpherePdf instance.
    pub fn new() -> Self { SpherePDF }
    
    /// Evaluates the PDF value for a given direction. For a uniform sphere, this is constant.
    #[inline]
    fn value(&self, _direction: &Vec3) -> f64 {
        1.0 / (4.0 * PI)
    }

    /// Generates a random direction uniformly distributed over the sphere.
    #[inline]
    fn generate(&self) -> Vec3 {
        Vec3::random_unit_vector()
    }
}

// ----- Cosine PDF -----

/// A cosine-weighted PDF oriented around a given normal direction.
#[derive(Clone)]
pub struct CosinePDF {
    uvw: ONB,
}

impl CosinePDF {
    /// Creates a new CosinePdf instance with the given normal direction.
    pub fn new(w: &Vec3) -> Self {
        Self { uvw: ONB::new(w) }
    }

    /// Evaluates the PDF value for a given direction.
    /// This is the cosine of the angle between the direction and the normal, divided by PI.
    #[inline]
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine_theta = Vec3::dot(&Vec3::unit_vector(direction), &self.uvw.w());
        f64::max(0.0, cosine_theta) / PI
    }

    /// Generates a random direction according to the cosine-weighted distribution.
    #[inline]
    fn generate(&self) -> Vec3 {
        self.uvw.transform(&Vec3::random_cosine_direction())
    }
}

// ----- Hittable PDF -----
#[derive(Clone)]
pub struct HittablePDF {
    objects: Arc<Hittable>,
    origin: Point3,
}

impl HittablePDF {
    /// Creates a new HittablePdf instance for the given hittable objects and origin point.
    pub fn new(objects: Arc<Hittable>, origin: Point3) -> Self {
        Self { objects, origin }
    }

    /// Evaluates the PDF value for a given direction by delegating to the hittable objects.
    #[inline]
    fn value(&self, direction: &Vec3) -> f64 {
        self.objects.pdf_value(&self.origin, direction)
    }

    /// Generates a random direction according to the PDF defined by the hittable objects.
    #[inline]
    fn generate(&self) -> Vec3 {
        self.objects.random(&self.origin)
    }
}

// ----- Mixture PDF -----
#[derive(Clone)]
pub struct MixturePDF {
    pdfs: [Arc<PDF>; 2],
}

impl MixturePDF {
    /// Creates a new MixturePdf instance with the given PDFs.
    pub fn new(pdf1: Arc<PDF>, pdf2: Arc<PDF>) -> Self {
        Self { pdfs: [pdf1, pdf2] }
    }

    /// Evaluates the PDF value for a given direction by averaging the values from the two PDFs.
    #[inline]
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * self.pdfs[0].value(direction) + 0.5 * self.pdfs[1].value(direction)
    }

    /// Generates a random direction according to the mixture PDF by randomly choosing one of the two PDFs.
    #[inline]
    fn generate(&self) -> Vec3 {
        if random_f64() < 0.5 {
            self.pdfs[0].generate()
        } else {
            self.pdfs[1].generate()
        }
    }
}