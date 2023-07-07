use crate::{
    dim1_func::Dim1Func,
    scalar::base_float::BaseFloat,
    tensor::vector::{Len, VecStorage, Vector},
};

/// TODO: impl Display for Polynomial (issue: [1](https://github.com/UnrealInReal/numerical/issues/1))
pub struct Polynomial<T, S1: VecStorage<T>, S2: VecStorage<T>> {
    degree: usize,
    coefficients: Vector<T, S1>,
    base_points: Option<Vector<T, S2>>,
}

impl<T, S1, S2> Polynomial<T, S1, S2>
where
    S1: VecStorage<T>,
    S2: VecStorage<T>,
    T: BaseFloat,
{
    pub fn new(
        degree: usize,
        coefficients: Vector<T, S1>,
        base_points: Option<Vector<T, S2>>,
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

    pub fn from_coefficients(coefficients: Vector<T, S1>) -> Self {
        Self::new(coefficients.len() - 1, coefficients, None)
    }

    pub fn from_coefficients_base_points(
        coefficients: Vector<T, S1>,
        base_points: Vector<T, S2>,
    ) -> Self {
        Self::new(coefficients.len() - 1, coefficients, Some(base_points))
    }

    pub fn nest_mul(&self, x: T) -> T {
        let d = self.degree;
        let mut y: T = self.coefficients[d];

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

impl<T, S1, S2> Dim1Func<T> for Polynomial<T, S1, S2>
where
    S1: VecStorage<T>,
    S2: VecStorage<T>,
    T: BaseFloat,
{
    fn eval(&self, x: T) -> T {
        self.nest_mul(x)
    }
}
