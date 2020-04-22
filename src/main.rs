#![allow(dead_code, unused_imports, unused_variables)]
extern crate image;
extern crate nalgebra as na;

mod constants;
mod sphere;

use std::{
    error::Error
};

use na::Vector3;

fn main() -> Result<(), Box<dyn Error>> {
    let mut scene: Vec<sphere::Sphere> = Vec::new();

    let mut image = image::ImageBuffer::new(
        constants::IMAGE_WIDTH, constants::IMAGE_HEIGHT
    );

    scene.push(sphere::Sphere::new(
        Vector3::new(157f32, 1011f32, 414f32),
        5.0,
        (0x8Du8, 0x61u8, 0xEDu8)
    ));

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