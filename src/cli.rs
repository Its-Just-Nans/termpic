use crate::print::print_image;
use crate::term::determine_size;
use clap::Parser;
use image::GenericImageView;
use std::io::Write;
use std::path::Path;

const DESC: &str = "By default it will use as much of the current terminal window as possible, while maintaining the aspect ratio of the input image. This can be overridden as follows.";

#[derive(Parser, Debug)]
#[command(version, about, long_about = DESC)]
pub struct TermpicArgs {
    #[clap(short, long)]
    pub width: Option<u32>,

    #[clap(short = 'l', long)]
    pub height: Option<u32>,

    #[clap(short, long, alias = "true-color")]
    pub true_colour: bool,

    pub file: String,

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
