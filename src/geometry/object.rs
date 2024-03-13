use super::point::Point;
use super::ray::Ray;
use super::vector::Vector;
use crate::error::GeometryError;

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
