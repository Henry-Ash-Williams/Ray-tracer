#![allow(dead_code, unused_imports, unused_variables)]
extern crate image;
extern crate utils;

use std::{
    error::Error
};

mod scene;

use utils::{
    constants,
    shapes::*,
    Vector3
};

fn main() -> Result<(), Box<dyn Error>> {
    // Create the image buffer with a 1080p resolution (1920x1080)
    let mut image = image::ImageBuffer::new(
        constants::IMAGE_WIDTH, constants::IMAGE_HEIGHT
    );

    let scene: Vec<Box<dyn Intersection>> = Vec::new();

    scene.push(Box::new(sphere::Sphere::new(
        Vector3::new(2.01009f32, 2.3031f32, 0.466287f32),
        1f32, 
        (0x66u8, 0x00u8, 0xFFu8)
    )));
    
    scene.push(Box::new(cube::Cube::new(
        Vector3::new(2.01009f32, 2.3031f32, 0.466287f32),
        1f32, 
        (0x66u8, 0x00u8, 0xFFu8)
    )));

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