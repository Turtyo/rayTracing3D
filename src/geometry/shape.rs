use super::point::Point;
use super::ray::Ray;
use super::vector::Vector;
use crate::error::RayTracingError;
use crate::object::Object;
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
        #[cfg(test)]
        println!(
            "point distance to center : {0} | sphere radius : {1}",
            point_distance_to_center, self.radius
        );
        approx_eq!(
            f64,
            self.radius,
            point_distance_to_center,
            ulps = 2,
            epsilon = 1e-12_f64
        )
    }

    pub fn source_is_above_horizon(
        &self,
        sphere_point: &Point,
        source: &Point,
    ) -> Result<bool, RayTracingError> {
        /*
        source is above the horizon if the scalar product between the normal to the sphere at the point on the sphere
        and the vector going from the sphere point to the source, is positive
        */
        #[cfg(test)]
        println!(
            "sphere_point : {0:?} | source : {1:?}",
            sphere_point, source
        );

        if self.point_is_on_sphere(sphere_point) {
            let normal = Vector::new_from_points(&self.center, sphere_point)?;
            let point_source_vec = Vector::new_from_points(sphere_point, source)?;
            Ok(normal.scalar_product(&point_source_vec) >= 0.)
        } else {
            Err(RayTracingError::PointNotOnSphere(*sphere_point, *self))
        }
    }

    pub fn source_is_visible_from_sphere_point(
        objects: &Vec<&Object>,
        sphere_index: usize,
        sphere_point: &Point,
        source: &Point,
    ) -> Result<bool, RayTracingError> {
        let current_object = objects.get(sphere_index);
        let current_object = match current_object {
            Some(object) => *object,
            _ => {
                return Err(RayTracingError::NoSphereAtIndex(
                    sphere_index,
                    objects.len(),
                ))
            }
        };

        if current_object
            .shape
            .source_is_above_horizon(sphere_point, source)?
        {
            let ray = Ray::new_from_points(source, sphere_point)?;
            if let Some(hit_info) = ray.first_point_hit_by_ray(objects, None)? {
                Ok(*sphere_point == hit_info.point_hit)
            } else {
                Err(RayTracingError::RayBetweenPointsDoesNotHitPoint(
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
    use crate::optic::material::Material;

    use super::*;

    const CENTER: Point = Point {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    const OUTER: Point = Point {
        x: 15.,
        y: 12.,
        z: -2.3,
    };

    fn make_test_sphere() -> Sphere {
        // can't make sphere const because new_from_points is not a const function
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
    fn test_source_is_above_horizon() -> Result<(), RayTracingError> {
        let sphere = make_test_sphere();

        let source_not_visible = Point::new(6.64, 24.69, 10.);
        let source_visible = Point::new(3.4676614813, 28.0630097447, 6.);

        assert!(sphere.source_is_above_horizon(&OUTER, &source_visible)?);
        assert!(!sphere.source_is_above_horizon(&OUTER, &source_not_visible)?);

        Ok(())
    }

    #[test]
    fn test_source_is_visible_from_sphere() -> Result<(), RayTracingError> {
        let center_1 = Point::new(0., 6.5, -2.);
        let sphere_1 = Sphere::new_from_radius(&center_1, 4.);
        let center_2 = Point::new(-6.055414909, 1.6263876648, 0.);
        let sphere_2 = Sphere::new_from_radius(&center_2, 3.);
        let source = Point::new(-10.4900536536, -7.8544458162, 2.3623341028);
        let object_1 = Object {
            shape: sphere_1,
            material: Material::default(),
        };
        let object_2 = Object {
            shape: sphere_2,
            material: Material::default(),
        };
        let mut objects = vec![&object_1, &object_2];

        let sphere_2_point_1 = Point::new(-7.016967003341442, -0.7687901334943, 1.529228852862008); // should view source
        let sphere_2_point_2 = Point::new(-3.065121810985787, 1.680180347891836, 0.235060705381277); // should not view source
        let sphere_1_point_1 = Point::new(-1.72455556675089, 3.40165042437406, -0.149017016715949); // should not view source

        assert!(Sphere::source_is_visible_from_sphere_point(
            &objects,
            1,
            &sphere_2_point_1,
            &source
        )?);
        assert!(!Sphere::source_is_visible_from_sphere_point(
            &objects,
            1,
            &sphere_2_point_2,
            &source
        )?);
        assert!(!Sphere::source_is_visible_from_sphere_point(
            &objects,
            0,
            &sphere_1_point_1,
            &source
        )?);
        let mut sphere_2_modified = sphere_2;
        sphere_2_modified.radius = 2.;
        let mut object_2_modified = object_2;
        object_2_modified.shape = sphere_2_modified;

        objects.remove(1);
        objects.push(&object_2_modified);

        // sphere 1 point 1 should now view the source

        assert!(Sphere::source_is_visible_from_sphere_point(
            &objects,
            0,
            &sphere_1_point_1,
            &source
        )?);

        Ok(())
    }
}
