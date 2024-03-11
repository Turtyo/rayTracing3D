use std::ops::Sub;
use std::ops::Add;
use crate::geometry::vector::{Vector, BaseVector, UnitVector};

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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

impl Add<&BaseVector> for &Point {
    type Output = Point;
    fn add(self, rhs: &BaseVector) -> Self::Output {
        let mut new_point = *self;
        new_point.x += rhs.x;
        new_point.y += rhs.y;
        new_point.z += rhs.z;

        new_point
    }
}

impl Add<&Point> for &BaseVector {
    type Output = Point;
    fn add(self, rhs: &Point) -> Self::Output {
        rhs + self
    }
}