use crate::error::RayTracingError;
use crate::object::Object;

use super::point::Point;
use super::shape::Sphere;
use super::vector::{UnitVector, Vector};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: UnitVector,
}

impl Ray {
    pub fn new_from_points(origin: &Point, destination: &Point) -> Result<Self, RayTracingError> {
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

    pub fn intersect<'a>(
        &self,
        object: &'a Object,
    ) -> Result<Option<HitInfo<'a>>, RayTracingError> {
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
        let vector_co = Vector::new_from_points(&object.shape.center, &self.origin)?;
        let b = 2. * &self.direction.to_vector().scalar_product(&vector_co);
        let c = vector_co.scalar_product(&vector_co) - object.shape.radius.powi(2);
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
        objects: Vec<&'a Object>, // ? should this be a reference to a vector of objects so we don't need to clone the vector when we modify things in it after
    ) -> Result<Option<HitInfo<'a>>, RayTracingError> {
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
    ) -> Result<Self, RayTracingError> {
        if !object.source_is_above_horizon(surface_point, source)? {
            // this already checks if the surface point is on the object
            Err(RayTracingError::SourceNotVisibleFromPoint(format!(
                "The object is {0:?} | The object point is {1:?} | The source point is {2:?}",
                object, surface_point, source
            )))
        } else {
            let point_to_source_vector = Vector::new_from_points(surface_point, source)?;
            let mut surface_normal_vector = Vector::new_from_points(&object.center, surface_point)?;
            // Let D such as R(ay) = N(ormal) + D, thus D = R - N
            // the sym S is : S = N - D = N - (R - N) = 2N - R
            // we also need a right angle between D and N for this to work, so we normalise N to the correct norme
            let teta = point_to_source_vector.angle_with(&surface_normal_vector);
            surface_normal_vector.norme = point_to_source_vector.norme * teta.cos();
            let sym_vector = (&(2. * &surface_normal_vector) - &point_to_source_vector)?;
            Ok(Ray {
                origin: *surface_point,
                direction: sym_vector.direction,
            })
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HitInfo<'a> {
    pub object: &'a Object,
    pub point_hit: Point,
    pub hit_distance: f64,
}

#[cfg(test)]
mod tests {
    use crate::optic::material::Material;

    use super::*;
    use float_cmp::approx_eq;

    const ORIGIN: Point = Point {
        x: 0.,
        y: 6.5,
        z: -2.,
    };
    const DESTINATION: Point = Point {
        x: 45.,
        y: -89.,
        z: -0.1,
    };

    const ORIGIN_2: Point = Point {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    const DESTINATION_2: Point = Point {
        x: 1.,
        y: 1.,
        z: 1.,
    };

    #[test]
    fn test_new_from_points() -> Result<(), RayTracingError> {
        let ray = Ray::new_from_points(&ORIGIN, &DESTINATION)?;
        let direction = UnitVector::new_from_points(&ORIGIN, &DESTINATION)?;

        assert_eq!(ray.direction, direction);
        assert_eq!(ray.origin, ORIGIN);

        Ok(())
    }

    #[test]
    fn test_point_at_a_distance() -> Result<(), RayTracingError> {
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
    fn test_intersect_none() -> Result<(), RayTracingError> {
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
        let object = Object {
            shape: sphere,
            material: Material::default(),
        };

        let ray = Ray::new_from_points(&ORIGIN_2, &DESTINATION_2)?;

        let intersect = ray.intersect(&object)?;

        assert!(intersect.is_none());

        Ok(())
    }

    #[test]
    fn test_intersect_once() -> Result<(), RayTracingError> {
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
        let object = Object {
            shape: sphere,
            material: Material::default(),
        };

        let ray = Ray::new_from_points(&ORIGIN_2, &DESTINATION_2)?;

        let intersect = ray.intersect(&object)?;

        assert!(intersect.is_some());
        if let Some(result_hit) = intersect {
            assert_eq!(&(result_hit.object.shape), &sphere);
            assert_eq!(&(result_hit.point_hit), &outer);
            assert!(approx_eq!(f64, result_hit.hit_distance, 2. / f64::sqrt(3.)));
        }

        Ok(())
    }

    #[test]
    fn test_intersect_twice() -> Result<(), RayTracingError> {
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
        let object = Object {
            shape: sphere,
            material: Material::default(),
        };

        let ray = Ray::new_from_points(&ORIGIN, &DESTINATION)?;

        let intersect = ray.intersect(&object)?;

        assert!(intersect.is_some());
        if let Some(result_hit) = intersect {
            assert_eq!(&(result_hit.object.shape), &sphere);
            assert_eq!(&(result_hit.point_hit), &expected_hit_point);
            assert!(approx_eq!(f64, result_hit.hit_distance, 23.611665975469712));
        }

        Ok(())
    }

    #[test]
    fn test_first_point_hit_by_ray() -> Result<(), RayTracingError> {
        let sphere_1 = Sphere::new_from_radius(&ORIGIN, 4.);
        let center_2 = Point::new(-6.055414909, 1.6263876648, 0.);
        let sphere_2 = Sphere::new_from_radius(&center_2, 3.);
        let source = Point::new(-10.4900536536, -7.8544458162, 2.3623341028);
        let ray_destination = Point::new(-2.244677331, 2.7337430702, 0.);
        let ray = Ray::new_from_points(&source, &ray_destination)?;
        let object_1 = Object {
            shape: sphere_1,
            material: Material::default(),
        };
        let object_2 = Object {
            shape: sphere_2,
            material: Material::default(),
        };
        let mut objects = vec![&object_1, &object_2];

        /* First hit test, should hit sphere 2 */

        if let Some(hit) = ray.first_point_hit_by_ray(objects.clone())? {
            assert_eq!(&(hit.object.shape), &sphere_2);
            let expected_point =
                Point::new(-5.256205273754008, -1.133469952831104, 0.862815095680144);
            let expected_distance = 8.64946487777813;
            assert_eq!(hit.point_hit, expected_point);
            assert!(
                approx_eq!(f64, hit.hit_distance, expected_distance, ulps = 2),
                "expected distance {0}, got distance {1}",
                expected_distance,
                hit.hit_distance
            );
        } else {
            panic!("No point hit by ray when it should have hit something")
        }

        /* Modify the sphere 2 to make it so the ray hits the sphere 1 */

        // ? this is not ideal, is there a better way

        let mut sphere_2_modified = sphere_2;
        sphere_2_modified.radius = 2.;
        let mut object_2_modified = object_2;
        object_2_modified.shape = sphere_2_modified;

        objects.remove(1);
        objects.push(&object_2_modified);

        if let Some(hit) = ray.first_point_hit_by_ray(objects.clone())? {
            assert_eq!(&(hit.object.shape), &sphere_1);
            let expected_point =
                Point::new(-1.72455556675089, 3.40165042437406, -0.149017016715949);
            let expected_distance = 14.48587393749909;
            assert_eq!(hit.point_hit, expected_point);
            assert!(
                approx_eq!(f64, hit.hit_distance, expected_distance, ulps = 2),
                "expected distance {0}, got distance {1}",
                expected_distance,
                hit.hit_distance
            );
        } else {
            panic!("No point hit by ray when it should have hit something")
        }

        /* Modify the sphere 1 too so that the ray doesn't hit anything */

        let mut sphere_1_modified = sphere_1;
        sphere_1_modified.radius = 1.3;
        let mut object_1_modified = object_1;
        object_1_modified.shape = sphere_1_modified;

        objects.remove(0);
        objects.push(&object_1_modified);

        if let Some(hit) = ray.first_point_hit_by_ray(objects.clone())? {
            panic!("Ray should have hit nothing but got hit info : {:?}", hit);
        } else {
            Ok(())
        }
    }

    #[test]
    fn test_reflected_ray() -> Result<(), RayTracingError> {
        let source = Point::new(3., 3., 3.);
        let surface_point = Point::new(0., 3. * 2_f64.sqrt() / 2., 3. * 2_f64.sqrt() / 2.);
        let sphere_center = Point::new(0., 0., 0.);

        let object = Sphere::new_from_points(&sphere_center, &surface_point);

        let reflected_ray = Ray::reflected_ray(&source, &object, &surface_point)?;
        let expected_destination = Point::new(-3., 3., 3.);
        let expected_ray = Ray::new_from_points(&surface_point, &expected_destination)?;

        assert_eq!(reflected_ray, expected_ray);

        Ok(())
    }
}
