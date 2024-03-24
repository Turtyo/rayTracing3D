use ray_tracing_3d::{
    geometry::{point::Point, shape::Sphere},
    object::Object,
    optic::{
        color::{self, DiffusionCoefficient},
        material::Material,
    },
    ray_trace_image,
};

fn main() {
    /*----- Ray trace parameters -----*/
    let number_of_points_per_pixel = 3;
    let number_of_bounces = 1;
    let export_path = "ray_traced_images/some_spheres.png";
    /*----- End Ray trace parameters -----*/

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
        x: 200.,
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
    // ray_trace_image(number_of_points_per_pixel, number_of_bounces, export_path).unwrap();
}
