use crate::{dim1_func::Dim1Func, scalar::base_float::BaseFloat};

use super::Dim1Solver;

pub struct BrentSolver<T: PartialEq + PartialOrd> {
    search_range: [T; 2],
    error_tolerance: T,
}

impl<T: BaseFloat> BrentSolver<T> {
    pub fn new(search_range: [T; 2], error_tolerance: T) -> Self {
        assert!(search_range[0] < search_range[1]);
        assert!(error_tolerance > T::epsilon());
        Self {
            search_range,
            error_tolerance,
        }
    }
}

impl<T: BaseFloat> Dim1Solver<T> for BrentSolver<T> {
    fn solve(&self, func: &impl Dim1Func<T>) -> Option<T> {
        let zero = T::from(0_f32);
        let (a, b) = (self.search_range[0], self.search_range[1]);
        let mut xy_pairs = [a, (a + b) * T::from(0.5), b].map(|a| XYPair::new(a, func.eval(a)));
        assert!(
            xy_pairs[0].y.signum() != xy_pairs[2].y.signum(),
            "Illegal initial guess: {:?}",
            xy_pairs
        );

        let mut iter_num: usize = 0;
        while (xy_pairs[2].x - xy_pairs[0].x) > T::from(2.) * self.error_tolerance {
            iter_num += 1;

            // IQI
            let (mut max_error_index, mut max_error) = (0_usize, xy_pairs[0].y.abs());

            for i in 1..=2_usize {
                let error = xy_pairs[i].y.abs();
                if error > max_error {
                    max_error_index = i;
                    max_error = error;
                }
            }

            if max_error_index != 1 {
                let next_x = inverse_quadratic_iterpolate(&xy_pairs);
                let next_y = func.eval(next_x);

                if next_y == zero {
                    log::info!("Brent solve, iteration number = {}", iter_num);
                    return Some(next_x);
                } else if next_y.abs() < max_error {
                    let axis_crossed: bool;
                    let new_interval: T;

                    if max_error_index == 0 && next_x > xy_pairs[1].x {
                        new_interval = xy_pairs[2].x - xy_pairs[1].x;
                        axis_crossed = xy_pairs[1].y.signum() != xy_pairs[2].y.signum();
                    } else if max_error_index == 2 && next_x < xy_pairs[1].x {
                        new_interval = xy_pairs[1].x - xy_pairs[0].x;
                        axis_crossed = xy_pairs[0].y.signum() != xy_pairs[1].y.signum();
                    } else {
                        if max_error_index == 0 {
                            new_interval = xy_pairs[2].x - next_x;
                            axis_crossed = next_y.signum() != xy_pairs[2].y.signum();
                        } else {
                            new_interval = next_x - xy_pairs[0].x;
                            axis_crossed = next_y.signum() != xy_pairs[0].y.signum();
                        }
                    }

                    let old_interval = xy_pairs[2].x - xy_pairs[0].x;

                    if axis_crossed && old_interval > new_interval * T::from(2.) {
                        let new_pair = XYPair::new(next_x, next_y);

                        // reorder
                        if max_error_index == 0 && next_x > xy_pairs[1].x {
                            xy_pairs[0] = xy_pairs[1];
                            xy_pairs[1] = new_pair;
                        } else if max_error_index == 2 && next_x < xy_pairs[1].x {
                            xy_pairs[2] = xy_pairs[1];
                            xy_pairs[1] = new_pair;
                        } else {
                            xy_pairs[max_error_index] = new_pair;
                        }

                        log::info!("Brent Solver, IQI");

                        continue;
                        // } else {
                        //     log::info!("axis_crossed = false")
                    }
                }
            }

            // Method of False Position
            let replaced_index: usize;
            let (next_x, old_interval) = if xy_pairs[0].y.signum() != xy_pairs[1].y.signum() {
                replaced_index = 2;
                (
                    secant_iter(&[xy_pairs[0], xy_pairs[1]]),
                    xy_pairs[1].x - xy_pairs[0].x,
                )
            } else {
                debug_assert!(
                    xy_pairs[1].y.signum() != xy_pairs[2].y.signum(),
                    "y0, y1, y2, y0 sig, y1 sig, y2 sig = {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
                    xy_pairs[0].y,
                    xy_pairs[1].y,
                    xy_pairs[2].y,
                    xy_pairs[0].y.signum(),
                    xy_pairs[1].y.signum(),
                    xy_pairs[2].y.signum()
                );
                replaced_index = 0;
                (
                    secant_iter(&[xy_pairs[1], xy_pairs[2]]),
                    xy_pairs[2].x - xy_pairs[1].x,
                )
            };
            let next_y = func.eval(next_x);
            let new_interval = if replaced_index == 0 {
                if next_y.signum() == xy_pairs[1].y.signum() {
                    xy_pairs[2].x - next_x
                } else {
                    next_x - xy_pairs[1].x
                }
            } else {
                if next_y.signum() == xy_pairs[0].y.signum() {
                    xy_pairs[1].x - next_x
                } else {
                    next_x - xy_pairs[0].x
                }
            };

            if next_y == zero {
                log::info!("Brent solve, iteration number = {}", iter_num);
                return Some(next_x);
            } else {
                if next_y.abs() < xy_pairs[replaced_index].y.abs()
                    && old_interval > T::from(2.) * new_interval
                {
                    log::info!("Brent Solver, Method of False Position");
                    xy_pairs[replaced_index] = xy_pairs[1];
                    xy_pairs[1] = XYPair::new(next_x, next_y);
                } else {
                    // Bisection
                    log::info!("Brent Solver, Bisection");
                    let other_two_x = [1, 2].map(|i| xy_pairs[(replaced_index + i) % 3].x);
                    let next_x = (other_two_x[0] + other_two_x[1]) * T::from(0.5);
                    let next_y = func.eval(next_x);
                    if next_y == zero {
                        log::info!("Brent solve, iteration number = {}", iter_num);
                        return Some(next_y);
                    } else {
                        xy_pairs[replaced_index] = xy_pairs[1];
                        xy_pairs[1] = XYPair::new(next_x, next_y);
                    }
                }
            }
        }

        log::info!("Brent solve, iteration number = {}", iter_num);

        Some(xy_pairs[1].x)
    }
}

fn inverse_quadratic_iterpolate<T: BaseFloat>(xy_pairs: &[XYPair<T>; 3]) -> T {
    let one = T::from(1_f32);
    let (a, fa) = (xy_pairs[0].x, xy_pairs[0].y);
    let (b, fb) = (xy_pairs[1].x, xy_pairs[1].y);
    let (c, fc) = (xy_pairs[2].x, xy_pairs[2].y);

    let q = fa / fb;
    let r = fc / fb;
    let s = fc / fa;
    c - (r * (r - q) * (c - b) + (one - r) * s * (c - a)) / ((q - one) * (r - one) * (s - one))
}

// Method of False Position
fn secant_iter<T: BaseFloat>(xy_pairs: &[XYPair<T>; 2]) -> T {
    let (a, fa) = (xy_pairs[0].x, xy_pairs[0].y);
    let (b, fb) = (xy_pairs[1].x, xy_pairs[1].y);
    // assume b > a
    (b * fa - a * fb) / (fa - fb)
}

#[derive(Clone, Copy, Debug)]
struct XYPair<T> {
    pub x: T,
    pub y: T,
}

impl<T> XYPair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
