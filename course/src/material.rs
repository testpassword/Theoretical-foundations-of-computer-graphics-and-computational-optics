use lazy_static::lazy_static;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Material {
    pub name: String,
    pub specular_reflection: f64,
    pub diffuse_reflection: f64,
    pub color: Vec3
}

impl Material {
    pub fn brdf(&self, light_dir: Vec3, camera_dir: Vec3, include_kd: bool) -> f64 {
        let brdf_ks = 0.0_f64.max(light_dir.dot(camera_dir)) * self.specular_reflection;
        let brdf_kd = if include_kd { self.diffuse_reflection } else { 0.0 };
        brdf_kd + brdf_ks
    }
}

lazy_static! {
    pub static ref MATERIAL_LIBRARY: [Material; 5] = [
        ("black", 0.4, 0.2, (1.0, 1.0, 1.0)),
        ("white", 0.4, 0.1, (0.0, 0.0, 0.0)),
        ("white", 0.4, 0.65, (0.0, 0.0, 0.0)),
        ("blue", 0.4, 0.2, (0.0 / 255.0, 125.0 / 255.0, 195.0 / 255.0)),
        ("white", 0.4, 0.34, (0.0, 0.0, 0.0)),
    ].map(|(name, specular_reflection, diffuse_reflection, color)|
        Material {
            name: name.to_string(),
            specular_reflection,
            diffuse_reflection,
            color: Vec3::from(color)
        }
    );
}
