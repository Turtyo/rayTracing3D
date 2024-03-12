use crate::geometry::vector::Vector;
use std::cmp::PartialEq;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn distance(&self, other: &Point) -> f64 {
        let Point { x, y, z } = self - other;
        Vector::norme(x, y, z)
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
        new_point.x += rhs.norme * rhs.x();
        new_point.y += rhs.norme * rhs.y();
        new_point.z += rhs.norme * rhs.z();

        new_point
    }
}

impl Add<&Point> for &Vector {
    type Output = Point;
    fn add(self, rhs: &Point) -> Self::Output {
        rhs + self
    }
}
