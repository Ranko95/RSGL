use std::fs::File;
use std::io::Write;
use std::{ slice, mem };
use std::io::{ self };
use std::path::Path;

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

  pub fn set(&mut self, x: i32, y: i32, color: TGAColor) -> Result<(), &str> {
    if x >= self.width || y >= self.height {
      return Err("x and y must be within initial image boundaries");
    }

    self.data[(x + y * self.width) as usize] = color;
    Ok(())
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

  pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: TGAColor) {
    let mut dx = x1 - x0;
    let mut dy = y1 - y0;

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

    let mut x = x0;
    let mut y = y0;

    self.set(x, y, color).unwrap();

    if dx > dy {
      let mut p = dy - (dx >> 1);

      while x != x1 {
        x += step_x;
        if p >= 0 {
          y += step_y;
          p -= dx;
        }
        p += dy;

        self.set(x, y, color).unwrap();
      }
    } else {
      let mut p = dx - (dy >> 1);

      while y != y1 {
        y += step_y;

        if p >= 0 {
          x += step_x;
          p -= dy;
        }
        p += dx;

        self.set(x, y, color).unwrap();
      }
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
}
