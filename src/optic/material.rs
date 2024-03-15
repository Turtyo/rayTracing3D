use crate::error::ColorError;

use super::color::*;

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub emission_color: Color,
    // ? emission strength
    pub diffusion_coefficients: DiffusionCoefficient,
    reflection_coeff: f32,
}

impl Material {
    pub fn new(
        emission_color: Color,
        diffusion_coefficients: DiffusionCoefficient,
        reflection_coeff: f32,
    ) -> Result<Self, ColorError> {
        if !(0. ..=1.).contains(&reflection_coeff) {
            Err(ColorError::ReflectionCoefficientOOB(reflection_coeff))
        } else {
            Ok(Material {
                emission_color,
                diffusion_coefficients,
                reflection_coeff,
            })
        }
    }

    pub fn reflection_coeff(&self) -> f32 {
        self.reflection_coeff
    }
}

impl Default for Material {
    fn default() -> Self {
        let white_diff = match WHITE.to_diffusion_coefficient() {
            Ok(diff) => diff,
            _ => panic!(
                "White color should always be able to convert to diffusion coefficient, check code"
            ),
        };
        Material {
            emission_color: BLACK,
            diffusion_coefficients: white_diff,
            reflection_coeff: 0.,
        }
    }
}
