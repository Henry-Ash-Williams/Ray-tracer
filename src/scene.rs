extern crate image;
extern crate utils;

use utils::{
    constants,
    shapes::*,
    Vector3
};

pub struct Scene {
    pub UpDirection: Vector3<f32>,
    pub LeftDirection: Vector3<f32>,
    pub LookDirection: Vector3<f32>,
    pub EyePosition: Vector3<f32>,
    pub MeshTopLeft: Vector3<f32>,
    pub MeshDistance: f32,
    pub PixelWidth: u32,
    pub ScreenWidth: u32,
    pub ScreenHeight: u32,
    pub scene_objects: Vec<Box<dyn Intersection>> 
}

impl Scene {
    pub fn new(
        LeftDirection: Vector3<f32>,
        LookDirection: Vector3<f32>,
        EyePosition: Vector3<f32>,
        MeshDistance: f32,
        PixelWidth: u32,
        ScreenWidth: u32,
        ScreenHeight: u32,
    ) -> Scene {
        let UpDirection = LookDirection.cross(&LeftDirection).normalize();
        let MeshTopLeft = EyePosition + (MeshDistance * LookDirection) + ((PixelWidth * ScreenWidth / 2) as f32 * LeftDirection) + ((PixelWidth * ScreenHeight / 2) as f32 * UpDirection);
        
        Scene {
            UpDirection: UpDirection,
            LeftDirection: LeftDirection,
            LookDirection: LookDirection,
            EyePosition: EyePosition,
            MeshTopLeft: MeshTopLeft,
            MeshDistance: MeshDistance,
            PixelWidth: PixelWidth,
            ScreenWidth: ScreenWidth,
            ScreenHeight: ScreenHeight,
            scene_objects: Vec::new()
        }
    }

    pub fn add_scene_object<O: 'static>(&mut self, object: O) where O: Intersection + Clone {
        self.scene_objects.push(Box::new(object));
    } 

    pub fn GetRayToMesh(&self, x: u32, y: u32) -> Vector3<f32> {
        let pixel_pos = self.MeshTopLeft - ((self.PixelWidth * x) as f32 * self.LeftDirection) - ((self.PixelWidth * y) as f32 * self.UpDirection);
        let ray = (pixel_pos - self.EyePosition).normalize();

        ray
    }

    pub fn get_ray_to_mesh(&self, x: u32, y: u32) -> Vector3<f32> {
        unimplemented!()
    }
}