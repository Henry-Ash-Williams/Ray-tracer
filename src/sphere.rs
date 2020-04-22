extern crate nalgebra as na;

use na::Vector3;

use super::constants;

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
    /// Tests if a vector `v` intersects with the sphere
    pub fn intersects(&self, v: Vector3<f32>) -> bool {
        // TODO: actually implement this 
        unimplemented!()
    }
}