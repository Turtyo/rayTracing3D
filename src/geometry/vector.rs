use float_cmp::approx_eq;

use crate::error::RayTracingError;
use crate::geometry::point::Point;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {

    pub fn new_from_points(origin: &Point, destination: &Point) -> Self {
        let Point { x, y, z } = destination - origin;
        Vector{x, y, z}
    }

    pub fn new_from_coordinates(x: f64, y: f64, z: f64) -> Self {
        Vector{x,y,z}
    }

    pub fn scalar_product(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn norme(x: f64, y: f64, z: f64) -> f64 {
        (x * x + y * y + z * z).sqrt()
    }

    pub fn norme_vec(&self) -> f64 {
        let &Vector { x, y, z } = self;
        Self::norme(x,y,z)
    }

    pub fn normalize(&self) -> Result<Vector, RayTracingError> {
        let norme = self.norme_vec();
        let &Vector { mut x, mut y, mut z } = self;
        if norme == 0. {
            Err(RayTracingError::VectorHasNormeZero)
        } else {
            x /= norme;
            y /= norme;
            z /= norme;

            Ok(Vector { x, y, z })
        }
    }

    pub fn angle_with(&self, other: &Self) -> f64 {
        let scalar_product = self.scalar_product(other);
        (scalar_product / (self.norme_vec() * other.norme_vec())).acos()
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        approx_eq!(f64, self.x, other.x, ulps = 2) && approx_eq!(f64, self.y, other.y, ulps = 2) && approx_eq!(f64, self.z, other.z, ulps = 2)
    }
}

impl Add for &Vector {
    type Output = Vector;
    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;

        Vector{x, y, z}
    }
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;

        Vector{x, y, z}
    }
}

impl Mul<f64> for &Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;

        Vector{x, y, z}
    }
}

impl Mul<&Vector> for f64 {
    type Output = Vector;
    fn mul(self, rhs: &Vector) -> Self::Output {
        let x = rhs.x * self;
        let y = rhs.y * self;
        let z = rhs.z * self;

        Vector{x, y, z}
    }
}

impl Sub for &Vector {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        let neg_vec = -1. * rhs;
        // ? is there a way to oneline this with a closure maybe
        self + &neg_vec
    }
}

impl Div<f64> for &Vector {
    type Output = Vector;
    fn div(self, rhs: f64) -> Self::Output {
        (1. / rhs) * self
    }
}

/* ----- Tests ----- */

#[cfg(test)]
mod tests {

    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn test_vector_new_from_points() {
        let origin = Point {
            x: 1.,
            y: 0.,
            z: -1.,
        };
        let destination = Point {
            x: 1.,
            y: 2.,
            z: 3.,
        };

        let vector = Vector::new_from_points(&origin, &destination);

        let expected_vector = Vector {x: 0., y: 2., z: 4.};

        assert_eq!(vector, expected_vector);
    }

    #[test]
    fn test_scalar_product() -> Result<(), RayTracingError> {
        let first_vector = Vector::new_from_coordinates(-1.5, 1., 45.);
        let second_vector = Vector::new_from_coordinates(0.458, -78., 12.);
        let expected_value = -1.5 * 0.458 - 78. + 45. * 12.;

        assert!(approx_eq!(
            f64,
            first_vector.scalar_product(&second_vector),
            expected_value,
            ulps = 2
        ));
        Ok(())
    }

    #[test]
    fn test_norme() {
        let x = 541.4856;
        let y = 0.11457;
        let z = f64::sqrt(42.);
        let expected_value = (x * x + y * y + z * z).sqrt();
        assert!(approx_eq!(
            f64,
            Vector::norme(x, y, z),
            expected_value,
            ulps = 2
        ));
    }

    #[test]
    fn test_norme_vec() -> Result<(), RayTracingError> {
        let vector = Vector::new_from_coordinates(0.458, -78., 12.);
        let expected_value = f64::sqrt(0.458 * 0.458 + 78. * 78. + 12. * 12.);

        assert!(approx_eq!(
            f64,
            vector.norme_vec(),
            expected_value,
            ulps = 2
        ));
        Ok(())
    }

    #[test]
    fn test_angle_with() -> Result<(), RayTracingError> {
        let first_vector = Vector::new_from_coordinates(0., 12.5, 0.);
        let a = f64::sqrt(2.) / 2.;
        let second_vector = Vector::new_from_coordinates(a, a, 0.);

        assert!(approx_eq!(
            f64,
            first_vector.angle_with(&second_vector),
            std::f64::consts::FRAC_PI_4,
            ulps = 2
        ));

        Ok(())
    }

    #[test]
    fn test_return_error() {
        let zero_vector = Vector::new_from_coordinates(0., 0., 0.);
        assert!(Vector::normalize(&zero_vector).is_err());
    }
}
