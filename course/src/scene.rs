use rayon::prelude::*;
use std::{
    fs::File,
    io::{ Write },
    collections::HashMap,
};
use crate::{
    ray::Ray,
    vec3::Vec3,
    light::Light,
    camera::Camera,
    polygon::Polygon,
    extended_math::create_grid,
    geom_loaders::load_geometry,
    material::{ Material, MATERIAL_LIBRARY }
};

pub struct Scene<'s> {
    pub path: String,
    pub light: &'s Light,
    pub geometry: Vec<Polygon<'s>>,
    pub camera: &'s Camera,
    pub radiance_buffer: Vec<HashMap<i64, f64>>,
    pub width: usize,
    pub height: usize
}

impl<'s> Scene<'_> {
    pub fn new(path: &str, light: &'s Light, camera: &'s Camera) -> Scene<'s> {
        Scene {
            path: path.to_string(),
            light,
            geometry: load_geometry(path),
            camera,
            radiance_buffer: vec![],
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
        if depth > 5 || !intersect { ray }
        else {
            for (wl, kd) in &material.diffuse_reflection {
                *ray.bright_coefs.get_mut(wl).unwrap() *= kd;
            }
            let camera_dir = (ray.position - hit).normalize();
            let reflect_dir = ray.direction.normalize().reflect(N.normalize()).normalize();
            let reflect_origin = hit + N * 1e-3;
            let mut reflect_ray = Ray::new(reflect_origin, reflect_dir, self.light);
            if material.specular_reflection > 0.0 && reflect_dir.dot(N) > 0.0 {
                reflect_ray = self.cast_ray(reflect_ray, depth + 1);
            }
            let light_dir = (self.light.position - hit).normalize();
            let minus_light_dir = (hit - self.light.position).normalize();
            let dist = (self.light.position - hit).len();
            let cos_theta = light_dir.dot(N);
            let mut include_Kd = if cos_theta <= 0.0 { false } else { true };
            let mut brdf_Kd = 0.0;
            let mut brdf_Ks = 0.0;
            let shadow_origin = hit + N * 1e-3;
            let shadow_ray = Ray {
                position: shadow_origin,
                direction: light_dir,
                ..Default::default()
            };
            let (shadow_intersect, shadow_material, shadow_hit, shadow_N) = self.scene_intersect(&shadow_ray);
            if shadow_intersect && (shadow_hit - shadow_origin).len() < dist {
                include_Kd = false;
            }
            for (l1, l2) in ray.radiance.iter_mut() {
                let e = (self.light.color_distribution[l1] * cos_theta) / (dist.powi(2));
                if include_Kd {
                    brdf_Kd = ray.bright_coefs[l1];
                }
                brdf_Ks = 0.0_f64.max(minus_light_dir.reflect(N).dot(camera_dir)).powf(material.transparency) * material.specular_reflection;
                let brdf = brdf_Kd + brdf_Ks;
                *l2 = ((e * brdf) / std::f64::consts::PI) + (reflect_ray.radiance[l1]) * material.specular_reflection;
            }
            ray
        }
    }

    pub fn save(&self, path: String) {
        let mut results = File::create(path).unwrap();
        for wl in &vec![400, 500, 600, 700] {
            write!(results, "wavelength {}\n", wl);
            for i in 0..self.height {
                for j in 0..self.width {
                    write!(results, "{}", (if j == 0 { "" } else { " " }).to_string() + &(self.radiance_buffer[j + i * self.width].get(wl).unwrap() / 100.0).to_string());
                }
                write!(results, "\n");
            }
            write!(results, "\n");
        }
    }

    pub fn render(&mut self, width: usize, height: usize) -> &mut Self {
        self.width = width;
        self.height = height;
        let w = width as f64;
        let h = height as f64;
        self.radiance_buffer =
            create_grid(self.height, self.width)
                .par_iter()
                .map(|(x, y)|
                    self.cast_ray(
                        self.camera.create_ray_from_camera(
                            -(2.0 * (y.clone() as f64 + 0.5) / w - 1.0) * (self.camera.fov / 2.0).tan() * w / h,
                            -(2.0 * (x.clone() as f64 + 0.5) / h - 1.0) * (self.camera.fov / 2.0).tan(),
                            self.light
                        ),
                        0
                    ).radiance
                ).collect();
        self
    }
}
