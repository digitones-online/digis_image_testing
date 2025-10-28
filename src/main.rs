use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Rgba};
use indicatif::{self, ProgressBar};
use std::io;

#[derive(Debug, PartialEq, Eq)]
struct ParseRgbaError;

fn main() {
    // Get forground image path
    let fg_path = get_input("Foreground Image:");

    // Get background image path
    let bg_path = get_input("Background Image:");

    // Get target color
    let target_color = get_input("Target Color:");
    let target_color_result = rgba_from_str(target_color);

    let target_color = match target_color_result {
        Ok(c) => c,
        Err(e) => panic!("Couldn't parse color: {e:?}"),
    };

    // Get chroma key range
    let range_string = get_input("Chroma Key Range:");
    let range_size_result = range_string.trim().parse::<i32>();

    let range = match range_size_result {
        Ok(v) => v,
        Err(e) => panic!("Invalid Input: {e:?}"),
    };

    // Get save path
    let save_path = get_input("Save Path:");

    // Open foreground image
    let img_result = image::open(fg_path.trim());
    let img;
    match img_result {
        Ok(i) => img = i,
        Err(e) => panic!("Failed to open image {e:?}"),
    };

    // Open background image
    let img2_result = image::open(bg_path.trim());
    let img2;
    match img2_result {
        Ok(i) => img2 = i,
        Err(e) => panic!("Failed to open image {e:?}"),
    };

    println!("\nWorking...");

    let img3 = chroma_key(&img, &img2, target_color, range as u8);

    // Output image
    println!("Done");
    let img_save_result = img3.save(save_path.trim());
    match img_save_result {
        Ok(_) => (),
        Err(e) => panic!("Failed to save image {e:?}"),
    };
}

// Iterates over pixels, and applies chroma key
fn chroma_key(
    fg_img: &DynamicImage,
    bg_img: &DynamicImage,
    tgt_color: Rgba<u8>,
    range: u8,
) -> DynamicImage {
    let mut dst_img = copy_img(fg_img);
    let pb = ProgressBar::new(20);
    pb.set_style(indicatif::ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("##-"));

    // Iterates over foreground pixels
    for fg_pixel in pb.wrap_iter(fg_img.pixels()) {
        // Compares current pixel with the target color
        if pixel_comp(fg_pixel.2.channels(), tgt_color.0, range) {
            let bg_pixel;

            // Avoid indexing background image out of bounds
            if fg_pixel.0 >= bg_img.width() || fg_pixel.1 >= bg_img.height() {
                // Black if out of bounds
                bg_pixel = Rgba([0, 0, 0, 255]);
            } else {
                bg_pixel = bg_img.get_pixel(fg_pixel.0, fg_pixel.1);
            }
            dst_img.put_pixel(fg_pixel.0, fg_pixel.1, bg_pixel);
        }
    }

    dst_img
}

// Compares to pixels (p1, p2), with a margin of error (range)
fn pixel_comp(p1: &[u8], p2: [u8; 4], range: u8) -> bool {
    let mut state = true;
    let mut i = 0;

    while i <= 3 {
        if p1[i] as i16 >= p2[i] as i16 + range as i16
            || p1[i] as i16 <= p2[i] as i16 - range as i16
        {
            state = false;
        }
        i = i + 1;
    }

    state
}

// Copies one image to another
fn copy_img(src_img: &DynamicImage) -> DynamicImage {
    let img_width = src_img.width();
    let img_height = src_img.height();

    let mut dst_img = DynamicImage::new(img_width, img_height, src_img.color());
    let _ = dst_img.copy_from(src_img, 0, 0);

    return dst_img;
}

// Get user input with prompt string
fn get_input(s: &str) -> String {
    println!("{}", s);
    let mut new_string = String::new();

    io::stdin()
        .read_line(&mut new_string)
        .expect("Failed to read line");

    new_string
}

// Converts a string to an RGBA color
fn rgba_from_str(s: String) -> Result<Rgba<u8>, ParseRgbaError> {
    let c1 = s
        .strip_prefix("(")
        .and_then(|s| s.strip_suffix(")\n"))
        .ok_or(ParseRgbaError)?;

    let c: Vec<&str> = c1.split_terminator(",").collect();

    let r = c[0].parse::<u8>().map_err(|_| ParseRgbaError)?;
    let g = c[1].parse::<u8>().map_err(|_| ParseRgbaError)?;
    let b = c[2].parse::<u8>().map_err(|_| ParseRgbaError)?;
    let a = c[3].parse::<u8>().map_err(|_| ParseRgbaError)?;

    //println!("{},{},{},{}", r, g, b, a);

    Ok(Rgba([r, g, b, a]))
}
