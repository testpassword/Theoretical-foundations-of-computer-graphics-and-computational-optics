use std::collections::HashMap;
use crate::vec3::Vec3;
use crate::light::Light;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub bright_coefs: HashMap<i32, f32>,
    pub L: HashMap<i32, f32> // L - яркость
}

impl Ray {
    pub fn fill_wavelength(&mut self, ls: Light) {
        self.bright_coefs.clear();
        self.L.clear();
        for wavelength in ls.color_distribution.into_keys() {
            self.bright_coefs.insert(wavelength, 1.0);
            self.L.insert(wavelength, 0.0);
        }
        // todo: не забыть, что в hashmap не опеределён порядок
    }
}

impl Default for Ray {
    fn default() -> Self {
       Ray {
           origin: Vec3 { ..Default::default() },
           direction: Vec3 {
               x: 0.0,
               y: 1.0,
               z: 0.0
           },
           bright_coefs: Default::default(),
           L: Default::default(),
       }
    }
}
