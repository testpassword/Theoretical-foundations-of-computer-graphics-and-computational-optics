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

// add there LightSource and static method with file reading
impl Scene<'_> {
    pub fn load(path: String, light: Light) -> Scene<'static> {
        let mut vertices: Vec<Vec3> = vec![];
        let mut geometry: Vec<Polygon> = vec![];;
        let mut object_ids: Vec<usize> = vec![];;
        let mut state = 0;
        let mut vertices_count: usize = 0;
        // todo: state превратить в enum
        let file =  File::open(&path).unwrap();
        let reader = BufReader::new(file);
        for l in reader.lines() {
            let line = l.unwrap().trim().to_string();
            if line.contains("Number of parts") {
                state = 0;
                continue
            }
            if line.contains("Number of triangles") {
                state = 3;
                continue
            }
            if state == 3 {
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
                state = 1;
                continue
            }
            if line.contains("Number of vertices") {
                state = 2;
                vertices_count = vertices.len();
                continue;
            }
            if state == 2 {
                let coords: Vec<f32> = line
                    .split(" ")
                    .filter(|&it| !it.eq(""))
                    .map(|it| it.parse::<f32>().unwrap())
                    .collect();
                vertices.push(
                    Vec3 {
                        x: coords[0],
                        y: coords[1],
                        z: coords[2]
                    }
                )
            }
        };
        Scene { path, light, geometry }
    }

    fn scene_intersect(ray: &Ray, polygons: &Vec<Polygon>, hit: Vec3, N: Vec3, material: &Material) -> (bool, Vec3) {
        let mut t: f32 = 0.0;
        let mut n_hit = Vec3 { ..Default::default() };
        let mut triangle_dist = f32::MAX;
        for p in polygons {
            let (intersect, res) = p.intersected(ray, t);
            t = res;
            if intersect && t < triangle_dist {
                triangle_dist = t;
                n_hit = ray.origin + ray.direction * t;
                //N = p.normal_by_observer(ray.origin - n_hit);

            }
        }
        todo!()
    }

    fn cast_ray(ray: &Ray, polygons: &Vec<Polygon>, depth: i32) -> Ray {
        todo!()
    }

    pub fn render(&self, width: u16, height: u16) {
        todo!()
    }
}
