use std::collections::HashMap;
use crate::{
    vec3::Vec3,
    lights::point_light::PointLight,
};

#[derive(Debug)]
pub struct Ray {
    pub position: Vec3,
    pub direction: Vec3,
    pub radiance: f64,
}

impl Default for Ray {
    fn default() -> Self {
        Ray {
            position: Default::default(),
            direction: Default::default(),
            radiance: 0.0,
        }
    }
}
