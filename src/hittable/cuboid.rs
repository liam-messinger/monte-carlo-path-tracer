use super::{Hittable, HitRecord, AABB, Quad, HittableList};

use crate::hittable::BVHNode;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

use std::sync::Arc;

/// A cuboid defined by two opposite corners and made from 6 quads.
#[derive(Clone)]
pub struct Cuboid {
    sides: BVHNode,
}

impl Cuboid {
    /// Constructor given two opposite corners and a material, returns the 3D box as a BVHNode of 6 quads.
    pub fn new(a: &Point3, b: &Point3, material: Arc<Material>) -> Self {
        // Returns the 3D box (six sides) that contains the two opposite vertices a & b.
        let mut sides = HittableList::new();

        // Construct the two opposite vertices with the minimum and maximum coordinates.
        let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
        let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

        let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

        sides.add(Quad::new(&Point3::new(min.x(), min.y(), max.z()), &dx, &dy, material.clone())); // Front
        sides.add(Quad::new(&Point3::new(max.x(), min.y(), max.z()), &(-dz), &dy, material.clone())); // Right
        sides.add(Quad::new(&Point3::new(max.x(), min.y(), min.z()), &(-dx), &dy, material.clone())); // Back
        sides.add(Quad::new(&Point3::new(min.x(), min.y(), min.z()), &dz, &dy, material.clone())); // Left
        sides.add(Quad::new(&Point3::new(min.x(), max.y(), max.z()), &dx, &(-dz), material.clone())); // Top
        sides.add(Quad::new(&Point3::new(min.x(), min.y(), min.z()), &dx, &dz, material.clone())); // Bottom

        Self { sides: sides.into_bvh() }
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
        &self.sides.bounding_box()
    }

    /// Check if a ray hits the cuboid
    #[inline]
    pub fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        self.sides.hit(r, ray_t, rec)
    }
}

// From Cuboid to Hittable implementation
impl From<Cuboid> for Hittable {
    fn from(cuboid: Cuboid) -> Self {
        Hittable::BVHNode(cuboid.sides)
    }
}