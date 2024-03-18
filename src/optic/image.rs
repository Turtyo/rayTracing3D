use crate::{
    error::RayTracingError,
    geometry::{point::Point, vector::UnitVector},
};

use super::color::Color;

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

#[derive(Debug)]
struct Grid {
    pub color: [[Color; GRID_WIDTH]; GRID_HEIGHT],
}

impl Grid {
    #[allow(unused_variables)] // ! number of points will be used later on for the random selection
    pub fn pixel_point_selection(
        pixel_width_index: usize,
        pixel_height_index: usize,
        number_of_points: usize,
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

    pub fn ray_eye_pixel_point(
        pixel_width_index: usize,
        pixel_height_index: usize,
        number_of_points: usize,
    ) -> Result<Vec<UnitVector>, RayTracingError> {
        let pixel_points =
            Self::pixel_point_selection(pixel_width_index, pixel_height_index, number_of_points);
        let mut unit_vector_list: Vec<UnitVector> = Vec::new();
        for point in pixel_points {
            let u_vec = UnitVector::new_from_points(&EYE_POINT, &point)?;
            unit_vector_list.push(u_vec);
        }
        Ok(unit_vector_list)
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
