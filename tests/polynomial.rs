use numerical::{
    dim1_func::Dim1Func,
    polynomial::{Polynomial, PolynomialInnerVec},
    tensor::vector::Vector,
};

#[test]
fn test_nest_mul_inner_array() {
    let poly = Polynomial::new(
        4,
        Vector::<f64, [f64; 5]>::new([-1., 5., -3., 3., 2.]),
        Some(Vector::<f64, [f64; 4]>::new([0.; 4])),
    );
    let y = poly.nest_mul(0.5);
    assert!(y == 1.25);
}

#[test]
fn test_nest_mul_inner_array_and_vec() {
    let poly = Polynomial::new(
        4,
        Vector::<f64, [f64; 5]>::new([-1., 5., -3., 3., 2.]),
        Some(Vector::<f64, Vec<f64>>::new(vec![0., 0., 0., 0.])),
    );
    let y = poly.nest_mul(0.5);
    assert!(y == 1.25);
}

#[test]
fn test_nest_mul_inner_vec_0() {
    let poly = PolynomialInnerVec::new(
        4,
        Vector::new(vec![-1., 5., -3., 3., 2.]),
        Some(Vector::new(vec![0., 0., 0., 0.])),
    );
    let y = poly.nest_mul(0.5);
    assert!(y == 1.25);
}

#[test]
fn test_new_0() {
    let poly = PolynomialInnerVec::from_coefficients(Vector::new(vec![-1., 5., -3., 3., 2.]));
    let y = poly.nest_mul(0.5);
    assert!(y == 1.25, "y = {y:?}");
}

#[test]
fn test_nest_mul_inner_vec_1() {
    let poly = PolynomialInnerVec::new(
        3,
        Vector::new(vec![1., 0.5, 0.5, -0.5]),
        Some(Vector::new(vec![0., 2., 3.])),
    );
    let y = poly.nest_mul(1.);
    assert!(y == 0.);
}

#[test]
fn test_new_1() {
    let poly = PolynomialInnerVec::from_coefficients_base_points(
        Vector::new(vec![1., 0.5, 0.5, -0.5]),
        Vector::new(vec![0., 2., 3.]),
    );
    let y = poly.nest_mul(1.);
    assert!(y == 0.);
}

#[test]
fn test_eval_0() {
    let poly = PolynomialInnerVec::from_coefficients_base_points(
        Vector::new(vec![1., 0.5, 0.5, -0.5]),
        Vector::new(vec![0., 2., 3.]),
    );
    let y = poly.eval(1.);
    assert!(y == 0.);
}
