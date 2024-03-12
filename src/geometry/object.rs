use super::point::Point;
use super::vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Sphere {
    pub fn new_from_points(center: &Point, outer: &Point) -> Self {
        let Point { x, y, z } = outer - center;
        let radius = Vector::norme(x, y, z);
        Sphere {
            center: *center,
            radius,
        }
    }
}
