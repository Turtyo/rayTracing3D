use std::ops::{Add, Mul};

use crate::{
    error::RayTracingError,
    geometry::{ray::Ray, vector::Vector},
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn to_diffusion_coefficient(&self) -> Result<DiffusionCoefficient, RayTracingError> {
        let Color { r, g, b } = *self;
        let r = r as f32 / (u8::MAX) as f32;
        let g = g as f32 / (u8::MAX) as f32;
        let b = b as f32 / (u8::MAX) as f32;
        DiffusionCoefficient::new(r, g, b)
    }
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

impl Add for &Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output {
        let r = self.r.saturating_add(rhs.r);
        let g = self.g.saturating_add(rhs.g);
        let b = self.b.saturating_add(rhs.b);
        Color { r, g, b }
    }
}

#[allow(dead_code)]
pub const BLACK: Color = Color {
    r: u8::MIN,
    g: u8::MIN,
    b: u8::MIN,
};
#[allow(dead_code)]
pub const WHITE: Color = Color {
    r: u8::MAX,
    g: u8::MAX,
    b: u8::MAX,
};
#[allow(dead_code)]
pub const RED: Color = Color {
    r: u8::MAX,
    g: u8::MIN,
    b: u8::MIN,
};
#[allow(dead_code)]
pub const GREEN: Color = Color {
    r: u8::MIN,
    g: u8::MAX,
    b: u8::MIN,
};
#[allow(dead_code)]
pub const BLUE: Color = Color {
    r: u8::MIN,
    g: u8::MIN,
    b: u8::MAX,
};

#[derive(Clone, Copy, Debug)]
pub struct DiffusionCoefficient {
    dr: f32, // should be between 0 and 1
    dg: f32,
    db: f32,
}

impl DiffusionCoefficient {
    pub fn new(dr: f32, dg: f32, db: f32) -> Result<Self, RayTracingError> {
        if !(0. ..=1.).contains(&dr) || !(0. ..=1.).contains(&dg) || !(0. ..=1.).contains(&db) {
            Err(RayTracingError::DiffusionCoefficientOOB(dr, dg, db))
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
) -> Result<Color, RayTracingError> {
    // ! start by checking if source is visible from the point
    // since we only have spheres for now, this is done by checking the scalar product normal.ray is <= 0
    // source is above the horizon if normal.(ray from surface to source) >= 0, since we have the ray from source to surface it's the opposite
    let vector_from_surface_to_source = -1. * &source_ray.direction.to_vector();
    let normal_ray_scalar_prod =
        surface_normal_vector.scalar_product(&vector_from_surface_to_source);
    if normal_ray_scalar_prod <= 0. {
        Err(RayTracingError::SourceNotVisibleFromPoint(format!("Source has ray : {0:?} | Surface normal vector is : {1:?} | Their scalar product is {2}", source_ray, surface_normal_vector, normal_ray_scalar_prod)))
    } else {
        let angle = vector_from_surface_to_source.angle_with(&surface_normal_vector);
        Ok(&(&source_color * &object_diffusion_coefficient) * angle.cos())
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::point::Point;

    use super::*;

    #[test]
    fn test_diffused_color_black() -> Result<(), Box<dyn std::error::Error>> {
        let source_color = BLACK;
        let object_diffusion_coefficient = DiffusionCoefficient::new(1., 1., 1.)?;

        let source = Point::new(-10.4900536536, -7.8544458162, 2.3623341028);
        let ray_destination = Point::new(-2.244677331, 2.7337430702, 0.);
        let source_ray = Ray::new_from_points(&source, &ray_destination)?;

        let surface_normal_vector =
            Vector::new_from_coordinates(0.799209635245991, -2.759857617631104, 0.862815095680144)?;

        let result_color = super::diffused_color(
            source_color,
            object_diffusion_coefficient,
            source_ray,
            surface_normal_vector,
        )?;
        let expected_color = BLACK;

        assert_eq!(result_color, expected_color);

        let result_color_2 = super::diffused_color(
            WHITE,
            BLACK.to_diffusion_coefficient()?,
            source_ray,
            surface_normal_vector,
        )?;
        assert_eq!(result_color_2, expected_color);

        Ok(())
    }

    #[test]
    fn test_diffused_color_blue() -> Result<(), Box<dyn std::error::Error>> {
        let source = Point::new(-10.4900536536, -7.8544458162, 2.3623341028);
        let ray_destination = Point::new(-2.244677331, 2.7337430702, 0.);
        let source_ray = Ray::new_from_points(&source, &ray_destination)?;

        let object_diffusion_coefficient = DiffusionCoefficient::new(0., 0., 0.5)?;
        let source_color = Color::new(100, 240, u8::MAX);

        let surface_normal_vector =
            Vector::new_from_coordinates(0.799209635245991, -2.759857617631104, 0.862815095680144)?;

        let result_color = super::diffused_color(
            source_color,
            object_diffusion_coefficient,
            source_ray,
            surface_normal_vector,
        )?;

        let b_expected = ((u8::MAX as f64) * 0.5 * 52.879169845223565_f64.to_radians().cos()) as u8;

        let expected_color = Color::new(0, 0, b_expected);

        assert_eq!(result_color, expected_color);

        Ok(())
    }
}
