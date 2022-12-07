use crate::{
    vec3::Vec3,
    lights::{
        ray::Ray,
        point_light::PointLight
    },
};

pub struct Camera {
    pub fov: f64,
    pub position: Vec3,
}

impl Camera {
    pub fn create_ray_from_camera(&self, ray_x: f64, ray_y: f64, light: &PointLight) -> Ray {
        Ray::new(
            self.position,
            Vec3::from((ray_x, ray_y, -1.0)).normalize(),
            light
        )
    }
}
