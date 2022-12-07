use std::collections::HashMap;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct PointLight {
    pub position: Vec3,
    pub intensity: f64,
    pub color_distribution: HashMap<i64, f64> // wavelengths (nm), density
}
