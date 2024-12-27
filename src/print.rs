use std::io::Write;

use crate::term::ANSI_COLOURS;
use ansi_term::ANSIStrings;
use ansi_term::Colour::Fixed;
use image::{
    imageops::{self, FilterType},
    Pixel,
};

pub fn print_image(
    img: image::DynamicImage,
    writer: &mut impl Write,
    true_colour: bool,
    width: u32,
    height: u32,
) {
    let img = imageops::resize(&img, width, height, FilterType::Nearest);

    if !true_colour {
        for y in 0..height {
            //TODO: inc by 2 instead
            if y % 2 == 1 || y + 1 == height {
                continue;
            }

            let row: Vec<_> = (0..width)
                .map(|x| {
                    let mut top = img[(x, y)];
                    let mut bottom = img[(x, y + 1)];
                    blend_alpha(&mut top);
                    blend_alpha(&mut bottom);
                    let top_colour = find_colour_index(top.to_rgb().channels());
                    let bottom_colour = find_colour_index(bottom.to_rgb().channels());
                    Fixed(bottom_colour).on(Fixed(top_colour)).paint("▄")
                })
                .collect();

            let to_print = format!("{}\n", ANSIStrings(&row));
            writer.write_all(to_print.as_bytes()).unwrap();
        }
    } else {
        let mut row = Vec::new();
        for y in 0..height {
            //TODO: inc by 2 instead
            if y % 2 == 1 || y + 1 == height {
                continue;
            }

            for x in 0..width {
                let mut top = img[(x, y)];
                let mut bottom = img[(x, y + 1)];
                blend_alpha(&mut top);
                blend_alpha(&mut bottom);
                write!(
                    row,
                    "\x1b[48;2;{};{};{}m\x1b[38;2;{};{};{}m▄",
                    top[0], top[1], top[2], bottom[0], bottom[1], bottom[2]
                )
                .unwrap();
            }
            writeln!(row, "\x1b[m").unwrap();
            writer.write_all(&row).unwrap();
            row.clear();
        }
    }
}

pub(crate) fn find_colour_index(pixel: &[u8]) -> u8 {
    let mut best = 0;
    let mut best_distance = 255 * 255 * 3 + 1;
    for (idx, ansi_colour) in ANSI_COLOURS.iter().enumerate() {
        let dr = ansi_colour[0] - pixel[0] as i32;
        let dg = ansi_colour[1] - pixel[1] as i32;
        let db = ansi_colour[2] - pixel[2] as i32;
        let distance = dr * dr + dg * dg + db * db;

        if distance < best_distance {
            best_distance = distance;
            best = idx as u8;
        }
    }

    best
}

pub(crate) fn blend_alpha(pixel: &mut image::Rgba<u8>) {
    let alpha = pixel[3] as i32 as f32 / 255.0;
    pixel[0] = (alpha * (pixel[0] as i32 as f32) + (1.0 - alpha) * 38.0) as u8;
    pixel[1] = (alpha * (pixel[1] as i32 as f32) + (1.0 - alpha) * 38.0) as u8;
    pixel[2] = (alpha * (pixel[2] as i32 as f32) + (1.0 - alpha) * 38.0) as u8;
}
