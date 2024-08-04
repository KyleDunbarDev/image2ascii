use image::{self, DynamicImage, GenericImageView, ImageResult, Pixel, Rgba};
use std::{fs::File, io::Write};

const IMG_PATH: &str = "makima.png";

fn load_image(path: &str) -> ImageResult<DynamicImage> {
    let img = image::open(path).expect("File not found:");
    Ok(img)
}

fn resize_image(image: &DynamicImage, width: u32, height: u32) -> DynamicImage {
    image.resize(width, height, image::imageops::FilterType::Lanczos3)
}

fn rgb_to_grey(pixel: Rgba<u8>) -> f32 {
    //weights for rgb -> grey
    let (wr, wg, wb) = (0.2125, 0.7154, 0.0721);
    return (wr * pixel[0] as f32 + wg * pixel[1] as f32 + wb * pixel[2] as f32);
}

fn threshold_ascii(image: &DynamicImage) -> String {
    let width: u32 = image.width();
    let height: u32 = image.height();

    let mut ascii_art = String::new();
    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            let ascii_char = match rgb_to_grey(pixel) {
                0.0..=25.0 => " ",
                25.0..=45.0 => "'",
                50.0..=75.0 => "(",
                75.0..=100.0 => "/",
                100.0..=125.0 => "+",
                125.0..=150.0 => "*",
                150.0..=175.0 => "&",
                175.0..=200.0 => "%",
                200.0..=225.0 => "X",
                _ => "#",
            };
            ascii_art.push_str(&ascii_char);
        }
        ascii_art.push_str("\n");
    }
    ascii_art
}

fn get_ascii_art<F>(func: F, image: &DynamicImage) -> String
where
    F: Fn(&DynamicImage) -> String,
{
    func(image)
}

fn main() {
    match load_image(IMG_PATH) {
        Ok(x) => {
            let img = resize_image(&x, 250, 250); //TODO: research image resizing
            let (x, y) = (img.width(), img.height());
            println!("Image dimensions are: {} x {}", x, y);
            let mut file = File::create("test.txt").unwrap();
            let result = get_ascii_art(threshold_ascii, &img);
            file.write_all(result.as_bytes()).unwrap();
        }
        Err(err) => println!("Error: {}", err),
    };
}
