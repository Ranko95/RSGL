use std::fs::File;
use std::io::Write;
use std::slice;
use std::mem;
use std::io;

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
  pub width: usize,
  pub height: usize,
  pub data: Vec<TGAColor>,
}

impl TGAImage {
  pub fn new(width: usize, height: usize) -> TGAImage {
    let bytes_count = width * height;
    TGAImage {
      width,
      height,
      data: vec![TGAColor(0, 0, 0, 0); bytes_count],
    }
  }

  pub fn set(&mut self, x: usize, y: usize, color: TGAColor) -> Result<(), &str> {
    if x >= self.width || y >= self.height {
      return Err("x and y must be within initial image boundaries");
    }

    self.data[x + y * self.width] = color;
    Ok(())
  }

  // TODO! Optimize the algorithm. Now O(n) is n^2.
  pub fn flip_vertically(&mut self) -> () {
    let half = self.height >> 1;
    for i in 0..half {
      for j in 0..self.width {
        let pixel_idx = i * self.width + j;
        let pixel_to_swap_x_coord = self.height - 1 - i;
        let pixel_to_swap_idx = pixel_to_swap_x_coord * self.width + j;
        self.data.swap(pixel_idx, pixel_to_swap_idx);
      }
    }
  }

  pub fn write_tga_file(&self, filename: &str) -> io::Result<()> {
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
