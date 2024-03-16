use image::{imageops::FilterType, open, DynamicImage, GenericImageView};
use std::{clone::Clone, cmp::PartialEq, ops::Rem};

fn gcd<T>(a: T, b: T) -> T
where
    T: Rem<Output = T> + PartialEq + From<u8> + Clone,
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

    source
        .resize(new_width, new_height, FilterType::Nearest)
        .resize(width, height, FilterType::Nearest)
}
