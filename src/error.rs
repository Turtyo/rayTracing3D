use crate::geometry::{point::Point, shape::Sphere};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RayTracingError {
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
    #[error("Diffusion value should be a float coefficient between 0 and 1, got : dr = {0} | dg = {1} | db = {2}")]
    DiffusionCoefficientOOB(f32, f32, f32),
    #[error("Reflection value should be a float coefficient between {1} and {2}, got {0}")]
    CoefficientOOB(f32, f32, f32),
    #[error("The iterator doesn't have values anymore")]
    IteratorDepleted(),
}
