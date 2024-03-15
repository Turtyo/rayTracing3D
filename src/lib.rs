mod error;
pub mod geometry;
pub mod object;
pub mod optic;

// use float_cmp;

// impl PartialEq for f64 {
//     fn eq(&self, other: &Self) -> bool {

//     }
// }
// * should use a wrapper with a deref on f64 to get all methods on f64 but that means changing all f64 references in the codebase
// * this would allow to not have to implement PartialEq with float_cmp for each struct that uses f64
