use crate::prelude::*;
use crate::hittable::{HitRecord, Hittable};

// External crates
use image::ImageBuffer;
use indicatif::{ProgressBar, ProgressStyle};

pub struct Camera {
    pub aspect_ratio: f64, // Ratio of image width over height
    pub image_width: u32,  // Rendered image width in pixel count
    image_height: u32,     // Rendered image height
    center: Point3,        // Camera center
    pixel00_loc: Point3,   // Location of pixel 0, 0
    pixel_delta_u: Vec3,   // Offeset to pixel to the right
    pixel_delta_v: Vec3,   // Offset to pixel below
}

impl Camera {
    // ----- Private -----

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

    // Initialize camera parameters based on current settings
    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        self.image_height = if self.image_height < 1 { 1 } else { self.image_height };

        self.center = Point3::zero();

        // Determine viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    // Compute the color seen along a ray
    fn ray_color(&self, r: &Ray, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::new();

        if world.hit(r, Interval::new(0.0, f64::INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }

        // Sky background
        let unit_direction = unit_vector(r.direction);
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    // ----- Public -----

    // Render the scene from this camera's point of view
    pub fn render (&mut self, world: &dyn Hittable) {
        self.initialize();

        let mut img = ImageBuffer::new(self.image_width, self.image_height); // Create image buffer

        let pb = Self::create_progress_bar(self.image_height as u64); // Create progress bar

        // Loop over each pixel in the image
        for j in 0..self.image_height {
            pb.set_position(self.image_height as u64 - j as u64);
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc + (i as f64) * self.pixel_delta_u + (j as f64) * self.pixel_delta_v;
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);

                let pixel_color = self.ray_color(&r, world);
                img.put_pixel(i, j, pixel_color.to_rgb());
            }
        }

        pb.finish_with_message("Render complete!");

        // Save the image
        img.save("output.png").unwrap();
        eprint!("Image saved to output.png\n");
    }
}

// Implement default camera settings
impl Default for Camera {
    fn default() -> Self {
        let cam = Camera {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            image_height: 0, // Will be set in initialize()
            center: Point3::zero(),
            pixel00_loc: Point3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
        };
        cam
    }
}