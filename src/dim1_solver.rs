pub mod bisection;

use crate::dim1_func::Dim1Func;

pub trait Dim1Solver<T> {
    fn solve(&self, func: &impl Dim1Func<T>) -> T;
}
