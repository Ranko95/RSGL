use std::fmt;
use std::cmp::{ max, min };
use std::ops::{ Sub, Add, Mul, BitXor };


#[derive(Debug, Clone, Copy)]
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

impl <T: Sub<Output = T>> Sub for Vector2D<T> {
  type Output = Self;

  fn sub(self, other: Vector2D<T>) -> Vector2D<T> {
    Vector2D::new(self.x - other.x, self.y - other.y)
  }
}

impl <T: Add<Output = T>> Add for Vector2D<T> {
  type Output = Self;

  fn add(self, other: Vector2D<T>) -> Vector2D<T> {
    Vector2D::new(self.x + other.x, self.y + other.y)
  }
}

impl <T: Mul<Output = T> + Add<Output = T>> Mul for Vector2D<T> {
  type Output = T;

  fn mul(self, other: Vector2D<T>) -> T {
    self.x * other.x + self.y * other.y
  }
}

impl <T: Mul<Output = T> + Copy> Mul<T> for Vector2D<T> {
  type Output = Self;

  fn mul(self, other: T) -> Vector2D<T> {
    Vector2D::new(self.x * other, self.y * other)
  }
}

#[derive(Debug, Clone, Copy)]
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

impl Vector3D<f32> {
  pub fn norm(self) -> f32 {
    (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
  }

  pub fn normalize(self) -> Vector3D<f32> {
    let length = self.norm();
    let inv_length = 1. / length;

    self * inv_length
  }
}

impl<T: fmt::Display> fmt::Display for Vector3D<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "(x={}, y={}, z={})", self.x, self.y, self.z)
  }
}

impl <T: Sub<Output = T>> Sub for Vector3D<T> {
  type Output = Self;

  fn sub(self, other: Vector3D<T>) -> Vector3D<T> {
    Vector3D::new(self.x - other.x, self.y - other.y, self.z - other.z)
  }
}

impl <T: Add<Output = T>> Add for Vector3D<T> {
  type Output = Self;

  fn add(self, other: Vector3D<T>) -> Vector3D<T> {
    Vector3D::new(self.x + other.x, self.y + other.y, self.z + other.z)
  }
}

impl <T: Mul<Output = T> + Add<Output = T>> Mul for Vector3D<T> {
  type Output = T;

  fn mul(self, other: Vector3D<T>) -> T {
    self.x * other.x + self.y * other.y + self.z * other.z
  }
}

impl <T: Mul<Output = T> + Copy> Mul<T> for Vector3D<T> {
  type Output = Self;

  fn mul(self, other: T) -> Vector3D<T> {
    Vector3D::new(self.x * other, self.y * other, self.z * other)
  }
}

impl <T: Mul<Output = T> + Sub<Output = T> + Copy> BitXor for Vector3D<T> {
  type Output = Self;

  fn bitxor(self, other: Vector3D<T>) -> Vector3D<T> {
    Vector3D::new(
      self.y * other.z - self.z * other.y,
      self.z * other.x - self.x * other.z,
      self.x * other.y - self.y * other.x,
    )
  }
}

pub fn barycentric(
  p: Vector2D<i32>,
  pts: [Vector2D<i32>; 3],
) -> Vector3D<f32> {
  let [t0, t1, t2] = pts;

  let v1 = Vector3D::new(
    (t2.x - t0.x) as f32,
    (t1.x - t0.x) as f32,
    (t0.x - p.x) as f32
  );
  let v2 = Vector3D::new(
    (t2.y - t0.y) as f32,
    (t1.y - t0.y) as f32,
    (t0.y - p.y) as f32
  );

  let u = v1 ^ v2;

  if u.z.abs() < 1.0 {
    return Vector3D::new(-1.0, 1.0, 1.0);
  }

  return Vector3D::new(1.0 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z);
}

pub fn find_triangle_bounding_box(
  pts: &[Vector2D<i32>; 3],
) -> [Vector2D<i32>; 2] {
  let [t0, t1, t2] = pts;

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
  pts: &[Vector2D<i32>; 3],
) -> bool {
  let [t0, t1, t2] = pts;
  let a_side = (t0.y - t1.y) * p.x + (t1.x - t0.x) * p.y + (t0.x * t1.y - t1.x * t0.y);
  let b_side = (t1.y - t2.y) * p.x + (t2.x - t1.x) * p.y + (t1.x * t2.y - t2.x * t1.y);
  let c_side = (t2.y - t0.y) * p.x + (t0.x - t2.x) * p.y + (t2.x * t0.y - t0.x * t2.y);

  return (a_side >= 0 && b_side >= 0 && c_side >=0) || (a_side < 0 && b_side < 0 && c_side < 0 );
}
