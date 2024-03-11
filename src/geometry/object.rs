use super::point::Point;
use super::vector::{Vector, BaseVector};

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Sphere {
    fn new_from_points(center: &Point, outer: &Point) -> Self {
        let radius = BaseVector::norme(&BaseVector::new_from_points(center, outer));
        Sphere {center : *center, radius}
    }
}
