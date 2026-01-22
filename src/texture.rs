use std::sync::Arc;

use crate::prelude::*;
use crate::image_data::ImageData;
use crate::noise::Noise;

// Todo: Split texture types into separate files if this file gets too large

/// Enum for different texture types.
/// A texture is a mapping from a (u,v) texture coordinate to a Color value.
#[derive(Clone)]
pub enum Texture { // Update for each new texture type
    SolidColor(SolidColor),
    CheckerTexture(CheckerTexture),
    ImageTexture(ImageTexture),
    NoiseTexture(NoiseTexture),
}

impl Texture {
    /// Implementation of the value method for Texture enum.
    #[inline]
    pub fn value(&self, u: f64, v: f64, p: &Point3) -> Color { // Update for each new texture type
        match self {
            Texture::SolidColor(tex) => tex.value(u, v, p),
            Texture::CheckerTexture(tex) => tex.value(u, v, p),
            Texture::ImageTexture(tex) => tex.value(u, v, p),
            Texture::NoiseTexture(tex) => tex.value(u, v, p),
        }
    }

    // Convenience Arc constructors

    /// Create an Arc<Texture> solid color from a Color.
    pub fn solid(albedo: Color) -> Arc<Texture> {
        Arc::new(Texture::SolidColor(SolidColor::new(albedo)))
    }
    /// Create an Arc<Texture> checker texture from scale and two Colors.
    pub fn checker(scale: f64, even: Color, odd: Color) -> Arc<Texture> {
        Arc::new(Texture::CheckerTexture(CheckerTexture::from_colors(scale, even, odd)))
    }
    /// Create an Arc<Texture> checker texture from scale and two Textures.
    pub fn checker_tex(scale: f64, even: Texture, odd: Texture) -> Arc<Texture> {
        Arc::new(Texture::CheckerTexture(CheckerTexture::new(scale, even, odd)))
    }
    /// Create an Arc<Texture> image texture from a filename.
    pub fn image(filename: &str) -> Arc<Texture> {
        Arc::new(Texture::ImageTexture(ImageTexture::new(filename)))
    }
    /// Create an Arc<Texture> noise texture from a scale.
    pub fn noise(scale: f64) -> Arc<Texture> {
        Arc::new(Texture::NoiseTexture(NoiseTexture::new(scale)))
    }
}

// ----- Macros to implement From trait for texture types -----
// From texture type to Texture
macro_rules! impl_texture_from {
    ($($variant:ident),+ $(,)?) => {
        $(
            impl From<$variant> for Texture {
                fn from(tex: $variant) -> Self {
                    Texture::$variant(tex)
                }
            }
        )+
    };
}
impl_texture_from!(SolidColor, CheckerTexture, ImageTexture, NoiseTexture);

// From texture type to Arc<Texture>
macro_rules! impl_arc_texture_from {
    ($($variant:ident),+ $(,)?) => {
        $(
            impl From<$variant> for Arc<Texture> {
                fn from(tex: $variant) -> Self {
                    Arc::new(Texture::$variant(tex))
                }
            }
        )+
    };
}
impl_arc_texture_from!(SolidColor, CheckerTexture, ImageTexture, NoiseTexture);

// ----- Solid Color Texture -----

/// Solid color texture that returns the same color regardless of (u,v,p).
#[derive(Clone)]
pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    /// Contructor for SolidColor from a Color.
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    /// Contructor for SolidColor from RGB values.
    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self {
            albedo: Color::new(r, g, b),
        }
    }

    /// Value method returns the solid color.
    #[inline]
    pub fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.albedo
    }
}

// ----- Checker Texture -----

/// Checker texture that alternates between two textures based on position.
#[derive(Clone)]
pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<Texture>,
    odd: Arc<Texture>,
}

impl CheckerTexture {
    /// Constructor from scale and two textures. 
    pub fn new(scale: f64, even: Texture, odd: Texture) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Arc::new(even),
            odd: Arc::new(odd),
        }
    }

    /// Constructor from scale and two colors.
    pub fn from_colors(scale: f64, even: Color, odd: Color) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Arc::new(Texture::from(SolidColor::new(even))),
            odd: Arc::new(Texture::from(SolidColor::new(odd))),
        }
    }

    /// Value method returns the checker texture color at (u,v,p).
    #[inline]
    pub fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x_int = (p.x() * self.inv_scale).floor() as i32;
        let y_int = (p.y() * self.inv_scale).floor() as i32;
        let z_int = (p.z() * self.inv_scale).floor() as i32;

        let is_even = (x_int + y_int + z_int) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

// ----- Image Texture -----

/// Image texture that maps (u,v) coordinates to pixel colors from an image.
#[derive(Clone)]
pub struct ImageTexture {
    image_data: ImageData, // Ownes the image data
}

impl ImageTexture {
    /// Constructor from a filename.
    pub fn new(filename: &str) -> Self {
        Self {
            image_data: ImageData::new(filename),
        }
    }

    /// Value method returns the color at (u,v) from the image data.
    #[inline]
    pub fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        // If we have no texture data, then return solid cyan as a debugging aid.
        if self.image_data.height() == 0 { return Color::new(0.0, 1.0, 1.0); }

        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0); // Flip V to image coordinates (0 at top left)

        let i = (u * self.image_data.width() as f64) as u32; // u * width = pixel location in x direction
        let j = (v * self.image_data.height() as f64) as u32; // v * height = pixel location in y direction

        self.image_data.pixel_data(i, j) // Get the pixel color at (i, j)
    }
}

// ----- Noise Texture -----

/// Noise texture that uses Perlin noise to generate a procedural texture.
#[derive(Clone)]
pub struct NoiseTexture {
    noise: Arc<Noise>,
    scale: f64,
}

impl NoiseTexture {
    /// Constructor from a scale.
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Arc::new(Noise::perlin()),
            scale,
        }
    }

    /// Value method returns the noise texture color at (u,v,p).
    #[inline]
    pub fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new(0.5, 0.5, 0.5) * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turbulence(p, 7)).sin())
    }
}