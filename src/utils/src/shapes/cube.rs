use crate::Vector3; 

use super::Intersection;

#[derive(Debug, Clone, Copy)]
/// Declaration of a sphere
pub struct Cube {
    /// Cartesian position of the origin of the cube 
    origin: Vector3<f32>,

    /// Side length of the cube 
    side_length: f32,

    /// Colour of the cube
    colour: (u8, u8, u8)
}

impl Cube {
    /// Initialization of a sphere 
    pub fn new(
        origin: Vector3<f32>,
        side_length: f32,
        colour: (u8, u8, u8)
    ) -> Cube {
        Cube {
            origin: origin,
            side_length: side_length,
            colour: colour
        }
    }
}

impl Intersection for Cube {
    fn intersects(&self, s: Vector3<f32>, d: Vector3<f32>) -> Option<Vector3<f32>> {
        None 
    }
}