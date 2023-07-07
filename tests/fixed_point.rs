use numerical::dim1_solver::{fixed_point::FixedPointSolver, Dim1Solver};

#[test]
fn test_fixed_point_0() {
    let _ = env_logger::try_init();
    let g = |x: f64| 1. / x - x / 2.;
    let solver = FixedPointSolver::new(10., 1e-14, 100);
    let result = solver.solve(&g).unwrap();
    assert!((result - 2.0_f64.sqrt()).abs() < 1e-3, "root = {}", result);
}

#[test]
fn test_fixed_point_1() {
    let _ = env_logger::try_init();
    let g = |x: f64| x.cos() - x;
    let solver = FixedPointSolver::new(10., 1e-14, 100);
    let result = solver.solve(&g).unwrap();
    assert!(g(result).abs() < 1e-10, "root = {}", result);
}

#[test]
fn test_fixed_point_2() {
    let _ = env_logger::try_init();
    let g = |x: f64| 2.5 * x - 2.5;
    let solver = FixedPointSolver::new(10.3, 1e-14, 100);
    let result = solver.solve(&g);
    assert!(result.is_none());
}
