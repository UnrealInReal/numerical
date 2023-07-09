use crate::scalar::base_float::BaseFloat;

use super::Dim1Solver;

pub struct FixedPointSolver<T> {
    start_point: T,
    error_tolerance: T,
    max_iter_num: usize,
}

impl<T> FixedPointSolver<T> {
    pub fn new(start_point: T, error_tolerance: T, max_iter_num: usize) -> Self {
        Self {
            start_point,
            error_tolerance,
            max_iter_num,
        }
    }
}

impl<T: BaseFloat> Dim1Solver<T> for FixedPointSolver<T> {
    fn solve(&self, func: &impl crate::dim1_func::Dim1Func<T>) -> Option<T> {
        let mut root = self.start_point;

        for i in 0..self.max_iter_num {
            let next = func.eval(root) + root;
            let diff = (next - root).abs();
            let relative_diff = diff / next.abs().max(T::ONE);
            root = next;
            if relative_diff < self.error_tolerance {
                log::info!("Fixed Point Iteration Num = {i:?}");
                return Some(root);
            }
        }

        None
    }
}
