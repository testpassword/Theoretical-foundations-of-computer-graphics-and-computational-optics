#[macro_use]
use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Material {
    pub name: String,
    pub specular_reflection: f64,
    pub diffuse_reflection: f64,
    pub color: Vec3
}

// todo: metal, mirror, glass

lazy_static! {
    pub static ref MATERIAL_LIBRARY: [Material; 5] = [
        (0.2, 0.0, "gray", (0.443, 0.47, 0.56)),
        (0.1, 0.0, "mint", (0.039, 0.819, 0.521)),
        (0.65, 0.0, "pink", (0.921, 0.164, 0.38)),
        (0.34, 0.5, "white", (1.0, 1.0, 1.0)),
        (0.34, 0.5, "white", (1.0, 1.0, 1.0))
    ].map(|(kd, specular_reflection, name, color)|
        Material {
            diffuse_reflection: kd,
            specular_reflection,
            name: name.to_string(),
            color: Vec3::from(color)
        }
    );
}
