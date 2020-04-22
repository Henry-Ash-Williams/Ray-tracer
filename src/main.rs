#![allow(dead_code)]
extern crate image;

mod constants;

use std::{
    error::Error
};

/// Declaration of a sphere
pub struct Sphere {
    /// Cartesian position of the origin of a sphere 
    position: (f32, f32, f32),
    /// Radius of the sphere 
    radius: f32,
    /// RGB representation of the sphere 
    colour: (u8, u8, u8)
}

impl Sphere {
    /// Initialization of a sphere 
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
    let mut image = image::ImageBuffer::new(constants::IMAGE_WIDTH, constants::IMAGE_HEIGHT);

    // Fills the image with a background
    for y in 0 .. constants::IMAGE_HEIGHT {
        for x in 0 .. constants::IMAGE_WIDTH {
            let pixel = image.get_pixel_mut(x, y);
            let image::Rgb(_data) = *pixel;
            *pixel = image::Rgb(constants::IMAGE_BACKGROUND);
        }
    }

    image.save("image.png")?;

    Ok(())
}