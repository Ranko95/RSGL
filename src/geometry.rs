use std::fmt;
use std::cmp::{ max, min };

#[derive(Debug)]
pub struct Vector2D<T> {
  pub x: T,
  pub y: T,
}

impl<T> Vector2D<T> {
  pub fn new(x: T, y: T) -> Vector2D<T> {
    Vector2D {
      x,
      y,
    }
  }
}

impl<T: fmt::Display> fmt::Display for Vector2D<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "(x={}, y={})", self.x, self.y)
  }
}

#[derive(Debug)]
pub struct Vector3D<T> {
  pub x: T,
  pub y: T,
  pub z: T,
}

impl<T> Vector3D<T> {
  pub fn new(x: T, y: T, z: T) -> Vector3D<T> {
    Vector3D {
      x,
      y,
      z,
    }
  }
}

impl<T: fmt::Display> fmt::Display for Vector3D<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "(x={}, y={}, z={})", self.x, self.y, self.z)
  }
}

pub fn find_triangle_bounding_box(
  t0: &Vector2D<i32>,
  t1: &Vector2D<i32>,
  t2: &Vector2D<i32>,
) -> [Vector2D<i32>; 2] {
  let min_x = min(t0.x, min(t1.x, t2.x));
  let max_x = max(t0.x, max(t1.x, t2.x));

  let min_y = min(t0.y, min(t1.y, t2.y));
  let max_y = max(t0.y, max(t1.y, t2.y));

  let top_left = Vector2D::new(min_x, max_y);
  let bottom_right = Vector2D::new(max_x, min_y);

  return [top_left, bottom_right];
}

pub fn is_in_triangle(
  p: &Vector2D<i32>,
  t0: &Vector2D<i32>,
  t1: &Vector2D<i32>,
  t2: &Vector2D<i32>,
) -> bool {
  let a_side = (t0.y - t1.y) * p.x + (t1.x - t0.x) * p.y + (t0.x * t1.y - t1.x * t0.y);
  let b_side = (t1.y - t2.y) * p.x + (t2.x - t1.x) * p.y + (t1.x * t2.y - t2.x * t1.y);
  let c_side = (t2.y - t0.y) * p.x + (t0.x - t2.x) * p.y + (t2.x * t0.y - t0.x * t2.y);

  return (a_side >= 0 && b_side >= 0 && c_side >=0) || (a_side < 0 && b_side < 0 && c_side < 0 );
}
