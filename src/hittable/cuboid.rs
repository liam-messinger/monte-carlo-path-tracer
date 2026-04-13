use super::{Hittable, HitRecord, AABB, BVHNode, Quad, HittableList};

use crate::interval::Interval;
use crate::material::Material;
use crate::prelude::random_f64;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

use std::sync::Arc;

/// A cuboid defined by two opposite corners and made from 6 quads.
#[derive(Clone)]
pub struct Cuboid {
    side_list: [Quad; 6],
    side_weights: [f64; 6],
    side_bvh: BVHNode,
}

impl Cuboid {
    /// Constructor given two opposite corners and a material, returns the 3D box as a BVHNode of 6 quads.
    pub fn new(a: &Point3, b: &Point3, material: Arc<Material>) -> Self {
        // Construct the two opposite vertices with the minimum and maximum coordinates.
        let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
        let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

        let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

        let front = Quad::new(&Point3::new(min.x(), min.y(), max.z()), &dx, &dy, material.clone());
        let right = Quad::new(&Point3::new(max.x(), min.y(), max.z()), &(-dz), &dy, material.clone());
        let back = Quad::new(&Point3::new(max.x(), min.y(), min.z()), &(-dx), &dy, material.clone());
        let left = Quad::new(&Point3::new(min.x(), min.y(), min.z()), &dz, &dy, material.clone());
        let top = Quad::new(&Point3::new(min.x(), max.y(), max.z()), &dx, &(-dz), material.clone());
        let bottom = Quad::new(&Point3::new(min.x(), min.y(), min.z()), &dx, &dz, material.clone());

        let side_list = [front, right, back, left, top, bottom];

        // Find the area of each face to use as weights for random sampling of the cuboid surface.
        let face_areas: [f64; 6] = [
            side_list[0].area(),
            side_list[1].area(),
            side_list[2].area(),
            side_list[3].area(),
            side_list[4].area(),
            side_list[5].area(),
        ];
        let total_area: f64 = face_areas.iter().sum();
        let side_weights: [f64; 6] = face_areas.map(|area| area / total_area);

        // Build the bvh for the 6 sides of the cuboid.
        let mut side_hittable_list = HittableList::new();
        for side in side_list.iter() {
            side_hittable_list.add(side.clone());
        }
        
        Self {
            side_list,
            side_weights,
            side_bvh: side_hittable_list.into_bvh(),
        }
    }

    /// Create a cuboid from a center point and side lengths
    pub fn from_center(center: &Point3, dimensions: &Vec3, material: Arc<Material>) -> Self {
        let min = *center - *dimensions / 2.0;
        let max = *center + *dimensions / 2.0;
        Self::new(&min, &max, material)
    }

    /// Create a cuboid from a center point, dimensions, 
    /// and a rotation about its center (in degrees) around the Y axis
    pub fn from_center_rotate_y(center: &Point3, dimensions: &Vec3, angle: f64, material: Arc<Material>) -> Hittable {
        // Build at origin so rotation is around the cuboid center, then translate to requested center
        let base = Cuboid::from_center(&Point3::new(0.0, 0.0, 0.0), dimensions, material);
        Hittable::rotate_y_translate(base, angle, *center)
    }

    /// Get the bounding box of the cuboid
    pub fn bounding_box(&self) -> &AABB {
        &self.side_bvh.bounding_box()
    }

    /// Check if a ray hits the cuboid
    #[inline]
    pub fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        self.side_bvh.hit(r, ray_t, rec)
    }

    /// Get the PDF value for a ray hitting the cuboid from a given origin in a given direction.
    pub fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        let mut sum = 0.0;
        for i in 0..6 {
            sum += self.side_weights[i] * self.side_list[i].pdf_value(origin, direction);
        }
        sum
    }

    /// Generate a random direction from the given origin towards the cuboid.
    pub fn random(&self, origin: &Point3) -> Vec3 {
        let r = random_f64();
        let mut cumulative_weight = 0.0;

        for i in 0..6 {
            cumulative_weight += self.side_weights[i];
            if r < cumulative_weight {
                return self.side_list[i].random(origin);
            }
        }

        // Fallback
        self.side_list[5].random(origin)
    }
}

// From Cuboid to Hittable implementation
impl From<Cuboid> for Hittable {
    fn from(cuboid: Cuboid) -> Self {
        Hittable::Cuboid(cuboid)
    }
}