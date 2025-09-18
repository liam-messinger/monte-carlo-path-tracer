#![allow(non_snake_case)]

// Internal modules
mod color;
mod hittable;
mod ray;
mod vec3;

use color::{Color, rgb};
use ray::Ray;
use vec3::{Point3, Vec3, unit_vector};

use crate::hittable::{HitRecord, Hittable, Sphere};

// External crates
use image::ImageBuffer;
use indicatif::{ProgressBar, ProgressStyle};

// Create and configure a progress bar
fn create_progress_bar(total: u64) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} scanlines ({eta}) {msg}")
            .unwrap()
            .progress_chars("#>-")
    );
    pb
}

// Compute the color seen along a ray
pub fn ray_color(r: &Ray) -> Color {
    // Create sphere
    let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    let mut rec = HitRecord::new();

    // Check for sphere hit
    if sphere.hit(r, 0.0, 5.0, &mut rec) {
        return 0.5
            * Color::new(
                rec.normal.x() + 1.0,
                rec.normal.y() + 1.0,
                rec.normal.z() + 1.0,
            );
    }

    // Sky background
    let unit_direction = unit_vector(*r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * rgb(1.0, 1.0, 1.0) + a * rgb(0.5, 0.7, 1.0)
}

fn main() {
    // ----- Image -----

    let ASPECT_RATIO: f64 = 16.0 / 9.0;
    let IMAGE_WIDTH: u32 = 400;
    let IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    // ----- Camera -----

    let FOCAL_LENGTH: f64 = 1.0;
    let VIEWPORT_HEIGHT: f64 = 2.0;
    let VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);
    let CAMERA_CENTER = Point3::zero();

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        CAMERA_CENTER - Vec3::new(0.0, 0.0, FOCAL_LENGTH) - (viewport_u / 2.0) - (viewport_v / 2.0);
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // ----- Render -----

    let mut img = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT); // Create image buffer

    let pb = create_progress_bar(IMAGE_HEIGHT as u64); // Create progress bar

    for j in (0..IMAGE_HEIGHT).rev() {
        pb.set_position(IMAGE_HEIGHT as u64 - j as u64); // Progress indicator
        for i in 0..IMAGE_WIDTH {
            let pixel_center =
                pixel00_loc + (i as f64) * pixel_delta_u + (j as f64) * pixel_delta_v;
            let ray_direction = pixel_center - CAMERA_CENTER;
            let r = Ray::new(CAMERA_CENTER, ray_direction);

            let pixel_color = ray_color(&r);
            img.put_pixel(i, j, pixel_color.to_rgb());
        }
    }

    pb.finish_with_message("Rendering complete!");

    // Save the image
    img.save("output.png").unwrap();
    eprintln!("Image saved as output.png");
}