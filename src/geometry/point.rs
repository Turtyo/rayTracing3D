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

