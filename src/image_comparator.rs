extern crate image;

pub struct Images {
    image_1    : Option<image::DynamicImage>,
    image_2    : Option<image::DynamicImage>,
    acpt_diff  : f64,
    image_1_rgb: Option<image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>>,
    image_2_rgb: Option<image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>>
}

impl Images {
    pub fn new() -> Images {
        Images {
            image_1    : None,
            image_2    : None,
            acpt_diff  : 0_f64,
            image_1_rgb: None,
            image_2_rgb: None,
        }
    }

    pub fn set_image_1(&mut self, image_path: String) {
        self.image_1 = Some(image::open(image_path).expect("failed to open image 1"))
    }

    pub fn set_image_2(&mut self, image_path: String) {
        self.image_2 = Some(image::open(image_path).expect("failed to open image 1"))        
    }

    pub fn set_acpt_diff(&mut self, acpt_diff: f64) {
        self.acpt_diff = acpt_diff
    }

    pub fn get_acpt_diff(&self) -> f64 {
        self.acpt_diff
    }

    pub fn get_similarity(&mut self) -> f64 {

        let matching = if self.both_are_some() {  self.comp_image() } else { 0_usize };
        matching as f64 /f64::from(self.get_image_1_rgb_width() * self.get_image_1_rgb_height()) * 100_f64
    }

    fn get_image_1_rgb_width(&mut self) -> u32 {
        self.get_image_1_rgb().width()
    }

    fn get_image_1_rgb_height(&mut self) -> u32 {
        self.get_image_1_rgb().height()
    }

    fn get_image_2_rgb_width(&mut self) -> u32 {
        self.get_image_2_rgb().width()
    }

    fn get_image_2_rgb_height(&mut self) -> u32 {
        self.get_image_2_rgb().height()
    }

    fn comp_image(&mut self) -> usize {
        if self.both_are_some() {
            
            if !self.same_size() {
                println!("WARNING - images don't have the same size, this might affect the precision")
            }

            return
            self.get_image_1_rgb().pixels()
            .zip(self.get_image_2_rgb().pixels())
            .filter(|(pixel_1, pixel_2)| { 
                let diff_red   = (f64::from(pixel_1[0]) - f64::from(pixel_2[0])).abs()/255f64;
                let diff_green = (f64::from(pixel_1[1]) - f64::from(pixel_2[1])).abs()/255f64;
                let diff_blue  = (f64::from(pixel_1[2]) - f64::from(pixel_2[2])).abs()/255f64;

                let avg_color_diff =  (diff_red + diff_green + diff_blue) / 3f64 * 100f64;

                avg_color_diff <= self.get_acpt_diff()
            })
            .count()
        }
        0usize
    }

    fn get_image_1_rgb(&mut self) -> image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> {
        if self.image_1_is_some() {
            if !self.image_1_rgb_is_some() {
                self.image_1_rgb = Some(self.image_1.clone().unwrap().to_rgb());
            }
        } else {
            panic!("Image 1 wasn't set")
        }

        self.image_1_rgb.clone().unwrap()
    }

    fn get_image_2_rgb(&mut self) -> image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> {
        if self.image_2_is_some() {
            if !self.image_2_rgb_is_some() {
                self.image_2_rgb = Some(self.image_2.clone().unwrap().to_rgb());
            }
        } else {
            panic!("Image 2 wasn't set")
        }

        self.image_2_rgb.clone().unwrap()
    }

    fn both_are_some(&self) -> bool {
        self.image_1_is_some() && self.image_2_is_some()
    }

    fn image_1_is_some(&self) -> bool {
        self.image_1.is_some()
    }

    fn image_2_is_some(&self) -> bool {
        self.image_2.is_some()
    }

    fn image_1_rgb_is_some(&self) -> bool {
        self.image_1_rgb.is_some()
    }

    fn image_2_rgb_is_some(&self) -> bool {
        self.image_2_rgb.is_some()
    }

    fn same_size(&mut self) -> bool {
        if self.both_are_some() {
            self.get_image_1_rgb_width() == self.get_image_2_rgb_width() && 
            self.get_image_1_rgb_height() == self.get_image_2_rgb_height()
        } else {
            panic!("One or both of the images wasn't set")
        }
    }
}