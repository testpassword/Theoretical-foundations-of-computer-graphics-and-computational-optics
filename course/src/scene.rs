use std::ops::Mul;
use rayon::prelude::*;
use image::{
    Rgb,
    RgbImage
};
use crate::{
    vec3::Vec3,
    camera::Camera,
    polygon::Polygon,
    geom_loaders::load_geometry,
    utils::{
        create_grid,
        to0_255_color_format
    },
    lights::{
        ray::Ray,
        point_light::PointLight
    },
    material::{
        Material,
        MATERIAL_LIBRARY
    }
};

pub struct Scene<'s> {
    pub path: String,
    pub point_light: &'s PointLight,
    pub geometry: Vec<Polygon<'s>>,
    pub camera: &'s Camera,
    pub width: u32,
    pub height: u32,
    pixel_buffer: Vec<(u32, u32, Ray)>, // radiance and color
}

impl<'s> Scene<'_> {
    pub fn new(
        path: &str,
        point_light: &'s PointLight,
        camera: &'s Camera
    ) -> Scene<'s> {
        Scene {
            path: path.to_string(),
            point_light,
            geometry: load_geometry(path),
            camera,
            pixel_buffer: vec![],
            width: 0,
            height: 0
        }
    }

    fn scene_intersect(&self, ray: &Ray) -> (bool, &Material, Vec3, Vec3) {
        let mut t: f64 = 0.0;
        let mut n_hit = Vec3 { ..Default::default() };
        let mut n_n = Vec3 { ..Default::default() };
        let mut n_m = &MATERIAL_LIBRARY[0]; // just use first material as default
        let mut triangle_dist = f64::MAX;
        for p in &self.geometry {
            let (intersect, res) = p.intersected(ray, t);
            t = res;
            if intersect && t < triangle_dist {
                triangle_dist = t;
                n_hit = ray.position + ray.direction * t;
                n_n = p.normal_by_observer(ray.position - n_hit);
                n_m = &p.material;
            }
        }
        (triangle_dist < f64::MAX, n_m, n_hit, n_n)
    }

    fn cast_ray(&self, mut ray: Ray, depth: i64) -> Ray {
        let (intersect, material, hit, N) = self.scene_intersect(&ray);
        ray.color = material.color;
        if depth > 5 || !intersect { ray }
        else {
            let camera_dir = (ray.position - hit).normalize();
            let reflect_dir = ray.direction.normalize().reflect(N.normalize()).normalize();
            let reflect_origin = hit + N * 1e-3;
            let mut reflect_ray = Ray {
                position: reflect_origin,
                direction: reflect_dir,
                ..Default::default()
            };
            if material.reflectiveness > 0.0 && reflect_dir.dot(N) > 0.0 {
                reflect_ray = self.cast_ray(reflect_ray, depth + 1);
            }
            let light_dir = (self.point_light.position - hit).normalize();
            let minus_light_dir = (hit - self.point_light.position).normalize();
            let dist = (self.point_light.position - hit).len();
            let cos_theta = light_dir.dot(N);
            let mut include_Kd = if cos_theta <= 0.0 { false } else { true };
            let shadow_origin = hit + N * 1e-3;
            let shadow_ray = Ray {
                position: shadow_origin,
                direction: light_dir,
                ..Default::default()
            };
            let (shadow_intersect, _shadow_material, shadow_hit, _shadow_N) = self.scene_intersect(&shadow_ray);
            if shadow_intersect && (shadow_hit - shadow_origin).len() < dist {
                include_Kd = false;
            }
            let e = (self.point_light.intensity * cos_theta) / (dist.powi(2));
            let brdf_Ks = 0.0_f64.max(minus_light_dir.reflect(N).dot(camera_dir)) * material.specular_reflection;
            let brdf = (if include_Kd { material.diffuse_reflection } else { 0.0 }) + brdf_Ks;
            ray.radiance = ((e * brdf) / std::f64::consts::PI) + (reflect_ray.radiance) * material.specular_reflection;
            ray
        }
    }

    pub fn save(&self, path: &str) {
        let mut img = RgbImage::new(self.width, self.height);
        self.pixel_buffer
            .iter()
            .for_each(|(y, x, ray)| {
                *img.get_pixel_mut(x.clone(), y.clone()) = Rgb(
                    to0_255_color_format(
                        ray.color * (if ray.radiance > 1.0 { 1.0 } else { ray.radiance })
                    )
                )
            });
        img.save(path);
    }

    pub fn render(&mut self, width: u32, height: u32) -> &mut Self {
        self.width = width;
        self.height = height;
        let w = width as f64;
        let h = height as f64;
        self.pixel_buffer =
            create_grid(self.height, self.width)
                .par_iter()
                .map(|&(x, y)| (
                    x, y, self.cast_ray(
                        self.camera.create_ray_from_camera(
                            -(2.0 * (y as f64 + 0.5) / w - 1.0) * (self.camera.fov / 2.0).tan() * w / h,
                            -(2.0 * (x as f64 + 0.5) / h - 1.0) * (self.camera.fov / 2.0).tan(),
                        ),
                        0
                    )
                ))
                .collect();
        self
    }
}
