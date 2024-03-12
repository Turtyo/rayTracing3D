use crate::geometry::point::Point;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Clone, Copy)]
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

    pub fn new_from_points(origin: &Point, destination: &Point) -> Self {
        {
            let Point { x, y, z } = destination - origin;
            Self::new_from_coordinates(x, y, z)
        }
    }

    pub fn new_from_coordinates(x: f64, y: f64, z: f64) -> Self {
        let norme = Self::norme(x, y, z);
        let direction = Self::normalize(x, y, z, norme);
        Vector { norme, direction }
    }

    pub fn scalar_product(vector_1: &Vector, vector_2: &Vector) -> f64 {
        vector_1.norme * vector_2.norme * vector_1.direction.scalar_product(&vector_2.direction)
    }

    pub fn norme(x: f64, y: f64, z: f64) -> f64 {
        (x * x + y * y + z * z).sqrt()
    }

    fn normalize(x: f64, y: f64, z: f64, norme: f64) -> UnitVector {
        let mut unit_vector = UnitVector { x, y, z };
        // ! need to check norme != 0 and maybe return a result ?
        unit_vector.x /= norme;
        unit_vector.y /= norme;
        unit_vector.z /= norme;

        // ? is vector discarded here

        unit_vector
    }
}

impl Add for &Vector {
    type Output = Vector;
    fn add(self, rhs: Self) -> Self::Output {
        let new_vector = *self;

        let x = new_vector.norme * new_vector.x() + rhs.norme * rhs.x();
        let y = new_vector.norme * new_vector.y() + rhs.norme * rhs.y();
        let z = new_vector.norme * new_vector.z() + rhs.norme * rhs.z();

        Vector::new_from_coordinates(x, y, z)
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

// ! should be used in a way that this always has a norme of 1
#[derive(Clone, Copy)]
pub struct UnitVector {
    x: f64,
    y: f64,
    z: f64,
}

impl UnitVector {
    pub fn scalar_product(&self, other: &UnitVector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn new_from_points(origin: &Point, destination: &Point) -> Self {
        {
            let Point { x, y, z } = destination - origin;
            Self::new_from_coordinates(x, y, z)
        }
    }

    pub fn new_from_coordinates(x: f64, y: f64, z: f64) -> Self {
        let norme = Vector::norme(x, y, z);
        Vector::normalize(x, y, z, norme)
    }

    pub fn to_vector(&self) -> Vector {
        Vector {
            norme: 1.,
            direction: *self,
        }
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
    type Output = Vector;
    fn add(self, rhs: &Vector) -> Self::Output {
        // convert the unit vector to a Vector
        &self.to_vector() + rhs
    }
}
