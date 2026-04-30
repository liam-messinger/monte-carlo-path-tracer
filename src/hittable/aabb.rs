use std::ops::Add;

use crate::interval::Interval;
use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;
use crate::prelude::AABB_MIN_PADDING;

/// Axis-aligned bounding box (AABB) struct
/// Holds three intervals, one for each axis
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    // Constants
    const EMPTY: AABB = AABB { x: Interval::EMPTY, y: Interval::EMPTY, z: Interval::EMPTY };
    const UNIVERSE: AABB = AABB { x: Interval::UNIVERSE, y: Interval::UNIVERSE, z: Interval::UNIVERSE };

    /// Constructor for AABB from three intervals
    /// Pads to minimum size if necessary
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut n = Self { x, y, z };
        n.pad_to_minimum();
        n
    }

    // TODO: Find where else to add const fns
    /// Empty AABB constructor
    pub const fn empty() -> Self {
        Self::EMPTY
    }

    /// Universe AABB constructor
    pub const fn universe() -> Self {
        Self::UNIVERSE
    }

    // TODO: create a constructor that takes a point and a radius
    // TODO: change to remove branching
    /// Create an axis-aligned bounding box from two points, padding to minimum
    pub fn from_corners(p1: &Point3, p2: &Point3) -> Self {
        let x = if p1.x() < p2.x() { Interval::new(p1.x(), p2.x()) } else { Interval::new(p2.x(), p1.x()) };
        let y = if p1.y() < p2.y() { Interval::new(p1.y(), p2.y()) } else { Interval::new(p2.y(), p1.y()) };
        let z = if p1.z() < p2.z() { Interval::new(p1.z(), p2.z()) } else { Interval::new(p2.z(), p1.z()) };
        let mut n = Self { x, y, z };
        n.pad_to_minimum();
        n
    }

    /// Create an axis-aligned bounding box from two ordered points (min, max), padding to minimum
    pub fn from_ordered_corners(min: &Point3, max: &Point3) -> Self {
        let mut n = Self {
            x: Interval::new(min.x(), max.x()),
            y: Interval::new(min.y(), max.y()),
            z: Interval::new(min.z(), max.z()),
        };
        n.pad_to_minimum();
        n
    }

    /// Create an axis-aligned bounding box from three points, padding to minimum
    pub fn from_point_triplet(p1: &Point3, p2: &Point3, p3: &Point3) -> Self {
        let x_min = p1.x().min(p2.x()).min(p3.x());
        let x_max = p1.x().max(p2.x()).max(p3.x());
        let y_min = p1.y().min(p2.y()).min(p3.y());
        let y_max = p1.y().max(p2.y()).max(p3.y());
        let z_min = p1.z().min(p2.z()).min(p3.z());
        let z_max = p1.z().max(p2.z()).max(p3.z());

        Self::from_ordered_corners(
            &Point3::new(x_min, y_min, z_min), 
            &Point3::new(x_max, y_max, z_max)
        )
    }

    /// Construct an axis-aligned bounding box from two input boxes
    pub fn merge(a: &AABB, b: &AABB) -> Self {
        Self {
            x: Interval::merge(&a.x, &b.x),
            y: Interval::merge(&a.y, &b.y),
            z: Interval::merge(&a.z, &b.z),
        }
    }

    /// Construct an axis-aligned bounding box that tightly encloses the input box and point.
    pub fn merge_point(bbox: &AABB, point: &Point3) -> Self {
        Self {
            x: Interval::merge(&bbox.x, &Interval::new(point.x(), point.x())),
            y: Interval::merge(&bbox.y, &Interval::new(point.y(), point.y())),
            z: Interval::merge(&bbox.z, &Interval::new(point.z(), point.z())),
        }
    }

    /// Get the interval for the specified axis (0 = x, 1 = y, 2 = z)
    pub fn axis_interval(&self, n: usize) -> &Interval {
        match n {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Axis index out of bounds"),
        }
    }

    /// Check if a ray intersects the bounding box within a given interval
    #[inline]
    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> bool {
        let mut t_min = ray_t.min;
        let mut t_max = ray_t.max;

        for axis in 0..3 {
            let ax_int = self.axis_interval(axis);
            let axis_dir_inv = 1.0 / r.direction[axis];

            let mut t0 = (ax_int.min - r.origin[axis]) * axis_dir_inv;
            let mut t1 = (ax_int.max - r.origin[axis]) * axis_dir_inv;

            if axis_dir_inv < 0.0 { // Direction is negative -> t1 is near side.
                std::mem::swap(&mut t0, &mut t1);
            }

            if t0 > t_min { t_min = t0; }
            if t1 < t_max { t_max = t1; }

            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    /// Like `hit()`, but returns the parameter `t` of the near-side intersection on a hit.
    /// Used for BVH traversal to find the closest hit of node children.
    #[inline]
    pub fn hit_with_t(&self, r: &Ray, ray_t: &Interval) -> Option<f64> {
        let mut t_min = ray_t.min;
        let mut t_max = ray_t.max;

        for axis in 0..3 {
            let ax_int = self.axis_interval(axis);
            let axis_dir_inv = 1.0 / r.direction[axis];

            let mut t0 = (ax_int.min - r.origin[axis]) * axis_dir_inv;
            let mut t1 = (ax_int.max - r.origin[axis]) * axis_dir_inv;

            if axis_dir_inv < 0.0 { // Direction is negative -> t1 is near side.
                std::mem::swap(&mut t0, &mut t1);
            }
            if t0 > t_min { t_min = t0; }
            if t1 < t_max { t_max = t1; }
            if t_max <= t_min { // No hit
                return None;
            }
        }
        Some(t_min)
    }

    /// Get the index of the longest axis of the bounding box
    pub fn longest_axis(&self) -> usize {
        // Returns the index of the longest axis of the bounding box.
        let sx = self.x.size();
        let sy = self.y.size();
        let sz = self.z.size();

        if sx > sy {
            if sx > sz { 0 } else { 2 }
        } else if sy > sz { 1 } else { 2 }
    }

    /// Pad the AABB to ensure no side is narrower than a minimum delta
    fn pad_to_minimum(&mut self) {
        // Adjust the AABB so that no side is narrower than some delta, padding if necessary
        self.x.pad_to_minimum(AABB_MIN_PADDING);
        self.y.pad_to_minimum(AABB_MIN_PADDING);
        self.z.pad_to_minimum(AABB_MIN_PADDING);
    }

    /// Get the surface area of the AABB. Clamped to zero if any side is degenerate.
    pub fn surface_area(&self) -> f64 {
        let dx = self.x.size().max(0.0);
        let dy = self.y.size().max(0.0);
        let dz = self.z.size().max(0.0);
        2.0 * (dx * dy + dy * dz + dz * dx)
    }
}

// Default AABB is empty
impl Default for AABB {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Add<Vec3> for AABB {
    // aabb + vec3
    type Output = AABB;
    fn add(self, offset: Vec3) -> AABB {
        AABB {
            x: self.x + offset.x(),
            y: self.y + offset.y(),
            z: self.z + offset.z(),
        }
    }
}

impl Add<Vec3> for &AABB {
    // &aabb + vec3
    type Output = AABB;
    fn add(self, offset: Vec3) -> AABB {
        AABB {
            x: &self.x + offset.x(),
            y: &self.y + offset.y(),
            z: &self.z + offset.z(),
        }
    }
}

impl Add<AABB> for Vec3 {
    // vec3 + aabb
    type Output = AABB;
    fn add(self, bbox: AABB) -> AABB {
        bbox + self
    }
}