use image::{Rgb, RgbImage, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use std::path::Path;

fn main() {
    let path = Path::new("./borat.jpeg");
    let path2 = Path::new("./borat-2.jpeg");
    // let path = Path::new("/Users/nisheeth.barthwal/work/projects/rust-img/wrench.jpeg");
    // let path2 = Path::new("/Users/nisheeth.barthwal/work/projects/rust-img/wrench-2.jpeg");

    let mut image = image::open(path).expect("failed reading image").to_rgba8(); //RgbImage::new(200, 200);

    let font = Vec::from(include_bytes!("./DejaVuSans.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();


    
    let text = "/@nbaztec";
    draw_text_mut(&mut image, Rgba([0u8, 0u8, 0u8, 255u8]), 0, 0, scale(25.0), &font, text);
    draw_text_mut(&mut image, Rgba([0u8, 0u8, 0u8, 255u8]), 2, 2, scale(25.0), &font, text);
    draw_text_mut(&mut image, Rgba([255, 255, 255, 255u8]), 1, 1, scale(25.0), &font, text);
    let _ = image.save(path2).unwrap();
}

fn scale(height: f32) -> Scale {
    Scale {
        x: height * 1.0,
        y: height,
    }
}