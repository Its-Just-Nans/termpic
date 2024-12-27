use crate::print::print_image;
use crate::term::determine_size;
use clap::Parser;
use image::GenericImageView;
use std::io::Write;
use std::path::Path;

const DESC: &str = "By default it will use as much of the current terminal window as possible, while maintaining the aspect ratio of the input image. This can be overridden as follows.";

/// Display images in the terminal
#[derive(Parser, Debug)]
#[command(version, about, long_about = DESC)]
pub struct TermpicArgs {
    /// Width of the output image
    #[clap(short, long)]
    pub width: Option<u32>,

    /// Height of the output image
    #[clap(short = 's', long)]
    pub height: Option<u32>,

    /// Use true colour (24-bit) output
    #[clap(short, long, visible_alias = "true-color")]
    pub true_colour: bool,

    /// Input file
    #[clap(name = "FILE")]
    pub file: String,

    /// Output file
    #[clap(short = 'o', long)]
    pub output: Option<String>,
}

pub fn main_cli() {
    let args = TermpicArgs::parse();

    let img = image::open(Path::new(&args.file)).unwrap();
    let (orig_width, orig_height) = img.dimensions();
    let (width, height) = determine_size(&args, orig_width, orig_height);

    let mut output_writer: Box<dyn Write> = if let Some(output) = args.output {
        Box::new(std::fs::File::create(output).unwrap())
    } else {
        Box::new(std::io::stdout())
    };

    print_image(img, &mut output_writer, args.true_colour, width, height);
}
