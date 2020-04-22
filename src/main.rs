#![allow(dead_code)]
extern crate image;

use std::{
    error::Error
};

/// Width of the image in pixels 
pub const IMAGE_WIDTH: u32 = 1920;

/// Height of the image in pixels 
pub const IMAGE_HEIGHT: u32 = 1080;

/// Background colour of the image
pub const IMAGE_BACKGROUND: [u8; 3] = [0x00u8, 0x00u8, 0x00u8];

pub struct Sphere {
    position: (f32, f32, f32),
    radius: f32,
    colour: (u8, u8, u8)
}

impl Sphere {
    pub fn new(
        position: (f32, f32, f32),
        radius: f32,
        colour: (u8, u8, u8)
    ) -> Sphere {
        Sphere {
            position: position,
            radius: radius,
            colour: colour
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut image = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    // Fills the image with a background
    for y in 0 .. IMAGE_HEIGHT {
        for x in 0 .. IMAGE_WIDTH {
            let pixel = image.get_pixel_mut(x, y);
            let image::Rgb(_data) = *pixel;
            *pixel = image::Rgb(IMAGE_BACKGROUND);
        }
    }

    image.save("image.png")?;

    Ok(())
}