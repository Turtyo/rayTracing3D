use std::ops::Mul;

use crate::{
    error::{ColorError, GeometryError},
    geometry::{ray::Ray, vector::Vector},
};

#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Mul<f64> for &Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Self::Output {
        // Rust 1.45 introduced overflow control with the as keyword
        // setting the value to the crossed bound
        let r = (self.r as f64 * rhs) as u8;
        let g = (self.g as f64 * rhs) as u8;
        let b = (self.b as f64 * rhs) as u8;

        Color { r, g, b }
    }
}

impl Mul<&Color> for f64 {
    type Output = Color;
    fn mul(self, rhs: &Color) -> Self::Output {
        rhs * self
    }
}

static BLACK: Color = Color {
    r: u8::MIN,
    g: u8::MIN,
    b: u8::MIN,
};
static WHITE: Color = Color {
    r: u8::MAX,
    g: u8::MAX,
    b: u8::MAX,
};

pub struct DiffusionCoefficient {
    dr: f32, // should be between 0 and 1
    dg: f32,
    db: f32,
}

impl DiffusionCoefficient {
    pub fn new(dr: f32, dg: f32, db: f32) -> Result<Self, ColorError> {
        if (0. ..=1.).contains(&dr) || (0. ..=1.).contains(&dg) || (0. ..=1.).contains(&db) {
            Err(ColorError::DiffusionCoefficientOOB(format!(
                "dr : {0} | dg : {1} | db : {2}",
                dr, dg, db
            )))
        } else {
            Ok(DiffusionCoefficient { dr, dg, db })
        }
    }
}

impl Mul<&Color> for &DiffusionCoefficient {
    type Output = Color;
    fn mul(self, rhs: &Color) -> Self::Output {
        let r = (rhs.r as f32 * self.dr) as u8;
        let g = (rhs.g as f32 * self.dg) as u8;
        let b = (rhs.b as f32 * self.db) as u8;

        Color { r, g, b }
    }
}

impl Mul<&DiffusionCoefficient> for &Color {
    type Output = Color;
    fn mul(self, rhs: &DiffusionCoefficient) -> Self::Output {
        rhs * self
    }
}

pub fn diffused_color(
    source_color: Color,
    object_diffusion_coefficient: DiffusionCoefficient,
    source_ray: Ray,
    surface_normal_vector: Vector,
) -> Result<Color, GeometryError> {
    // ! start by checking if source is visible from the point
    // since we only have spheres for now, this is done by checking the scalar product normal.ray is <= 0
    // source is above the horizon if normal.(ray from surface to source) >= 0, since we have the ray from source to surface it's the opposite
    let vector_from_surface_to_source = -1. * &source_ray.direction.to_vector();
    let normal_ray_scalar_prod =
        surface_normal_vector.scalar_product(&vector_from_surface_to_source);
    if normal_ray_scalar_prod <= 0. {
        Err(GeometryError::SourceNotVisibleFromPoint(format!("Source has ray : {0:?} | Surface normal vector is : {1:?} | Their scalar product is {2}", source_ray, surface_normal_vector, normal_ray_scalar_prod)))
    } else {
        let angle = vector_from_surface_to_source.angle_with(&surface_normal_vector);
        Ok(&(&source_color * &object_diffusion_coefficient) * angle.cos())
    }
}
