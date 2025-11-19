use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub};
use crate::prelude::{random_f64, random_f64_range, EPSILON};

// Struct definition
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub e: [f64; 3],
}

// Type aliases
pub type Point3 = Vec3;

// ----------------- Implementations -----------------
impl Vec3 {
    // Constructors
    pub fn zero() -> Self {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }

    // Accessors
    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }
    
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    // ----------------- Utility functions -----------------

    // vec.length() 
    pub fn length(&self) -> f64 { // length of the vector
        self.length_squared().sqrt()
    }

    // vec.length_squared()
    pub fn length_squared(&self) -> f64 { // squared length of the vector
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    // vec.near_zero()
    pub fn near_zero(&self) -> bool { // checks if the vector is close to zero in all dimensions
        self.e[0].abs() < EPSILON && self.e[1].abs() < EPSILON && self.e[2].abs() < EPSILON
    }

    // Vec3::random()
    pub fn random() -> Self { // random vector with each component in [0,1)
        Vec3::new(random_f64(), random_f64(), random_f64())
    }

    // Vec3::random_range(min, max)
    pub fn random_range(min: f64, max: f64) -> Self { // random vector with each component in [min,max)
        Vec3::new( 
            random_f64_range(min, max),
            random_f64_range(min, max),
            random_f64_range(min, max),
        )
    }

    // Vec3::dot(u, v)
    pub fn dot(u: &Vec3, v: &Vec3) -> f64 { // dot product of two vectors
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    }

    // Vec3::cross(u, v)
    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 { // cross product of two vectors
        Vec3 {
            e: [
                u.e[1] * v.e[2] - u.e[2] * v.e[1],
                u.e[2] * v.e[0] - u.e[0] * v.e[2],
                u.e[0] * v.e[1] - u.e[1] * v.e[0],
            ],
        }
    }

    // Vec3::unit_vector(v)
    pub fn unit_vector(v: &Vec3) -> Vec3 { // returns the unit vector in the direction of v
        *v / v.length()
    }

    // Vec3::random_in_unit_circle()
    pub fn random_in_unit_circle() -> Vec3 { // Generates a random point in the unit disk in the XY plane
        loop {
            let p = Vec3::new(random_f64_range(-1.0, 1.0), random_f64_range(-1.0, 1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    // Vec3::random_unit_vector()
    pub fn random_unit_vector() -> Vec3 { // Generates a random unit vector uniformly distributed over the unit sphere
        loop {
            let v = Vec3::random_range(-1.0, 1.0);
            let lensq = v.length_squared();
            if 1e-160 < lensq && lensq < 1.0 {
                return v / lensq.sqrt();
            }
        }
    }

    // Vec3::random_in_hemisphere(normal)
    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 { // Using a normal, generates a random vector in the "same" direction
        let on_unit_sphere = Vec3::random_unit_vector();
        if Vec3::dot(&on_unit_sphere, normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    // Vec3::reflect(v, n)
    pub fn reflect(vec: &Vec3, normal: &Vec3) -> Vec3 { // Reflects vector vec around a normal
        (*vec) - 2.0 * Vec3::dot(vec, normal) * (*normal)
    }

    // Vec3::refract(ray_in, normal, etai_over_etat)
    pub fn refract(ray_in: &Vec3, normal: &Vec3, etai_over_etat: f64) -> Vec3 { // Refracts ray_in with normal and ratio of indices of refraction
        let cos_theta = f64::min(Vec3::dot(&-*ray_in, normal), 1.0);
        let r_out_perp = etai_over_etat * (*ray_in + cos_theta * (*normal));
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * (*normal);
        r_out_perp + r_out_parallel
    }
}

// ----------------- Operator overloads for Vec3 -----------------

impl Neg for Vec3 {
    // -v
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl Index<usize> for Vec3 {
    // v[i]
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        &self.e[i]
    }
}

impl AddAssign for Vec3 {
    // v += u
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl MulAssign<f64> for Vec3 {
    // v *= t
    fn mul_assign(&mut self, t: f64) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

impl DivAssign<f64> for Vec3 {
    // v /= t
    fn div_assign(&mut self, t: f64) {
        let inv_t = 1.0 / t;
        self.e[0] *= inv_t;
        self.e[1] *= inv_t;
        self.e[2] *= inv_t;
    }
}

impl Add for Vec3 {
    // v + u
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}

impl Sub for Vec3 {
    // v - u
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}

impl Mul for Vec3 {
    // v * u (element-wise multiplication)
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [ 
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        }
    }
}

impl Mul<f64> for Vec3 {
    // v * t
    type Output = Vec3;
    fn mul(self, t: f64) -> Vec3 {
        Vec3 {
            e: [self.e[0] * t, self.e[1] * t, self.e[2] * t],
        }
    }
}

impl Mul<Vec3> for f64 {
    // t * v
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            e: [v.e[0] * self, v.e[1] * self, v.e[2] * self],
        }
    }
}

impl Div<f64> for Vec3 {
    // v / t
    type Output = Vec3;
    fn div(self, t: f64) -> Vec3 {
        let inv_t = 1.0 / t;
        Vec3 {
            e: [self.e[0] * inv_t, self.e[1] * inv_t, self.e[2] * inv_t],
        }
    }
}

// ----------------- Display trait implementation -----------------
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

// ----------------- Default trait implementation -----------------
impl Default for Vec3 {
    fn default() -> Self {
        Self::zero()
    }
}