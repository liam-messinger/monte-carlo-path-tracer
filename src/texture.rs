use std::sync::Arc;

use crate::prelude::*;
use crate::image_data::ImageData;

// Todo: Split texture types into separate files if this file gets too large

// ----- Enum for different texture types -----
// A texture is a mapping from a (u,v) texture coordinate to a Color value.
#[derive(Clone)]
pub enum Texture { // Update for each new texture type
    SolidColor(SolidColor),
    CheckerTexture(CheckerTexture),
    ImageTexture(ImageTexture),
}

// Implementation of the value method for Texture enum
impl Texture {
    #[inline]
    pub fn value(&self, u: f64, v: f64, p: &Point3) -> Color { // Update for each new texture type
        match self {
            Texture::SolidColor(tex) => tex.value(u, v, p),
            Texture::CheckerTexture(tex) => tex.value(u, v, p),
            Texture::ImageTexture(tex) => tex.value(u, v, p),
        }
    }
}

// ----- Solid Color Texture -----
#[derive(Clone)]
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
#[derive(Clone)]
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

// From CheckerTexture to Texture implementation
impl From<CheckerTexture> for Texture {
    fn from(tex: CheckerTexture) -> Self {
        Texture::CheckerTexture(tex)
    }
}

// ----- Image Texture -----
#[derive(Clone)]
pub struct ImageTexture {
    image_data: ImageData, // Ownes the image data
}

impl ImageTexture {
    // Constructor from filename
    pub fn from_file(filename: &str) -> Self {
        Self {
            image_data: ImageData::new(filename),
        }
    }

    // Value method returns the color from the image at (u, v)
    #[inline]
    pub fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        // If we have no texture data, then return solid cyan as a debugging aid.
        if self.image_data.height() <= 0 { return Color::new(0.0, 1.0, 1.0); }

        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0); // Flip V to image coordinates (0 at top left)

        let i = (u * self.image_data.width() as f64) as u32; // u * width = pixel location in x direction
        let j = (v * self.image_data.height() as f64) as u32; // v * height = pixel location in y direction

        self.image_data.pixel_data(i, j) // Get the pixel color at (i, j)
    }
}