use crate::vec3::Vec3;

pub type Color = Vec3;

pub trait Utility {
  fn write_color(&self);
}

impl Utility for Vec3 {
  fn write_color(&self) {
    let ir: u8 = (255.999 * self.e[0]) as u8;
    let ig: u8 = (255.999 * self.e[1]) as u8;
    let ib: u8 = (255.999 * self.e[2]) as u8;

    println!("{} {} {}", ir, ig, ib)
  }
}