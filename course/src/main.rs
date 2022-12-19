mod vec3;
mod material;
mod polygon;
mod scene;
mod geom_loaders;
mod camera;
mod utils;
mod lights;

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
    #[arg(short = 'I', long = "intensity", default_value_t = 1400009.0)] intensity: f64,

    /// antialiased
    #[arg(short = 'A', long = "antialiased", default_value_t = true)] antialiased: bool,

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
    #[arg(default_value_t = 1.04)] cf: f64
}

fn main() {
    // todo: normal shadow based on different lights
    // todo: antialiasing
    // todo: tone mapping
    // todo: draw through OpenGL
    // todo: new scene
    let args = Args::parse();
    Scene::new(
        &args.scene,
        &PointLight {
            position: Vec3::from((args.lx, args.ly, args.lz)),
            intensity: args.intensity
        },
        &Camera {
            fov: args.cf,
            position: Vec3::from((args.cx, args.cy, args.cz))
        }
    ).render(
        args.width,
        args.height,
        false
        //args.antialiased
    ).save(&(
        if args.render_path.is_empty() {
            args.scene.split("/").last().unwrap().split(".").next().unwrap().to_string() + ".png"
        } else { args.render_path }
    ));
}
