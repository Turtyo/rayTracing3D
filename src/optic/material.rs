use crate::error::RayTracingError;

use super::color::*;

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub emission_color: Color,
    emission_strength: f64,
    pub diffusion_coefficients: DiffusionCoefficient,
    reflection_coeff: f64,
}

impl Material {
    pub fn new(
        emission_color: Color,
        emission_strength: f64,
        diffusion_coefficients: DiffusionCoefficient,
        reflection_coeff: f64,
    ) -> Result<Self, RayTracingError> {
        if !(0. ..=1.).contains(&reflection_coeff) {
            Err(RayTracingError::CoefficientOOB(reflection_coeff, 0., 1.))
        } else if !(0. ..=1.).contains(&emission_strength) {
            Err(RayTracingError::CoefficientOOB(emission_strength, 0., 1.))
        } else {
            Ok(Material {
                emission_color,
                emission_strength,
                diffusion_coefficients,
                reflection_coeff,
            })
        }
    }

    pub fn emission_strength(&self) -> f64 {
        self.emission_strength
    }

    pub fn reflection_coeff(&self) -> f64 {
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
            emission_strength: 0.,
            diffusion_coefficients: white_diff,
            reflection_coeff: 0.,
        }
    }
}
