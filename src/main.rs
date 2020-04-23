#![allow(dead_code, unused_imports, unused_variables)]
extern crate image;
extern crate nalgebra as na;
extern crate utils;

use std::{
    error::Error
};

use utils::{
    constants,
    shapes::* 
};

use na::Vector3;

fn main() -> Result<(), Box<dyn Error>> {
    // Create the image buffer with a 1080p resolution (1920x1080)
    let mut image = image::ImageBuffer::new(
        constants::IMAGE_WIDTH, constants::IMAGE_HEIGHT
    );

    let scene: Vec<dyn Intersection> = Vec::new();

    // Fills the image with a background
    for y in 0 .. constants::IMAGE_HEIGHT {
        for x in 0 .. constants::IMAGE_WIDTH {
            let pixel = image.get_pixel_mut(x, y);
            *pixel = image::Rgb(constants::IMAGE_BACKGROUND);
        }
    }

    image.save(constants::FILENAME)?;

    Ok(())
}