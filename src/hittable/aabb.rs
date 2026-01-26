use std::ops::Add;

use crate::interval::Interval;
use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;
use crate::prelude::AABB_MIN_PADDING;

/// Axis-aligned bounding box (AABB) struct
/// Holds three intervals, one for each axis
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone)]
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

    /// Create an axis-aligned bounding box from two points, padding to minimum
    pub fn from_points(p1: &Point3, p2: &Point3) -> Self {
        let x = if p1.x() < p2.x() { Interval::new(p1.x(), p2.x()) } else { Interval::new(p2.x(), p1.x()) };
        let y = if p1.y() < p2.y() { Interval::new(p1.y(), p2.y()) } else { Interval::new(p2.y(), p1.y()) };
        let z = if p1.z() < p2.z() { Interval::new(p1.z(), p2.z()) } else { Interval::new(p2.z(), p1.z()) };
        let mut n = Self { x, y, z };
        n.pad_to_minimum();
        n
    }

    /// Assign the AABB to tightly enclose two points
    pub fn assign_from_points(&mut self, p1: &Point3, p2: &Point3) {
        self.x = if p1.x() < p2.x() { Interval::new(p1.x(), p2.x()) } else { Interval::new(p2.x(), p1.x()) };
        self.y = if p1.y() < p2.y() { Interval::new(p1.y(), p2.y()) } else { Interval::new(p2.y(), p1.y()) };
        self.z = if p1.z() < p2.z() { Interval::new(p1.z(), p2.z()) } else { Interval::new(p2.z(), p1.z()) };
    }

    /// Construct an axis-aligned bounding box from two input boxes
    pub fn merge(a: &AABB, b: &AABB) -> Self {
        Self {
            x: Interval::merge(&a.x, &b.x),
            y: Interval::merge(&a.y, &b.y),
            z: Interval::merge(&a.z, &b.z),
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
    #[inline(always)]
    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> bool {
        let ray_orig: Point3 = r.origin;
        let ray_dir: Vec3 = r.direction;

        let mut t_min = ray_t.min;
        let mut t_max = ray_t.max;

        for axis in 0..3 {
            let ax_int = self.axis_interval(axis);
            let axis_dir_inv = 1.0 / ray_dir[axis];

            let t0 = (ax_int.min - ray_orig[axis]) * axis_dir_inv;
            let t1 = (ax_int.max - ray_orig[axis]) * axis_dir_inv;

            if t0 < t1 {
                if t0 > t_min { t_min = t0; }
                if t1 < t_max { t_max = t1; }
            } else {
                if t1 > t_min { t_min = t1; }
                if t0 < t_max { t_max = t0; }
            }

            if t_max <= t_min {
                return false;
            }
        }
        true
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