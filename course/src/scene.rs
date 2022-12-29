use {
    rayon::prelude::*,
    std::time::{
        Duration,
        Instant
    },
    image::{
        ImageBuffer,
        Rgb,
        RgbImage
    }
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
        MATERIAL_LIBRARY as ML
    }
};

pub struct Scene<'s> {
    pub path: String,
    pub point_light: &'s PointLight,
    pub geometry: Vec<Polygon<'s>>,
    pub camera: &'s Camera,
    pub width: u32,
    pub height: u32,
    pixel_buffer: Vec<(u32, u32, f64, Vec3)>
}

const AA_STRENGTH: f64 = 2.0;
const MAX_REFLECTIONS_DEPTH: i64 = 5;

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
            height: 0,
        }
    }

    fn scene_intersect(&self, ray: &Ray) -> (bool, &Material, Vec3, Vec3) {
        let mut t = 0.0;
        let mut hit = Vec3 { ..Default::default() };
        let mut normal = Vec3 { ..Default::default() };
        let mut material = &ML[0]; // use first material as default
        let mut triangle_dist = self.camera.far;
        for p in &self.geometry {
            let (intersect, res) = p.intersected(ray, t);
            t = res;
            if intersect && t < triangle_dist {
                triangle_dist = t;
                hit = ray.position + ray.direction * t;
                normal = p.normal_by_observer(ray.position - hit);
                material = &p.material;
            }
        }
        (triangle_dist < f64::MAX, material, hit, normal)
    }

    fn cast_ray(&self, mut ray: Ray, depth: i64, reflections_on: bool) -> Ray {
        let (intersect, material, hit, N) = self.scene_intersect(&ray);
        ray.color = material.color;
        if depth > MAX_REFLECTIONS_DEPTH || !intersect { ray }
        else {
            let reflect_dir = ray.direction.normalize().reflect(N.normalize()).normalize();
            let mut reflect_ray = Ray {
                position: hit + N * 1e-8, // reflect_origin
                direction: reflect_dir,
                ..Default::default()
            };
            if material.specular_reflection > 0.0 && reflect_dir.dot(N) > 0.0 && reflections_on {
                reflect_ray = self.cast_ray(reflect_ray, depth + 1, true);
            }
            let light_dir = (self.point_light.position - hit).normalize();
            let distance = (self.point_light.position - hit).len();
            let cosθ = light_dir.dot(N) / (light_dir.len() * N.len());
            let shadow_origin = hit + N * 1e-8;
            let (shadow_intersect, _, shadow_hit, _) = self.scene_intersect(
                &Ray { // shadow_ray
                    position: shadow_origin,
                    direction: light_dir,
                    ..Default::default()
                }
            );
            let illumination = (self.point_light.intensity * cosθ) / (distance.powi(2));
            let brdf = material.brdf(
                (hit - self.point_light.position).normalize().reflect(N), // reverse light dir
                (ray.position - hit).normalize(), // camera dir
                !(cosθ <= 0.0 || (shadow_intersect && (shadow_hit - shadow_origin).len() < distance)) // include Kd
            );
            ray.radiance = ((illumination * brdf) / std::f64::consts::PI) + reflect_ray.radiance * material.specular_reflection;
            ray.color = ray.color * (1.0 - material.specular_reflection) + reflect_ray.color * material.specular_reflection;
            ray
        }
    }

    fn ssaa(&mut self) {
        /*
        TODO:
         1. разбить на чанки длиной AA_STRENGTH
         2. рассчитать среднее значения для каждого чанка
         3. транспонировать матрицу (https://stackoverflow.com/questions/38627087/taking-the-transpose-of-a-matrix-in-c-with-1d-arrays)
            (можно не транспонировать а проходить в массиве длиной массива шага 2 / AA_STRENGTH и брать сл. эл по индексу)
         4. разбить на чанки длиной AA_STRENGTH
         5. рассчитать среднее значения для каждого чанка
         */
    }

    pub fn save(&self, path: &str) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut img = RgbImage::new(self.width, self.height);
        self.pixel_buffer.iter().for_each(|&(y, x, radiance, color)|
            *img.get_pixel_mut(x, y) = Rgb(
                to0_255_color_format(
                    color * radiance
                )
            )
        );
        img.save(path).expect("unexpectedly, unable to save image");
        img
    }

    pub fn render(&mut self, width: u32, height: u32, antialiased: bool, reflections_on: bool, ) -> &mut Self {
        let mapped_res = |res|
            res as f64 * (if antialiased { AA_STRENGTH } else { 1.0 });
        let projection_center = |side: u32, side_size: f64|
            -(2.0 * (side as f64 + 0.5) / side_size - 1.0) * (self.camera.fov.to_radians() / 2.0).tan();
        self.width = width;
        self.height = height;
        let w_f64 = mapped_res(width);
        let h_f64 = mapped_res(height);
        let start = Instant::now();
        self.pixel_buffer =
            create_grid(width, height)
                .par_iter()
                .map(|&(x, y)| {
                    let ray = self.cast_ray(
                        self.camera.create_ray_from_camera(
                            projection_center(y, w_f64) * w_f64 / h_f64,
                            projection_center(x, h_f64),
                        ),
                        0,
                        reflections_on
                    );
                    (x, y, ray.radiance, ray.color)
                })
                .collect();
        println!("render function time taken = {:?}", start.elapsed());
        if antialiased { self.ssaa() };
        self
    }
}
