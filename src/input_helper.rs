use std::env;

pub fn get_image_path(index:usize) -> String {
    let args: Vec<String> = env::args().collect();

    if index >= args.len() {
        panic!("Image path wasn't given");
    }

    format!("./assets/{}", args[index].clone())
}

pub fn get_acpt_diff() -> f64 {
    let args: Vec<String> = env::args().collect();

    if 3_usize >= args.len() { 
        0_f64
    } else {
        match args[3_usize].clone().to_string().parse::<f64>() {
            Ok(value) => value,
            Err(_)    => {
                println!("failed casting to float, default value of 0 will be used instead");
                0_f64
            }  
        }
    }
}