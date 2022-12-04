mod shp_loader;

use crate::{
    polygon::Polygon,
    geom_loaders::shp_loader::SHPLoader
};

pub trait GeomLoader {
    fn load(path: &str) -> Vec<Polygon<'static>>;
}

#[derive(PartialEq)]
pub enum LoadState {
    PartsReading,
    TrianglesReading,
    IdReading,
    VerticesReading
}

pub fn load_geometry(path: &str) -> Vec<Polygon<'static>> {
    match path.split(".").last().unwrap() {
        "shp" => SHPLoader::load(path),
        _ => panic!("Unsupported file format")
    }
}
