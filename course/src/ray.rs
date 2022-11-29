use std::collections::HashMap;
use crate::vec3::Vec3;
use crate::light::Light;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub bright_coefs: HashMap<i32, f32>,
    pub radiance: HashMap<i32, f32>
}

impl Ray {
    pub fn fill_wavelength(&mut self, ls: Light) {
        self.bright_coefs.clear();
        self.radiance.clear();
        for wavelength in ls.color_distribution.into_keys() {
            self.bright_coefs.insert(wavelength, 1.0);
            self.radiance.insert(wavelength, 0.0);
        }
        // todo: не забыть, что в hashmap не опеределён порядок
    }
}
