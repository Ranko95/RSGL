use rsgl::{ TGAColor, TGAImage };

fn main() {
    let white = TGAColor(255, 255, 255, 255);
    let red = TGAColor(255, 0, 0, 255);

    let mut image = TGAImage::new(100, 100);
    image.set(52, 41, red);
    println!("{:?}", image);
}
