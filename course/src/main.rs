mod vec3;
mod light;
mod material;
mod polygon;
mod scene;
mod ray;

use std::collections::HashMap;
use std::process;
use std::fs::File;
use std::io::{ self, prelude::*, BufRead, BufReader };
use clap::{ Error, Parser };
use crate::light::Light;
use crate::material::{ Material, MATERIAL_LIBRARY };
use crate::polygon::Polygon;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::scene::Scene;

/// Simple ray tracer working with Lumicept: https://integra.jp
#[derive(Parser)]
struct Args {
    /// Path to scene file
    #[arg(short = 'S', long = "scene_path")]
    scene: String,
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

fn main() {
    // todo: конструировать сцены из объекта shp_loader с интерфейсов loader
    // todo: перейти на двойную точность
    let args = Args::parse();
    let total_intensity = 200.0;
    let s = Scene::load(
        args.scene,
        Light {
            position: Vec3 {
                x: args.lx,
                y: args.ly,
                z: args.lz
            },
            intensity: total_intensity,
            color_distribution: HashMap::from([
                (400, 0.0 / 2100.0),
                (500, 400.0 / 2100.0),
                (600, 780.0 / 2100.0),
                (700, 920.0 / 2100.0)
            ]),
        }
    );
}
