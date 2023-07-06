use core::ops::Index;

use crate::{
    scalar::{base_float::BaseFloat, float},
    tensor::vector::Vector,
};

pub struct Polynomial<T, S1, S2> {
    pub degree: usize,
    pub coefficients: Vector<T, S1>,
    pub base_points: Option<Vector<T, S2>>,
}

impl<T, S1, S2> Polynomial<float<T>, S1, S2>
where
    S1: Index<usize, Output = float<T>>,
    S2: Index<usize, Output = float<T>>,
    T: Copy + Clone + BaseFloat,
{
    pub fn nest_mul(&self, x: float<T>) -> float<T> {
        let d = self.degree;
        let mut y: float<T> = self.coefficients[d];

        if let Some(b) = &self.base_points {
            for i in (0..d).rev() {
                y = y * (x - b[i]) + self.coefficients[i];
            }
        } else {
            for i in (0..=d).rev() {
                y = y * x + self.coefficients[i];
            }
        }

        y
    }
}

pub type PolynomialInnerVec<T> = Polynomial<T, Vec<T>, Vec<T>>;
