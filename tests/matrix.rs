use numerical::tensor::matrix::{Matrix, MatrixOps, MatrixShape};

#[test]
fn test_matrix_ops_0() {
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

#[test]
fn test_matrix_ops_1() {
    let inner_0 = vec![1., 2., 3., 4., 5., 6., 7., 8., 9.];
    let inner_1 = vec![1., 2., 3., 4., 5., 6., 7., 8., 9.];
    let inner_2 = vec![1., 2., 3.];
    let inner_3 = vec![30., 36., 42., 66., 81., 96., 102., 126., 150.];

    let mat_0 = Matrix::<f64, Vec<f64>>::new((3, 3).into(), inner_0.clone());
    let mat_1 = Matrix::<f64, Vec<f64>>::new((3, 3).into(), inner_1.clone());
    let mat_2 = Matrix::<f64, Vec<f64>>::new((3, 1).into(), inner_2.clone());
    let mat_3 = Matrix::<f64, Vec<f64>>::new((3, 3).into(), inner_3.clone());

    assert!(mat_0 == mat_1);
    assert!(mat_0.mul(&mat_1) == mat_3);
    assert!(
        mat_0.clone() * mat_2.clone()
            == Matrix::<f64, Vec<f64>>::new((3, 1).into(), vec![30., 66., 102.])
    );

    assert!(
        mat_0.clone() * Matrix::<f64, Vec<f64>>::new((3, 2).into(), vec![1., 2., 4., 5., 7., 8.])
            == Matrix::<f64, Vec<f64>>::new((3, 2).into(), vec![30., 36., 66., 81., 102., 126.])
    );

    assert!(
        mat_0.clone() + mat_1.clone()
            == Matrix::<f64, Vec<f64>>::new(
                (3, 3).into(),
                inner_0.iter().map(|x| 2. * x).collect()
            )
    );

    assert!(
        mat_0.clone().add(&mat_1.clone())
            == Matrix::<f64, Vec<f64>>::new(
                (3, 3).into(),
                inner_0.iter().map(|x| 2. * x).collect()
            )
    );

    assert!(
        mat_3.clone() - mat_0.clone()
            == Matrix::<f64, Vec<f64>>::new(
                (3, 3).into(),
                vec![29., 34., 39., 62., 76., 90., 95., 118., 141.]
            )
    );

    assert!(
        mat_3.clone().sub(&mat_0.clone())
            == Matrix::<f64, Vec<f64>>::new(
                (3, 3).into(),
                vec![29., 34., 39., 62., 76., 90., 95., 118., 141.]
            )
    );
}

#[test]
fn test_matrix_ops_2() {
    let inner_0 = vec![1., 2., 3., 4., 5., 6., 7., 8., 9.];
    let inner_1 = vec![1., 2., 3., 4., 5., 6., 7., 8., 9.];

    let mut mat_0 = Matrix::<f64, Vec<f64>>::new((3, 3).into(), inner_0.clone());
    let mat_1 = Matrix::<f64, Vec<f64>>::new((3, 3).into(), inner_1.clone());

    mat_0 += &mat_1;
    assert!(
        mat_0
            == Matrix::<f64, Vec<f64>>::new(
                (3, 3).into(),
                inner_0.iter().map(|x| 2. * x).collect()
            )
    );

    mat_0 -= &mat_1;
    assert!(mat_0 == mat_1);

    mat_0.add_assign(&mat_1);
    assert!(
        mat_0
            == Matrix::<f64, Vec<f64>>::new(
                (3, 3).into(),
                inner_0.iter().map(|x| 2. * x).collect()
            )
    );

    mat_0.sub_assign(&mat_1);
    assert!(mat_0 == mat_1);
}
