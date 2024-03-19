use float_cmp::approx_eq;

use crate::error::RayTracingError;
use crate::geometry::point::Point;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub norme: f64,
    pub direction: UnitVector,
}

impl Vector {
    pub fn x(&self) -> f64 {
        self.direction.x
    }

    pub fn y(&self) -> f64 {
        self.direction.y
    }

    pub fn z(&self) -> f64 {
        self.direction.z
    }

    pub fn new_from_points(origin: &Point, destination: &Point) -> Result<Self, RayTracingError> {
        {
            let Point { x, y, z } = destination - origin;
            Self::new_from_coordinates(x, y, z)
        }
    }

    pub fn new_from_coordinates(x: f64, y: f64, z: f64) -> Result<Self, RayTracingError> {
        let norme = Self::norme(x, y, z);
        let direction = Self::normalize(x, y, z, norme)?;
        Ok(Vector { norme, direction })
    }

    pub fn scalar_product(&self, vector_2: &Self) -> f64 {
        self.norme * vector_2.norme * self.direction.scalar_product(&vector_2.direction)
    }

    pub fn norme(x: f64, y: f64, z: f64) -> f64 {
        (x * x + y * y + z * z).sqrt()
    }

    pub fn norme_vec(&self) -> f64 {
        let UnitVector { x, y, z } = self.direction;
        Self::norme(self.norme * x, self.norme * y, self.norme * z)
    }

    fn normalize(x: f64, y: f64, z: f64, norme: f64) -> Result<UnitVector, RayTracingError> {
        let mut unit_vector = UnitVector { x, y, z };
        if norme == 0. {
            Err(RayTracingError::VectorHasNormeZero)
        } else {
            unit_vector.x /= norme;
            unit_vector.y /= norme;
            unit_vector.z /= norme;

            Ok(unit_vector)
        }
    }

    pub fn angle_with(&self, other: &Self) -> f64 {
        let scalar_product = self.scalar_product(other);
        (scalar_product / (self.norme_vec() * other.norme_vec())).acos()
    }

    fn cross_product(&self, other: &Vector) -> Result<Vector, RayTracingError> {
        Ok(self.norme_vec()
            * other.norme_vec()
            * self.angle_with(&other).sin()
            * &(self.direction.cross_product(&other.direction)?))
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.direction == other.direction && approx_eq!(f64, self.norme, other.norme, ulps = 2)
    }
}

impl Add for &Vector {
    type Output = Result<Vector, RayTracingError>;
    fn add(self, rhs: Self) -> Self::Output {
        let new_vector = *self;

        let x = new_vector.norme * new_vector.x() + rhs.norme * rhs.x();
        let y = new_vector.norme * new_vector.y() + rhs.norme * rhs.y();
        let z = new_vector.norme * new_vector.z() + rhs.norme * rhs.z();

        Vector::new_from_coordinates(x, y, z)
    }
}

impl Add for Vector {
    type Output = Result<Vector, RayTracingError>;
    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl Mul<f64> for &Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Self::Output {
        let mut mult_vec = *self;

        mult_vec.norme *= rhs;

        mult_vec
    }
}

impl Mul<&Vector> for f64 {
    type Output = Vector;
    fn mul(self, rhs: &Vector) -> Self::Output {
        rhs * self
    }
}

impl Sub for &Vector {
    type Output = Result<Vector, RayTracingError>;
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

// ! should be used in a way that this always has a norme of 1
#[derive(Clone, Copy, Debug)]
pub struct UnitVector {
    x: f64,
    y: f64,
    z: f64,
}

impl UnitVector {
    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn scalar_product(&self, other: &UnitVector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn new_from_points(origin: &Point, destination: &Point) -> Result<Self, RayTracingError> {
        {
            let Point { x, y, z } = destination - origin;
            Self::new_from_coordinates(x, y, z)
        }
    }

    pub fn new_from_coordinates(x: f64, y: f64, z: f64) -> Result<Self, RayTracingError> {
        let norme = Vector::norme(x, y, z);
        Vector::normalize(x, y, z, norme)
    }

    pub fn to_vector(&self) -> Vector {
        Vector {
            norme: 1.,
            direction: *self,
        }
    }

    pub fn angle_with(&self, other: &Self) -> f64 {
        let scalar_product = self.scalar_product(other);
        scalar_product.acos()
    }

    fn cross_product(&self, other: &UnitVector) -> Result<Self, RayTracingError> {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        UnitVector::new_from_coordinates(x, y, z)
    }

    pub fn tangent_plane_vectors(&self) -> [Self; 2] {
        let UnitVector { x, y, .. } = *self;
        if x != 0. || y != 0. {
            let v1 = Vector::new_from_coordinates(-y, x, 0.)
                .expect("This vector cannot have a norme of zero since either x or y are not zero");
            let v2 = self
                .cross_product(&(v1.direction))
                .expect("This should not have a norme of zero");
            [v1.direction, v2]
        } else {
            let v1 = Vector::new_from_coordinates(1., 0., 0.)
                .expect("This vector cannot have a norme of 0 as it is (1,0,0)");
            let v2 = Vector::new_from_coordinates(0., 1., 0.)
                .expect("This vector cannot have a norme of 0 as it is (0,1,0)");
            [v1.direction, v2.direction]
        }
    }
}

impl PartialEq for UnitVector {
    fn eq(&self, other: &Self) -> bool {
        let ulp = 2;
        approx_eq!(f64, self.x, other.x, ulps = ulp)
            && approx_eq!(f64, self.y, other.y, ulps = ulp)
            && approx_eq!(f64, self.z, other.z, ulps = ulp)
    }
}

impl Mul<f64> for &UnitVector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector {
            norme: rhs,
            direction: *self,
        }
    }
}

impl Mul<&UnitVector> for f64 {
    type Output = Vector;
    fn mul(self, rhs: &UnitVector) -> Self::Output {
        rhs * self
    }
}

impl Add<&Vector> for &UnitVector {
    type Output = Result<Vector, RayTracingError>;
    fn add(self, rhs: &Vector) -> Self::Output {
        // convert the unit vector to a Vector
        &self.to_vector() + rhs
    }
}

/* ----- Tests ----- */

#[cfg(test)]
mod tests {

    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn test_new_vector_from_coordinates() -> Result<(), RayTracingError> {
        let vector = Vector::new_from_coordinates(1., 1., 1.)?;

        let norme = f64::sqrt(3.);
        let float_norme = 1. / norme;

        assert_eq!(vector.norme, norme);
        assert_eq!(vector.direction.x, float_norme);
        assert_eq!(vector.direction.y, float_norme);
        assert_eq!(vector.direction.z, float_norme);
        Ok(())
    }

    #[test]
    fn test_vector_new_from_points() -> Result<(), RayTracingError> {
        let origin = Point {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        let destination = Point {
            x: 1.,
            y: 1.,
            z: 1.,
        };

        let vector = Vector::new_from_points(&origin, &destination)?;

        let norme = f64::sqrt(3.);
        let float_norme = 1. / norme;

        let expected_vector = Vector {
            norme,
            direction: UnitVector::new_from_coordinates(float_norme, float_norme, float_norme)?,
        };

        assert_eq!(vector, expected_vector);
        Ok(())
    }

    #[test]
    fn test_scalar_product() -> Result<(), RayTracingError> {
        let first_vector = Vector::new_from_coordinates(-1.5, 1., 45.)?;
        let second_vector = Vector::new_from_coordinates(0.458, -78., 12.)?;
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
        let vector = Vector::new_from_coordinates(0.458, -78., 12.)?;
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
        let first_vector = Vector::new_from_coordinates(0., 12.5, 0.)?;
        let a = f64::sqrt(2.) / 2.;
        let second_vector = Vector::new_from_coordinates(a, a, 0.)?;

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
        assert!(Vector::new_from_coordinates(0., 0., 0.).is_err());
        let point = Point {
            x: 415.25454,
            y: -54.,
            z: 0.2254,
        };
        assert!(Vector::new_from_points(&point, &point).is_err());
        assert!(UnitVector::new_from_coordinates(0., 0., 0.).is_err());
        assert!(UnitVector::new_from_points(&point, &point).is_err());
    }
}
