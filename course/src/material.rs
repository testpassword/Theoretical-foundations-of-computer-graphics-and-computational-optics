#[macro_use]
use std::collections::HashMap;
use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Material {
    pub specular_reflection: f32,
    pub transparency: f32,
    pub diffuse_reflection: HashMap<i32, f32> // wavelength (nm), Kd
}

lazy_static! {
    pub static ref MATERIAL_LIBRARY: Vec<Material> = vec![
        Material {
            specular_reflection: 0.0,
            transparency: 0.0,
            diffuse_reflection: HashMap::new()
        }
    ];
}
