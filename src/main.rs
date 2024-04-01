use ray_tracing_3d::{
    geometry::{point::Point, shape::Sphere},
    object::Object,
    optic::{
        color::{self, DiffusionCoefficient},
        material::Material,
    },
    ray_trace_image,
};

use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about = "A 3D ray tracing tool", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the ray tracer
    Run(RunArgs),
    /// Set the default parameters
    Set(SetArgs),
}

#[derive(Args, Debug)]
struct RunArgs {
    /// number of points per pixel
    #[arg(short, long, default_value = "See default_param.txt")]
    point_per_pixel: Option<usize>,

    /// max number of bounces for a ray of light
    #[arg(short, long, default_value = "See default_param.txt")]
    bounces: Option<usize>,

    /// where to export the completed image
    #[arg(short, long)]
    output: PathBuf,
}

#[derive(Args, Debug)]
struct SetArgs {
    /// number of points per pixel
    #[arg(short, long)]
    point_per_pixel: Option<usize>,

    /// max number of bounces for a ray of light
    #[arg(short, long)]
    bounces: Option<usize>,
}

fn read_default_args() {} // ? use clap::load_yaml

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run(run_args) => {
            let number_of_points_per_pixel= run_args.point_per_pixel.unwrap_or(5);
            let number_of_bounces = run_args.bounces.unwrap_or(3) as u64;
            let export_path= &run_args.output;

            println!("Running with the following parameters:");
            println!("number of points per pixel: {}", number_of_points_per_pixel);
            println!("max number of bounces for a light ray: {}", number_of_bounces);
            println!("file output at: {:?}", export_path);

            // * need to define the objects in the scene
            let sphere_support_center = Point {
                x: 0.,
                y: 40.,
                z: 40.,
            };
            let sphere_support = Sphere::new_from_radius(&sphere_support_center, 40.);

            let small_sphere_center_1 = Point {
                x: 14.4437385751,
                y: 0.7296131655,
                z: 40.,
            };
            let small_sphere_center_2 = Point {
                x: 9.0200152824,
                y: -1.9883262324,
                z: 40.,
            };
            let small_sphere_center_3 = Point {
                x: 0.8040704898,
                y: -5.012849843,
                z: 40.,
            };
            let small_sphere_center_4 = Point {
                x: -14.4200019067,
                y: -7.7571493705,
                z: 40.,
            };
            let small_sphere_1 = Sphere::new_from_radius(&small_sphere_center_1, 2.);
            let small_sphere_2 = Sphere::new_from_radius(&small_sphere_center_2, 3.);
            let small_sphere_3 = Sphere::new_from_radius(&small_sphere_center_3, 5.);
            let small_sphere_4 = Sphere::new_from_radius(&small_sphere_center_4, 10.);

            let light_source_center = Point {
                x: 100.,
                y: -30.,
                z: 30.,
            };
            let light_source = Sphere::new_from_radius(&light_source_center, 50.);

            let sphere_support_material = Material::new(
                color::BLACK,
                0.,
                DiffusionCoefficient::new(186. / 255., 181. / 255., 120. / 255.).unwrap(),
                0.,
            )
            .unwrap();
            let object_support = Object {
                shape: sphere_support,
                material: sphere_support_material,
            };

            let small_sphere_1_material = Material::new(
                color::BLACK,
                0.,
                color::GREEN.to_diffusion_coefficient().unwrap(),
                0.,
            )
            .unwrap();
            let small_sphere_2_material = Material::new(
                color::BLACK,
                0.,
                color::BLUE.to_diffusion_coefficient().unwrap(),
                0.,
            )
            .unwrap();
            let small_sphere_3_material = Material::new(
                color::BLACK,
                0.,
                color::RED.to_diffusion_coefficient().unwrap(),
                0.,
            )
            .unwrap();
            let small_sphere_4_material = Material::new(
                color::BLACK,
                0.,
                color::WHITE.to_diffusion_coefficient().unwrap(),
                0.,
            )
            .unwrap();
            let object_small_sphere_1 = Object {
                shape: small_sphere_1,
                material: small_sphere_1_material,
            };
            let object_small_sphere_2 = Object {
                shape: small_sphere_2,
                material: small_sphere_2_material,
            };
            let object_small_sphere_3 = Object {
                shape: small_sphere_3,
                material: small_sphere_3_material,
            };
            let object_small_sphere_4 = Object {
                shape: small_sphere_4,
                material: small_sphere_4_material,
            };

            let light_source_material = Material::new(
                color::WHITE,
                1.,
                color::BLACK.to_diffusion_coefficient().unwrap(),
                0.,
            )
            .unwrap();
            let object_light_source = Object {
                shape: light_source,
                material: light_source_material,
            };

            let objects = vec![
                &object_support,
                &object_small_sphere_1,
                &object_small_sphere_2,
                &object_small_sphere_3,
                &object_small_sphere_4,
                &object_light_source,
            ];
            // println!("objects: {:?}", objects);
            // let objects = vec![];
            // println!("{}", number_of_points_per_pixel);
            ray_trace_image(
                number_of_points_per_pixel,
                number_of_bounces,
                &objects,
                export_path,
            )
            .unwrap();
        }
        Commands::Set(_) => println!("Unimplemented"),
    }
    
}
