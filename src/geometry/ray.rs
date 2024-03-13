use crate::error::GeometryError;

use super::object::Sphere;
use super::point::Point;
use super::vector::{UnitVector, Vector};

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: UnitVector,
}

impl Ray {
    pub fn new_from_points(origin: &Point, destination: &Point) -> Result<Self, GeometryError> {
        let dest = UnitVector::new_from_points(origin, destination)?;
        Ok(Ray {
            origin: *origin,
            direction: dest,
        })
    }
    pub fn point_at_a_distance(&self, scalar: f64) -> Point {
        let Ray { origin, direction } = self;
        let total_displacement = scalar * direction;
        origin + &total_displacement
    }

    pub fn intersect<'a>(&self, object: &'a Sphere) -> Result<Option<HitInfo<'a>>, GeometryError> {
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
        let vector_co = Vector::new_from_points(&object.center, &self.origin)?;
        let b = 2. * &self.direction.to_vector().scalar_product(&vector_co);
        let c = vector_co.scalar_product(&vector_co) - object.radius;
        let delta = b * b - 4. * c;

        if delta < 0. {
            Ok(None)
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
            Ok(Some(HitInfo {
                object,
                point_hit,
                hit_distance,
            }))
        }
    }

    pub fn first_point_hit_by_ray<'a>(
        &self,
        objects: Vec<&'a Sphere>,
    ) -> Result<Option<HitInfo<'a>>, GeometryError> {
        let mut hit_info_closest_point = HitInfo {
            object: objects[0],
            point_hit: Point {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            hit_distance: f64::MAX,
        };
        let mut ray_has_hit_object = false;
        for object in objects {
            if let Some(hit_info) = self.intersect(object)? {
                if hit_info.hit_distance <= hit_info_closest_point.hit_distance {
                    hit_info_closest_point = hit_info;
                    // no else, it means the point is further away
                }
                ray_has_hit_object = true
            }
        }
        if ray_has_hit_object {
            Ok(Some(hit_info_closest_point))
        } else {
            Ok(None)
        }
    }

    pub fn reflected_ray(
        source: &Point,
        object: &Sphere,
        surface_point: &Point,
    ) -> Result<Self, GeometryError> {
        if !object.source_is_above_horizon(surface_point, source)? {
            // this already checks if the surface point is on the object
            Err(GeometryError::SourceNotVisibleFromPoint(format!(
                "The object is {0:?} | The object point is {1:?} | The source point is {2:?}",
                object, surface_point, source
            )))
        } else {
            let point_to_source_vector = Vector::new_from_points(surface_point, source)?;
            let surface_normal_vector = Vector::new_from_points(&object.center, surface_point)?;
            // Let D such as R(ay) = N(ormal) + D, thus D = R - N
            // the sym S is : S = N - D = N - (R - N) = 2N - R
            let sym_vector = (&(2. * &surface_normal_vector) - &point_to_source_vector)?;
            Ok(Ray {
                origin: *surface_point,
                direction: sym_vector.direction,
            })
        }
    }
}

pub struct HitInfo<'a> {
    pub object: &'a Sphere,
    pub point_hit: Point,
    pub hit_distance: f64,
}
