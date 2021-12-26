#[derive(Copy, Clone, Debug)]
pub struct TGAColor(pub u8, pub u8, pub u8, pub u8);

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

  pub fn set(&mut self, x: usize, y: usize, color: TGAColor) -> bool {
    if x >= self.width || y >= self.height {
      return false;
    }

    self.data[x + y * self.width] = color;
    true
  }

  pub fn flip_vertically(&mut self) -> bool {
    true
  }

  pub fn write_tga_file(filename: String) -> bool {
    true
  }
}
