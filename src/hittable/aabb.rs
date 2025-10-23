use crate::interval::Interval;
use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;

// Axis-aligned bounding box (AABB) struct
#[derive(Clone)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    // Constants
    pub const EMPTY: AABB = AABB { x: Interval::EMPTY, y: Interval::EMPTY, z: Interval::EMPTY };
    pub const UNIVERSE: AABB = AABB { x: Interval::UNIVERSE, y: Interval::UNIVERSE, z: Interval::UNIVERSE };

    // Constructor for AABB
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    // TODO: Find where else to add const fns
    // Empty AABB
    pub const fn empty() -> Self {
        Self::EMPTY
    }

    // Universe AABB
    pub const fn universe() -> Self {
        Self::UNIVERSE
    }

    // Create an axis-aligned bounding box from two points
    pub fn from_points(p1: &Point3, p2: &Point3) -> Self {
        let x = if p1.x() < p2.x() { Interval::new(p1.x(), p2.x()) } else { Interval::new(p2.x(), p1.x()) };
        let y = if p1.y() < p2.y() { Interval::new(p1.y(), p2.y()) } else { Interval::new(p2.y(), p1.y()) };
        let z = if p1.z() < p2.z() { Interval::new(p1.z(), p2.z()) } else { Interval::new(p2.z(), p1.z()) };
        Self { x, y, z }
    }

    // Assign the AABB to tightly enclose two points
    pub fn assign_from_points(&mut self, p1: &Point3, p2: &Point3) {
        self.x = if p1.x() < p2.x() { Interval::new(p1.x(), p2.x()) } else { Interval::new(p2.x(), p1.x()) };
        self.y = if p1.y() < p2.y() { Interval::new(p1.y(), p2.y()) } else { Interval::new(p2.y(), p1.y()) };
        self.z = if p1.z() < p2.z() { Interval::new(p1.z(), p2.z()) } else { Interval::new(p2.z(), p1.z()) };
    }

    // Construct an axis-aligned bounding box from two input boxes
    pub fn merge(a: &AABB, b: &AABB) -> Self {
        Self {
            x: Interval::merge(&a.x, &b.x),
            y: Interval::merge(&a.y, &b.y),
            z: Interval::merge(&a.z, &b.z),
        }
    }

    pub fn axis_interval(&self, n: usize) -> &Interval {
        match n {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Axis index out of bounds"),
        }
    }

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

    pub fn longest_axis(&self) -> usize {
        // Returns the index of the longest axis of the bounding box.
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() { 0 } else { 2 }
        } else {
            if self.y.size() > self.z.size() { 1 } else { 2 }
        }
    }
}

// Default AABB is empty intervals
impl Default for AABB {
    fn default() -> Self {
        Self::EMPTY
    }
}
