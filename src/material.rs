use std::sync::Arc;

use crate::hittable::{HitRecord};
use crate::prelude::*;
use crate::texture::{Texture, SolidColor};

// ----- Enum for different material types -----

/// Material enum encapsulating different material types.
#[derive(Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
    Isotropic(Isotropic),
    // Etc.
}

impl Material {
    /// Implementation of scatter method for Material enum
    #[inline]
    pub fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        match self {
            Material::Lambertian(mat) => mat.scatter(ray_in, rec, attenuation, scattered),
            Material::Metal(mat) => mat.scatter(ray_in, rec, attenuation, scattered),
            Material::Dielectric(mat) => mat.scatter(ray_in, rec, attenuation, scattered),
            Material::DiffuseLight(_) => false, // DiffuseLight does not scatter
            Material::Isotropic(mat) => mat.scatter(ray_in, rec, attenuation, scattered),
            // Etc.
        }
    }

    /// Implementation of emitted method for Material enum.
    #[inline]
    pub fn emitted(&self, u: f64, v: f64, point: &Point3) -> Color {
        match self {
            Material::DiffuseLight(mat) => mat.emitted(u, v, point),
            _ => Color::zero(), // Non-emissive materials emit no light
        }
    }

    // Convenience Arc constructors

    /// Create an Arc<Material> lambertian from a Color.
    pub fn lambertian(albedo: Color) -> Arc<Material> {
        Arc::new(Material::Lambertian(Lambertian::new(albedo)))
    }
    /// Create an Arc<Material> lambertian from a Texture.
    pub fn lambertian_tex(tex: Arc<Texture>) -> Arc<Material> {
        Arc::new(Material::Lambertian(Lambertian::from_texture(tex)))
    }
    /// Create an Arc<Material> metal from albedo Color and fuzz factor.
    pub fn metal(albedo: Color, fuzz: f64) -> Arc<Material> {
        Arc::new(Material::Metal(Metal::new(albedo, fuzz)))
    }
    /// Create an Arc<Material> dielectric from refraction index.
    pub fn dielectric(refraction_index: f64) -> Arc<Material> {
        Arc::new(Material::Dielectric(Dielectric::new(refraction_index)))
    }
    /// Create an Arc<Material> diffuse light from emit Color.
    pub fn diffuse_light(emit_color: Color) -> Arc<Material> {
        Arc::new(Material::DiffuseLight(DiffuseLight::new(emit_color)))
    }
    /// Create an Arc<Material> diffuse light from Texture.
    pub fn diffuse_light_tex(tex: Arc<Texture>) -> Arc<Material> {
        Arc::new(Material::DiffuseLight(DiffuseLight::from_texture(tex)))
    }
    /// Create an Arc<Material> isotropic from a Color.
    pub fn isotropic(albedo: Color) -> Arc<Material> {
        Arc::new(Material::Isotropic(Isotropic::new(albedo)))
    }
    /// Create an Arc<Material> isotropic from a Texture.
    pub fn isotropic_tex(tex: Arc<Texture>) -> Arc<Material> {
        Arc::new(Material::Isotropic(Isotropic::from_texture(tex)))
    }
}

// Default material (Lambertian gray)
impl Default for Material {
    fn default() -> Self {
        Material::Lambertian(Lambertian::new(Color::new(0.5, 0.5, 0.5)))
    }
}

// ----- Macros to implement From trait for material types -----
// From material type to Material
macro_rules! impl_material_from {
    ($($variant:ident),+ $(,)?) => {
        $(
            impl From<$variant> for Material {
                fn from(mat: $variant) -> Self {
                    Material::$variant(mat)
                }
            }
        )+
    };
}
impl_material_from!(Lambertian, Metal, Dielectric, DiffuseLight);

// From material type to Arc<Material>
macro_rules! impl_arc_material_from {
    ($($variant:ident),+ $(,)?) => {
        $(
            impl From<$variant> for Arc<Material> {
                fn from(mat: $variant) -> Self {
                    Arc::new(Material::$variant(mat))
                }
            }
        )+
    };
}
impl_arc_material_from!(Lambertian, Metal, Dielectric, DiffuseLight);

// ----- Lambertian (diffuse) Material -----

/// A Lambertian material defined by a texture.
#[derive(Clone)]
pub struct Lambertian {
    tex: Arc<Texture>,
}

impl Lambertian {
    /// Constructor from a Color.
    pub fn new(albedo: Color) -> Self {
        Self { 
            tex: Arc::new(Texture::from(SolidColor::new(albedo))),
        }
    }

    /// Constructor from a Texture reference counter.
    pub fn from_texture(tex: Arc<Texture>) -> Self {
        Self { 
            tex,
        }
    }

    /// Scatter method for a Lambertian material.
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

// ----- Metal Material -----
// TODO: Add textures to Metal material

/// A Metal material defined by an albedo color and fuzziness.
#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    /// Constructor for Metal material.
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { 
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }

    /// Scatter method for a Metal material.
    #[inline]
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut reflected = Vec3::reflect(&ray_in.direction, &rec.normal);
        reflected = Vec3::unit_vector(&reflected) + (self.fuzz * Vec3::random_unit_vector());
        *scattered = Ray::new_with_time(rec.point, reflected, ray_in.time);
        *attenuation = self.albedo;
        Vec3::dot(&scattered.direction, &rec.normal) > 0.0
    }
}

// ----- Dielectric (glass-like) Material -----
// TODO: Add textures to Dielectric material

/// A Dielectric material defined by its refractive index.
#[derive(Clone)]
pub struct Dielectric {
    // Refractive index in vacuum or air, or the ratio of the material's refractive index over
    // the refractive index of the enclosing media
    refraction_index: f64, // Index of Refraction
}

impl Dielectric {
    /// Constructor for a Dielectric material.
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    /// A reflectance function using Schlick's approximation.
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0: f64 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0_squared: f64 = r0 * r0;
        r0_squared + (1.0 - r0_squared) * f64::powf(1.0 - cosine, 5.0)
    }

    /// Scatter method for a Dielectric material.
    #[inline]
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0); // No attenuation for Dielectric
        let ri: f64 = if rec.front_face { 1.0 / self.refraction_index } else { self.refraction_index };

        let unit_direction = Vec3::unit_vector(&ray_in.direction);
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

// ----- Diffuse Light Material -----

/// A Diffuse Light material defined by its emission texture.
#[derive(Clone)]
pub struct DiffuseLight {
    tex: Arc<Texture>,
}

impl DiffuseLight {
    /// Constructor from a Color.
    pub fn new(emit_color: Color) -> Self {
        Self { 
            tex: Arc::new(Texture::from(SolidColor::new(emit_color))),
        }
    }

    /// Constructor from a Texture reference counter.
    pub fn from_texture(tex: Arc<Texture>) -> Self {
        Self { 
            tex,
        }
    }

    /// Emitted light function.
    #[inline]
    pub fn emitted(&self, u: f64, v: f64, point: &Point3) -> Color {
        self.tex.value(u, v, point)
    }
}

// ----- Isotropic (fully scattering) Material -----

/// An Isotropic material that scatters light uniformly in all directions. Primarily used for fog/volume rendering.
#[derive(Clone)]
pub struct Isotropic {
    tex: Arc<Texture>,
}

impl Isotropic {
    /// Constructor from a color.
    pub fn new(albedo: Color) -> Self {
        Self {
            tex: Arc::new(Texture::from(SolidColor::new(albedo))),
        }
    }

    /// Constructor from a texture.
    pub fn from_texture(tex: Arc<Texture>) -> Self {
        Self { tex }
    }

    /// Scatter method for Isotropic material.
    #[inline]
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *scattered = Ray::new_with_time(rec.point, Vec3::random_unit_vector(), ray_in.time);
        *attenuation = self.tex.value(rec.u, rec.v, &rec.point);
        true
    }
}