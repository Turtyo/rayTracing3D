use thiserror::Error;

pub mod point;
pub mod vector;
pub mod ray;
pub mod object;

#[derive(Error, Debug)]
pub enum GeometryErrors {
    #[error("Can't create vector of norme 1 from vector of norme 0")]
    UnitVectorFromZeroVector(String)
}
