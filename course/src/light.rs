use std::collections::HashMap;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Light {
    pub position: Vec3, // it is static coords, but we can use Vec3 as a vertex
    pub intensity: f32,
    pub color_distribution: HashMap<i32, f32> // wavelengths (nm), density
}
