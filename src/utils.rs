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

pub fn pixelize(source: DynamicImage, scale: u32) -> DynamicImage {
    let (width, height) = source.dimensions();
    let pixel_size = gcd(width, height);
    if pixel_size <= 1 {
        // Basically crop them image into either 16:9 or 9:20 aspect ratio
        let (rw, rh) = if width > height { (16, 9) } else { (9, 20) };
        source
            .resize_exact(rw * scale, rh * scale, FilterType::Nearest)
            .resize(width, height, FilterType::Nearest)
    } else {
        let pixel_size = pixel_size / 8;
        let new_width = width / pixel_size;
        let new_height = height / pixel_size;
        source
            .resize(new_width, new_height, FilterType::Nearest)
            .resize(width, height, FilterType::Nearest)
    }
}
