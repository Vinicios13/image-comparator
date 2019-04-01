#![allow(dead_code, unused_variables)]
extern crate image;
use std::env;
use image::GenericImageView;

fn main() {
    let image_path1 = get_image_path(1usize);
    let image_path2 = get_image_path(2usize);

    let image_1 = image::open(image_path1).expect("failed to open image 1");
    let image_2 = image::open(image_path2).expect("failed to open image 2");

    let image_1_rgb = image_1.to_rgb();
    let image_2_rgb = image_2.to_rgb();

    if image_2.dimensions() != image_2.dimensions() {
        panic!("Images with different dimensions");
    }

    let matching = image_2_rgb.iter().zip(image_1_rgb.iter()).filter(|(a, b)| a == b).count();

    println!("image 1 and 2 are {:.2}% equal", (matching as f32/image_2_rgb.len() as f32) * 100f32);
}

fn get_image_path(index:usize) -> String {
    let args: Vec<String> = env::args().collect();

    if index >= args.len() {
        panic!("Image path wasn't given");
    }

    String::from(format!("./assets/{}", args[index].clone()))
}