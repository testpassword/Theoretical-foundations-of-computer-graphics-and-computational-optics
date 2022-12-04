use std::collections::HashMap;
use crate::{
    light::Light,
    vec3::Vec3
};

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub bright_coefs: HashMap<i64, f64>,
    pub radiance: HashMap<i64, f64>
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, light_source: &Light) -> Ray {
        let fill = |i: f64| -> HashMap<_, _> {
            light_source
                .color_distribution
                .keys()
                .map(|wl| (*wl, i))
                .collect()
        };
        Ray {
            origin,
            direction,
            bright_coefs: fill(1.0),
            radiance: fill(0.0)
        }
    }
}

impl Default for Ray {
    fn default() -> Self {
        Ray {
            origin: Default::default(),
            direction: Default::default(),
            bright_coefs: HashMap::new(),
            radiance: HashMap::new()
        }
    }
}
