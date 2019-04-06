mod image_comparator;
mod input_helper;
fn main() {
    let image_path1 = input_helper::get_image_path(1usize);
    let image_path2 = input_helper::get_image_path(2usize);
    let acpt_diff   = input_helper::get_acpt_diff();
    
    let mut images = image_comparator::Images::new();

    images.set_image_1(image_path1);
    images.set_image_2(image_path2);
    images.set_acpt_diff(acpt_diff);

    print!("image 1 and 2 are {:.2}% equal -> acceptable difference {}%", images.get_similarity(), images.get_acpt_diff());
}

