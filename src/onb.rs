use crate::vec3::Vec3;

/// Orthonormal Basis (ONB) struct (u, v, w), where w is aligned with the input normal.
/// This is used to transform between local and world coordinates for sampling directions.
#[derive(Debug, Clone, Copy, Default)]
pub struct ONB {
    axis : [Vec3; 3], // [u, v, w]
}

impl ONB {
    /// Constructor for ONB given a normal vector. The normal is aligned with w.
    #[inline]
    pub fn new(n: &Vec3) -> Self {
        let w = Vec3::unit_vector(n);

        // Branch-light ONB construction (Frisvad/Duff style)
        // ...existing code...
        let sign = 1.0_f64.copysign(w.z());
        let a = -1.0 / (sign + w.z());
        let b = w.x() * w.y() * a;

        let u = Vec3::new(
            1.0 + sign * w.x() * w.x() * a,
            sign * b,
            -sign * w.x(),
        );

        let v = Vec3::new(
            b,
            sign + w.y() * w.y() * a,
            -w.y(),
        );

        Self { axis: [u, v, w] }
    }

    /// Get the u axis of the ONB.
    #[inline] pub fn u(&self) -> Vec3 { self.axis[0] }
    /// Get the v axis of the ONB.
    #[inline] pub fn v(&self) -> Vec3 { self.axis[1] }
    /// Get the w axis of the ONB (aligned with the normal).
    #[inline] pub fn w(&self) -> Vec3 { self.axis[2] }

    /// Transform from basis coordinates (local) to world coordinates.
    #[inline]
    pub fn local(&self, v: &Vec3) -> Vec3 {
        v.x() * self.u() + v.y() * self.v() + v.z() * self.w()
    }
}