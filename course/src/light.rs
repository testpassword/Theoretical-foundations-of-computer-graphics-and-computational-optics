use std::collections::HashMap;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Light {
    pub position: Vec3,
    pub intensity: f32,
    pub color_distribution: HashMap<i32, f32> // wavelengths (nm), density
}
