use crate::scalar::base_float::BaseFloat;

use super::Dim1Solver;

pub struct Dim1NewtonSolver<'a, T> {
    start_point: T,
    error_tolerance: T,
    max_iter_num: usize,
    func: &'a dyn crate::dim1_func::Dim1Func<T>,
    func_first_derivative: &'a dyn crate::dim1_func::Dim1Func<T>,
}

impl<'a, T> Dim1NewtonSolver<'a, T> {
    pub fn new(
        start_point: T,
        error_tolerance: T,
        max_iter_num: usize,
        func: &'a impl crate::dim1_func::Dim1Func<T>,
        func_first_derivative: &'a impl crate::dim1_func::Dim1Func<T>,
    ) -> Self {
        Self {
            start_point,
            error_tolerance,
            max_iter_num,
            func,
            func_first_derivative,
        }
    }
}

impl<T: BaseFloat> Dim1Solver<T> for Dim1NewtonSolver<'_, T> {
    fn solve(&self, _: &impl crate::dim1_func::Dim1Func<T>) -> Option<T> {
        let mut root = self.start_point;

        for i in 0..self.max_iter_num {
            let next = root - self.func.eval(root) / self.func_first_derivative.eval(root);
            let diff = (next - root).abs();
            let relative_diff = diff / next.abs().max(T::ONE);
            root = next;

            if relative_diff < self.error_tolerance {
                log::info!("Dim1 Newton Iteration Num = {i:?}");
                return Some(root);
            }
        }

        None
    }
}
