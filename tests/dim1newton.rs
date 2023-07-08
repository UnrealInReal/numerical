use numerical::{
    dim1_solver::{dim1_newton::Dim1NewtonSolver, Dim1Solver},
    polynomial::PolynomialInnerVec,
    tensor::vector::Vector,
};

#[test]
fn test_dim1_newton_0() {
    let _ = env_logger::try_init();
    let g = |x: f64| 1. / x - x / 2.;
    let dg = |x: f64| -1. / (x * x) - 0.5;
    let solver = Dim1NewtonSolver::new(10., 1e-14, 100, &g, &dg);
    let result = solver.solve(&g).unwrap();
    assert!((result - 2.0_f64.sqrt()).abs() < 1e-8, "root = {}", result);
}

#[test]
fn test_dim1_newton_1() {
    let _ = env_logger::try_init();
    let g = PolynomialInnerVec::from_coefficients(Vector::new(vec![-1., 1., 0., 1.]));
    let dg = |x: f64| 3. * (x * x) + 1.;
    let solver = Dim1NewtonSolver::new(10., 1e-14, 100, &g, &dg);
    let result = solver.solve(&g).unwrap();
    assert!((result - 0.68232780).abs() < 1e-8, "root = {}", result);
}

#[test]
fn test_dim1_newton_2() {
    let _ = env_logger::try_init();
    let g = PolynomialInnerVec::from_coefficients(Vector::new(vec![-11. / 4., 0., -6., 0., 4.]));
    let dg = |x: f64| 16. * (x * x * x) - 12. * x;
    let solver = Dim1NewtonSolver::new(0.5, 1e-14, 100, &g, &dg);
    assert!(solver.solve(&g).is_none());
    let solver = Dim1NewtonSolver::new(0.65, 1e-14, 100, &g, &dg);
    assert!(!solver.solve(&g).is_none());
}
