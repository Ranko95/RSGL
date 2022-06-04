mod image;
mod model;
mod geometry;
mod utils;
mod gl;

use std::path::Path;
use image::{ Image };
use model::{ Model };
use gl::{ GL };

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;

fn main() {
  let image = Image::new(WIDTH, HEIGHT);

  let mut gl = GL::new(image);

  let model = Model::new(&Path::new("obj/african_head.obj"));

  gl.render(&model);
}
