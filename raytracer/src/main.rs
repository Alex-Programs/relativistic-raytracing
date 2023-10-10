use std::io::Cursor;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage, Rgb, Pixel};

fn main() {
    let mut img: RgbImage = ImageBuffer::new(512, 512);

    // Create gradient pattern
    for x in 0..512 {
        for y in 0..512 {
            let r = (x as f32 / 512.0) * 255.0;
            let g = (y as f32 / 512.0) * 255.0;
            let b = 0;

            img.put_pixel(x, y, image::Rgb([r as u8, g as u8, b]));
        }
    }

    img.save("out.png").unwrap();
}
