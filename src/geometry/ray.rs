use super::vector::{Vector, UnitVector, BaseVector};
use super::point::Point;
use super::object::Sphere;

pub struct Ray {
    pub origin: Point,
    pub direction: UnitVector,
}

impl Ray {

    fn new_from_points(origin: &Point, destination: &Point) -> Self {
        let dest = UnitVector::new_from_points(origin, destination);
        Ray {origin : *origin, direction : dest}
    }
    fn point_at_a_distance(&self, scalar: f64) -> Point {
        let Ray { origin, direction } = self;
        let total_displacement = scalar * direction;
        origin + &total_displacement
    }

    fn intersect<'a> (object: &'a Sphere, ray: &Ray) -> Option<(&'a Sphere, f64)> {
        /* A sphere and a ray intersect if and only if the equation:
        d^2 + 2d(u . CO) + CO^2 - r^2 = 0
        has solutions, where:
        C : sphere center
        O : ray origin 
        . : scalar product
        u : unit vector defining the ray
        d : distance from A to intersection point
        r : radius of the sphere

        Following, we write:
        b = 2(u . CO)
        c = CO^2 - r^2
        The equation is d^2 + bd + c = 0 (classic quadratic form)
        */
        let vector_co = BaseVector::new_from_points(&object.center, &ray.origin);
        let b = 2. * BaseVector::scalar_product(&ray.direction, &vector_co);
        let c = BaseVector::scalar_product(&vector_co, &vector_co) - object.radius;
        let delta = b*b - 4.*c;
        if delta < 0. {
            None
        }
        else {
            let first_distance = (-b - delta.sqrt())/2.;
            if delta == 0. {
                Some((object, first_distance))
            }
            else {
                let second_distance = (-b + delta.sqrt())/2.;
                if first_distance < second_distance {
                    Some((object, first_distance))
                }
                else {
                    Some((object, second_distance))
                }
            }
        }
        
    }
}