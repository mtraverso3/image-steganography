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
    todo!("Implement me!");
}

fn extract_image(input_image_path: &str, output_image_path: &str) {
    todo!("Implement me!");
}