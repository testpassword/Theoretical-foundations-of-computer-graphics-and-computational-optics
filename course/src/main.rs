mod vec3;
mod material;
mod polygon;
mod scene;
mod geom_loaders;
mod camera;
mod extended_math;
mod lights;

use clap::Parser;
use rayon::prelude::*;
use std::collections::HashMap;
use crate::{
    vec3::Vec3,
    scene::Scene,
    camera::Camera,
    lights::{
        point_light::PointLight,
        ambient_light::AmbientLight
    },
};

/// Simple ray tracer working with Lumicept: https://integra.jp
#[derive(Parser)]
struct Args {
    /// Path to scene file
    #[arg(short = 'S', long = "scene_path")] scene: String,
    /// Width of rendered image
    #[arg(short = 'W', long = "width", default_value_t = 1280)] width: usize,
    /// Height of rendered image
    #[arg(short = 'H', long = "height", default_value_t = 720)] height: usize,
    /// Light intensity
    #[arg(short = 'I', long = "intensity", default_value_t = 200.0)] intensity: f64,
    /// Output file path
    #[arg(short = 'R', long = "render_path", default_value_t = String::from(""))] render_path: String,
    /// X of light position
    #[arg(default_value_t = 0.0)] lx: f64,
    /// Y of light position
    #[arg(default_value_t = 0.0)] ly: f64,
    /// Z of light position
    #[arg(default_value_t = 0.0)] lz: f64,
    /// X of camera position
    #[arg(default_value_t = 0.0)] cx: f64,
    /// Y of camera position
    #[arg(default_value_t = 0.0)] cy: f64,
    /// Z of camera position
    #[arg(default_value_t = 0.0)] cz: f64,
    /// FOV of camera
    #[arg(default_value_t = 1.04)] cf: f64,
}

fn main() {
    // todo: норм тени
    let args = Args::parse();
    let total_intensity = args.intensity;
    Scene::new(
        &args.scene,
        &PointLight {
            position: Vec3::from((args.lx, args.ly, args.lz)),
            intensity: total_intensity,
            color_distribution: HashMap::from([
                (400, total_intensity * (0.0 / 2100.0)),
                (500, total_intensity * (400.0 / 2100.0)),
                (600, total_intensity * (780.0 / 2100.0)),
                (700, total_intensity * (920.0 / 2100.0))
            ]),
        },
        &Camera {
            fov: args.cf,
            position: Vec3::from((args.cx, args.cy, args.cz))
        }
    ).render(
        args.width,
        args.height
    ).save(&(
        if args.render_path.is_empty() {
            args.scene.split("/").last().unwrap().split(".").next().unwrap().to_string() + ".txt"
        } else { args.render_path }
    ));
}
