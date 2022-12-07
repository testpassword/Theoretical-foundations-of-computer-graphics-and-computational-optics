use std::{
    fs::File,
    io::{ BufRead, BufReader }
};
use crate::{
    vec3::Vec3,
    polygon::Polygon,
    material::MATERIAL_LIBRARY as ML,
    geom_loaders::{ GeomLoader, LoadState }
};

pub struct SHPLoader;

impl GeomLoader for SHPLoader {
    fn load(path: &str) -> Vec<Polygon<'static>> {
        let mut state = LoadState::PartsReading;
        let mut vertices: Vec<Vec3> = vec![];
        let mut geometry: Vec<Polygon> = vec![];
        let mut object_ids: Vec<usize> = vec![];
        let mut vertices_count: usize = 0;
        for l in BufReader::new(File::open(path).unwrap()).lines() {
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
                let get_vertex = |i| -> Vec3 {
                    let triangles: Vec<usize> = line
                        .split(" ")
                        .map(|it| it.parse::<usize>().unwrap())
                        .collect();
                    vertices[vertices_count + triangles[i]]
                };
                // if there is no material with with idx, take first material
                geometry.push(
                    Polygon {
                        vertices: (get_vertex(0), get_vertex(1), get_vertex(2)),
                        material: &ML.get(object_ids.last().unwrap().clone()).unwrap_or(&ML.first().unwrap()),
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
                            .map(|it| it.parse::<f64>().unwrap())
                            .collect::<Vec<f64>>()
                    )
                )
            }
        };
        geometry
    }
}
