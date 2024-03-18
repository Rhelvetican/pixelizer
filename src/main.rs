mod utils;
use rprompt::prompt_reply;
use std::fs::read_dir;
use utils::FilterMode;

const CONFIG: &str = "config/config.json";

const INTRO: &str = r#"
__________.__              .__  .__                     
\______   \__|__  ___ ____ |  | |__|_______ ___________ 
 |     ___/  \  \/  // __ \|  | |  \___   // __ \_  __ \
 |    |   |  |>    <\  ___/|  |_|  |/    /\  ___/|  | \/
 |____|   |__/__/\_ \\___  >____/__/_____ \\___  >__|   
                   \/    \/              \/    \/       
 ____   ________    _______      _______                
 \   \ /   /_   |   \   _  \     \   _  \               
  \   Y   / |   |   /  /_\  \    /  /_\  \              
   \     /  |   |   \  \_/   \   \  \_/   \             
    \___/   |___| /\ \_____  / /\ \_____  /             
                  \/       \/  \/       \/              
"#;

const SEPERATOR: &str = r#"

#########################################################

"#;
fn main() {
    // Initialize the program folders.
    utils::init();

    println!("{}", INTRO);

    let config = utils::read_json(CONFIG).unwrap();
    let contents = read_dir("input").unwrap();

    let scale = config["config"]["scale"].as_u64().unwrap_or(8) as u32;
    let radius = config["config"]["blur_radius"].as_u64().unwrap_or(3) as u32;
    let mode = FilterMode::from_str(config["config"]["filter_mode"].as_str().unwrap());
    let manual = config["manual"].as_bool().unwrap();
    let mut file_count = 0;

    for file in contents {
        if manual {
            match file {
                Ok(file) => {
                    println!("{}", SEPERATOR);
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
                        let img = utils::pixelize(img, scale, radius, mode);
                        img.save(&output_path).unwrap();
                        println!("Saved image to {}", output_path);
                        file_count += 1;
                    } else {
                        println!("{}", SEPERATOR);
                        println!("Not an image. Skipping... {}", path.to_str().unwrap())
                    }
                }
                Err(e) => {
                    println!("{}", SEPERATOR);
                    println!("Error reading file: {}", e)
                }
            }
        } else {
            match file {
                Ok(file) => {
                    println!("{}", SEPERATOR);
                    let path = file.path();
                    if utils::check_image(path.to_str().unwrap()) {
                        let img = utils::read_image(path.to_str().unwrap());
                        let output_path =
                            format!("output/{}.png", path.file_stem().unwrap().to_str().unwrap());
                        let img = utils::pixelize(img, scale, radius, mode);
                        img.save(&output_path).unwrap();
                        println!("Saved image to {}", output_path);
                        file_count += 1;
                    } else {
                        println!("{}", SEPERATOR);
                        println!("Not an image. Skipping... {}", path.to_str().unwrap())
                    }
                }
                Err(e) => {
                    println!("{}", SEPERATOR);
                    println!("Error reading file: {}", e)
                }
            }
        }
    }
    println!("{}", SEPERATOR);
    if file_count == 0 {
        println!("No images found in the input folder.");
    } else {
        println!("Processed {} images.", file_count);
    }
}
