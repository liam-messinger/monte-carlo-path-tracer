use crate::vec3::Vec3;
use image::Rgb;

/// Color type alias for Vec3.
pub type Color = Vec3;

impl Color {
    /// Convert a color with components in [0,1] range to RGB bytes [0,255].
    pub fn as_rgb(&self) -> Rgb<u8> {
        let r = self.x();
        let g = self.y();
        let b = self.z();

        // Apply a linear to gamma transform for gamma 2
        let r = Color::linear_to_gamma(r);
        let g = Color::linear_to_gamma(g);
        let b = Color::linear_to_gamma(b);

        // Translate the [0,1] component values to the byte range [0,255]
        let rbyte = (255.999 * r.clamp(0.0, 1.0)) as u8;
        let gbyte = (255.999 * g.clamp(0.0, 1.0)) as u8;
        let bbyte = (255.999 * b.clamp(0.0, 1.0)) as u8;

        // Return as an RGB struct
        Rgb([rbyte, gbyte, bbyte])
    }

    /// Apply a linear to gamma correction (gamma 2) to a color component.
    fn linear_to_gamma(linear_component: f64) -> f64 {
        if linear_component > 0.0 {
            linear_component.sqrt()
        } else {
            0.0
        }
    }

    // ----------------- Utility functions -----------------

    /// Create a new color from RGB components in [0,1] range.
    pub fn rgb(r: f64, g: f64, b: f64) -> Color {
        Color::new(r, g, b)
    }

    /// Create a new color from RGB components in [0,255] range.
    pub fn rgb_bytes(r: u8, g: u8, b: u8) -> Color {
        Color::new(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0)
    }
}

// ----------------- Buffer color conversion helpers -----------------

/// Convert a single linear RBG channel (f32) to sRGB u8 with gamma 2.0 correction.
#[inline]
pub fn linear_to_srgb(x: f32) -> u8 {
    let g = if x > 0.0 { x.sqrt() } else { 0.0 };
    (255.999 * g.clamp(0.0, 1.0)) as u8
}

/// Convert a buffer of linear RGB channels (f32) to sRGB u8 with gamma 2.0 correction.
/// Input: row-major RGBRGB..., length divisible by 3.
pub fn linear_to_srgb_u8(color: &[f32]) -> Vec<u8> {
    let mut out = vec![0u8; color.len()];
    for i in 0..(color.len() / 3) {
        out[3 * i] = linear_to_srgb(color[3 * i]);
        out[3 * i + 1] = linear_to_srgb(color[3 * i + 1]);
        out[3 * i + 2] = linear_to_srgb(color[3 * i + 2]);
    }
    out
}