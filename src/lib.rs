mod error;
pub mod geometry;
pub mod object;
pub mod optic;

use std::path::PathBuf;

// use float_cmp;

// impl PartialEq for f64 {
//     fn eq(&self, other: &Self) -> bool {

//     }
// }
// * should use a wrapper with a deref on f64 to get all methods on f64 but that means changing all f64 references in the codebase
// * this would allow to not have to implement PartialEq with float_cmp for each struct that uses f64

pub fn ray_trace_image(
    number_of_points_per_pixel: usize,
    number_of_bounces: u64,
    objects: &Vec<&object::Object>,
    export_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut grid = optic::image::Grid::default();
    grid.make_image(number_of_points_per_pixel, number_of_bounces, objects)?;
    grid.export_image(export_path)?;

    Ok(())
}
