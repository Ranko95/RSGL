mod image;
mod model;

use std::path::Path;
use image::{ TGAColor, TGAImage };
use model::{ Model };

const WHITE: TGAColor = TGAColor(255, 255, 255, 255);
const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;

fn main() {
    let mut image = TGAImage::new(WIDTH, HEIGHT);

    let model = Model::new(&Path::new("obj/african_head.obj"));

    image.render(&model, WHITE);
}
