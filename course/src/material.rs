use std::collections::HashMap;

pub struct Material {
    pub specular_reflection: f32,
    pub transparency: f32,
    pub diffuse_reflection: HashMap<i32, f32> // wavelength (nm), Kd
}
