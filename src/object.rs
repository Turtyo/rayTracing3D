use crate::{geometry::shape::Sphere, optic::material::Material};

#[derive(Clone, Copy, Debug)]
pub struct Object {
    pub shape: Sphere, // * should be changed to use a trait like Shape for example
    pub material: Material,
}
