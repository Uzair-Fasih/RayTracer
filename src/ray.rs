use crate::vec3::{Point3, Vec3};

pub struct Ray {
  pub orig: Point3,
  pub dirc: Vec3
}

impl Ray {
  pub fn new(origin: Point3, direction: Vec3) -> Ray {
    Ray {orig: origin, dirc: direction}
  }

  pub fn at(&self, t: f64) -> Point3 {
    self.orig.clone() + (self.dirc.clone() * t)
  }
}
