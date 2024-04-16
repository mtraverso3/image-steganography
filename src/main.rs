use image::{GenericImageView, ImageBuffer, Rgb};
use clap::Parser;

/// A simple steganography tool that can embed and extract images from other images.
#[derive(Parser)]
enum Command {
    Extract(Extract),
    Embed(Embed),
}

/// Extracts an embedded image from another image.
#[derive(Parser)]
struct Extract {
    /// Input image path
    #[clap(short, long, default_value = "in.png")]
    input: String,
    /// Output image path
    #[clap(short, long, default_value = "out.png")]
    output: String,
}

/// Embeds an image into another image using the two least significant bits of each pixel color channel.
#[derive(Parser)]
struct Embed {
    /// Image to embed
    #[clap(short, long, default_value = "in.png")]
    input: String,
    /// Image to embed into
    #[clap(short, long)]
    target: String,
    /// Output image path
    #[clap(short, long, default_value = "out.png")]
    output: String,
}

fn main() {
    let command = Command::parse();

    match command {
        Command::Extract(extract) => {
            extract_image(&extract.input, &extract.output);
        }
        Command::Embed(embed) => {
            embed_image(&embed.input, &embed.target, &embed.output);
        }
    }
}

fn embed_image(input_image_path: &str, target_image_path: &str, output_image_path: &str) {
    let img = image::open(input_image_path).unwrap();
    let target_img = image::open(target_image_path).unwrap();
    let (width, height) = img.dimensions();
    
    if target_img.dimensions() != (width, height) {
        eprintln!("Error: The input image and target image must have the same dimensions.");
    }

    let mut new_img = ImageBuffer::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let pixel = img.get_pixel(x, y).0;
            let target_pixel = target_img.get_pixel(x, y).0;
            let mut new_pixel = [0; 3];

            // Grab the two least significant bits of each pixel color channel
            for k in 0..3 {
                new_pixel[k] = (target_pixel[k] & 0b11111100) | (pixel[k] >> 6);
            }

            new_img.put_pixel(x, y, Rgb(new_pixel));
        }
    }

    new_img.save(output_image_path).unwrap();
}

fn extract_image(input_image_path: &str, output_image_path: &str) {
    let img = image::open(input_image_path).unwrap();
    let (width, height) = img.dimensions();

    let mut new_img = ImageBuffer::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let pixel = img.get_pixel(x, y).0;
            let mut new_pixel = [0; 3];

            // Grab only the two least significant bits of each pixel color channel
            for k in 0..3 {
                new_pixel[k] = (pixel[k] & 0b00000011) << 6;
            }

            new_img.put_pixel(x, y, Rgb(new_pixel));
        }
    }

    new_img.save(output_image_path).unwrap();
}