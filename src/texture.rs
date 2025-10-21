use crate::prelude::*;

// ----- Enum for different texture types -----
pub enum Texture {
    SolidColor(SolidColor),
}

// Implementation of the value method for Texture enum
impl Texture {
    #[inline]
    pub fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        match self {
            Texture::SolidColor(tex) => tex.value(u, v, p),
        }
    }
}

// ----- Solid Color Texture -----
pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    // Contructor from Color
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    // Contructor from RGB values
    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self {
            albedo: Color::new(r, g, b),
        }
    }

    // Value method returns the solid color
    #[inline]
    pub fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.albedo
    }
}

// From SolidColor to Texture implementation
impl From<SolidColor> for Texture {
    fn from(tex: SolidColor) -> Self {
        Texture::SolidColor(tex)
    }
}