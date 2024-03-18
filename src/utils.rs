use image::{imageops::FilterType, open, DynamicImage, GenericImageView, ImageBuffer};
use imageproc::filter::median_filter;
use serde_json::{from_reader, Result, Value};
use std::{
    clone::Clone,
    cmp::PartialEq,
    fs::{write, DirBuilder, File},
    io::BufReader,
    ops::Rem,
    path::Path,
    vec::Vec,
};

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

pub fn pixelize(source: DynamicImage, scale: u32, radius: u32) -> DynamicImage {
    let (width, height) = source.dimensions();
    let pixel_size = gcd(width, height);
    if pixel_size <= 1 {
        // Basically crop them image into either 16:9 or 9:20 aspect ratio
        let (rw, rh) = if width > height {
            (16 * scale, 9 * scale)
        } else {
            (9 * scale, 20 * scale)
        };
        pixelize_image(source, radius, rw, rh)
    } else {
        let pixel_size = pixel_size / scale;
        let new_width = width / pixel_size;
        let new_height = height / pixel_size;
        pixelize_image(source, radius, new_width, new_height)
    }
}

fn pixelize_image(source: DynamicImage, radius: u32, w: u32, h: u32) -> DynamicImage {
    let (width, height) = source.dimensions();
    let source: ImageBuffer<_, Vec<_>> = ImageBuffer::<image::Rgb<u8>, Vec<u8>>::from(source);
    let source = median_filter(&source, radius, radius);
    DynamicImage::from(source)
        .resize(w, h, FilterType::Nearest)
        .resize(width, height, FilterType::Nearest)
}

pub fn read_json(path: &str) -> Result<Value> {
    let json_reader = BufReader::new(match File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("Path {} not found.", path),
    });
    from_reader(json_reader)
}

pub fn init() {
    if !Path::new("output").exists() {
        DirBuilder::new().create("output").unwrap();
    }
    if !Path::new("input").exists() {
        DirBuilder::new().create("input").unwrap();
    }
    if !Path::new("config").exists() {
        DirBuilder::new().create("config").unwrap();
    }
    if !Path::new("config/config.json").exists() {
        let default_config = r#"{
            "manual": false,
            "config": {
                "scale": 8,
                "blur_radius": 3
            }
        }"#;
        write("config/config.json", default_config).unwrap();
    }
}
