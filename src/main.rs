use rsgl::{ TGAColor, TGAImage };

fn main() {
    let red = TGAColor(255, 0, 0, 255);

    let mut image = TGAImage::new(100, 100);
    image.set(52, 41, red).unwrap();
    image.flip_vertically();
    image.write_tga_file("output.tga").unwrap();
}
