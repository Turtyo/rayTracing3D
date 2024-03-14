use super::point::Point;
use super::ray::Ray;
use super::vector::Vector;
use crate::error::GeometryError;
use float_cmp::{self, approx_eq};

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

    pub fn new_from_radius(center: &Point, radius: f64) -> Self {
        Sphere {
            center: *center,
            radius,
        }
    }

    pub fn point_is_on_sphere(&self, point: &Point) -> bool {
        let Point { x, y, z } = point - &self.center;
        let point_distance_to_center = Vector::norme(x, y, z);
        self.radius == point_distance_to_center
    }

    pub fn source_is_above_horizon(
        &self,
        sphere_point: &Point,
        source: &Point,
    ) -> Result<bool, GeometryError> {
        /*
        source is above the horizon if the scalar product between the normal to the sphere at the point on the sphere
        and the vector going from the sphere point to the source, is positive
        */
        println!(
            "sphere_point : {0:?} | source : {1:?}",
            sphere_point, source
        );
        if self.point_is_on_sphere(sphere_point) {
            let normal = Vector::new_from_points(&self.center, sphere_point)?;
            let point_source_vec = Vector::new_from_points(sphere_point, source)?;
            Ok(normal.scalar_product(&point_source_vec) >= 0.)
        } else {
            Err(GeometryError::PointNotOnSphere(*sphere_point, *self))
        }
    }

    pub fn source_is_visible_from_sphere_point(
        objects: Vec<&Self>,
        sphere_index: usize,
        sphere_point: &Point,
        source: &Point,
    ) -> Result<bool, GeometryError> {
        let current_sphere = objects.get(sphere_index);
        let current_sphere = match current_sphere {
            Some(sphere) => *sphere,
            _ => return Err(GeometryError::NoSphereAtIndex(sphere_index, objects.len())),
        };

        if current_sphere.source_is_above_horizon(sphere_point, source)? {
            let ray = Ray::new_from_points(source, sphere_point)?;
            if let Some(hit_info) = ray.first_point_hit_by_ray(objects)? {
                Ok(*sphere_point == hit_info.point_hit)
            } else {
                Err(GeometryError::RayBetweenPointsDoesNotHitPoint(
                    *source,
                    *sphere_point,
                ))
            }
        } else {
            Ok(false)
        }
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center && approx_eq!(f64, self.radius, other.radius, ulps = 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static CENTER: Point = Point {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    static OUTER: Point = Point {
        x: 15.,
        y: 12.,
        z: -2.3,
    };

    fn make_test_sphere() -> Sphere {
        // can't make sphere static because new_from_points is not a static function
        Sphere::new_from_points(&CENTER, &OUTER)
    }

    #[test]
    fn test_new() {
        let sphere = make_test_sphere();

        assert_eq!(&(sphere.center), &CENTER);
        assert_eq!(sphere.radius, Vector::norme(15., 12., -2.3));
    }

    #[test]
    fn test_point_is_on_sphere() {
        let sphere = make_test_sphere();

        assert!(sphere.point_is_on_sphere(&OUTER));
    }

    #[test]
    fn test_source_is_above_horizon() -> Result<(), GeometryError> {
        let sphere = make_test_sphere();

        let source_not_visible = Point::new(6.64, 24.69, 10.);
        let source_visible = Point::new(3.4676614813, 28.0630097447, 6.);

        assert!(sphere.source_is_above_horizon(&OUTER, &source_visible)?);
        assert!(!sphere.source_is_above_horizon(&OUTER, &source_not_visible)?);

        Ok(())
    }
}
