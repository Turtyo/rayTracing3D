use crate::geometry::vector::Vector;
use std::cmp::PartialEq;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }
    pub fn distance(&self, other: &Point) -> f64 {
        let Point { x, y, z } = self - other;
        Vector::norme(x, y, z)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        let Point { x, y, z } = self - other;
        let distance = Vector::norme(x, y, z);
        (0. ..=1e-12_f64).contains(&distance)
        // ie if points are closer than a picometer, which is pretty small (smaller than an atom of hydrogen by a factor 10 at least)
    }
}

impl Sub for &Point {
    type Output = Point;
    fn sub(self, other: &Point) -> Self::Output {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        Point { x, y, z }
    }
}

impl Add for &Point {
    type Output = Point;
    fn add(self, other: &Point) -> Self::Output {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;
        Point { x, y, z }
    }
}

impl Add<&Vector> for &Point {
    type Output = Point;
    fn add(self, rhs: &Vector) -> Self::Output {
        let mut new_point = *self;
        new_point.x += rhs.x;
        new_point.y += rhs.y;
        new_point.z += rhs.z;

        new_point
    }
}

impl Add<&Point> for &Vector {
    type Output = Point;
    fn add(self, rhs: &Point) -> Self::Output {
        rhs + self
    }
}

#[cfg(test)]
mod tests {
    use crate::error::RayTracingError;

    use super::*;

    const POINT_1: Point = Point {
        x: 0.5,
        y: 7.,
        z: -2.,
    };
    const POINT_2: Point = Point {
        x: -10.,
        y: 0.,
        z: -2.,
    };
    // only passed as reference so no problem with declaring them here

    #[test]
    fn test_sub() {
        let expected_point = Point {
            x: 10.5,
            y: 7.,
            z: 0.,
        };

        assert_eq!(&POINT_1 - &POINT_2, expected_point);
    }

    #[test]
    fn test_add() {
        let expected_point = Point {
            x: -9.5,
            y: 7.,
            z: -4.,
        };

        assert_eq!(&POINT_1 + &POINT_2, expected_point);
    }

    #[test]
    fn test_add_vector() -> Result<(), RayTracingError> {
        let vector = Vector::new_from_coordinates(48.7, -154., 42.69);

        let expected_point = Point {
            x: 49.2,
            y: -147.,
            z: 40.69,
        };

        assert_eq!(&POINT_1 + &vector, expected_point);

        Ok(())
    }
}
