use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg, Index, AddAssign, MulAssign, DivAssign};

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
  pub fn x(&self) -> f64 { self.e[0] }
  pub fn y(&self) -> f64 { self.e[1] }
  pub fn z(&self) -> f64 { self.e[2] }

  // Additional methods
  pub fn length(&self) -> f64 {
    self.length_squared().sqrt()
  }

  pub fn length_squared(&self) -> f64 {
    self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
  }
}

// ----------------- Utility functions -----------------
pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
  u.e[0]*v.e[0] + u.e[1]*v.e[1] + u.e[2]*v.e[2]
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
  Vec3 {
    e: [
      u.e[1]*v.e[2] - u.e[2]*v.e[1],
      u.e[2]*v.e[0] - u.e[0]*v.e[2],
      u.e[0]*v.e[1] - u.e[1]*v.e[0],
    ]
  }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
  v / v.length()
}

// ----------------- Operator overloads for Vec3 -----------------

impl Neg for Vec3 { // -v
  type Output = Vec3;
  fn neg(self) -> Vec3 {
    Vec3 { e: [-self.e[0], -self.e[1], -self.e[2]] }
  }
}

impl Index<usize> for Vec3 { // v[i]
  type Output = f64;
  fn index(&self, i: usize) -> &f64 {
    &self.e[i]
  }
}

impl AddAssign for Vec3 { // v += u
  fn add_assign(&mut self, other: Vec3) {
    self.e[0] += other.e[0];
    self.e[1] += other.e[1];
    self.e[2] += other.e[2];
  }
}

impl MulAssign<f64> for Vec3 { // v *= t
  fn mul_assign(&mut self, t: f64) {
    self.e[0] *= t;
    self.e[1] *= t;
    self.e[2] *= t;
  }
}

impl DivAssign<f64> for Vec3 { // v /= t
  fn div_assign(&mut self, t: f64) {
    let inv_t = 1.0 / t;
    self.e[0] *= inv_t;
    self.e[1] *= inv_t;
    self.e[2] *= inv_t;
  }
}

impl Add for Vec3 { // v + u
  type Output = Vec3;
  fn add(self, other: Vec3) -> Vec3 {
    Vec3 { e: [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]] }
  }
}

impl Sub for Vec3 { // v - u
  type Output = Vec3;
  fn sub(self, other: Vec3) -> Vec3 {
    Vec3 { e: [self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]] }
  }
}

impl Mul for Vec3 { // v * u (dot product)
  type Output = f64;
  fn mul(self, other: Vec3) -> f64 {
    self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
  }
}

impl Mul<f64> for Vec3 { // v * t
  type Output = Vec3;
  fn mul(self, t: f64) -> Vec3 {
    Vec3 { e: [self.e[0] * t, self.e[1] * t, self.e[2] * t] }
  }
}

impl Mul<Vec3> for f64 { // t * v
  type Output = Vec3;
  fn mul(self, v: Vec3) -> Vec3 {
    Vec3 { e: [v.e[0] * self, v.e[1] * self, v.e[2] * self] }
  }
}

impl Div<f64> for Vec3 { // v / t
  type Output = Vec3;
  fn div(self, t: f64) -> Vec3 {
    let inv_t = 1.0 / t;
    Vec3 { e: [self.e[0] * inv_t, self.e[1] * inv_t, self.e[2] * inv_t] }
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