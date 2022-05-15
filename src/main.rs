mod image;
mod model;
mod geometry;
mod utils;

use std::path::Path;
use image::{ TGAImage };
use model::{ Model };

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;

fn main() {
    let mut image = TGAImage::new(WIDTH, HEIGHT);

    let model = Model::new(&Path::new("obj/african_head.obj"));

    image.render(&model);
}
