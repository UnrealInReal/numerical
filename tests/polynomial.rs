use numerical::{
    polynomial::{Polynomial, PolynomialInnerVec},
    scalar::{basef64, float},
    tensor::vector::Vector,
};

#[test]
fn test_nest_mul_inner_array() {
    let poly = Polynomial::new(
        4,
        Vector::<basef64, [basef64; 5]>::new([
            float(-1.),
            float(5.),
            float(-3.),
            float(3.),
            float(2.),
        ]),
        Some(Vector::<basef64, [basef64; 4]>::new([float(0.); 4])),
    );
    let y = poly.nest_mul(float(0.5));
    assert!(y == float(1.25));
}

#[test]
fn test_nest_mul_inner_array_and_vec() {
    let poly = Polynomial::new(
        4,
        Vector::<basef64, [basef64; 5]>::new([
            float(-1.),
            float(5.),
            float(-3.),
            float(3.),
            float(2.),
        ]),
        Some(Vector::<basef64, Vec<basef64>>::new(vec![
            float(0.),
            float(0.),
            float(0.),
            float(0.),
        ])),
    );
    let y = poly.nest_mul(float(0.5));
    assert!(y == float(1.25));
}

#[test]
fn test_nest_mul_inner_vec_0() {
    let poly = PolynomialInnerVec::new(
        4,
        Vector::new(vec![
            float(-1.),
            float(5.),
            float(-3.),
            float(3.),
            float(2.),
        ]),
        Some(Vector::new(vec![
            float(0.),
            float(0.),
            float(0.),
            float(0.),
        ])),
    );
    let y = poly.nest_mul(float(0.5));
    assert!(y == float(1.25));
}

#[test]
fn test_new_0() {
    let poly = PolynomialInnerVec::from_coefficients(Vector::new(vec![
        float(-1.),
        float(5.),
        float(-3.),
        float(3.),
        float(2.),
    ]));
    let y = poly.nest_mul(float(0.5));
    assert!(y == float(1.25), "y = {y:?}");
}

#[test]
fn test_nest_mul_inner_vec_1() {
    let poly = PolynomialInnerVec::new(
        3,
        Vector::new(vec![float(1.), float(0.5), float(0.5), float(-0.5)]),
        Some(Vector::new(vec![float(0.), float(2.), float(3.)])),
    );
    let y = poly.nest_mul(float(1.));
    assert!(y == float(0.));
}

#[test]
fn test_new_1() {
    let poly = PolynomialInnerVec::from_coefficients_base_points(
        Vector::new(vec![float(1.), float(0.5), float(0.5), float(-0.5)]),
        Vector::new(vec![float(0.), float(2.), float(3.)]),
    );
    let y = poly.nest_mul(float(1.));
    assert!(y == float(0.));
}
