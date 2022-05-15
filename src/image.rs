use std::fs::File;
use std::io::Write;
use std::io::{ self };
use std::path::Path;

use crate::model::Model;
use crate::geometry::{
  Vector2D,
  Vector3D,
  find_triangle_bounding_box,
  barycentric,
};
use crate::utils::{
  slice_to_u8_slice,
  struct_to_u8_slice,
};

#[derive(Copy, Clone, Debug)]
pub struct TGAColor(pub u8, pub u8, pub u8, pub u8);

#[repr(C, packed)]
#[derive(Default)]
struct TGAHeader {
  id_length: u8,
  color_map_type: u8,
  image_type: u8,
  c_map_start: u16,
  c_map_length: u16,
  c_map_depth: u8,
  x_offset: u16,
  y_offset: u16,
  width: u16,
  height: u16,
  pixel_depth: u8,
  image_descriptor: u8,
}

#[derive(Debug)]
pub struct TGAImage {
  pub width: i32,
  pub height: i32,
  pub data: Vec<TGAColor>,
}

impl TGAImage {
  pub fn new(width: i32, height: i32) -> TGAImage {
    let bytes_count = width * height;
    TGAImage {
      width,
      height,
      data: vec![TGAColor(0, 0, 0, 0); bytes_count as usize],
    }
  }

  pub fn set_point(&mut self, x: i32, y: i32, color: TGAColor) -> () {
    if x < 0 || y < 0 {
      return;
    }
    if x >= self.width || y >= self.height {
      return;
    }

    self.data[(x + y * self.width) as usize] = color;
  }

  // TODO! Optimize the algorithm. Now O(n) is n^2.
  pub fn flip_vertically(&mut self) -> () {
    let half = self.height >> 1;
    for i in 0..half {
      for j in 0..self.width {
        let pixel_idx = (i * self.width + j) as usize;
        let pixel_to_swap_x_coord = self.height - 1 - i;
        let pixel_to_swap_idx = (pixel_to_swap_x_coord * self.width + j) as usize;
        self.data.swap(pixel_idx, pixel_to_swap_idx);
      }
    }
  }

  pub fn draw_line(
    &mut self,
    p0: &Vector2D<i32>,
    p1: &Vector2D<i32>,
    color: TGAColor,
  ) -> () {
    let mut dx = p1.x - p0.x;
    let mut dy = p1.y - p0.y;

    let mut step_x = 0;
    let mut step_y = 0;

    if dx < 0 {
      dx = dx.abs();
      step_x = -1;
    } else {
      step_x = 1;
    }

    if dy < 0 {
      dy = dy.abs();
      step_y = -1;
    } else {
      step_y = 1;
    }

    dx <<= 1;
    dy <<= 1;

    let mut x = p0.x;
    let mut y = p0.y;

    self.set_point(x, y, color);

    if dx > dy {
      let mut p = dy - (dx >> 1);

      while x != p1.x {
        x += step_x;
        if p >= 0 {
          y += step_y;
          p -= dx;
        }
        p += dy;

        self.set_point(x, y, color);
      }
    } else {
      let mut p = dx - (dy >> 1);

      while y != p1.y {
        y += step_y;

        if p >= 0 {
          x += step_x;
          p -= dy;
        }
        p += dx;

        self.set_point(x, y, color);
      }
    }
  }

  pub fn draw_triangle(
    &mut self,
    pts: [Vector2D<i32>; 3],
    color: TGAColor,
  ) -> () {
    let [top_left, bottom_right] = find_triangle_bounding_box(&pts);

    let mut x = top_left.x;
    let mut y = bottom_right.y;
  
    while x <= bottom_right.x {
      while y <= top_left.y {
        let bc_screen = barycentric(Vector2D::new(x, y), pts);

        if bc_screen.x >= 0.0 && bc_screen.y >= 0.0 && bc_screen.z >= 0.0 {
          self.set_point(x, y, color);
        }

        y += 1;
      }
      y = bottom_right.y;
      x += 1;
    }
  }

  pub fn write_tga_file<P>(&self, filename: &P) -> io::Result<()>
  where P: AsRef<Path>, {
    let mut file = File::create(filename).unwrap();
    let header = TGAHeader {
      image_type: 2,
      width: self.width as u16,
      height: self.height as u16,
      pixel_depth: 32,
      ..TGAHeader::default()
    };
    unsafe {
      file.write_all(struct_to_u8_slice(&header)).unwrap();
      file.write_all(slice_to_u8_slice(&self.data[..])).unwrap();
    }
    Ok(())
  }

  pub fn render(&mut self, model: &Model) -> () {
    let light_dir = Vector3D::new(0.0, 0.0, -1.0);

    let half_width = self.width as f32/2.;
    let half_height = self.height as f32/2.;  

    for i in 0..model.n_faces() {
      let face = model.face(i);

      let mut screen_coords: [Vector2D<i32>; 3] = [Vector2D::new(0, 0); 3];
      let mut world_coords: [Vector3D<f32>; 3] = [Vector3D::new(0., 0., 0.); 3];

      for j in 0..3 {
        let v = model.vertex(face[j] as usize);
        let x = ((v.x + 1.) * half_width) as i32;
        let y = ((v.y + 1.) * half_height) as i32;

        let p = Vector2D::new(x, y);

        screen_coords[j] = p;
        world_coords[j] = *v;
      }

      let n = (world_coords[2] - world_coords[1]) ^ (world_coords[1] - world_coords[0]);
      let normalized = n.normalize();

      let intensity = normalized * light_dir;

      if intensity > 0.0 {
        let alpha = (intensity * 255.0) as u8;
        self.draw_triangle(screen_coords, TGAColor(alpha, alpha, alpha, 255));
      }

    }

    self.write_tga_file(&Path::new("output.tga")).unwrap();
  }
}
