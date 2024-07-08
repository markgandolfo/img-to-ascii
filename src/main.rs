use image::GenericImageView;
use std::env;
use std::process::exit;

fn get_str_ascii(intensity: u8) -> &'static str {
    // divide by 32 to get ~8 pieces
    let index = intensity / 32;
    let ascii = [" ", ".", ",", "-", "~", "+", "=", "@"];
    return ascii[index as usize];
}

fn get_image(file: &str, scale: u32) {
    if let Ok(image) = image::open(file) {
        let (width, height) = image.dimensions();

        // loop through the image and get the intensity of each pixel
        // Since images are flipped, we start with Y axis (which is really the X)
        for y in 0..height {
            for x in 0..width {
                if y % (scale * 2) == 0 && x % scale == 0 {
                    let pixel = image.get_pixel(x, y);
                    let mut intensity = pixel[0] / 3 + pixel[1] / 3 + pixel[2] / 3;

                    // if the value is transparent make it 0
                    if pixel[3] == 0 {
                        intensity = 0;
                    }

                    print!("{}", get_str_ascii(intensity));
                }
            }

            // print the newline character if we are at the end of the line
            if y % (scale * 2) == 0 {
                println!("");
            }
        }
    } else {
        eprintln!("Erorr: Could not open image file: {}", file);
        exit(1);
    }
}

fn print_usage_instructions() {
    println!("Usage: img-to-ascii <image file> <scale>");
    exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: No image file provided");
        print_usage_instructions();
        exit(1);
    } else if args.len() < 3 {
        eprintln!("Error: Please enter a scale");
        print_usage_instructions();
        exit(1);
    }

    let scale: u32 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: Scale must be a number");
            exit(1);
        }
    };

    get_image(&args[1], scale);
}
