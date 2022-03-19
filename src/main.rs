mod image;
mod model;

use image::{ TGAColor, TGAImage };

fn main() {
    const RED: TGAColor = TGAColor(255, 0, 0, 255);
    const WHITE: TGAColor = TGAColor(255, 255, 255, 255);

    let mut image = TGAImage::new(100, 100);
    image.draw_line(13, 20, 80, 40, WHITE);
    image.draw_line(20, 13, 40, 80, RED);
    image.draw_line(80, 40, 13, 20, RED);
    image.write_tga_file("output.tga").unwrap();
}
