use std::sync::Arc;

use crate::hittable::{HitRecord};
use crate::prelude::*;
use crate::texture::{Texture, SolidColor};

// ----- Enum for different material types -----
#[derive(Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

// Implementation of scatter method for Material enum
impl Material {
    #[inline]
    pub fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        match self {
            Material::Lambertian(mat) => mat.scatter(ray_in, rec, attenuation, scattered),
            Material::Metal(mat) => mat.scatter(ray_in, rec, attenuation, scattered),
            Material::Dielectric(mat) => mat.scatter(ray_in, rec, attenuation, scattered),
        }
    }
}

// Default material (Lambertian gray)
impl Default for Material {
    fn default() -> Self {
        Lambertian::new(Color::new(0.5, 0.5, 0.5)).into()
    }
}

// ----- Lambertian (diffuse) Material -----

#[derive(Clone)]
pub struct Lambertian {
    tex: Arc<Texture>,
}

impl Lambertian {
    // Constructor from Color
    pub fn new(albedo: Color) -> Self {
        Self { 
            tex: Arc::new(Texture::from(SolidColor::new(albedo))),
        }
    }

    // Constructor from Texture reference counter
    pub fn from_texture(tex: Arc<Texture>) -> Self {
        Self { tex }
    }

    #[inline]
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new_with_time(rec.point, scatter_direction, ray_in.time);
        *attenuation = self.tex.value(rec.u, rec.v, &rec.point);
        true
    }   
}

// From Lambertian to Material implementation
impl From<Lambertian> for Material {
    fn from(mat: Lambertian) -> Self {
        Material::Lambertian(mat)
    }
}

// ----- Metal Material -----

#[derive(Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { 
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }

    #[inline]
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut reflected = Vec3::reflect(&ray_in.direction, &rec.normal);
        reflected = Vec3::unit_vector(reflected) + (self.fuzz * Vec3::random_unit_vector());
        *scattered = Ray::new_with_time(rec.point, reflected, ray_in.time);
        *attenuation = self.albedo;
        Vec3::dot(&scattered.direction, &rec.normal) > 0.0
    }
}

// From Metal to Material implementation
impl From<Metal> for Material {
    fn from(mat: Metal) -> Self {
        Material::Metal(mat)
    }
}

// ----- Dielectric (glass-like) Material -----

#[derive(Clone)]
pub struct Dielectric {
    // Refractive index in vacuum or air, or the ratio of the material's refractive index over
    // the refractive index of the enclosing media
    refraction_index: f64, // Index of Refraction
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    // Use Schlick's approximation for reflectance
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0: f64 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0_squared: f64 = r0 * r0;
        r0_squared + (1.0 - r0_squared) * f64::powf(1.0 - cosine, 5.0)
    }

    #[inline]
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0); // No attenuation for Dielectric
        let ri: f64 = if rec.front_face { 1.0 / self.refraction_index } else { self.refraction_index };

        let unit_direction = Vec3::unit_vector(ray_in.direction);
        let cos_theta: f64 = f64::min(Vec3::dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta: f64 = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract: bool = ri * sin_theta > 1.0;

        let direction: Vec3 = if cannot_refract || Dielectric::reflectance(cos_theta, ri) > random_f64() {
            Vec3::reflect(&unit_direction, &rec.normal)
        } else {
            Vec3::refract(&unit_direction, &rec.normal, ri)
        };

        *scattered = Ray::new_with_time(rec.point, direction, ray_in.time);
        true
    }
}

// From Dielectric to Material implementation
impl From<Dielectric> for Material {
    fn from(mat: Dielectric) -> Self {
        Material::Dielectric(mat)
    }
}