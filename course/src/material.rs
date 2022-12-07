#[macro_use]
use std::collections::HashMap;
use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Material {
    pub name: String,
    pub specular_reflection: f64,
    pub diffuse_reflection: HashMap<i64, f64> // wavelength (nm), Kd
}

// todo: metal, mirror, glass
// todo: 450, 550, 650
// todo: get most near kd from diffuse_reflection or ONE kd for all

lazy_static! {
    pub static ref MATERIAL_LIBRARY: [Material; 5] = [
        ([0.343, 0.747, 0.740, 0.737], 0.0, ""),
        ([0.092, 0.285, 0.160, 0.159], 0.0, ""),
        ([0.040, 0.058, 0.287, 0.642], 0.0, ""),
        ([0.343, 0.747, 0.740, 0.737], 0.5, ""),
        ([0.343, 0.747, 0.740, 0.737], 0.5, "")
    ].map(|(dr, specular_reflection, name)|
        Material {
            diffuse_reflection: HashMap::from([(400, dr[0]), (500, dr[1]), (600, dr[2]), (700, dr[3])]),
            specular_reflection,
            name: name.to_string()
        }
    );
}
