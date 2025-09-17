#![allow(non_snake_case)]

// Internal modules
mod vec3;
mod color;
mod ray;

use vec3::{Vec3, Point3, unit_vector, dot, cross};
use color::{Color, rgb};
use ray::Ray;

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

// Check if hit a sphere
fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
  let oc: Vec3 = center - *r.origin();
  let a = r.direction().length_squared();
  let h = dot(r.direction(), &oc);
  let c = oc.length_squared() - radius * radius;
  let discriminant = h*h - a*c;
  
  if discriminant < 0.0 {
    return -1.0
  } else {
    return (h - discriminant.sqrt()) / a;
  }
}

// Compute the color seen along a ray
pub fn ray_color(r: &Ray) -> Color {
  let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
  if t > 0.0 {
    let N: Vec3 = unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
    return 0.5 * Color::new(N.x()+1.0, N.y()+1.0, N.z()+1.0);
  }

  let unit_direction = unit_vector(*r.direction());
  let a = 0.5 * (unit_direction.y() + 1.0);
  (1.0 - a) * rgb(1.0, 1.0, 1.0) + a * rgb(0.5, 0.7, 1.0)
}

fn main() {
  // ----- Image -----

  let aspect_ratio: f64 = 16.0 / 9.0;
  let image_width: u32 = 400;
  let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

  // ----- Camera -----

  let focal_length: f64 = 1.0;
  let viewport_height: f64 = 2.0;
  let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
  let camera_center = Point3::zero();

  // Calculate the vectors across the horizontal and down the vertical viewport edges.
  let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
  let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
  
  // Calculate the horizontal and vertical delta vectors from pixel to pixel.
  let pixel_delta_u = viewport_u / image_width as f64;
  let pixel_delta_v = viewport_v / image_height as f64;

  // Calculate the location of the upper left pixel.
  let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) 
                            - (viewport_u / 2.0) - (viewport_v / 2.0);
  let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

  // ----- Render -----

  let mut img = ImageBuffer::new(image_width, image_height); // Create image buffer

  let pb = create_progress_bar(image_height as u64); // Create progress bar

  for j in (0..image_height).rev() {
    pb.set_position(image_height as u64 - j as u64); // Progress indicator
    for i in 0..image_width {
      let pixel_center = pixel00_loc + (i as f64) * pixel_delta_u + (j as f64) * pixel_delta_v;
      let ray_direction = pixel_center - camera_center;
      let r = Ray::new(camera_center, ray_direction);

      let pixel_color = ray_color(&r);
      img.put_pixel(i, j, pixel_color.to_rgb());
    }
  }
  
  pb.finish_with_message("Rendering complete!");
  
  // Save the image
  img.save("output.png").unwrap();
  eprintln!("Image saved as output.png");
}