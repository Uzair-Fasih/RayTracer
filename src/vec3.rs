use std::ops::{Neg, Index, IndexMut, Add, Mul, Div, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
  pub e: [f64; 3],
}

// Alias for Vec3
pub type Point3 = Vec3;

impl Vec3 {
  pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 { e: [x, y, z]}
  }

  pub fn clone(&self) -> Vec3 {
    Vec3 { e: [self.x(), self.y(), self.z()] }
  }

  pub fn x(&self) -> f64 { self.e[0] }
  
  pub fn y(&self) -> f64 { self.e[1] }
  
  pub fn z(&self) -> f64 { self.e[2] }
  
  pub fn length_sq(&self) -> f64 {
    self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
  }

  pub fn length(&self) -> f64 {
    f64::sqrt(self.length_sq())
  }

  pub fn unit_vector(&self) -> Vec3 {
    self.clone() / self.length()
  }

  pub fn dot(&self, other: Vec3) -> f64 {
    self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
  }
}

// Overloading the unary subtraction operator
impl Neg for Vec3 {
  type Output = Self;
  fn neg(self) -> Self::Output {
    Vec3 {e: [-self.e[0], -self.e[1], -self.e[2]]}
  }
}

// Overloading the addition operator
impl Add<Vec3> for Vec3 {
  type Output = Self;
  fn add(self, other: Self) -> Self::Output {
    Vec3 {e: [self.x() + other.x(), self.y() + other.y(), self.z() + other.z()]}
  }
}

// Overloading the subtraction operator
impl Sub<Vec3> for Vec3 {
  type Output = Self;
  fn sub(self, other: Self) -> Self::Output {
    Vec3 {e: [self.x() - other.x(), self.y() - other.y(), self.z() - other.z()]}
  }
}

// Overloading the multiplication operator
impl Mul<f64> for Vec3 {
  type Output = Self;
  fn mul(self, other: f64) -> Self::Output {
    Vec3 {e: [self.x() * other, self.y() * other, self.z() * other]}
  }
}

// Overloading the division operator
impl Div<f64> for Vec3 {
  type Output = Self;
  fn div(self, other: f64) -> Self::Output {
    Vec3 {e: [self.x() / other, self.y() / other, self.z() / other]}
  }
}

// Overloading the [] operator
impl Index<usize> for Vec3 {
  type Output = f64;
  fn index(&self, index: usize) -> &Self::Output {
      &self.e[index]
  }
}

impl IndexMut<usize> for Vec3 {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
      &mut self.e[index]
  }
}