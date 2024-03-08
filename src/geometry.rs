use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl<'a, 'b> Sub<&'b Point> for &'a Point {
    type Output = Point;
    fn sub(self, other: &'b Point) -> Self::Output {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        Point { x, y, z }
    }
}

impl<'a, 'b> Add<&'b Point> for &'a Point {
    type Output = Point;
    fn add(self, other: &'b Point) -> Self::Output {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;
        Point { x, y, z }
    }
}

pub trait Vector {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;

    fn make_vector(origin: &Point, destination: &Point) -> impl Vector;

    fn scalar_product(vector_1: &impl Vector, vector_2: &impl Vector) -> f64 {
        vector_1.x() * vector_2.x() + vector_1.y() * vector_2.y() + vector_1.z() * vector_2.z()
    }

    fn norme(vector: &impl Vector) -> f64 {
        Self::scalar_product(vector, vector).sqrt()
    }
}

// ? how to implement mul for all &Vector
// impl Mul for dyn Vector{
//     type Output = dyn Vector;
//     fn mul(self, rhs: Self) -> Self::Output {

//     }
// }

// ? should also see how to do point + vector or point - vector

#[derive(Clone, Copy)]
pub struct BaseVector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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
    fn make_vector(origin: &Point, destination: &Point) -> impl Vector {
        {
            let Point { x, y, z } = destination - origin;
            BaseVector { x, y, z }
        }
    }
}

pub struct UnitVector {
    x: f64,
    y: f64,
    z: f64,
}

impl UnitVector {
    // ? how to automatically call new
    fn new_from_coordinates(x: f64, y: f64, z: f64) -> Self {
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

    fn new_from_vector<T: Vector>(vector: &T) -> Self {
        Self::new_from_coordinates(vector.x(), vector.y(), vector.z())
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

    // ? how to make this more compact by merging code with BaseVector
    fn make_vector(origin: &Point, destination: &Point) -> impl Vector {
        let Point { x, y, z } = destination - origin;
        UnitVector::new_from_coordinates(x, y, z)
    }
}

pub struct Ray {
    pub origin: Point,
    pub direction: UnitVector,
}

impl Ray {
    fn point_at_a_distance(&self, scalar: f64) -> Point {
        let Ray { origin, direction } = self;
        let mut point = Point {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        // could be simplified if we could do scalar * vector as an operation
        point.x = origin.x + scalar * direction.x();
        point.x = origin.y + scalar * direction.y();
        point.x = origin.z + scalar * direction.z();

        point
    }
}
