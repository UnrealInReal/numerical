use numerical::tensor::matrix::{Matrix, Shape};

#[test]
fn test_matrix_ops() {
    let inner = vec![1., 2., 3., 4., 5., 6., 7., 8., 9.];
    let mut mat = Matrix::<f64, Vec<f64>>::new((3, 3).into(), inner);
    assert!(mat.shape() == (3, 3).into());
    assert!(mat.row_size() == 3);
    assert!(mat.col_size() == 3);
    assert!(mat[(1, 0).into()] == 4.);
    assert!(mat[(2, 1).into()] == 8.);
    mat[(1, 0).into()] = -4.;
    assert!(mat[(1, 0).into()] == -4.);
}
