use numerical::{
    dim1_solver::{bisection::BisectionSolver, Dim1Solver},
    polynomial::PolynomialInnerVec,
    scalar::base_float::BaseFloat,
    tensor::vector::Vector,
};

#[test]
fn test_bisection_0() {
    let _ = env_logger::try_init();
    let f = |x: f64| 2. * x + 1.;
    let solver = BisectionSolver::new([-10., 10.], 1e-5);
    let result = solver.solve(&f).unwrap();
    assert!((result + 0.5).abs() < 1e-3, "root = {}", result);
}

#[test]
fn test_bisection_1() {
    let _ = env_logger::try_init();
    let f = PolynomialInnerVec::from_coefficients(Vector::new(vec![2., -3., 1.]));
    let solver_a = BisectionSolver::new([-10., 1.5], 1e-5);
    let solver_b = BisectionSolver::new([1.5, 10.], 1e-5);
    let result_a = solver_a.solve(&f).unwrap();
    let result_b = solver_b.solve(&f).unwrap();
    assert!((result_a - 1.).abs() < 1e-3, "root a = {}", result_a);
    assert!((result_b - 2.).abs() < 1e-3, "root b = {}", result_b);
}

#[test]
fn test_bisection_2() {
    let _ = env_logger::try_init();
    let f = PolynomialInnerVec::from_coefficients(Vector::new(vec![-8. / 27., 4. / 3., -2., 1.]));
    let solver = BisectionSolver::new([0., 2.], 1e-14);
    let result = solver.solve(&f).unwrap();
    // large forward error
    assert!((result - 2. / 3.).abs() > 1e-6, "root b = {}", result);
}
