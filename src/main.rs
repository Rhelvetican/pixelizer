mod utils;
use rprompt::prompt_reply;
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
    if !Path::new("config").exists() {
        DirBuilder::new().create("config").unwrap();
    }
    let contents = read_dir("input").unwrap();

    for file in contents {
        match file {
            Ok(file) => {
                let path = file.path();
                if utils::check_image(path.to_str().unwrap()) {
                    let img = utils::read_image(path.to_str().unwrap());
                    let output_path =
                        format!("output/{}.png", path.file_stem().unwrap().to_str().unwrap());
                    let scale = match prompt_reply("Enter scale (Default to 8): ") {
                        Ok(scale) => scale.parse::<u32>().unwrap_or(8),
                        Err(_) => 8,
                    };
                    let radius = match prompt_reply("Enter radius (Default to 3): ") {
                        Ok(radius) => radius.parse::<u32>().unwrap_or(3),
                        Err(_) => 3,
                    };
                    let img = utils::pixelize(img, scale, radius);
                    img.save(&output_path).unwrap();
                    println!("Saved image to {}", output_path);
                } else {
                    println!("Not an image. Skipping...")
                }
            }
            Err(e) => println!("Error reading file: {}", e),
        }
    }
}
