use image::RgbImage;
pub struct KeyImg {}

impl KeyImg {
    pub fn new() -> Self {
        KeyImg {}
    }

    pub fn new_black_image(width: u32, height: u32) -> RgbImage {
        RgbImage::new(width, height)
    }

    pub fn new_filled_image(width: u32, height: u32, r: u8, g: u8, b: u8, o: u8) -> RgbImage {
        // let mut img = RgbaImage::new(width, height);

        // for pixel in img.pixels_mut() {
        //     *pixel = image::Rgba([r, g, b, 255]);
        // }
        // img
        let pixel = image::Rgb([r, g, b]);

        // Create an image filled with that specific pixel
        let img = RgbImage::from_pixel(width, height, pixel);
        img
    }
}
