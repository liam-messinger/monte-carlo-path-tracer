mod vec3;

use image::{ImageBuffer, Rgb};
use indicatif::{ProgressBar, ProgressStyle};

fn main() {

    // Image

    let image_width: u32 = 265;
    let image_height: u32 = 265;

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
            let r = i as f32 / (image_width - 1) as f32;
            let g = j as f32 / (image_height - 1) as f32;
            let b = 0.0;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            // Set pixel in image buffer
            img.put_pixel(i, j, Rgb([ir, ig, ib]));
        }
    }
    
    pb.finish_with_message("Rendering complete!");
    
    // Save the image
    img.save("output.png").unwrap();
    eprintln!("Image saved as output.png");
}