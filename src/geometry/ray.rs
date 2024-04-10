use crate::error::RayTracingError;
use crate::object::Object;

use super::point::Point;
use super::shape::Sphere;
use super::vector::Vector;

use rand_xorshift::{self, XorShiftRng};
use rand_distr::{self, DistIter, UnitDisc, UnitSphere};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new_from_points(origin: &Point, destination: &Point) -> Result<Self, RayTracingError> {
        let dest = Vector::new_from_points(origin, destination);
        Ok(Ray {
            origin: *origin,
            direction: dest,
        })
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
        let normalized_dir = &self.direction.normalize()?;
        let eps = 1.0e-12_f64;
        let vector_co = Vector::new_from_points(&object.shape.center, &self.origin);
        let b = 2. * normalized_dir.scalar_product(&vector_co);
        let c = vector_co.scalar_product(&vector_co) - object.shape.radius.powi(2);
        let delta = b * b - 4. * c;

        if cfg!(test) {
            println!("Ray::intersect : delta = {}", delta)
        }

        if delta < -eps {
            return Ok(None)
        } else {
            // ? so it is possible to have non assigned value if later on we see we will always assign something to it
            let hit_distance: f64;

            if (-eps..=eps).contains(&delta) {
                hit_distance = -b / 2.;
            } else {
                let first_distance = (-b - delta.sqrt()) / 2.;
                let second_distance = (-b + delta.sqrt()) / 2.;
                if first_distance >= 0. && second_distance >= 0. {
                    hit_distance = first_distance.min(second_distance);
                }
                else if first_distance >= 0. {
                    // meaning the second distance was <= 0 due to lazy evaluation of the &&
                    // previous check necessarily failed due to the second_distance < 0
                    hit_distance = first_distance;
                } else  {
                    //both are negative, we know this because of the lazy evaluation 
                    return Ok(None)
                }
            }
            let point_hit = &self.origin + &(hit_distance * normalized_dir);
            let normal = Vector::new_from_points(&(object.shape.center), &point_hit);
            Ok(Some(HitInfo {
                object,
                point_hit,
                normal,
                hit_distance,
            }))
        }
    }

    pub fn first_point_hit_by_ray<'a>(
        &self,
        objects: &Vec<&'a Object>,
        ignore_object: Option<&Object>, 
        /* this is needed in the case where we don't want a ray to be trapped inside a sphere 
        due to float point error when calculating intersections in several consecutive bounces */ 
    ) -> Result<Option<HitInfo<'a>>, RayTracingError> {
        let mut hit_info_closest_point = HitInfo {
            object: objects[0],
            normal: Vector::new_from_coordinates(1., 0., 0.),
            point_hit: Point {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            hit_distance: f64::MAX,
        };
        let mut ray_has_hit_object = false;
        let objects_to_iter = objects.iter().filter(|object| match ignore_object {
            Some(object_to_ignore) => object.shape != object_to_ignore.shape,
            None => true,
        });
        // ! PERF : it might be faster to just check against the value inside the for loop
        for object in objects_to_iter {
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
            let point_to_source_vector = Vector::new_from_points(surface_point, source).normalize()?;
            let mut surface_normal_vector = Vector::new_from_points(&object.center, surface_point).normalize()?;
            // Let D such as R(ay) = N(ormal) + D, thus D = R - N
            // the sym S is : S = N - D = N - (R - N) = 2N - R
            // we also need a right angle between D and N for this to work, so we normalise N to the correct norme
            let teta = point_to_source_vector.angle_with(&surface_normal_vector);
            surface_normal_vector = &point_to_source_vector * teta.cos();
            let sym_vector = &(2. * &surface_normal_vector) - &point_to_source_vector;
            Ok(Ray {
                origin: *surface_point,
                direction: sym_vector,
            })
        }
    }
   
    pub fn cos_weighted_random_ray_unit_sphere(
        point: &Point,
        normal: &Vector,
        unit_sphere_iter: &mut DistIter<UnitSphere, XorShiftRng, [f64; 3]>,
    ) -> Result<Self, RayTracingError> {
        // based on https://www.iue.tuwien.ac.at/phd/ertl/node100.html
        let [x,y,z] = match unit_sphere_iter.next() {
            Some(arr) => arr,
            None => return Err(RayTracingError::IteratorDepleted()),
        };
        let direction = normal.normalize()? + Vector::new_from_coordinates(x, y, z);

        Ok(Ray{origin: *point, direction: direction.normalize()?})
        
    }
    
    // pub fn cos_weighted_random_ray_unit_disc(
    //     point: &Point,
    //     normal: &Vector,
    //     unit_disc_iter: &mut DistIter<UnitDisc, XorShiftRng, [f64; 2]>,
    // ) -> Result<Self, RayTracingError> {
    //     // based on https://www.pbr-book.org/3ed-2018/Monte_Carlo_Integration/2D_Sampling_with_Multidimensional_Transformations
    //     let [t, v] = normal.tangent_plane_vectors();
    //     // ! rng should be generated as XorShiftRng::seed_from_u64(seed);
    //     // let iter_rng: DistIter<UnitDisc, XorShiftRng, [f64; 2]> = UnitDisc.sample_iter(rng);
    //     // this allows for seeding
    //     let [x, y] = match unit_disc_iter.next() {
    //         Some(values) => values,
    //         None => return Err(RayTracingError::IteratorDepleted()),
    //     };

    //     // ! this might go wrong
    //     let radius_2 = x * x + y * y;
    //     let ray_vector = ((x * &t + y * &v)? + (1. - radius_2).sqrt() * normal)?;

    //     Ok(Ray {
    //         origin: *point,
    //         direction: ray_vector.direction,
    //     })
    // }
    
    pub fn uniform_weighted_random_ray(
        point: &Point,
        normal: &Vector,
        unit_sphere_iter: &mut DistIter<UnitSphere, XorShiftRng, [f64; 3]>,
    ) -> Result<Self, RayTracingError> {
        let [x,y,z] = match unit_sphere_iter.next() {
            Some(arr) => arr,
            None => return Err(RayTracingError::IteratorDepleted()),
        };
        let direction = Vector::new_from_coordinates(x, y, z);
        if normal.scalar_product(&direction) < 0. {
            let reverse_direction = -1. * &direction;
            Ok(Ray{origin: *point, direction: reverse_direction})
        }
        else {
            Ok(Ray{origin: *point, direction})
        }
        
    }

    
}

#[derive(Debug, Clone, Copy)]
pub struct HitInfo<'a> {
    pub object: &'a Object,
    pub point_hit: Point,
    pub normal: Vector,
    pub hit_distance: f64,
}

#[cfg(test)]
mod tests {

    use crate::optic::material::Material;

    use super::*;
    use float_cmp::approx_eq;

    use plotters::prelude::*;
    use rand::SeedableRng;
    use rand_distr::{Distribution, UnitSphere};

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
        let direction = Vector::new_from_points(&ORIGIN, &DESTINATION);

        assert_eq!(ray.direction, direction);
        assert_eq!(ray.origin, ORIGIN);

        Ok(())
    }

    // #[test]
    // fn test_point_at_a_distance() -> Result<(), RayTracingError> {
    //     let ray = Ray::new_from_points(&ORIGIN, &DESTINATION)?;
    //     let scalar = 7.;
    //     let result_point = ray.point_at_a_distance(scalar);

    //     let expected_point = Point {
    //         x: ORIGIN.x + ray.direction.x() * scalar,
    //         y: ORIGIN.y + ray.direction.y() * scalar,
    //         z: ORIGIN.z + ray.direction.z() * scalar,
    //     };

    //     assert_eq!(result_point, expected_point);

    //     Ok(())
    // }

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

        if let Some(hit) = ray.first_point_hit_by_ray(&objects, None)? {
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

        if let Some(hit) = ray.first_point_hit_by_ray(&objects, None)? {
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

        if let Some(hit) = ray.first_point_hit_by_ray(&objects, None)? {
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

    #[test]
    #[ignore]
    fn draw_uniform_weighted_random_ray() -> Result<(), Box<dyn std::error::Error>> {
        const OUT_FILE_NAME: &str = "plot_output/uniform_weighted_random_ray.gif";
        let factor = 2;
        let eps = 0.01 / (factor as f64);
        let total_frame_number = 157;

        let seed = 2;
        let rng = XorShiftRng::seed_from_u64(seed);
        let mut iter_rng: DistIter<UnitSphere, XorShiftRng, [f64; 3]> = UnitSphere.sample_iter(rng);

        let random_points = {
            let point_number = 2000 * factor;
            let mut temp_vec = vec![(0., 0., 0.); point_number];
            for i in 0..point_number {
                let [x, y, z] = match iter_rng.next() {
                    Some(arr) => arr,
                    _ => panic!("Iterator over the sphere surface is over when is shouldn't be\n"),
                };
                temp_vec[i] = (x, y.abs(), z);
            }
            temp_vec
        };

        let root = BitMapBackend::gif(OUT_FILE_NAME, (600, 400), 100)?.into_drawing_area();

        for pitch in 0..total_frame_number {
            println!("frame : {}/{}", pitch + 1, total_frame_number);
            root.fill(&WHITE)?;

            let mut chart = ChartBuilder::on(&root)
                .caption("2D Gaussian PDF", ("sans-serif", 20))
                .build_cartesian_3d(-1.0..1.0, -1.0..1.0, -1.0..1.0)?;
            chart.with_projection(|mut p| {
                p.pitch = 1.57 - (1.57 - pitch as f64 / 50.0).abs();
                p.yaw = 1.57 - (1.57 - pitch as f64 / 50.0).abs();
                p.scale = 0.7;
                p.into_matrix() // build the projection matrix
            });

            chart
                .configure_axes()
                .light_grid_style(BLACK.mix(0.15))
                .max_light_lines(3)
                .draw()?;

            chart.draw_series(random_points.iter().map(|(x, y, z)| {
                Cubiod::new(
                    [
                        (*x - eps, *y - eps, *z - eps),
                        (*x + eps, *y + eps, *z + eps),
                    ],
                    BLUE.mix(0.1),
                    BLUE.mix(0.1),
                )
            }))?;

            root.present()?;
        }

        // To avoid the IO failure being ignored silently, we manually call the present function
        root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
        println!("Result has been saved to {}", OUT_FILE_NAME);

        Ok(())
    }

    #[test]
    #[ignore]
    fn draw_cos_weighted_random_ray() -> Result<(), Box<dyn std::error::Error>> {
        const OUT_FILE_NAME: &str = "plot_output/cos_weighted_random_ray.gif";
        let factor = 2;
        let eps = 0.01 / (factor as f64);
        let total_frame_number = 157;

        let seed = 2;
        let rng = XorShiftRng::seed_from_u64(seed);
        let mut iter_rng: DistIter<UnitSphere, XorShiftRng, [f64; 3]> = UnitSphere.sample_iter(rng);

        let random_points = {
            let normal_vector = Vector::new_from_coordinates(0., 11., 0.);
            let origin_point = Point::new(-1., 0., 1.2);
            let point_number = 2000 * factor;
            let mut temp_vec = vec![(0., 0., 0.); point_number];
            for i in 0..point_number {
                let ray =
                    Ray::cos_weighted_random_ray_unit_sphere(&origin_point, &normal_vector, &mut iter_rng)?;
                temp_vec[i] = (ray.direction.x, ray.direction.y, ray.direction.z);
            }
            temp_vec
        };

        let root = BitMapBackend::gif(OUT_FILE_NAME, (600, 400), 100)?.into_drawing_area();

        for pitch in 0..total_frame_number {
            println!("frame : {}/{}", pitch + 1, total_frame_number);
            root.fill(&WHITE)?;

            let mut chart = ChartBuilder::on(&root)
                .caption("2D Gaussian PDF", ("sans-serif", 20))
                .build_cartesian_3d(-1.0..1.0, -1.0..1.0, -1.0..1.0)?;
            chart.with_projection(|mut p| {
                p.pitch = 1.57 - (1.57 - pitch as f64 / 50.0).abs();
                p.yaw = 1.57 - (1.57 - pitch as f64 / 50.0).abs();
                p.scale = 0.7;
                p.into_matrix() // build the projection matrix
            });

            chart
                .configure_axes()
                .light_grid_style(BLACK.mix(0.15))
                .max_light_lines(3)
                .draw()?;

            chart.draw_series(random_points.iter().map(|(x, y, z)| {
                Cubiod::new(
                    [
                        (*x - eps, *y - eps, *z - eps),
                        (*x + eps, *y + eps, *z + eps),
                    ],
                    BLUE.mix(0.1),
                    BLUE.mix(0.1),
                )
            }))?;

            root.present()?;
        }

        // To avoid the IO failure being ignored silently, we manually call the present function
        root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
        println!("Result has been saved to {}", OUT_FILE_NAME);

        Ok(())
    }

    // #[test]
    // fn test_uniform_weighted_random_ray() -> Result<(), RayTracingError> {
    //     // * We use the One-sample Kolmogorov–Smirnov statistic, see https://en.wikipedia.org/wiki/Kolmogorov-Smirnov_test
    //     // * reference function is cos x between 0 and pi/2 (integral over this interval is indeed 1)
    //     let seed = 2;
    //     let sample_number = 1e6 as u64; //cast is exact until 1e16
    //     let epsilon = 1e-2;
    //     let number_of_intervals = (std::f64::consts::FRAC_PI_2 / epsilon).ceil() as usize;
    //     // to compensate the normalization of all the ring surfaces later
    //     let egamma = 0.577215664901532860606512090082402431; // The Euler-Mascheroni constant
    //     let normalized_sample_number = (sample_number as f64) * epsilon * epsilon * (digamma(number_of_intervals as f64 + 2.) + egamma - 1.);
    //     println!("number of intervals : {}", number_of_intervals);
    //     let rng = rand_chacha::XorShiftRng::seed_from_u64(seed);
    //     let mut iter_rng: DistIter<UnitSphere, XorShiftRng, [f64; 3]> = UnitSphere.sample_iter(rng);
    //     let mut list_of_ray_distribution: Vec<u64> = vec![0; number_of_intervals];
    //     let normal_vector = UnitVector::new_from_coordinates(0.,0.,1.)?;
    //     let origin_point = Point::new(0.,0.,0.);
    //     for _ in 1..=sample_number {
    //         let [x,y,z] = match iter_rng.next(){
    //             Some(arr) => arr,
    //             _ => panic!("Iterator over the sphere surface is over when is shouldn't be\n"),
    //         };
    //         let destination = Point::new(x, y, z);
    //         // println!("sampled point: {:?}", &destination);
    //         let ray = Ray::new_from_points(&origin_point, &destination)?;
    //         let angle = match normal_vector.angle_with(&ray.direction) {
    //             x if (0. ..=std::f64::consts::FRAC_PI_2).contains(&x)=> x,
    //             x if (std::f64::consts::FRAC_PI_2 ..=std::f64::consts::PI).contains(&x)=> std::f64::consts::PI - x,
    //             y => panic!("The angle {} is not in the range 0 ~ pi\n", y),
    //         };
    //         let eps_index_of_angle = (angle / epsilon).floor() as usize;
    //         list_of_ray_distribution[eps_index_of_angle] += 1;
    //     }
    //     let mut list_of_probabilities = list_of_ray_distribution
    //         .into_iter()
    //         .map(|x| (x as f64) / (normalized_sample_number as f64));
    //     println!("list of probabilities : {:?}", list_of_probabilities.clone().collect::<Vec<f64>>());
    //     println!("total sum : {}", list_of_probabilities.clone().sum::<f64>());
    //     let mut empirical_distribution = vec![0.; number_of_intervals];
    //     let mut previous_sum = 0.;
    //     let mut list_of_indexes = vec![0; number_of_intervals];
    //     for i in 0..number_of_intervals {
    //         let current_val = match list_of_probabilities.next(){
    //             Some(val) => val,
    //             _ => panic!("Iterator is depleted at index {0} but we don't have all the values in the empirical distribution : {1:?}", i, empirical_distribution),
    //         };
    //         let surface_normalization_ratio = 1. + (i as f64)/2.;
    //         previous_sum += current_val/(surface_normalization_ratio);
    //         empirical_distribution[i] = previous_sum;
    //         list_of_indexes[i] = i;
    //     }

    //     previous_sum = 0.;
    //     let expected_distribution = list_of_indexes.into_iter().map(|x| {
    //         let val = x as f64 * epsilon;
    //         previous_sum += val;
    //         previous_sum
    //     });

    //     let mut max_ks_distance = 0.;

    //     let list_of_ks_distance: Vec<f64> = empirical_distribution
    //         .into_iter()
    //         .zip(expected_distribution)
    //         .map(|double| {
    //             let val = (double.0 - double.1).abs();
    //             if val >= max_ks_distance {
    //                 max_ks_distance = val;
    //             }
    //             val
    //         })
    //         .collect();

    //     assert!(
    //         max_ks_distance < 0.1,
    //         "list of ks distance: {:?}",
    //         list_of_ks_distance
    //     );

    //     Ok(())
    // }

    // #[test]
    // fn test_cos_weighted_random_ray() -> Result<(), RayTracingError> {
    //     // * We use the One-sample Kolmogorov–Smirnov statistic, see https://en.wikipedia.org/wiki/Kolmogorov-Smirnov_test
    //     // * reference function is cos x between 0 and pi/2 (integral over this interval is indeed 1)
    //     let seed = 2;
    //     let sample_number = 1e6 as u64; //cast is exact until 1e16
    //     let epsilon = 1e-2;
    //     let number_of_intervals = (std::f64::consts::FRAC_PI_2 / epsilon).ceil() as usize;
    //     println!("number of intervals : {}", number_of_intervals);
    //     let rng = rand_chacha::XorShiftRng::seed_from_u64(seed);
    //     let mut iter_rng: DistIter<UnitDisc, XorShiftRng, [f64; 2]> = UnitDisc.sample_iter(rng);
    //     let mut list_of_ray_distribution: Vec<u64> = vec![0; number_of_intervals];
    //     let normal_vector = UnitVector::new_from_coordinates(0.7, 12., -2.)?;
    //     let origin_point = Point::new(-1., 0., 1.2);
    //     for _ in 1..=sample_number {
    //         let ray = Ray::cos_weighted_random_ray(&origin_point, &normal_vector, &mut iter_rng)?;
    //         let angle = normal_vector.angle_with(&ray.direction).abs();
    //         assert!(angle <= std::f64::consts::FRAC_PI_2 && angle >= 0.);
    //         let eps_index_of_angle = (angle / epsilon).floor() as usize;
    //         list_of_ray_distribution[eps_index_of_angle] += 1;
    //     }
    //     let mut list_of_probabilities = list_of_ray_distribution
    //         .into_iter()
    //         .map(|x| (x as f64) / (sample_number as f64));
    //     println!("list of probabilities : {:?}", list_of_probabilities.clone().collect::<Vec<f64>>());
    //     println!("total sum : {}", list_of_probabilities.clone().sum::<f64>());
    //     let mut empirical_distribution = vec![0.; number_of_intervals];
    //     let mut previous_sum = 0.;
    //     let mut list_of_indexes = vec![0; number_of_intervals];
    //     for i in 0..number_of_intervals {
    //         let current_val = match list_of_probabilities.next(){
    //             Some(val) => val,
    //             _ => panic!("Iterator is depleted at index {0} but we don't have all the values in the empirical distribution : {1:?}", i, empirical_distribution),
    //         };
    //         previous_sum += current_val;
    //         empirical_distribution[i] = previous_sum;
    //         list_of_indexes[i] = i;
    //     }

    //     previous_sum = 0.;
    //     let expected_distribution = list_of_indexes.into_iter().map(|x| {
    //         let val = (x as f64 * epsilon).cos();
    //         previous_sum += val;
    //         previous_sum
    //     });

    //     let mut max_ks_distance = 0.;

    //     let list_of_ks_distance: Vec<f64> = empirical_distribution
    //         .into_iter()
    //         .zip(expected_distribution)
    //         .map(|double| {
    //             let val = (double.0 - double.1).abs();
    //             if val >= max_ks_distance {
    //                 max_ks_distance = val;
    //             }
    //             val
    //         })
    //         .collect();

    //     assert!(
    //         max_ks_distance < 0.1,
    //         "list of ks distance: {:?}",
    //         list_of_ks_distance
    //     );

    //     Ok(())
    // }
}
