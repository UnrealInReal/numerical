use core::ops::Index;

use crate::{
    scalar::{base_float::BaseFloat, float},
    tensor::vector::{Len, Vector},
};

pub struct Polynomial<T, S1: Len, S2: Len> {
    degree: usize,
    coefficients: Vector<T, S1>,
    base_points: Option<Vector<T, S2>>,
}

impl<T, S1, S2> Polynomial<float<T>, S1, S2>
where
    S1: Index<usize, Output = float<T>> + Len,
    S2: Index<usize, Output = float<T>> + Len,
    T: Copy + Clone + BaseFloat,
{
    pub fn new(
        degree: usize,
        coefficients: Vector<float<T>, S1>,
        base_points: Option<Vector<float<T>, S2>>,
    ) -> Self {
        assert!(!coefficients.is_empty());
        assert!(degree + 1 == coefficients.len());
        assert!(base_points.is_none() || base_points.as_ref().unwrap().len() == degree);
        Self {
            degree,
            coefficients,
            base_points,
        }
    }

    pub fn from_coefficients(coefficients: Vector<float<T>, S1>) -> Self {
        Self::new(coefficients.len() - 1, coefficients, None)
    }

    pub fn from_coefficients_base_points(
        coefficients: Vector<float<T>, S1>,
        base_points: Vector<float<T>, S2>,
    ) -> Self {
        Self::new(coefficients.len() - 1, coefficients, Some(base_points))
    }

    pub fn nest_mul(&self, x: float<T>) -> float<T> {
        let d = self.degree;
        let mut y: float<T> = self.coefficients[d];

        if let Some(b) = &self.base_points {
            for i in (0..d).rev() {
                y = y * (x - b[i]) + self.coefficients[i];
            }
        } else {
            for i in (0..d).rev() {
                y = y * x + self.coefficients[i];
            }
        }

        y
    }
}

pub type PolynomialInnerVec<T> = Polynomial<T, Vec<T>, Vec<T>>;
