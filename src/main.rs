use image::{DynamicImage, GenericImageView};
use std::cmp::{max, min};
use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::Path;

fn process_image(input_path: &str) -> Option<(u32, u32, DynamicImage)> {
    let img = image::open(&Path::new(input_path));
    if img.is_err() {
        println!("Error: {}", img.err().unwrap());
        return None;
    }

    let width = img.as_ref().unwrap().width();
    let height = img.as_ref().unwrap().height();

    return Some((width, height, img.as_ref().unwrap().clone()));
}
fn main() {
    let mut src_img_path = String::new();
    print!("Please enter image path: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut src_img_path)
        .expect("Did not enter a correct string");
    if let Some('\n') = src_img_path.chars().next_back() {
        src_img_path.pop();
    }
    if let Some('\r') = src_img_path.chars().next_back() {
        src_img_path.pop();
    }
    let img_info_result = process_image(&src_img_path);
    if img_info_result.is_none() {
        println!("Could not process image {}", src_img_path);
        return;
    }
    let img_info = img_info_result.unwrap();
    let mut new_img = image::ImageBuffer::new(img_info.0 * 4, img_info.1);
    let mut avg_r: u32 = 0;
    let mut avg_g: u32 = 0;
    let mut avg_b: u32 = 0;
    let mut count = 0;
    for pixel in img_info.2.pixels() {
        new_img.put_pixel(
            pixel.0,
            pixel.1,
            image::Rgb([pixel.2[0], pixel.2[1], pixel.2[2]]),
        );

        avg_r += pixel.2[0] as u32;
        avg_g += pixel.2[1] as u32;
        avg_b += pixel.2[2] as u32;
        count += 1;
    }
    avg_r /= count;
    avg_g /= count;
    avg_b /= count;
    for (idx, avg) in [
        (max(avg_r - 20, 0), max(avg_b - 20, 0), max(avg_b - 20, 0)),
        (avg_r, avg_g, avg_b),
        (
            min(avg_r + 20, 255),
            min(avg_b + 20, 255),
            min(avg_b + 20, 255),
        ),
    ]
    .iter()
    .enumerate()
    {
        for pixel in img_info.2.pixels() {
            let current_rgb = image::Rgb([pixel.2[0], pixel.2[1], pixel.2[2]]);
            if current_rgb[0] > avg.0 as u8
                && current_rgb[1] > avg.1 as u8
                && current_rgb[2] > avg.2 as u8
            {
                new_img.put_pixel(
                    pixel.0 + img_info.0 * (idx as u32 + 1),
                    pixel.1,
                    image::Rgb([255, 255, 255]),
                );
            } else {
                new_img.put_pixel(
                    pixel.0 + img_info.0 * (idx as u32 + 1),
                    pixel.1,
                    image::Rgb([0, 0, 0]),
                );
            }
        }
    }
    fs::create_dir_all("images").unwrap();
    new_img.save("images/comparison.png").unwrap();
}
