mod vec3;
mod light;
mod material;
mod polygon;
mod scene;
mod ray;

use clap::{ Error, Parser };
use std::{
    process,
    collections::HashMap,
    fs::File,
    io::{
        self,
        prelude::*,
        BufRead,
        BufReader
    }
};
use crate::{
    light::Light,
    polygon::Polygon,
    ray::Ray,
    vec3::Vec3,
    scene::Scene,
    material::{ Material, MATERIAL_LIBRARY }
};

/// Simple ray tracer working with Lumicept: https://integra.jp
#[derive(Parser)]
struct Args {
    /// Path to scene file
    #[arg(short = 'S', long = "scene_path")]
    scene: String,
    /// Width of rendered image
    #[arg(short = 'W', long = "width", default_value_t = 1280)]
    width: usize,
    /// Height of rendered image
    #[arg(short = 'H', long = "height", default_value_t = 720)]
    height: usize,
    /// X of light position
    #[arg(default_value_t = 0.0)]
    lx: f32,
    /// Y of light position
    #[arg(default_value_t = 0.0)]
    ly: f32,
    /// Z of light position
    #[arg(default_value_t = 0.0)]
    lz: f32,
    /// X of camera position
    #[arg(default_value_t = 0.0)]
    cx: f32,
    /// Y of camera position
    #[arg(default_value_t = 0.0)]
    cy: f32,
    /// Z of camera position
    #[arg(default_value_t = 0.0)]
    cz: f32
}

fn main() {
    // todo: camera class
    // todo: конструировать сцены из объекта shp_loader с интерфейсов loader
    // todo: перейти на двойную точность
    let args = Args::parse();
    let total_intensity = 200.0;
    Scene::new(
        &args.scene,
        &Light {
            position: Vec3::from((args.lx, args.ly, args.lz)),
            intensity: total_intensity,
            color_distribution: HashMap::from([
                (400, 0.0 / 2100.0),
                (500, 400.0 / 2100.0),
                (600, 780.0 / 2100.0),
                (700, 920.0 / 2100.0)
            ]),
        }
    ).render(args.width, args.height, Vec3::from((args.cx, args.cy, args.cz)));
}
