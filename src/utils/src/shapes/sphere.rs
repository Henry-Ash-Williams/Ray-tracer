use crate::Vector3;

use super::Intersection;

use std::marker::Sized;

#[derive(Debug, Clone, Copy)]
/// Declaration of a sphere
pub struct Sphere {
    /// Cartesian position of the origin of a sphere 
    position: Vector3<f32>,
    /// Radius of the sphere 
    radius: f32,
    /// RGB representation of the sphere 
    colour: (u8, u8, u8)
}

impl Sphere {
    /// Initialization of a sphere 
    pub fn new(
        position: Vector3<f32>,
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

impl Intersection for Sphere {
    fn intersects(&self, s: Vector3<f32>, d: Vector3<f32>) -> Option<Vector3<f32>> {
        None 
    }
}