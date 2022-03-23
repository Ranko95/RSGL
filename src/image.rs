use std::fs::File;
use std::io::Write;
use std::{ slice, mem };
use std::io::{ self };
use std::path::Path;

use crate::model::Model;
use crate::util::Vec2i;

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

unsafe fn struct_to_u8_slice<T>(s: &T) -> &[u8] {
  let data_ptr: *const u8 = mem::transmute(s);
  let slice = slice::from_raw_parts(data_ptr, mem::size_of::<T>());
  slice
}

unsafe fn slice_to_u8_slice<T>(s: &[T]) -> &[u8] {
  let data_ptr: *const u8 = mem::transmute(&s[0]);
  let slice = slice::from_raw_parts(data_ptr, mem::size_of::<T>() * s.len());
  slice
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

  pub fn set(&mut self, x: i32, y: i32, color: TGAColor) -> () {
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

  pub fn draw_line(&mut self, p0: &Vec2i, p1: &Vec2i, color: TGAColor) -> () {
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

    self.set(x, y, color);

    if dx > dy {
      let mut p = dy - (dx >> 1);

      while x != p1.x {
        x += step_x;
        if p >= 0 {
          y += step_y;
          p -= dx;
        }
        p += dy;

        self.set(x, y, color);
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

        self.set(x, y, color);
      }
    }
  }

  pub fn draw_triangle(&mut self, t0: &Vec2i, t1: &Vec2i, t2: &Vec2i, color: TGAColor) -> () {
    self.draw_line(t0, t1, color);
    self.draw_line(t1, t2, color);
    self.draw_line(t2, t0, color);
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

  pub fn render(&mut self, model: &Model, color: TGAColor) -> () {
    let half_width = self.width as f32/2.;
    let half_height = self.height as f32/2.;  

    for i in 0..model.n_faces() {
      let face = model.face(i);

      for j in 0..3 {
        let v0 = model.vertex(face[j] as usize);
        let v1 = model.vertex(face[(j + 1) % 3] as usize);
        let x0 = ((v0.x + 1.) * half_width) as i32;
        let y0 = ((v0.y + 1.) * half_height) as i32;
        let x1 = ((v1.x + 1.) * half_width) as i32;
        let y1 = ((v1.y + 1.) * half_height) as i32;

        let p0 = Vec2i::new(x0, y0);
        let p1 = Vec2i::new(x1, y1);

        self.draw_line(&p0, &p1, color);
      }
    }

    self.write_tga_file(&Path::new("output.tga")).unwrap();
  }
}
