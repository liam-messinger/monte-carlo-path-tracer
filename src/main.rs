mod vec3;
mod color;
mod ray;

use image::ImageBuffer;
use indicatif::{ProgressBar, ProgressStyle};
use color::Color;

fn main() {
  // Image parameters
  let aspect_ratio: f64 = 16.0 / 9.0;
  let image_width: u32 = 400;
  let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

  // Create image buffer
  let mut img = ImageBuffer::new(image_width, image_height);

  // Create progress bar
  let pb = ProgressBar::new(image_height as u64);
  pb.set_style(
    ProgressStyle::default_bar()
      .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} scanlines ({eta}) {msg}")
      .unwrap()
      .progress_chars("#>-")
  );

  // Render
  for j in (0..image_height).rev() {
    // Progress indicator
    pb.set_position(image_height as u64 - j as u64);

    for i in 0..image_width {
      let r = i as f64 / (image_width - 1) as f64;
      let g = j as f64 / (image_height - 1) as f64;
      let b = 0.0;

      // Create a Color
      let pixel_color = Color::rgb(r, g, b);
      
      // Convert to RGB and set pixel in image buffer
      img.put_pixel(i, j, pixel_color.to_rgb());
    }
  }
  
  pb.finish_with_message("Rendering complete!");
  
  // Save the image
  img.save("output.png").unwrap();
  eprintln!("Image saved as output.png");
}