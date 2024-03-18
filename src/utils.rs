use image::{
    imageops::{blur, FilterType},
    open, DynamicImage, GenericImageView, ImageBuffer,
};
use imageproc::filter::{gaussian_blur_f32, median_filter};
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

#[derive(Clone, Copy)]
pub enum FilterMode {
    Gaussian,
    Median,
    Classic,
}

impl FilterMode {
    pub fn from_str(s: &str) -> Self {
        match s {
            "gaussian" => FilterMode::Gaussian,
            "median" => FilterMode::Median,
            _ => FilterMode::Classic,
        }
    }
}

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

pub fn pixelize(
    source: DynamicImage,
    scale: u32,
    radius: u32,
    filter_mode: FilterMode,
) -> DynamicImage {
    let (width, height) = source.dimensions();
    let pixel_size = gcd(width, height);
    if pixel_size <= 1 {
        // Basically crop them image into either 16:9 or 9:20 aspect ratio
        let (rw, rh) = if width > height {
            (16 * scale, 9 * scale)
        } else {
            (9 * scale, 20 * scale)
        };
        pixelize_image(source, radius, rw, rh, filter_mode)
    } else {
        let pixel_size = pixel_size / scale;
        let new_width = width / pixel_size;
        let new_height = height / pixel_size;
        pixelize_image(source, radius, new_width, new_height, filter_mode)
    }
}

fn pixelize_image(
    source: DynamicImage,
    radius: u32,
    w: u32,
    h: u32,
    filter_mode: FilterMode,
) -> DynamicImage {
    let (width, height) = source.dimensions();
    let source: ImageBuffer<_, Vec<_>> = ImageBuffer::<image::Rgb<u8>, Vec<u8>>::from(source);
    let source = match filter_mode {
        FilterMode::Median => median_filter(&source, radius, radius),
        FilterMode::Gaussian => gaussian_blur_f32(&source, radius as f32 / 3f32),
        FilterMode::Classic => blur(&source, radius as f32 / 3f32),
    };
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
    let dirs: [&str; 3] = ["input", "output", "config"];
    for dir in dirs.iter() {
        init_dir(dir);
    }
    if !Path::new("config/config.json").exists() {
        let default_config = r#"{
            "manual": false,
            "config": {
                "scale": 8,
                "blur_radius": 6,
                "filter_mode": "median"
            }
        }"#;
        write("config/config.json", default_config).unwrap();
    }
}

fn init_dir(dir: &str) {
    if !Path::new(dir).exists() {
        DirBuilder::new().create(dir).unwrap();
    }
}
