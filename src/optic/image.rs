use crate::{
    error::RayTracingError,
    geometry::{point::Point, ray::Ray, vector::Vector},
    object::Object,
};

use image::{Rgb, RgbImage};
use rand::SeedableRng;
use rand_xorshift::{self, XorShiftRng};
use rand_distr::{self, DistIter, Distribution, UnitSphere};

use super::color::{self, Color};

use std::path::PathBuf;

const GRID_WIDTH: usize = 1920; // ! should be even
const GRID_HEIGHT: usize = 1080; // ! should be even
const PIXEL_SIZE: f64 = 1e-2;
const EYE_POINT: Point = Point {
    x: 0.,
    y: 0.,
    z: -10.,
}; // should change functions to take the position as an argument and not refer to this
const GRID_CENTER_POINT: Point = Point {
    x: 0.,
    y: 0.,
    z: 0.,
};

const GRID_CENTER_WIDTH_INDEX: usize = GRID_WIDTH / 2; 
const GRID_CENTER_HEIGHT_INDEX: usize = GRID_HEIGHT / 2;

/*----------------------------
Axis orientation

               z
               X--> x
               |
               v
               y

----------------------------*/

pub fn get_background_color() -> Result<Color, RayTracingError> {
    Color::new(0., 0., 0.0)
}

#[derive(Debug)]
pub struct Grid {
    width: usize,
    height: usize,
    pub colors: Vec<Vec<Color>>,
}

impl Grid {
    fn pixel_point_selection(
        pixel_width_index: usize,
        pixel_height_index: usize,
        number_of_points_per_pixel: usize,
    ) -> Vec<Point> {
        let pixel_center_point_x =
            (0.5 + pixel_width_index as f64 - GRID_CENTER_WIDTH_INDEX as f64) * PIXEL_SIZE
                + GRID_CENTER_POINT.x;
        let pixel_center_point_y =
            (0.5 + pixel_height_index as f64 - GRID_CENTER_HEIGHT_INDEX as f64) * PIXEL_SIZE
                + GRID_CENTER_POINT.y;
        let pixel_center_point = Point::new(pixel_center_point_x, pixel_center_point_y, 0.);
        // default implementation for now, just return the center point of the pixel
        // * will try to return a random distribution of points in the pixel for anti-aliasing later
        vec![pixel_center_point; number_of_points_per_pixel]
    }

    fn ray_eye_pixel_point(
        pixel_width_index: usize,
        pixel_height_index: usize,
        number_of_points_per_pixel: usize,
    ) -> Result<Vec<Vector>, RayTracingError> {
        let pixel_points = Self::pixel_point_selection(
            pixel_width_index,
            pixel_height_index,
            number_of_points_per_pixel,
        );
        let mut unit_vector_list: Vec<Vector> = Vec::new();
        for point in pixel_points {
            let u_vec = Vector::new_from_points(&EYE_POINT, &point);
            unit_vector_list.push(u_vec);
        }
        Ok(unit_vector_list)
    }

    fn trace_pixel_color(
        pixel_height_index: usize,
        pixel_width_index: usize,
        number_of_points_per_pixel: usize,
        number_of_bounces: u64,
        objects: &Vec<&Object>,
        unit_disc_iter: &mut DistIter<UnitSphere, XorShiftRng, [f64; 3]>,
    ) -> Result<Color, RayTracingError> {
        let vector_eye_pixel = Grid::ray_eye_pixel_point(
            pixel_width_index,
            pixel_height_index,
            number_of_points_per_pixel,
        )?;
        let mut total_ray_light = color::BLACK;
        let ray_has_hit = false;
        for vector in vector_eye_pixel {
            let mut ray_color = color::WHITE;
            let mut ray_light = color::BLACK;
            // make the vector bounce around the scene on objects
            // we get a color if we hit a light source, or else we get the background color
            let mut ray = Ray {
                origin: EYE_POINT,
                direction: vector,
            };
            let mut last_hit_sphere = None;
            for _ in 0..=number_of_bounces {
                let hit_info = match ray.first_point_hit_by_ray(objects, last_hit_sphere)? {
                    Some(point) => point,
                    None => {
                        if ray_has_hit {
                            break;
                        } else {
                            ray_light = self::get_background_color()?;
                            break;
                        }
                    }
                };
                
                last_hit_sphere = Some(hit_info.object);
                // make the ray bounce on the hit object randomly, cos weighted to take into account the Lambert reflectance law
                ray = Ray::cos_weighted_random_ray_unit_sphere(
                    &hit_info.point_hit,
                    &hit_info.normal,
                    unit_disc_iter,
                )?;
                let light_emitted_by_hit_object = &hit_info.object.material.emission_color
                    * hit_info.object.material.emission_strength();
                ray_light = &ray_light + &(&light_emitted_by_hit_object * &ray_color);
                ray_color = &ray_color * &hit_info.object.material.diffusion_coefficients;
                #[cfg(test)]
                {
                    println!("hit info : {:?}", hit_info);
                    println!("ray after bounce : {:?}", ray);
                    println!(
                        "ligth emitted by hit object : {:?}",
                        light_emitted_by_hit_object
                    );
                    println!("ray light : {:?}", ray_light);
                    println!("ray color : {:?}", ray_color);
                }
                // if the object hit is black, all subsequent bounces of the ray will be black, meaning we can exit early
                if ray_color == color::BLACK {
                    break;
                }
            }
            total_ray_light = &total_ray_light + &ray_light;
        }
        (&total_ray_light * (1. / number_of_points_per_pixel as f64)).new_from_color()
    }

    pub fn make_image(
        &mut self,
        number_of_points_per_pixel: usize,
        number_of_bounces: u64,
        objects: &Vec<&Object>,
    ) -> Result<(), RayTracingError> {
        let seed: u64 = 51468412518;
        let rng = XorShiftRng::seed_from_u64(seed);
        let mut unit_disc_iter: DistIter<UnitSphere, XorShiftRng, [f64; 3]> =
            UnitSphere.sample_iter(rng);
        for pixel_height_index in 0..self.height {
            for pixel_width_index in 0..self.width {
                let pixel_color = Grid::trace_pixel_color(
                    pixel_height_index,
                    pixel_width_index,
                    number_of_points_per_pixel,
                    number_of_bounces,
                    objects,
                    &mut unit_disc_iter,
                )?;
                self.colors[pixel_height_index][pixel_width_index] = pixel_color;
            }
        }
        Ok(())
    }

    pub fn export_image(self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let mut image = RgbImage::new(self.width as u32, self.height as u32);
        for (width_index, height_index, pixel) in image.enumerate_pixels_mut() {
            let (r, g, b) = self.colors[height_index as usize][width_index as usize].into_rgb()?;
            *pixel = Rgb([r, g, b])
        }

        image.save(path)?;
        Ok(())
    }
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            width: GRID_WIDTH,
            height: GRID_HEIGHT,
            colors: vec![vec![color::BLACK; GRID_WIDTH]; GRID_HEIGHT],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        error::RayTracingError,
        geometry::shape::Sphere,
        optic::{color, material::Material}, 
    };

    use super::*;

    #[test]
    fn test_pixel_point_selection() {
        let pixel_center_point = Grid::pixel_point_selection(GRID_WIDTH / 2, GRID_HEIGHT / 2, 1);
        let expected_point = Point::new(PIXEL_SIZE / 2., PIXEL_SIZE / 2., 0.);
        let pixel_center_point_2 =
            Grid::pixel_point_selection(GRID_WIDTH / 2 + 20, GRID_HEIGHT / 2 - 25, 1);
        let expected_point_2 = Point::new((0.5 + 20.) * PIXEL_SIZE, (0.5 - 25.) * PIXEL_SIZE, 0.);

        assert_eq!(pixel_center_point[0], expected_point);
        assert_eq!(pixel_center_point_2[0], expected_point_2);
    }

    #[test]
    fn test_ray_eye_pixel_point() -> Result<(), RayTracingError> {
        let unit_vector_list =
            Grid::ray_eye_pixel_point(GRID_WIDTH / 2 + 20, GRID_HEIGHT / 2 - 25, 1)?;

        let expected_point =
            Grid::pixel_point_selection(GRID_WIDTH / 2 + 20, GRID_HEIGHT / 2 - 25, 1)[0];
        let expected_unit_vector = Vector::new_from_points(&EYE_POINT, &expected_point);

        assert_eq!(unit_vector_list[0], expected_unit_vector);

        Ok(())
    }

    #[test]
    fn test_trace_pixel_color() -> Result<(), RayTracingError> {
        // * Define RNG
        let seed: u64 = 25;
        let rng = XorShiftRng::seed_from_u64(seed);
        let mut unit_disc_iter: DistIter<UnitSphere, XorShiftRng, [f64; 3]> =
            UnitSphere.sample_iter(rng);

        // * define parameters
        let number_of_points_per_pixel = 1;
        let number_of_bounces = 1;
        let pixel_height_index = 1080 / 2;
        let pixel_width_index = 1920 / 2;

        // * define the diffuse sphere of the scene
        let sphere_support_center = Point {
            x: 0.,
            y: -3.9,
            z: 10.,
        };
        let sphere_support = Sphere::new_from_radius(&sphere_support_center, 4.);
        let sphere_support_material = Material::new(
            color::BLACK,
            0.,
            color::RED.to_diffusion_coefficient().unwrap(),
            0.,
        )
        .unwrap();
        let object_support = Object {
            shape: sphere_support,
            material: sphere_support_material,
        };

        // * define the light source of the scene
        let light_source_center = Point {
            x: 4.8,
            y: 6.2,
            z: 8.37,
        };
        let light_source = Sphere::new_from_radius(&light_source_center, 3.18);
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

        // sphere positions are chosen so that the light source surface is very close to point defined as the ray going from the eye to the pixel, intersected with the sphere
        // it's close in the direction of the normal to the surface
        // not that changing the RNG may make this fail, as it is not guaranted the ray shooting for the surface of the hit sphere will hit the light source
        // since they are close, the probability of this happening is just very high

        // * object vector
        let objects = vec![&object_support, &object_light_source];

        let actual_color = Grid::trace_pixel_color(
            pixel_height_index,
            pixel_width_index,
            number_of_points_per_pixel,
            number_of_bounces,
            &objects,
            &mut unit_disc_iter,
        )?;
        let expected_color = color::RED;

        assert_eq!(actual_color, expected_color);

        Ok(())
    }
}
