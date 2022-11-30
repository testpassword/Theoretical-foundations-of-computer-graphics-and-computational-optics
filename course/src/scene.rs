use std::{
    cmp::max,
    fs::File,
    fmt::format,
    collections::HashMap,
    io::{ BufRead, BufReader, Write },
};
use crate::light::Light;
use crate::material::{ Material, MATERIAL_LIBRARY };
use crate::polygon::Polygon;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Scene<'s> {
    pub path: &'s String,
    pub light: &'s Light,
    pub geometry: Vec<Polygon<'s>>
}

#[derive(PartialEq)]
enum LoadState {
    PartsReading,
    TrianglesReading,
    IdReading,
    VerticesReading
}

impl<'s> Scene<'_> {
    pub fn new(path: &'s String, light: &'s Light) -> Scene<'s> {
        let mut state = LoadState::PartsReading;
        let mut vertices: Vec<Vec3> = vec![];
        let mut geometry: Vec<Polygon> = vec![];
        let mut object_ids: Vec<usize> = vec![];
        let mut vertices_count: usize = 0;
        for l in BufReader::new(File::open(&path).unwrap()).lines() {
            let line = l.unwrap().trim().to_string();
            if line.contains("Number of parts") {
                state = LoadState::PartsReading;
                continue
            }
            if line.contains("Number of triangles") {
                state = LoadState::TrianglesReading;
                continue
            }
            if state == LoadState::TrianglesReading {
                let num_of_triangles: Vec<usize> = line
                    .split(" ")
                    .map(|it| it.parse::<usize>().unwrap())
                    .collect();
                let get_vertex = |i| -> Vec3 { vertices[vertices_count + num_of_triangles[i]] };
                geometry.push(
                    Polygon {
                        vertices: (get_vertex(0), get_vertex(1), get_vertex(2)),
                        material: &MATERIAL_LIBRARY[0],
                    }
                );
                continue
            }
            if line.contains("define breps brs_") {
                object_ids.push(line.split("_").last().unwrap().parse::<usize>().unwrap());
                state = LoadState::IdReading;
                continue
            }
            if line.contains("Number of vertices") {
                state = LoadState::VerticesReading;
                vertices_count = vertices.len();
                continue
            }
            if state == LoadState::VerticesReading {
                vertices.push(
                    Vec3::from(
                        line
                            .split_whitespace()
                            .filter(|&it| !it.eq(""))
                            .map(|it| it.parse::<f32>().unwrap())
                            .collect::<Vec<f32>>()
                    )
                )
            }
        };
        Scene { path, light, geometry }
    }

    fn scene_intersect(&self, ray: &Ray) -> (bool, &Material, Vec3, Vec3) {
        let mut t: f32 = 0.0;
        let mut n_hit = Vec3 { ..Default::default() };
        let mut n_n = Vec3 { ..Default::default() };
        let mut n_m = &MATERIAL_LIBRARY[0];
        let mut triangle_dist = f32::MAX;
        for p in &self.geometry {
            let (intersect, res) = p.intersected(ray, t);
            t = res;
            if intersect && t < triangle_dist {
                triangle_dist = t;
                n_hit = ray.origin + ray.direction * t;
                n_n = p.normal_by_observer(ray.origin - n_hit);
                n_m = &p.material;
            }
        }
        return (triangle_dist < f32::MAX, n_m, n_hit, n_n)
    }

    fn cast_ray(&self, mut ray: Ray, depth: i32) -> Ray {
        let (intersect, material, hit, N) = self.scene_intersect(&ray);
        if depth > 5 || !intersect {
            ray
        } else {
            for (wl, kd) in &material.diffuse_reflection {
                *ray.bright_coefs.get_mut(wl).unwrap() *= kd;
            }
            let camera_dir = (ray.origin - hit).normalize();
            let reflect_dir = ray.direction.normalize().reflect(N.normalize()).normalize();
            let reflect_origin = hit + N * 1e-3;
            let mut reflect_ray = Ray {
                origin: reflect_origin,
                direction: reflect_dir,
                bright_coefs: Default::default(),
                radiance: Default::default(),
            };
            reflect_ray.fill_wavelength(self.light);
            if material.specular_reflection > 0.0 && reflect_dir.dot(N) > 0.0 {
                reflect_ray = self.cast_ray(reflect_ray, depth + 1);
            }
            let mut is_spec_material = false;
            let mut diffuse = 0.0;
            let specular = 0.0;
            let light_dir = (self.light.position - hit).normalize();
            let minus_light_dir = (hit - self.light.position).normalize();
            let dist = (self.light.position - hit).len();
            let cos_theta = light_dir.dot(N);
            let mut include_Kd = if cos_theta <= 0.0 { false } else { true };
            let mut brdf_Kd = 0.0;
            let mut brdf_Ks = 0.0;
            let shadow_origin = hit + N * 1e-3;
            let shadow_ray = Ray {
                origin: shadow_origin,
                direction: light_dir,
                bright_coefs: Default::default(),
                radiance: Default::default(),
            };
            let (shadow_intersect, shadow_material, shadow_hit, shadow_N) = self.scene_intersect(&shadow_ray);
            if shadow_intersect && (shadow_hit - shadow_origin).len() < dist {
                include_Kd = false;
            }
            for (l1, l2) in ray.radiance.iter_mut() {
                let E = (self.light.color_distribution[l1] * cos_theta) / (dist.powi(2));
                if include_Kd {
                    brdf_Kd = ray.bright_coefs[l1];
                }
                brdf_Ks = 0.0_f32.max(minus_light_dir.reflect(N).dot(camera_dir)).powf(material.transparency) * material.specular_reflection;
                let brdf = brdf_Kd * brdf_Ks;
                *l2 = ((E * brdf) / std::f32::consts::PI) + (reflect_ray.radiance[l1]) * material.specular_reflection;
            }
            ray
        }
    }

    fn create_ray_from_camera(&self, x: f32, y: f32, camera_coords: Vec3) -> Ray {
        let direction = Vec3::from((x, y, -1.0)).normalize();
        let mut ray = Ray {
            origin: camera_coords,
            direction,
            bright_coefs: HashMap::new(),
            radiance: HashMap::new()
        };
        ray.fill_wavelength(self.light);
        ray
    }

    pub fn render(&self, width: usize, height: usize, camera_position: Vec3) {
        let w = width as f32;
        let h = height as f32;
        let fov = std::f32::consts::PI / 3.0;
        let mut framebuffer: Vec<Ray> = Vec::with_capacity(width * height);
        let mut bright_buffer: Vec<HashMap<i32, f32>> = Vec::with_capacity(width* height);
        unsafe {
            framebuffer.set_len(width * height);
            bright_buffer.set_len(width * height);
        }
        let mut wavelengths = vec![400, 500, 600, 700];
        for i in 0..height {
            for j in 0..width {
                let x_center = -(2.0 * (j as f32 + 0.5) / w - 1.0) * (fov / 2.0).tan() * w / h;
                let y_center = -(2.0 * (i as f32 + 0.5) / h - 1.0) * (fov / 2.0).tan();
                let ray_center = self.create_ray_from_camera(x_center, y_center, camera_position);
                framebuffer[j + i * width] = self.cast_ray(ray_center, 0);
                bright_buffer[j + i * width] = framebuffer[j + i * width].radiance.clone();
            }
        }
        let mut results = File::create(self.path.split("/").last().unwrap().split(".").next().unwrap().to_string() + ".txt").unwrap();
        for wl in &wavelengths {
            results.write_all(format!("wavelength {}\n", wl).as_bytes());
            for i in 0..height {
                for j in 0..width {
                    let nums = if j == 0 {
                        format!("{}", bright_buffer[j + i * width].get(wl).unwrap() / 100.0)
                    } else {
                        format!(" {}", bright_buffer[j + i * width].get(wl).unwrap() / 100.0)
                    };
                    results.write_all(nums.as_bytes());
                }
                results.write_all("\n".as_bytes());
            }
        }
    }
}
