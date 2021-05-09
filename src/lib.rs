pub mod config;

use conv::ValueInto;
use image::imageops::FilterType;
use image::{imageops, GenericImage, GenericImageView, ImageBuffer, Pixel, Rgba};
use imageproc::{
    definitions::Clamp,
    drawing::{draw_text_mut, Canvas},
};
use rusttype::{Font, Scale};
use std::{ffi::OsStr, fs, path::PathBuf};

pub fn new_id(filename: &str, text: &str) -> String {
    let data = format!("{}_{}", filename, text);
    format!("{:?}", md5::compute(&data))
}

pub fn read_images(img_dir: &str) -> Vec<String> {
    let mut images = vec![];
    for entry in fs::read_dir(img_dir).expect("could not read directory 'img'") {
        let entry = entry.expect("could not read directory entry");
        let path = entry.path();
        if !path.is_dir() {
            match path.extension().and_then(OsStr::to_str) {
                Some("jpeg") | Some("jpg") | Some("png") => {
                    let name = entry.file_name().to_str().unwrap().to_string();
                    images.push(name);
                }
                _ => (),
            }
        }
    }
    images
}

pub fn generate(input_file: &PathBuf, output_file: &PathBuf, text: &str) {
    let image = image::open(input_file)
        .expect("failed reading image")
        .to_rgba8(); //RgbImage::new(200, 200);
    let mut image = resize(&image, 500, FilterType::Nearest);

    let font = Vec::from(include_bytes!("./impact.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let (w, h) = image.dimensions();
    let x_offset = (text.len() as u32 + 1) * 10;
    let x = w / 2 - x_offset;
    let y = h - 75;

    if !text.is_empty() {
        draw_stroked(
            &mut image,
            text,
            50.0,
            (x, y),
            &font,
            Rgba([255u8, 255u8, 255u8, 255u8]),
            Rgba([0u8, 0u8, 0u8, 255u8]),
        );
    }
    image.save(output_file).expect("failed saving image");
}

fn resize<I: GenericImageView>(
    image: &I,
    nwidth: u32,
    filter: FilterType,
) -> ImageBuffer<I::Pixel, Vec<<I::Pixel as Pixel>::Subpixel>>
where
    I::Pixel: 'static,
    <I::Pixel as Pixel>::Subpixel: 'static,
{
    let (w, h) = image.dimensions();
    if w <= nwidth {
        let mut b = ImageBuffer::new(w, h);
        b.copy_from(image, 0, 0)
            .expect("failed copying image buffer");
        b
    } else {
        let factor = w as f32 / nwidth as f32;
        let nheight = (h as f32 / factor) as u32;
        imageops::resize(image, nwidth, nheight, filter)
    }
}

fn draw_stroked<'a, C>(
    image: &'a mut C,
    text: &str,
    height: f32,
    (x, y): (u32, u32),
    font: &'a Font<'a>,
    color: C::Pixel,
    bg_color: C::Pixel,
) where
    C: Canvas,
    <C::Pixel as Pixel>::Subpixel: ValueInto<f32> + Clamp<f32>,
{
    let scale = Scale {
        x: height,
        y: height * 1.2,
    };

    let offset = 2;
    let y_up = y - offset;
    let y_down = y + offset;
    let x_left = x - offset;
    let x_right = x + offset;

    draw_text_mut(image, bg_color, x, y_up, scale.clone(), &font, text);

    draw_text_mut(image, bg_color, x, y_down, scale.clone(), &font, text);
    draw_text_mut(image, bg_color, x_left, y, scale.clone(), &font, text);
    draw_text_mut(image, bg_color, x_right, y, scale.clone(), &font, text);
    draw_text_mut(image, color, x, y, scale, &font, text);
}
