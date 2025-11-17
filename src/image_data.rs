use std::path::{PathBuf};

use crate::color::Color;

#[derive(Clone)]
pub struct ImageData {
    width: u32,
    height: u32,
    // Linear RGB floats, row-major, 3 floats per pixel
    data: Vec<f32>,
}

impl ImageData {
    // Loads an image from the given filename.
    // Searches for the  file in the current directory, in 'textures/' and '../textures/'.
    // Writes to "data" in row-major order, 3 floats per pixel (R, G, B).
    // Assumptions: The image is in a format supported by the 'image' crate and is RGB8.
    pub fn new(filename: &str) -> Self {
        let search_paths = [ // Search in multiple locations
            PathBuf::from(filename),
            PathBuf::from("textures").join(filename),
            PathBuf::from("../textures").join(filename),
        ];

        // Try to load the image from each path
        for path in &search_paths {
            if let Ok(img) = image::open(path) {
                let rgb_img = img.to_rgb32f();
                let w = rgb_img.width();
                let h = rgb_img.height();

                // Convert sRGB -> linear (kept as f32)
                let mut data = rgb_img.into_raw();
                for c in data.iter_mut() {
                    *c = srgb_unit_to_linear(*c);
                }

                return Self { width: w, height: h, data };
            }
        }

        // If loading failed, print an error and return an empty image
        eprintln!("ERROR: Could not load image file '{}'.", filename);
        Self {
            width: 0,
            height: 0,
            data: Vec::new(),
        }
    }

    // Returns true if the image has no data.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    // Returns the width of the image.
    pub fn width(&self) -> u32 {
        self.width
    }

    // Returns the height of the image.
    pub fn height(&self) -> u32 {
        self.height
    }

    // Returns the color of the pixel at (x, y).
    // If the image data is not available, returns magenta.
    // Coordinates are clamped to the image dimensions.
    // Assumes (0,0) is the top-left corner and (width-1, height-1) is the bottom-right corner.
    // Returns color components in [0.0, 1.0].
    pub fn pixel_data(&self, x: u32, y: u32) -> Color {
        if self.is_empty() {
            return Color::new(1.0, 0.0, 1.0); // Magenta
        }

        let x = x.clamp(0, self.width - 1);
        let y = y.clamp(0, self.height - 1);

        let offset = (y * self.width + x) as usize * 3;
        let r = self.data[offset] as f64;
        let g = self.data[offset + 1] as f64;
        let b = self.data[offset + 2] as f64;

        Color::new(r, g, b)
    }
}

// sRGB [0, 1] -> linear [0, 1] as f32
fn srgb_unit_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}