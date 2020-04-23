use super::scene::Scene;

extern crate image;
extern crate utils;

use utils::{
    constants,
    shapes::*,
    Vector3
};

pub struct RayTracer {}

impl RayTracer {
    pub fn trace(s: Scene) {
        let mut image = image::ImageBuffer::new(
            constants::IMAGE_WIDTH, constants::IMAGE_HEIGHT
        );
        for y in 0 .. s.ScreenHeight {
            for x in 0 .. s.ScreenWidth {
                let colour = RayTracer::find_colour(&s, s.EyePosition, s.GetRayToMesh(x, y));

                let pixel = image.get_pixel_mut(x, y);
                *pixel = image::Rgb([colour.0, colour.1, colour.2]);
            }
        }
    }

    pub fn find_colour(scene: &Scene, s: Vector3<f32>, d: Vector3<f32>) -> (u8, u8, u8) {
        let mut closest_intersection_point: Vector3<f32>;
        let mut closest_object: Option<Box<dyn Intersection>> = None;

        for o in scene.scene_objects.iter() {
            let position_of_intersection = o.intersects(s, d);

            if let None = position_of_intersection {
                if let None = closest_object {

                }
            }
        }
        unimplemented!()
    }
}