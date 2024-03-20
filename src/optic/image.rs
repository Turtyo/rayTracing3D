use crate::{
    error::RayTracingError,
    geometry::{point::Point, ray::Ray, vector::UnitVector},
    object::Object,
};

use image::{Rgb, RgbImage};
use rand::SeedableRng;
use rand_chacha::{self, ChaCha8Rng};
use rand_distr::{self, DistIter, Distribution, UnitDisc};

use super::color::{self, Color};

const GRID_WIDTH: usize = 1920; // ! should be even
const GRID_HEIGHT: usize = 1080; // ! should be even
const PIXEL_SIZE: f64 = 1e-2;
const EYE_POINT: Point = Point {
    x: 0.,
    y: 0.,
    z: -1.,
};
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
    Color::new(0.5, 0.5, 0.5)
}

#[derive(Debug)]
pub struct Grid {
    pub colors: [[Color; GRID_WIDTH]; GRID_HEIGHT],
}

impl Grid {
    #[allow(unused_variables)] // ! number of points will be used later on for the random selection
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
        vec![pixel_center_point]
    }

    fn ray_eye_pixel_point(
        pixel_width_index: usize,
        pixel_height_index: usize,
        number_of_points_per_pixel: usize,
    ) -> Result<Vec<UnitVector>, RayTracingError> {
        let pixel_points = Self::pixel_point_selection(
            pixel_width_index,
            pixel_height_index,
            number_of_points_per_pixel,
        );
        let mut unit_vector_list: Vec<UnitVector> = Vec::new();
        for point in pixel_points {
            let u_vec = UnitVector::new_from_points(&EYE_POINT, &point)?;
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
        unit_disc_iter: &mut DistIter<UnitDisc, ChaCha8Rng, [f64; 2]>,
    ) -> Result<Color, RayTracingError> {
        let vector_eye_pixel = Grid::ray_eye_pixel_point(
            pixel_width_index,
            pixel_height_index,
            number_of_points_per_pixel,
        )?;
        let mut ray_color = color::WHITE;
        let mut total_ray_light = color::BLACK;
        let ray_has_hit = false;
        for vector in vector_eye_pixel {
            let mut ray_light = color::BLACK;
            // make the vector bounce around the scene on objects
            // we get a color of we hit a light source, or else we get the background color
            let mut ray = Ray {
                origin: EYE_POINT,
                direction: vector,
            };
            for _ in 0..number_of_bounces {
                let hit_info = match ray.first_point_hit_by_ray(objects)? {
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
                // make the ray bounce on the hit object randomly, cos weighted to take into account the Lambert reflectance law
                ray = Ray::cos_weighted_random_ray(
                    &hit_info.point_hit,
                    &hit_info.normal,
                    unit_disc_iter,
                )?;
                let light_emitted_by_hit_object = &hit_info.object.material.emission_color
                    * hit_info.object.material.emission_strength();
                ray_light = &ray_light + &light_emitted_by_hit_object;
                ray_color = &ray_color * &hit_info.object.material.diffusion_coefficients;
                // if the object hit is black, all subsequent bounces of the ray will be black, meaning we can exit early
                if ray_color == color::BLACK {
                    break;
                }
            }
            total_ray_light = &total_ray_light + &ray_light;
        }
        (&total_ray_light * (1. / number_of_bounces as f64)).new_from_color()
    }

    pub fn make_image(
        &mut self,
        number_of_points_per_pixel: usize,
        number_of_bounces: u64,
        objects: &Vec<&Object>,
    ) -> Result<(), RayTracingError> {
        let seed: u64 = 1;
        let rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
        let mut unit_disc_iter: DistIter<UnitDisc, ChaCha8Rng, [f64; 2]> =
            UnitDisc.sample_iter(rng);
        for pixel_height_index in 0..GRID_HEIGHT {
            for pixel_width_index in 0..GRID_WIDTH {
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

    pub fn export_image(self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut image = RgbImage::new(GRID_WIDTH as u32, GRID_HEIGHT as u32);
        for (width, height, pixel) in image.enumerate_pixels_mut() {
            let (r, g, b) = self.colors[width as usize][height as usize].into_rgb()?;
            *pixel = Rgb([r, g, b])
        }

        image.save(path)?;
        Ok(())
    }
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            colors: [[color::BLACK; GRID_WIDTH]; GRID_HEIGHT],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::RayTracingError;

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
        let expected_unit_vector = UnitVector::new_from_points(&EYE_POINT, &expected_point)?;

        assert_eq!(unit_vector_list[0], expected_unit_vector);

        Ok(())
    }
}
