use std::sync::Arc;

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

// ----- Checker Texture -----
pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<Texture>,
    odd: Arc<Texture>,
}

impl CheckerTexture {
    // Constructor from scale, even texture, and odd texture
    pub fn new(scale: f64, even: Texture, odd: Texture) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Arc::new(even),
            odd: Arc::new(odd),
        }
    }

    // Constructor from scale and two colors
    pub fn from_colors(scale: f64, even: Color, odd: Color) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Arc::new(Texture::from(SolidColor::new(even))),
            odd: Arc::new(Texture::from(SolidColor::new(odd))),
        }
    }

    // Value method returns the checker pattern color
    #[inline]
    pub fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let xInt = (p.x() * self.inv_scale).floor() as i32;
        let yInt = (p.y() * self.inv_scale).floor() as i32;
        let zInt = (p.z() * self.inv_scale).floor() as i32;

        let isEven = (xInt + yInt + zInt) % 2 == 0;

        if isEven {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}