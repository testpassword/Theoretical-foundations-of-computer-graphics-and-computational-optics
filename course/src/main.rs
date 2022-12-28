mod vec3;
mod material;
mod polygon;
mod scene;
mod geom_loaders;
mod camera;
mod utils;
mod lights;

use std::path::PathBuf;
use clap::Parser;
use crate::{
    vec3::Vec3,
    scene::Scene,
    camera::Camera,
    lights::point_light::PointLight,
};

#[derive(Parser)]
struct Args {
    /// Path to scene file
    #[arg(short = 'S', long = "scene_path")] scene: String,
    /// Width of rendered image
    #[arg(short = 'W', long = "width", default_value_t = 1280)] width: u32,
    /// Height of rendered image
    #[arg(short = 'H', long = "height", default_value_t = 720)] height: u32,
    /// Light intensity
    #[arg(short = 'I', long = "intensity", default_value_t = 1000000.0)] intensity: f64,
    /// antialiased
    #[arg(short = 'A', long = "antialiased", default_value_t = false)] antialiased: bool,
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
    /// FOV of camera in radiance
    #[arg(default_value_t = 55.0)] cf: f64
}

/*
ROADMAP
todo: normal shadows based on different lights
todo: antialiasing
todo: tone mapping
todo: make glares white
todo: fix radiance
todo: make light colorful
todo: transmission summarize of Ray
todo: grouping polygons to surface or a object
*/
fn main() {
    let args = Args::parse();
    let build_img_name = || {
        if args.render_path.is_empty() {
            let mut path = PathBuf::from(&args.scene);
            path.set_extension("png");
            return path.to_str().unwrap().to_string();
        } else { args.render_path }
    };
    Scene::new(
        &args.scene,
        &PointLight {
            position: Vec3::from((args.lx, args.ly, args.lz)),
            intensity: args.intensity
        },
        &Camera {
            fov: args.cf,
            position: Vec3::from((args.cx, args.cy, args.cz)),
            far: f64::MAX,
        }
    ).render(
        args.width,
        args.height,
        args.antialiased,
        true
    ).save(&build_img_name());
}
