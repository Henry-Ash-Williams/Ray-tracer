use crate::Vector3;

pub mod sphere;
pub mod cube;

pub trait Intersection {
    fn intersects(&self, s: Vector3<f32>, d: Vector3<f32>) -> Option<Vector3<f32>>;
}