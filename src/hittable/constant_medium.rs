use super::{Hittable, HitRecord, AABB};

use crate::interval::Interval;
use crate::material::Material;
use crate::prelude::{random_f64, EPSILON};
use crate::ray::Ray;
use crate::texture::Texture;
use crate::color::Color;
use crate::vec3::Vec3;

use std::sync::Arc;

/// A constant-desnity medium that scatters rays randomly, bounding by a convex hittable (e.g., a sphere or box).
#[derive(Clone)]
pub struct ConstantMedium {
    boundary: Arc<Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<Material>,
    bounding_box: AABB,
}

impl ConstantMedium {
    /// Constructor from solid color phase function.
    pub fn new(boundary: Arc<Hittable>, density: f64, albedo: &Color) -> Self {
        let bbox = boundary.bounding_box().clone();
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Material::isotropic(*albedo),
            bounding_box: bbox,
        }
    }

    /// Constructor from texture and density.
    pub fn from_texture(boundary: Arc<Hittable>, density: f64, tex: Arc<Texture>) -> Self {
        let bbox = boundary.bounding_box().clone();
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Material::isotropic_tex(tex),
            bounding_box: bbox,
        }
    }

    /// Volume hit method. Sample a scattering event within the medium.
    #[inline]
    pub fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord::new();
        let mut rec2 = HitRecord::new();

        // Find entry/exit points on the convex boundary.
        if !self.boundary.hit(r, &Interval::new(f64::NEG_INFINITY, f64::INFINITY), &mut rec1) {
            return false;
        }
        if !self.boundary.hit(r, &Interval::new(rec1.t + EPSILON, f64::INFINITY), &mut rec2) {
            return false;
        }

        // Clamp to the ray's active interval.
        if rec1.t < ray_t.min { rec1.t = ray_t.min; }
        if rec2.t > ray_t.max { rec2.t = ray_t.max; }
        if rec1.t >= rec2.t { return false; }
        if rec1.t < 0.0 { rec1.t = 0.0; }

        // Sample a scattering distance inside the medium.
        let ray_length = r.direction.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * (random_f64().max(EPSILON)).ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.point = r.at(rec.t);

        // Arbitrary normal/face; medium scattering doesn't depend on surface orientation.
        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.material = Arc::clone(&self.phase_function);

        true
    }

    /// Medium bounding box is the bounding box of its boundary.
    pub fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}

// From ConstantMedium to Hittable implementation
impl From<ConstantMedium> for Hittable {
    fn from(medium: ConstantMedium) -> Self {
        Hittable::ConstantMedium(medium)
    }
}