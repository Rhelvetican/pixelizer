mod utils;
use std::{
    fs::{read_dir, DirBuilder},
    path::Path,
};

fn main() {
    if !Path::new("output").exists() {
        DirBuilder::new().create("output").unwrap();
    }
    if !Path::new("input").exists() {
        DirBuilder::new().create("input").unwrap();
    }
    let contents = read_dir("input").unwrap();

    for file in contents {
        let file = file.unwrap();
        let path = file.path();
        if utils::check_image(path.to_str().unwrap()) {
            let img = utils::read_image(path.to_str().unwrap());
            let output_path = format!("output/{}.png", path.file_stem().unwrap().to_str().unwrap());
            let img = utils::pixelize(img);
            img.save(&output_path).unwrap();
            println!("Saved image to {}", output_path);
        } else {
            println!("Not an image. Skipping...")
        }
    }
}
