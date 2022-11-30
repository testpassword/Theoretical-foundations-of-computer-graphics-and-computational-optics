use std::collections::HashMap;
use crate::{
    light::Light,
    vec3::Vec3
};

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub bright_coefs: HashMap<i32, f32>,
    pub radiance: HashMap<i32, f32>
}

impl Ray {
    pub fn fill_wavelength(&mut self, ls: &Light) {
        self.bright_coefs.clear();
        self.radiance.clear();
        for (wavelength, _) in &ls.color_distribution {
            self.bright_coefs.insert(wavelength.clone(), 1.0);
            self.radiance.insert(wavelength.clone(), 0.0);
        }
        // todo: не забыть, что в hashmap не опеределён порядок
    }
}
