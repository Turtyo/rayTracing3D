use crate::geometry::{object::Sphere, point::Point};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeometryError {
    #[error("Can't create vector of norme 1 from vector of norme 0")]
    UnitVectorFromZeroVector(String),
    #[error("The point {0:?} is not a point of the sphere {1:?}")]
    PointNotOnSphere(Point, Sphere),
    #[error("There is no sphere of index {0}, the number of spheres is {1}")]
    NoSphereAtIndex(usize, usize),
    #[error("The ray from point {0:?} to point {1:?} doesn't go through the point {1:?} (uh ?)")]
    RayBetweenPointsDoesNotHitPoint(Point, Point),
    #[error("Source is not visible from point: {0}")]
    SourceNotVisibleFromPoint(String),
    #[error("Cannot create a unit vector or vector of norme 0")]
    VectorHasNormeZero,
}

#[derive(Error, Debug)]
pub enum ColorError {
    #[error("Diffusion value should be a float coefficient between 0 and 1, got : {0}")]
    DiffusionCoefficientOOB(String),
}