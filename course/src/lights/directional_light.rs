use crate::vec3::Vec3;

#[derive(Debug)]
pub struct DirectionalLight {
    pub direction: Vec3,
    pub intensity: f64,
}
