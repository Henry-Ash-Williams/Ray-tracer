use crate::Vector3;

pub mod sphere;
pub mod cube;

pub trait Intersection {
    fn intersects(&self, v: Vector3<f32>) -> bool;
}