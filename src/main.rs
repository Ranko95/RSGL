mod image;
mod model;
mod geometry;

use std::path::Path;
use image::{ TGAColor, TGAImage };
use model::{ Model };
use geometry::Vector2D;

const WHITE: TGAColor = TGAColor(255, 255, 255, 255);
const RED: TGAColor = TGAColor(255, 0, 0, 255);
const GREEN: TGAColor = TGAColor(0, 255, 0, 255);

const WIDTH: i32 = 200;
const HEIGHT: i32 = 200;

fn main() {
    let mut image = TGAImage::new(WIDTH, HEIGHT);

    image.draw_triangle(Vector2D::new(10, 70), Vector2D::new(50, 160), Vector2D::new(70, 80), WHITE);
    image.draw_triangle(Vector2D::new(180, 50), Vector2D::new(150, 1), Vector2D::new(70, 180), RED);
    image.draw_triangle(Vector2D::new(180, 150), Vector2D::new(120, 160), Vector2D::new(130, 180), GREEN);

    image.write_tga_file(&Path::new("output.tga")).unwrap();

    // let model = Model::new(&Path::new("obj/african_head.obj"));

    // image.render(&model, WHITE);
}
