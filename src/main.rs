mod image;
mod model;
mod util;

use std::path::Path;
use image::{ TGAColor, TGAImage };
use model::{ Model };
use util::Vec2i;

const WHITE: TGAColor = TGAColor(255, 255, 255, 255);
const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;

fn main() {
    let mut image = TGAImage::new(WIDTH, HEIGHT);

    // image.draw_triangle(&Vec2i::new(10, 70), &Vec2i::new(50, 160), &Vec2i::new(70, 80), WHITE);

    // image.write_tga_file(&Path::new("output.tga")).unwrap();

    let model = Model::new(&Path::new("obj/african_head.obj"));

    image.render(&model, WHITE);
}
