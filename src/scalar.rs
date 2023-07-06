pub mod base_float;
pub use base_float::float;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub type basef32 = float<f32>;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub type basef64 = float<f64>;
