use crate::image_data::ImageData;
use crate::material::ScatterRecord;
use crate::prelude::*;
use crate::hittable::{Hittable, HitRecord};
use crate::pdf::*;
use crate::color::{linear_to_srgb_u8};

// External crates
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::sync::Arc;
use std::time::Instant;

/// Camera struct defining the viewpoint and rendering parameters.
pub struct Camera {
    /// Ratio of image width over height
    pub aspect_ratio: f64,
    /// Rendered image width in pixel count
    pub image_width: u32,
    /// Number of samples per pixel for anti-aliasing
    pub samples_per_pixel: u32,
    /// Maximum ray bounce depth
    pub max_depth: u32,
    /// Background color
    pub background: Color,

    /// Vertical view angle (field of view)
    pub v_fov: f64,
    /// Point camera is looking from
    pub look_from: Point3,
    /// Point camera is looking at
    pub look_at: Point3,
    /// "Up" direction for the camera
    pub v_up: Vec3,

    /// Variation angle of rays through each pixel
    pub aperture_angle: f64,
    /// Distance from camera lookfrom point to plane of perfect focus
    pub focus_dist: f64,

    /// Name of the scene for output file naming
    pub scene_name: String,
    /// Whether to append scene characteristics to output filename
    pub append_data: bool,

    /// Whether to apply OIDN denoising to the final image.
    /// If `color_buffer` is provided, this is overridden to true.
    pub denoise: bool,
    /// Whether to save additional Arbitrary Output Variables.
    /// If `compute_color` is false, this is overridden to true.
    pub save_aovs: bool,
    /// Whether to compute the color buffer. If false, overrides `save_aovs` to true.
    pub compute_color: bool,
    /// Optional buffer to hold linear RGB color data for denoising or AOV saving
    color_buffer: Option<ImageData>,

    /// Rendered image height
    image_height: u32,
    /// Color scale factor for a sum of pixel samples
    pixel_samples_scaled: f64,
    /// Square root of samples per pixel
    sqrt_spp: u32,
    /// Recpiprocal of square root of samples per pixel, used for stratified sampling offsets
    recip_sqrt_spp: f64,
    /// Camera center
    center: Point3,
    /// Location of pixel 0, 0
    pixel00_loc: Point3,
    /// Offset to pixel to the right
    pixel_delta_u: Vec3,
    /// Offset to pixel below
    pixel_delta_v: Vec3,
    /// Camera coordinate system basis vector u
    u: Vec3,
    /// Camera coordinate system basis vector v
    v: Vec3,
    /// Camera coordinate system basis vector w
    w: Vec3,
    /// Aperture disk basis vector u
    aperture_disk_u: Vec3,
    /// Aperture disk vertical radius
    aperture_disk_v: Vec3,
}

impl Camera {
    // ----- Public -----

    /// Render the given world and save the resulting image.
    /// If sample_target is provided, it is used for importance sampling in ray_color().
    /// Optionally saves AOVs for albedo and normal if save_aovs is true, which can be used for denoising or debugging.
    pub fn render (&mut self, world: impl Into<Hittable>, sample_target: Option<Arc<Hittable>>) {
        let world: Hittable = world.into();

        self.initialize();

        let start_time = Instant::now();

        let width = self.image_width;
        let height = self.image_height;
        let max_depth = self.max_depth;
        let row_len = (width as usize) * 3; // Number of f32 values in a row of RGB pixels

        // Flags
        let buffer_provided = self.color_buffer.is_some();
        self.compute_color = self.compute_color && !buffer_provided;
        let using_aovs = self.denoise || self.save_aovs || !self.compute_color; 
        self.denoise = self.denoise && (self.compute_color || buffer_provided);

        // Progress bar by row
        let bar = Self::create_progress_bar(height as u64);

        // Linear RGB buffers (f32) + AOVs, only allocate AOV buffers if needed
        //let mut color_linear = vec![0f32; row_len * (height as usize)];
        let mut color_linear = Vec::new();
        if buffer_provided {
            color_linear = self.color_buffer.take().unwrap().data_vec();
        } else if self.compute_color {
            color_linear = vec![0f32; row_len * (height as usize)];
        }
        let mut albedo = if using_aovs { vec![0f32; row_len * (height as usize)] } else { Vec::new() };
        let mut normal = if using_aovs { vec![0f32; row_len * (height as usize)] } else { Vec::new() };

        // Parallelize over rows, each row chunk is disjoint
        if self.compute_color && !using_aovs { // Fill only color_linear buffer
            color_linear
                .par_chunks_mut(row_len)
                .enumerate()
                .for_each(|(j, row_c)| {
                    for i in 0..(width as usize) {
                        let mut pixel_color = Color::zero();
                        let mut rec = HitRecord::new();
                        for s_j in 0..self.sqrt_spp {
                            for s_i in 0..self.sqrt_spp {
                                let r = self.get_ray(i as u32, j as u32, s_i, s_j);
                                // Color
                                pixel_color += self.ray_color(&r, max_depth, &world, sample_target.as_ref(), &mut rec);
                            }
                        }
                        pixel_color *= self.pixel_samples_scaled;
                        let off = i * 3; // Offset into the row for this pixel
                        row_c[off..off + 3].copy_from_slice(&[pixel_color.x() as f32, pixel_color.y() as f32, pixel_color.z() as f32]);
                    }
                    bar.inc(1);
                });
        } else if self.compute_color && using_aovs { // Fill color_linear, albedo, and normal buffers
            color_linear
                .par_chunks_mut(row_len)
                .zip(albedo.par_chunks_mut(row_len))
                .zip(normal.par_chunks_mut(row_len))
                .enumerate()
                .for_each(|(j, ((row_c, row_a), row_n))| {
                    for i in 0..(width as usize) {
                        let mut pixel_color = Color::zero();
                        let mut alb_acc = Color::zero();
                        let mut nrm_acc = Vec3::zero();
                        let mut rec = HitRecord::new();
                        for s_j in 0..self.sqrt_spp {
                            for s_i in 0..self.sqrt_spp {
                                let r: Ray = self.get_ray(i as u32, j as u32, s_i, s_j);
                                // Color
                                pixel_color += self.ray_color(&r, max_depth, &world, sample_target.as_ref(), &mut rec);
                                // AOVs
                                if let Some((alb, n)) = self.primary_aov(&r, &world) {
                                    alb_acc += alb;
                                    nrm_acc += n;
                                }
                            }
                        }
                        pixel_color *= self.pixel_samples_scaled;
                        let alb = alb_acc * self.pixel_samples_scaled;
                        let nrm = Vec3::safe_unit_vector(&(nrm_acc * self.pixel_samples_scaled));

                        let off = i * 3; // Offset into the row for this pixel
                        row_c[off..off + 3].copy_from_slice(&[pixel_color.x() as f32, pixel_color.y() as f32, pixel_color.z() as f32]);
                        row_a[off..off + 3].copy_from_slice(&[alb.x() as f32, alb.y() as f32, alb.z() as f32]);
                        row_n[off..off + 3].copy_from_slice(&[nrm.x() as f32, nrm.y() as f32, nrm.z() as f32]);
                    }
                    bar.inc(1);
                });
        } else { // Fill only albedo and normal buffers
            albedo
                .par_chunks_mut(row_len)
                .zip(normal.par_chunks_mut(row_len))
                .enumerate()
                .for_each(|(j, (row_a, row_n))| {
                    for i in 0..(width as usize) {
                        let mut alb_acc = Color::zero();
                        let mut nrm_acc = Vec3::zero();
                        for s_j in 0..self.sqrt_spp {
                            for s_i in 0..self.sqrt_spp {
                                let r: Ray = self.get_ray(i as u32, j as u32, s_i, s_j);
                                if let Some((alb, n)) = self.primary_aov(&r, &world) {
                                    alb_acc += alb;
                                    nrm_acc += n;
                                }
                            }
                        }
                        let alb = alb_acc * self.pixel_samples_scaled;
                        let nrm = Vec3::safe_unit_vector(&(nrm_acc * self.pixel_samples_scaled));

                        let off = i * 3; // Offset into the row for this pixel
                        row_a[off..off + 3].copy_from_slice(&[alb.x() as f32, alb.y() as f32, alb.z() as f32]);
                        row_n[off..off + 3].copy_from_slice(&[nrm.x() as f32, nrm.y() as f32, nrm.z() as f32]);
                    }
                    bar.inc(1);
                });
        }
        bar.finish_with_message("Render complete!");

        // Calculate elapsed time
        let elapsed = start_time.elapsed();
        let minutes = elapsed.as_secs() / 60;
        let seconds = elapsed.as_secs() % 60;
        let time_str = format!("{}m{}s", minutes, seconds);

        // Generate base filename with dimensions and characteristics
        if self.scene_name.is_empty() { self.scene_name = "render".to_string(); }
        let base = if self.append_data {
            format!("{}_{}x{}_{}spp_{}depth_{}", self.scene_name, width, height, self.samples_per_pixel, max_depth, time_str)
        } else {
            self.scene_name.clone()
        };
        
        // Save original (linear->sRGB u8)
        if !buffer_provided && !color_linear.is_empty() {
            let orig_u8 = linear_to_srgb_u8(&color_linear);
            image::RgbImage::from_raw(width, height, orig_u8)
                .expect("Buffer size mismatch")
                .save(format!("{}.png", base))
                .expect("Failed to save original image");
        }

        // Optional OIDN denoise
        if self.denoise {
            let den = self.denoise_oidn(&color_linear, Some(&albedo), Some(&normal), width, height);
            let den_u8 = linear_to_srgb_u8(&den);
            image::RgbImage::from_raw(width, height, den_u8)
                .expect("Buffer size mismatch")
                .save(format!("{}_denoised.png", base))
                .expect("Failed to save denoised image");
            eprintln!("Images saved to {}.png and {}_denoised.png", base, base);
        } else {
            eprintln!("Image saved to {}.png", base);
        }
        
        // Optional AOV visualizations
        if self.save_aovs {
            let alb_u8 = linear_to_srgb_u8(&albedo);
            let _ = image::RgbImage::from_raw(width, height, alb_u8)
                .and_then(|img| img.save(format!("{}_albedo.png", base)).ok());
            eprintln!("Albedo AOV saved to {}_albedo.png", base);

            // Map normals [-1,1] -> [0, 1] for viewing
            let mut nrm_vis = normal.clone();
            for v in nrm_vis.iter_mut() { *v = 0.5 * (*v + 1.0); }
            let nrm_u8 = linear_to_srgb_u8(&nrm_vis);
            let _ = image::RgbImage::from_raw(width, height, nrm_u8)
                .and_then(|img| img.save(format!("{}_normal.png", base)).ok());
            eprintln!("Normal AOV saved to {}_normal.png", base);
        }
    }

    /// Constructor for high-quality default camera.
    /// - Aspect Ratio: 16:9
    /// - Image Width: 1200
    /// - Samples per Pixel: 500
    /// - Max Depth: 50
    /// - Background Color: Light blue sky
    /// - Vertical FOV: 20 degrees
    pub fn high_quality_default() -> Self {
        let mut cam = Camera::default();
        cam.aspect_ratio = 16.0 / 9.0;
        cam.image_width = 1200;
        cam.samples_per_pixel = 500;
        cam.max_depth = 50;
        cam.v_fov = 20.0;
        cam
    }

    // ----- Private -----

    /// Create and configure a progress bar.
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

    /// Initialize camera parameters based on current settings.
    fn initialize(&mut self) {
        // Check if a color buffer was provided, if so set image dimensions
        if self.color_buffer.is_some() {
            let image = self.color_buffer.as_ref().unwrap();
            self.image_width = image.width();
            self.image_height = image.height();
        } else {
            // If no color buffer, calculate image height from width and aspect ratio
            self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
            self.image_height = if self.image_height < 1 { 1 } else { self.image_height };
        }

        self.sqrt_spp = (f64::sqrt(self.samples_per_pixel as f64)) as u32;
        self.pixel_samples_scaled = 1.0 / (self.sqrt_spp * self.sqrt_spp) as f64;
        self.recip_sqrt_spp = 1.0 / (self.sqrt_spp as f64);

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

    /// Get a ray from the camera through pixel (i,j) and subpixel sample (s_i, s_j).
    fn get_ray(&self, i: u32, j: u32, s_i: u32, s_j: u32) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j for stratified sample square s_i, s_j.

        let offset = self.sample_square_stratified(s_i, s_j);
        let pixel_sample = self.pixel00_loc
                         + (i as f64 + offset.x()) * self.pixel_delta_u
                         + (j as f64 + offset.y()) * self.pixel_delta_v;  

        // Use ideal or realistic aperture based on aperture setting
        let ray_origin = if self.aperture_angle <= 0.0 { self.center } else { self.aperture_disk_sample() }; 
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_f64();

        Ray::new_with_time(ray_origin, ray_direction, ray_time)
    }

    /// Returns a stratified random point in the unit square sub-pixel specified by grid
    /// indices s_i and s_j.
    fn sample_square_stratified(&self, s_i: u32, s_j: u32) -> Vec3 {
        // Returns the vector to a random point in the square sub-pixel specified by grid
        // indices s_i and s_j, for an idealized unit square pixel [-.5,-.5] to [+.5,+.5].

        let px = (s_i as f64 + random_f64()) * self.recip_sqrt_spp - 0.5;
        let py = (s_j as f64 + random_f64()) * self.recip_sqrt_spp - 0.5;
        
        Vec3::new(px, py, 0.0)
    }

    /// Returns a random point in the camera aperture disk.
    fn aperture_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_circle();
        self.center + (p.x() * self.aperture_disk_u) + (p.y() * self.aperture_disk_v)
    } 

    /// Compute the color seen along a ray.
    #[inline]
    fn ray_color(&self, r: &Ray, depth: u32, world: &Hittable, sample_target: Option<&Arc<Hittable>>, rec: &mut HitRecord) -> Color { // TODO: change method declarations all over the place to separate input parameters onto separate lines for readability
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 { return Color::zero(); }

        // If ray hits nothing, return background color
        if !world.hit(r, &Interval::new(0.001, f64::INFINITY), rec) {
            return self.background;
        }

        // Testing: return just the normal for debugging:
        #[cfg(feature = "normals")]
        { return 0.5 * rec.normal + Color::new(0.5, 0.5, 0.5); }

        // TODO: Consider simplifying emitted() to just take rec, since rec.u, rec.v, rec.point are redundant
        // Emitted light from the hit point itself, before scattering
        let emitted_color = rec.material.emitted(r, rec, rec.u, rec.v, &rec.point);

        // Ask the material how it wants to scatter
        let mut srec = ScatterRecord::default();
        if !rec.material.scatter(r, rec, &mut srec) {
            return emitted_color;
        }

        // Specular path: follow the provided ray with no PDF work
        if srec.skip_pdf {
            let spec_color = self.ray_color(&srec.skip_pdf_ray, depth - 1, world, sample_target, rec);
            return emitted_color + srec.attenuation * spec_color;
        }

        // Diffuse path: build mixture PDF or just use material PDF if no sample target
        let mat_pdf = srec
            .pdf_ptr
            .as_ref()
            .expect("scatter: pdf_ptr must be Some when skip_pdf is false")
            .clone();
        // Branch: with or without importance sampling
        let (scattered, pdf_value) = if let Some(target) = sample_target {
            // Importance sampling: build mixture PDF from light + material PDFs
            let importance_sampling_pdf = PDF::hittable(target.clone(), rec.point);
            let mixture_pdf = PDF::mixture(importance_sampling_pdf, mat_pdf);
            let s = Ray::new_with_time(rec.point, mixture_pdf.generate(), r.time);
            let v = mixture_pdf.value(&s.direction);
            (s, v)
        } else {
            // No importance sampling: just use the material PDF
            let s = Ray::new_with_time(rec.point, mat_pdf.generate(), r.time);
            let v = mat_pdf.value(&s.direction);
            (s, v)
        };

        // Guard against invalid or zero PDFs, which would cause NaNs (0/0, inf)
        if pdf_value <= 0.0 || !pdf_value.is_finite() {
            return emitted_color;
        }
        let scattering_pdf = rec.material.scattering_pdf(r, rec, &scattered);
        if scattering_pdf <= 0.0 || !scattering_pdf.is_finite() {
            return emitted_color;
        }
        
        let sample_color = self.ray_color(&scattered, depth - 1, world, sample_target, rec);
        let scattered_color = (srec.attenuation * scattering_pdf * sample_color) / pdf_value;

        emitted_color + scattered_color
    }

    /// First-hit AOVs for albedo and normal. World- or view-space normals accepted by OIDN.
    fn primary_aov(&self, r: &Ray, world: &Hittable) -> Option<(Color, Vec3)> {
        let mut rec = HitRecord::new();
        if world.hit(r, &Interval::new(0.001, f64::INFINITY), &mut rec) {
            let alb = rec.material.albedo_hint(&rec);
            let n = rec.normal;
            Some((alb, n))
        } else {
            None
        }
    }

    /// Run OIDN RayTracing denoiser with optional AOVs.
    fn denoise_oidn(
        &self,
        color: &[f32],
        albedo: Option<&[f32]>,
        normal: Option<&[f32]>,
        width: u32,
        height: u32,
    ) -> Vec<f32> {
        let device = oidn::Device::new();
        let mut out = vec![0f32; color.len()];
        let mut rt = oidn::RayTracing::new(&device);
        rt.hdr(true)
            .image_dimensions(width as usize, height as usize);
        rt.clean_aux(true);

        if let (Some(a), Some(n)) = (albedo, normal) {
            rt.albedo_normal(a, n);
        } else if let Some(a) = albedo {
            rt.albedo(a);
        }

        rt.filter(color, &mut out).expect("Filter config error!");
        if let Err(e) = device.get_error() { eprintln!("OIDN error: {}", e.1); }
        out
    }

    /// Set a color buffer to be used for denoising.
    pub fn set_color_buffer(&mut self, buffer: ImageData) {
        self.color_buffer = Some(buffer);
    }
}

impl Default for Camera {
    /// Implement default camera settings
    /// - Aspect Ratio: 16:9
    /// - Image Width: 400
    /// - Samples per Pixel: 100
    /// - Max Depth: 50
    /// - Background Color: Light blue
    /// - Vertical FOV: 90 degrees
    /// - Look From: (0, 0, 0)
    /// - Look At: (0, 0, -1)
    /// - V Up: (0, 1, 0)
    /// - Aperture Angle: 0 (pinhole)
    /// - Focus Dist: 1
    /// - Append Data: true
    fn default() -> Self {
        Camera {
            // Public
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            samples_per_pixel: 100,
            max_depth: 50,
            background: Color::new(0.70, 0.80, 1.00), // Light blue sky

            v_fov: 90.0,
            look_from: Point3::new(0.0, 0.0, 0.0),
            look_at: Point3::new(0.0, 0.0, -1.0),
            v_up: Vec3::new(0.0, 1.0, 0.0),

            aperture_angle: 0.0,
            focus_dist: 1.0,

            scene_name: String::new(),
            append_data: true,

            denoise: false,
            save_aovs: false,
            compute_color: true,
            color_buffer: None,

            // Private
            // Will be set in initialize()
            image_height: 0,
            pixel_samples_scaled: 0.0,
            sqrt_spp: 0,
            recip_sqrt_spp: 0.0,
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