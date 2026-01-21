use crate::hittable::Hittable;
use crate::prelude::*;
use crate::hittable::{HitRecord};

// External crates
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
//use image::ImageBuffer;

pub struct Camera {
    pub aspect_ratio: f64,      // Ratio of image width over height
    pub image_width: u32,       // Rendered image width in pixel count
    pub samples_per_pixel: u32, // Number of samples per pixel for anti-aliasing
    pub max_depth: u32,         // Maximum ray bounce depth
    pub background: Color,      // Background color

    pub v_fov: f64,             // Vertical view angle (field of view)
    pub look_from: Point3,      // Point camera is looking from
    pub look_at: Point3,        // Point camera is looking at
    pub v_up: Vec3,             // "Up" direction for the camera

    pub aperture_angle: f64,    // Variation angle of rays through each pixel
    pub focus_dist: f64,        // Distance from camera lookfrom point to plane of perfect focus

    image_height: u32,          // Rendered image height
    pixel_samples_scaled: f64,  // Color scale factor for a sum of pixel samples
    center: Point3,             // Camera center
    pixel00_loc: Point3,        // Location of pixel 0, 0
    pixel_delta_u: Vec3,        // Offeset to pixel to the right
    pixel_delta_v: Vec3,        // Offset to pixel below
    u: Vec3,                    // Camera coordinate system basis vector u
    v: Vec3,                    // Camera coordinate system basis vector v
    w: Vec3,                    // Camera coordinate system basis vector w
    aperture_disk_u: Vec3,      // Aperture disk horizontal radius
    aperture_disk_v: Vec3,      // Aperture disk vertical radius
}

impl Camera {
    // ----- Public -----

    // TODO: Make render take a HittableList and convert to BVH automatically
    // Render the scene from this camera's point of view
    pub fn render (&mut self, world: impl Into<Hittable>) {
        let world: Hittable = world.into();

        //*
        self.initialize();

        let width = self.image_width;
        let height = self.image_height;
        let spp = self.samples_per_pixel;
        let max_depth = self.max_depth;

        // Progress bar by row
        let pb = Self::create_progress_bar(height as u64);

        // Raw RGB buffer (u8) to avoid shared ImageBuffer mutation
        let mut raw_img: Vec<u8> = vec![0u8; (width as usize) * (height as usize) * 3];

        // Prallelize over rows; each row chunk is disjoint
        let pb_row = pb.clone();
        raw_img.par_chunks_mut((width as usize) * 3)
            .enumerate()
            .for_each(|(j, row)| {
                for i in 0..(width as usize) {
                    let mut pixel_color = Color::default();
                    let mut rec = HitRecord::new();
                    for _ in 0..spp {
                        let r: Ray = self.get_ray(i as u32, j as u32);
                        pixel_color += self.ray_color(&r, max_depth, &world, &mut rec);
                    }
                    pixel_color *= self.pixel_samples_scaled;
                    let rgb = pixel_color.as_rgb();
                    row[i * 3] = rgb[0];
                    row[i * 3 + 1] = rgb[1];
                    row[i * 3 + 2] = rgb[2];
                }
                pb_row.inc(1);
            });

        pb.finish_with_message("Render complete!");

        // Build the image and save
        // Default colorspace of an ImageBuffer is sRGB
        let img = image::RgbImage::from_raw(width, height, raw_img)
            .expect("Buffer size mismatch");
        img.save("output.png").expect("Failed to save output.png");
        eprintln!("Image saved to output.png");
        //*/

        /*
        self.initialize();

        let mut img = ImageBuffer::new(self.image_width, self.image_height); // Create image buffer

        let pb = Self::create_progress_bar(self.image_height as u64); // Create progress bar

        // Loop over each pixel in the image
        for j in 0..self.image_height {
            pb.set_position(j as u64);
            for i in 0..self.image_width {
                let mut pixel_color = Color::default();
                for _sample in 0..self.samples_per_pixel {
                    let r: Ray = self.get_ray(i, j);
                    pixel_color += Camera::ray_color(&r, self.max_depth, &world);
                }
                img.put_pixel(i, j, (self.pixel_samples_scaled * pixel_color).as_rgb());
            }
        }

        pb.finish_with_message("Render complete!");

        // Save the image
        img.save("output.png").unwrap();
        eprintln!("Image saved to output.png");
        */
    }

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

        self.pixel_samples_scaled = 1.0 / self.samples_per_pixel as f64;

        self.center = self.look_from;

        // Determine viewport dimensions
        let theta = self.v_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        self.w = Vec3::unit_vector(&(self.look_from - self.look_at));
        self.u = Vec3::unit_vector(&Vec3::cross(&self.v_up, &self.w));
        self.v = Vec3::cross(&self.w, &self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u: Vec3 = viewport_width * self.u;    // Vector across viewport horizontal edge
        let viewport_v: Vec3 = viewport_height * -self.v;  // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left: Point3 = self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;                           
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Calculate the camera aperture disk basis vectors.
        let aperture_radius = self.focus_dist * (self.aperture_angle.to_radians() / 2.0).tan();
        self.aperture_disk_u = self.u * aperture_radius;
        self.aperture_disk_v = self.v * aperture_radius;
    }

    // Get a ray from the camera through pixel (i,j)
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Construct a camera ray originating from the aperture disk and directed at a randomly
        // sampled point around the pixel location i, j.

        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
                         + (i as f64 + offset.x()) * self.pixel_delta_u
                         + (j as f64 + offset.y()) * self.pixel_delta_v;  

        // Use ideal or realistic aperture based on aperture setting
        let ray_origin = if self.aperture_angle <= 0.0 { self.center } else { self.aperture_disk_sample() }; 
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_f64();

        Ray::new_with_time(ray_origin, ray_direction, ray_time)
    }

    // Returns a random point on the unit square
    fn sample_square() -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
    }

    // Returns a random point in the camera aperture disk.
    fn aperture_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_circle();
        self.center + (p.x() * self.aperture_disk_u) + (p.y() * self.aperture_disk_v)
    } 

    // Compute the color seen along a ray
    #[inline]
    fn ray_color(&self, r: &Ray, depth: u32, world: &Hittable, rec: &mut HitRecord) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 { return Color::zero(); }

        // If ray hits nothing, return background color
        if !world.hit(r, &Interval::new(0.001, f64::INFINITY), rec) {
            return self.background;
        }

        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        let emitted_color = rec.material.emitted(rec.u, rec.v, &rec.point);

        // If the material does not scatter, return emitted light only
        if !rec.material.scatter(r, rec, &mut attenuation, &mut scattered) {
            return emitted_color;
        }

        let scattered_color = attenuation * self.ray_color(&scattered, depth - 1, world, rec);
        emitted_color + scattered_color
    }

    // Function to set camera parameters to a high-quality default
    pub fn set_high_quality_settings(&mut self) {
        self.aspect_ratio = 16.0 / 9.0;
        self.image_width = 1200;
        self.samples_per_pixel = 500;
        self.max_depth = 50;

        self.v_fov = 20.0;
    }

    // Constructor for high-quality default camera
    pub fn high_quality_default() -> Self {
        let mut cam = Camera::default();
        cam.set_high_quality_settings();
        cam
    }
}

// Implement default camera settings
impl Default for Camera {
    fn default() -> Self {
        Camera {
            // Public
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            samples_per_pixel: 50,
            max_depth: 50,
            background: Color::new(0.70, 0.80, 1.00), // Light blue sky

            v_fov: 90.0,
            look_from: Point3::new(0.0, 0.0, 0.0),
            look_at: Point3::new(0.0, 0.0, -1.0),
            v_up: Vec3::new(0.0, 1.0, 0.0),

            aperture_angle: 0.0,
            focus_dist: 1.0,

            // Private
            // Will be set in initialize()
            image_height: 0,
            pixel_samples_scaled: 0.0,
            center: Point3::zero(),
            pixel00_loc: Point3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
            u: Vec3::zero(),
            v: Vec3::zero(),
            w: Vec3::zero(),
            aperture_disk_u: Vec3::zero(),
            aperture_disk_v: Vec3::zero(),
        }
    }
}