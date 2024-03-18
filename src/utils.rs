use image::{imageops::FilterType, open, DynamicImage, GenericImageView, ImageBuffer};
use imageproc::filter::median_filter;
use serde::Serialize;
use serde_json::{
    from_reader, ser::PrettyFormatter, to_writer_pretty, Result as SerdeJsonResult, Serializer,
    Value,
};
use std::{clone::Clone, cmp::PartialEq, fs::File, io::BufReader, ops::Rem, vec::Vec};

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
        let (rw, rh) = if width > height { (16, 9) } else { (9, 20) };
        let source: ImageBuffer<_, Vec<_>> = ImageBuffer::<image::Rgb<u8>, Vec<u8>>::from(
            source.resize(rw * scale, rh * scale, FilterType::Nearest),
        );
        let source = median_filter(&source, radius, radius);
        DynamicImage::from(source).resize(width, height, FilterType::Nearest)
    } else {
        let pixel_size = pixel_size / 8;
        let new_width = width / pixel_size;
        let new_height = height / pixel_size;
        let source: ImageBuffer<_, Vec<_>> = ImageBuffer::<image::Rgb<u8>, Vec<u8>>::from(
            source.resize(new_width, new_height, FilterType::Nearest),
        );
        let source = median_filter(&source, radius, radius);
        DynamicImage::from(source).resize(width, height, FilterType::Nearest)
    }
}

pub fn read_json(path: &str) -> SerdeJsonResult<Value> {
    let json_reader = BufReader::new(match File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("Path {} not found.", path),
    });
    from_reader(json_reader)
}

pub fn write_json(path: &str, value: Value) -> SerdeJsonResult<()> {
    let file = File::create(path).unwrap();
    let fmt = PrettyFormatter::with_indent(b"    ");
    let mut buf = Vec::new();
    let mut ser = Serializer::with_formatter(&mut buf, fmt);
    value.serialize(&mut ser).unwrap();
    to_writer_pretty(file, &value)
}
