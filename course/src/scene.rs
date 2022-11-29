use std::char::MAX;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::light::Light;
use crate::material::{ Material, MATERIAL_LIBRARY };
use crate::polygon::Polygon;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Scene<'s> {
    pub path: String,
    pub light: Light,
    pub geometry: Vec<Polygon<'s>>
}

#[derive(PartialEq)]
enum LoadState {
    PartsReading,
    TrianglesReading,
    IdReading,
    VerticesReading
}

impl Scene<'_> {
    pub fn new(path: String, light: Light) -> Scene<'static> {
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
                            .split(" ")
                            .filter(|&it| !it.eq(""))
                            .map(|it| it.parse::<f32>().unwrap())
                            .collect::<Vec<f32>>()
                    )
                )
            }
        };
        Scene { path, light, geometry }
    }

    fn scene_intersect<'s>(ray: &'s Ray, polygons: &'s Vec<Polygon<'s>>) -> (bool, &'s Material, Vec3, Vec3) {
        let mut t: f32 = 0.0;
        let mut n_hit = Vec3 { ..Default::default() };
        let mut n_n = Vec3 { ..Default::default() };
        let mut n_m = &MATERIAL_LIBRARY[0];
        let mut triangle_dist = f32::MAX;
        for p in polygons {
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

    fn cast_ray(ray: Ray, polygons: Vec<Polygon>, depth: i32) -> Ray {
        if depth > 5 {
            Ray {
                origin: Vec3::from((0.0, 0.0, 0.0)),
                direction: Vec3::from((0.0, 0.0, 0.0)),
                bright_coefs: HashMap::new(),
                radiance: HashMap::new(),
            }
        } else { ray }
    }

    fn create_ray_from_camera() {}

    pub fn render(&self, width: u16, height: u16) {
        todo!()
    }
}
