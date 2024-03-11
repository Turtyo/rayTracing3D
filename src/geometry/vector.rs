use std::ops::Sub;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Div;
use crate::geometry::point::Point;

pub trait Vector {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;

    // ! not good, can't return any concrete type when using impl Vector
    // fn new_from_coordinates(x: f64, y:f64, z:f64) -> impl Vector;
    // fn new_from_points(origin: &Point, destination: &Point) -> impl Vector;

    fn scalar_product(vector_1: &impl Vector, vector_2: &impl Vector) -> f64 {
        vector_1.x() * vector_2.x() + vector_1.y() * vector_2.y() + vector_1.z() * vector_2.z()
    }

    fn norme(vector: &impl Vector) -> f64 {
        Self::scalar_product(vector, vector).sqrt()
    }
}

#[derive(Clone, Copy)]
pub struct BaseVector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl BaseVector {
    pub fn new_from_points(origin: &Point, destination: &Point) -> Self {
        {
            let Point { x, y, z } = destination - origin;
            BaseVector { x, y, z }
        }
    }

    pub fn new_from_coordinates(x: f64, y:f64, z:f64) -> Self {
        BaseVector{x, y, z}
    }
}

impl Vector for BaseVector {
    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
    fn z(&self) -> f64 {
        self.z
    }
}

// impl Add for &impl Vector {
//     type Output = impl Vector;
//     fn add(self, rhs: Self) -> Self::Output {
        
//     }
// }

impl Add for &BaseVector {
    type Output = BaseVector;
    fn add(self, rhs: Self) -> Self::Output {
        let mut new_vector = *self;

        new_vector.x += rhs.x;
        new_vector.y += rhs.y;
        new_vector.z += rhs.z;

        new_vector
    }
}


// ? how to implement mul for all &Vector
// impl<T: Vector> Mul<f64> for &T 
impl Mul<f64> for &BaseVector {
    type Output = BaseVector;
    fn mul(self, rhs: f64) -> Self::Output {
        let mut mult_vec = *self;
        mult_vec.x *= rhs;
        mult_vec.y *= rhs;
        mult_vec.z *= rhs;

        mult_vec

    }
}

impl Mul<&BaseVector> for f64 {
    type Output = BaseVector;
    fn mul(self, rhs: &BaseVector) -> Self::Output {
        rhs * self
    }
}

impl Sub for &BaseVector {
    type Output = BaseVector;
    fn sub(self, rhs: Self) -> Self::Output {
        let neg_vec = -1.* rhs;
        // ? is there a way to oneline this with a closure maybe
        self + &neg_vec
    }
}

impl Div<f64> for &BaseVector {
    type Output = BaseVector;
    fn div(self, rhs: f64) -> Self::Output {
        (1./rhs) * self 
    }
}

#[derive(Clone, Copy)]
pub struct UnitVector {
    x: f64,
    y: f64,
    z: f64,
}

impl UnitVector {
    // ? how to make this more compact by merging code with BaseVector
    pub fn new_from_points(origin: &Point, destination: &Point) -> Self {
        let Point { x, y, z } = destination - origin;
        UnitVector::new_from_coordinates(x, y, z)
    }

    pub fn new_from_coordinates(x: f64, y:f64, z:f64) -> Self {
        let vector = BaseVector { x, y, z };
        let norme = Self::norme(&vector);
        // ! need to check norme != 0 and maybe return a result ?
        let mut unit_vector = Self {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        unit_vector.x = x / norme;
        unit_vector.y = y / norme;
        unit_vector.z = z / norme;

        // ? is vector discarded here

        unit_vector
    }

    // ? how to make it return a UnitVector when new_from_coordinates returns a impl Vector

    fn normalize(vector: &BaseVector) -> impl Vector {
        let BaseVector{x,y,z} = *vector;
        UnitVector::new_from_coordinates(x, y, z)
    }
}

impl Vector for UnitVector {
    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
    fn z(&self) -> f64 {
        self.z
    }
    // ! this is the same code as BaseVector, but we can't have a field in a trait

}

impl Mul<f64> for &UnitVector {
    type Output = BaseVector;
    fn mul(self, rhs: f64) -> Self::Output {
        let UnitVector{mut x, mut y, mut z} = *self;
        x *= rhs;
        y *= rhs;
        z *= rhs;

        BaseVector{x, y, z}

    }
}

impl Mul<&UnitVector> for f64 {
    type Output = BaseVector;
    fn mul(self, rhs: &UnitVector) -> Self::Output {
        rhs * self
    }
}

impl Add<&BaseVector> for &UnitVector {
    type Output = BaseVector;
    fn add(self, rhs: &BaseVector) -> Self::Output {
        let mut new_vec = *rhs;
        new_vec.x += self.x;
        new_vec.y += self.y;
        new_vec.z += self.z;

        new_vec
    }
}
