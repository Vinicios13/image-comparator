#![allow(dead_code, unused_variables)]
extern crate image;
use std::env;
use image::{DynamicImage, ImageResult};  
use image::GenericImageView;  
fn main() {
    let image_path1 = get_image_path(1usize);
    let image_path2 = get_image_path(2usize);

    let image_1: ImageResult<DynamicImage> = image::open(format!("./assets/{}", image_path1));
    let image_2: ImageResult<DynamicImage> = image::open(format!("./assets/{}", image_path2));

    let img_1 = image_1.expect("failed to open image 1");
    let img_2 = image_2.expect("failed to open image 2");

    let img_1_rgb = img_1.to_rgb();
    let img_2_rgb = img_2.to_rgb();

    if img_1.dimensions() != img_2_rgb.dimensions() {
        panic!("Images with different dimensions");
    }

    let matching = img_2_rgb.iter().zip(img_1_rgb.iter()).filter(|(a, b)| a == b).count();

    println!("image 1 and 2 are {:.2}% equal", (matching as f32/img_2_rgb.len() as f32) * 100f32);
}

fn get_image_path(index:usize) -> String {
    let args: Vec<String> = env::args().collect();

    if index >= args.len() {
        panic!("Image path wasn't given");
    }

    String::from(args[index].clone())
}