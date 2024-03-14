use crate::error::GeometryError;

use super::object::Sphere;
use super::point::Point;
use super::vector::{UnitVector, Vector};

#[derive(Debug, Copy, Clone, PartialEq)]
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
        d : distance from O to intersection point
        r : radius of the sphere

        Following, we write:
        b = 2(u . CO)
        c = CO^2 - r^2
        The equation is d^2 + bd + c = 0 (classic quadratic form)
        */
        let eps = 1.0e-12_f64;
        let vector_co = Vector::new_from_points(&object.center, &self.origin)?;
        let b = 2. * &self.direction.to_vector().scalar_product(&vector_co);
        let c = vector_co.scalar_product(&vector_co) - object.radius.powi(2);
        let delta = b * b - 4. * c;

        if cfg!(test) {
            println!("Ray::intersect : delta = {}", delta)
        }

        if delta < -eps {
            Ok(None)
        } else {
            // ? so it is possible to have non assigned value if later on we see we will always assign something to it
            let hit_distance: f64;

            if (-eps..=eps).contains(&delta) {
                hit_distance = -b / 2.;
            } else {
                let first_distance = (-b - delta.sqrt()) / 2.;
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

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    static ORIGIN: Point = Point {
        x: 0.,
        y: 6.5,
        z: -2.,
    };
    static DESTINATION: Point = Point {
        x: 45.,
        y: -89.,
        z: -0.1,
    };

    static ORIGIN_2: Point = Point {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    static DESTINATION_2: Point = Point {
        x: 1.,
        y: 1.,
        z: 1.,
    };

    #[test]
    fn test_new_from_points() -> Result<(), GeometryError> {
        let ray = Ray::new_from_points(&ORIGIN, &DESTINATION)?;
        let direction = UnitVector::new_from_points(&ORIGIN, &DESTINATION)?;

        assert_eq!(ray.direction, direction);
        assert_eq!(ray.origin, ORIGIN);

        Ok(())
    }

    #[test]
    fn test_point_at_a_distance() -> Result<(), GeometryError> {
        let ray = Ray::new_from_points(&ORIGIN, &DESTINATION)?;
        let scalar = 7.;
        let result_point = ray.point_at_a_distance(scalar);

        let expected_point = Point {
            x: ORIGIN.x + ray.direction.x() * scalar,
            y: ORIGIN.y + ray.direction.y() * scalar,
            z: ORIGIN.z + ray.direction.z() * scalar,
        };

        assert_eq!(result_point, expected_point);

        Ok(())
    }

    #[test]
    fn test_intersect_none() -> Result<(), GeometryError> {
        let center = Point {
            x: 10.,
            y: 10.,
            z: 0.,
        };
        let outer = Point {
            x: 9.,
            y: 8.,
            z: 7.,
        };
        let sphere = Sphere::new_from_points(&center, &outer);

        let ray = Ray::new_from_points(&ORIGIN_2, &DESTINATION_2)?;

        let intersect = ray.intersect(&sphere)?;

        assert!(intersect.is_none());

        Ok(())
    }

    #[test]
    fn test_intersect_once() -> Result<(), GeometryError> {
        let center = Point {
            x: 1.,
            y: 0.,
            z: 1.,
        };
        let outer = Point {
            x: 2. / 3.,
            y: 2. / 3.,
            z: 2. / 3.,
        };
        let sphere = Sphere::new_from_points(&center, &outer);

        let ray = Ray::new_from_points(&ORIGIN_2, &DESTINATION_2)?;

        let intersect = ray.intersect(&sphere)?;

        assert!(intersect.is_some());
        if let Some(result_hit) = intersect {
            assert_eq!(result_hit.object, &sphere);
            assert_eq!(&(result_hit.point_hit), &outer);
            assert!(approx_eq!(f64, result_hit.hit_distance, 2. / f64::sqrt(3.)));
        }

        Ok(())
    }

    #[test]
    fn test_intersect_twice() -> Result<(), GeometryError> {
        let center = Point {
            x: 10.,
            y: -20.,
            z: 7.,
        };
        let expected_hit_point = Point {
            x: 10.062917533594398,
            y: -14.855747210183665,
            z: -1.575121259692681,
        };
        // thank you Geogebra for the calculations

        let sphere = Sphere::new_from_radius(&center, 10.);

        let ray = Ray::new_from_points(&ORIGIN, &DESTINATION)?;

        let intersect = ray.intersect(&sphere)?;

        assert!(intersect.is_some());
        if let Some(result_hit) = intersect {
            assert_eq!(result_hit.object, &sphere);
            assert_eq!(&(result_hit.point_hit), &expected_hit_point);
            assert!(approx_eq!(f64, result_hit.hit_distance, 23.611665975469712));
        }

        Ok(())
    }
}
