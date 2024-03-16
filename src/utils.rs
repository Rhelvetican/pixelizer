use image::{imageops::FilterType, open, DynamicImage, GenericImage, GenericImageView};
use std::{
    clone::Clone,
    cmp::PartialEq,
    ops::{Rem, Sub},
};

fn gcd<T>(a: T, b: T) -> T
where
    T: Rem<Output = T> + Sub<Output = T> + PartialEq + From<u8> + Clone,
{
    if b == T::from(0) {
        a
    } else {
        gcd(b.clone(), a % b)
    }
}

pub fn read_image(path: &str) -> DynamicImage {
    match open(path) {
        Ok(img) => img,
        Err(e) => panic!("Error reading image: {}", e),
    }
}

pub fn check_image(path: &str) -> bool {
    open(path).is_ok()
}

pub fn pixelize(source: DynamicImage) -> DynamicImage {
    let (width, height) = source.dimensions();
    let ratio = gcd(width, height) / 8;
    let new_width = width / ratio;
    let new_height = height / ratio;
    scale(
        source.resize(new_width, new_height, FilterType::Nearest),
        ratio,
        width,
        height,
    )
}

fn scale(source: DynamicImage, rate: u32, w: u32, h: u32) -> DynamicImage {
    let mut result = DynamicImage::new_rgb8(w, h);
    for (x, y, col) in source.pixels() {
        let x = x * rate;
        let y = y * rate;
        for i in 0..=rate {
            for j in 0..=rate {
                result.put_pixel(x + i, y + j, col);
            }
        }
    }
    result
}
