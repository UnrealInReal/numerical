use crate::{dim1_func::Dim1Func, scalar::base_float::BaseFloat};

use super::Dim1Solver;

pub struct BisectionSolver<T: PartialEq + PartialOrd> {
    search_range: [T; 2],
    error_tolerance: T,
}

impl<T: BaseFloat> BisectionSolver<T> {
    pub fn new(search_range: [T; 2], error_tolerance: T) -> Self {
        assert!(search_range[0] < search_range[1]);
        assert!(error_tolerance > T::epsilon());
        Self {
            search_range,
            error_tolerance,
        }
    }
}

impl<T: BaseFloat> Dim1Solver<T> for BisectionSolver<T> {
    fn solve(&self, func: &impl Dim1Func<T>) -> Option<T> {
        let (mut a, mut b) = (self.search_range[0], self.search_range[1]);

        assert!(func.eval(a).signum() * func.eval(b).signum() == -T::ONE);

        let mut iter_num: usize = 0;
        while (b - a) > T::from(2.) * self.error_tolerance {
            let c = (b + a).half();
            let yc = func.eval(c);

            if yc == T::ZERO {
                break;
            }

            if func.eval(a).signum() * yc.signum() == -T::ONE {
                b = c;
            } else {
                debug_assert!(func.eval(b).signum() * yc.signum() == -T::ONE);
                a = c;
            }

            iter_num += 1;
        }

        log::info!("Bisection solve, iteration number = {}", iter_num);

        Some((b + a).half())
    }
}
