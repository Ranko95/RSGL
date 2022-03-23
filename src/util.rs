use std::fmt;

#[derive(Debug)]
pub struct Vec2i {
  pub x: i32,
  pub y: i32,
}

impl Vec2i {
  pub fn new(x: i32, y: i32) -> Vec2i {
    Vec2i {
      x,
      y,
    }
  }
}

impl fmt::Display for Vec2i {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "(x={}, y={})", self.x, self.y)
  }
}

#[derive(Debug)]
pub struct Vec3f {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Vec3f {
  pub fn new(x: f32, y: f32, z: f32) -> Vec3f {
    Vec3f {
      x,
      y,
      z,
    }
  }
}

impl fmt::Display for Vec3f {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "(x={}, y={}, z={})", self.x, self.y, self.z)
  }
}
