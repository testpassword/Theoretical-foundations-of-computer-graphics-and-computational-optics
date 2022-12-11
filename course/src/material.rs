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

impl Material {
    pub fn brdf(&self, light_dir: Vec3, camera_dir: Vec3, include_kd: bool) -> f64 {
        let brdf_ks = 0.0_f64.max(light_dir.dot(camera_dir)) * self.specular_reflection;
        let brdf_kd = if include_kd { self.diffuse_reflection } else { 0.0 };
        brdf_kd + brdf_ks
    }
}

lazy_static! {
    pub static ref MATERIAL_LIBRARY: [Material; 5] = [
        ("yellow", 0.2, 0.2, 0.0, (1.0, 0.886, 0.749)),
        ("mint", 0.1, 0.1, 0.0, (0.039, 0.819, 0.521)),
        ("pink", 0.1, 0.65, 0.0, (0.921, 0.164, 0.38)),
        ("blue", 0.9, 0.2, 0.3, (0.0, 0.0, 1.0)),
        ("mirror", 0.3, 0.34, 0.5, (1.0, 1.0, 1.0)),
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
