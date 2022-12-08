#[macro_use]
use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Material {
    pub name: String,
    pub specular_reflection: f64,
    pub diffuse_reflection: f64,
    pub reflectiveness: f64,
    pub color: Vec3
}

lazy_static! {
    pub static ref MATERIAL_LIBRARY: [Material; 5] = [
        ("gray", 0.0, 0.2, 0.0, (0.443, 0.47, 0.56)),
        ("mint", 0.0, 0.1, 0.0, (0.039, 0.819, 0.521)),
        ("pink", 0.0, 0.65, 0.0, (0.921, 0.164, 0.38)),
        ("mirror", 0.3, 0.34, 1.0, (1.0, 1.0, 1.0)),
        ("mirror", 0.3, 0.34, 1.0, (1.0, 1.0, 1.0))
    ].map(|(name, specular_reflection, diffuse_reflection, reflectiveness, color)|
        Material {
            name: name.to_string(),
            specular_reflection,
            diffuse_reflection,
            reflectiveness,
            color: Vec3::from(color)
        }
    );
}
