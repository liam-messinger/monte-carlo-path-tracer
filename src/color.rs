use crate::vec3::Vec3;
use image::Rgb;

// Type alias for color
pub type Color = Vec3;

impl Color {
  // Convert a color with components in [0,1] range to RGB bytes [0,255]
  pub fn to_rgb(&self) -> Rgb<u8> {
    let r = self.x();
    let g = self.y();
    let b = self.z();

    // Translate the [0,1] component values to the byte range [0,255]
    let rbyte = (255.999 * r.clamp(0.0, 1.0)) as u8;
    let gbyte = (255.999 * g.clamp(0.0, 1.0)) as u8;
    let bbyte = (255.999 * b.clamp(0.0, 1.0)) as u8;

    Rgb([rbyte, gbyte, bbyte])
  }
}

// ----------------- Utility functions -----------------

// Create a new color from RGB components in [0,1] range
pub fn rgb(r: f64, g: f64, b: f64) -> Color {
  Color::new(r, g, b)
}

// Create a new color from RGB components in [0,255] range
pub fn rgb_bytes(r: u8, g: u8, b: u8) -> Color {
  Color::new(
    r as f64 / 255.0,
    g as f64 / 255.0,
    b as f64 / 255.0,
  )
}