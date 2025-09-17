use image::{ImageBuffer, Rgb};

fn main() {

    // Image

    let IMAGE_WIDTH: u32 = 265;
    let IMAGE_HEIGHT: u32 = 265;

    // Create image buffer
    let mut img = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    // Render
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let r = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let g = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let b = 0.0;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            // Set pixel in image buffer
            img.put_pixel(i, j, Rgb([ir, ig, ib]));
        }
    }
    
    // Save the image
    img.save("output.png").unwrap();
    eprintln!("Done. Image saved as output.png");
}