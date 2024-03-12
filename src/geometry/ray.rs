use super::object::Sphere;
use super::point::Point;
use super::vector::{UnitVector, Vector};

pub struct Ray {
    pub origin: Point,
    pub direction: UnitVector,
}

impl Ray {
    pub fn new_from_points(origin: &Point, destination: &Point) -> Self {
        let dest = UnitVector::new_from_points(origin, destination);
        Ray {
            origin: *origin,
            direction: dest,
        }
    }
    pub fn point_at_a_distance(&self, scalar: f64) -> Point {
        let Ray { origin, direction } = self;
        let total_displacement = scalar * direction;
        origin + &total_displacement
    }

    pub fn intersect<'a>(&self, object: &'a Sphere) -> Option<HitInfo<'a>> {
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
        let vector_co = Vector::new_from_points(&object.center, &self.origin);
        let b = 2. * Vector::scalar_product(&self.direction.to_vector(), &vector_co);
        let c = Vector::scalar_product(&vector_co, &vector_co) - object.radius;
        let delta = b * b - 4. * c;

        if delta < 0. {
            None
        } else {
            // ? so it is possible to have non assigned value if later on we see we will always assign something to it
            let hit_distance: f64;
            let first_distance = (-b - delta.sqrt()) / 2.;
            if delta == 0. {
                hit_distance = first_distance;
            } else {
                let second_distance = (-b + delta.sqrt()) / 2.;
                if first_distance < second_distance {
                    hit_distance = first_distance;
                } else {
                    hit_distance = second_distance;
                }
            }
            let point_hit = &self.origin + &(hit_distance * &self.direction);
            Some(HitInfo {
                object,
                point_hit,
                hit_distance,
            })
        }
    }
                }
            }
        }
        
    }
}

pub struct HitInfo<'a> {
    pub object: &'a Sphere,
    pub point_hit: Point,
    pub hit_distance: f64,
}
