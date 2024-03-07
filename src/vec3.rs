use std::ops::{Neg, Index, IndexMut, Add, Mul, Div};

pub struct Vec3 {
  pub e: Vec<f64>,
}

impl Vec3 {
  pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 { e: [x, y, z].to_vec()}
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
}

// Overloading the unary subtraction operator
impl Neg for Vec3 {
  type Output = Self;
  fn neg(self) -> Self::Output {
    Vec3 {e: [-self.e[0], -self.e[1], -self.e[2]].to_vec()}
  }
}

// Overloading the addition operator
impl Add<Vec3> for Vec3 {
  type Output = Self;
  fn add(self, other: Self) -> Self::Output {
    Vec3 {e: [self.x() + other.x(), self.y() + other.y(), self.z() + other.z()].to_vec()}
  }
}

// Overloading the multiplication operator
impl Mul<f64> for Vec3 {
  type Output = Self;
  fn mul(self, other: f64) -> Self::Output {
    Vec3 {e: [self.x() * other, self.y() * other, self.z() * other].to_vec()}
  }
}

// Overloading the division operator
impl Div<f64> for Vec3 {
  type Output = Self;
  fn div(self, other: f64) -> Self::Output {
    Vec3 {e: [self.x() / other, self.y() / other, self.z() / other].to_vec()}
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