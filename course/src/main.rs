mod vec3;
mod light;
mod material;
mod polygon;
mod scene;
mod ray;

use std::collections::HashMap;
use clap::Parser;
use crate::light::Light;
use crate::material::Material;
use crate::polygon::Polygon;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::scene::Scene;

/// Simple ray tracer working with Lumicept: https://integra.jp
#[derive(Parser)]
struct Args {
    /// Path to scene file
    #[arg(short = 'S', long = "scene_path")]
    scene_path: std::path::PathBuf,
    /// Width of rendered image
    #[arg(short = 'W', long = "width", default_value_t = 1280)]
    width: u16,
    /// Height of rendered image
    #[arg(short = 'H', long = "height", default_value_t = 720)]
    height: u16,
    /// X of light position
    #[arg(default_value_t = 0.0)]
    lx: f32,
    /// Y of light position
    #[arg(default_value_t = 0.0)]
    ly: f32,
    /// Z of light position
    #[arg(default_value_t = 0.0)]
    lz: f32
}
// путь к файлу сцены, разрешение, положение источник света

fn main() {
    let args = Args::parse();
    let light_pos = Vec3 {
        x: args.lx,
        y: args.ly,
        z: args.lz
    };

    // todo: конструировать сцены из статического метода класса Scene

   /* let total_intensity1 = 200.0;
    let spec_intensity1: HashMap<i32, f32> = HashMap::from([
        (400, 0.22),
        (500, 0.33)
    ]);
    let ls1 = Light { position: Vec3 { x: 278.0, y: 248.7, z: 0.0 }, intensity: total_intensity1, color_distribution: spec_intensity1 };
    let total_intensity2 = 600.0;
    let spec_intensity2: HashMap<i32, f32> = HashMap::from([
        (100, 0.543),
        (230, 0.123)
    ]);
    let ls2 = Light { position: Vec3 { x: 1.0, y: 34.0, z: 5.0 }, intensity: total_intensity2, color_distribution: spec_intensity2 };
    let mut r = Ray { ..Default::default() };
    r.fill_wavelength(ls1);
    r.fill_wavelength(ls2);*/

    /*let p = Polygon {
        vertices: (
            Vec3 { x: 0.0, y: 1.1, z: 1.2 },
            Vec3 { x: 3.0, y: 2.4, z: -0.3 },
            Vec3 { x: -1.0, y: -5.0, z: -2.0 },
        ),
        material: Material {
            specular_reflection: 0.0,
            diffuse_reflection: Default::default(),
            transparency: 0.0
        }
    };*/
}
