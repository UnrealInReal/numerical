pub mod bisection;
pub mod dim1_newton;
pub mod fixed_point;

use crate::dim1_func::Dim1Func;

pub trait Dim1Solver<T> {
    fn solve(&self, func: &impl Dim1Func<T>) -> Option<T>;
}
