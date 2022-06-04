use std::fs::File;
use std::io::Write;
use std::io::{ self };
use std::path::Path;

use crate::utils::{
  slice_to_u8_slice,
  struct_to_u8_slice,
};


#[derive(Copy, Clone, Debug)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

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
pub struct Image {
  pub width: i32,
  pub height: i32,
  pub data: Vec<Color>,
}

impl Image {
  pub fn new(width: i32, height: i32) -> Image {
    let bytes_count = width * height;
    Image {
      width,
      height,
      data: vec![Color(0, 0, 0, 0); bytes_count as usize],
    }
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

  pub fn write<P>(&self, filename: &P) -> io::Result<()>
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
