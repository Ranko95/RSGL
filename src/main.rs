mod image;
mod model;

use std::path::Path;
use image::{ TGAColor, TGAImage };
use model::{ Model };

const WHITE: TGAColor = TGAColor(255, 255, 255, 255);
const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;

const HALF_WIDTH: f32 = WIDTH as f32/2.;
const HALF_HEIGHT: f32 = HEIGHT as f32/2.;

fn main() {
    let mut image = TGAImage::new(WIDTH, HEIGHT);

    let model = Model::new(&Path::new("obj/african_head.obj"));    

    for i in 0..model.n_faces() {
        let face = model.face(i);

        for j in 0..3 {
            let v0 = model.vertex(face[j] as usize);
            let v1 = model.vertex(face[(j + 1) % 3] as usize);
            let x0 = ((v0.x + 1.) * HALF_WIDTH) as i32;
            let y0 = ((v0.y + 1.) * HALF_HEIGHT) as i32;
            let x1 = ((v1.x + 1.) * HALF_WIDTH) as i32;
            let y1 = ((v1.y + 1.) * HALF_HEIGHT) as i32;

            image.draw_line(x0, y0, x1, y1, WHITE);
        }
    }

    image.write_tga_file(&Path::new("output.tga")).unwrap();
}
