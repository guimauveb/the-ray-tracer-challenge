#[cfg(test)]
use {
    crate::{
        rt::matrix::{Cofactor, Determinant, Matrix, Minor, Submatrix},
        tuple::{point::Point, vector::Vector},
    },
    std::f64::consts::PI,
};

#[test]
fn can_create_4x4_matrix() {
    const M: Matrix<4> = Matrix::<4>::new([
        [1.0, 2.0, 3.0, 4.0],
        [5.5, 6.5, 7.5, 8.5],
        [9.0, 10.0, 11.0, 12.0],
        [13.5, 14.5, 15.5, 16.5],
    ]);

    assert_eq!(M[[0, 3]], 4.0);
    assert_eq!(M[[1, 0]], 5.5);
    assert_eq!(M[[1, 2]], 7.5);
    assert_eq!(M[[2, 2]], 11.0);
    assert_eq!(M[[3, 0]], 13.5);
    assert_eq!(M[[3, 2]], 15.5);
}

#[test]
fn can_create_3x3_matrix() {
    const M: Matrix<3> = Matrix::<3>::new([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);

    assert_eq!(M[[0, 0]], -3.0);
    assert_eq!(M[[1, 1]], -2.0);
    assert_eq!(M[[2, 2]], 1.0);
}

#[test]
fn can_create_2x2_matrix() {
    const M: Matrix<2> = Matrix::<2>::new([[-3.0, 5.0], [1.0, -2.0]]);

    assert_eq!(M[[0, 0]], -3.0);
    assert_eq!(M[[0, 1]], 5.0);
    assert_eq!(M[[1, 0]], 1.0);
    assert_eq!(M[[1, 1]], -2.0);
}

#[test]
fn matrix_equality_with_identical_matrices() {
    const A: Matrix<4> = Matrix::<4>::new([
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [5.0, 4.0, 3.0, 2.0],
    ]);

    const B: Matrix<4> = Matrix::<4>::new([
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [5.0, 4.0, 3.0, 2.0],
    ]);

    assert_eq!(A, B);
}

#[test]
fn matrix_equality_with_different_matrices() {
    const A: Matrix<4> = Matrix::<4>::new([
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [5.0, 4.0, 3.0, 2.0],
    ]);

    const B: Matrix<4> = Matrix::<4>::new([
        [2.0, 3.0, 4.0, 5.0],
        [6.0, 7.0, 8.0, 9.0],
        [8.0, 7.0, 6.0, 5.0],
        [4.0, 3.0, 2.0, 1.0],
    ]);

    assert_ne!(A, B);
}

#[test]
fn can_multiply_matrices() {
    const A: Matrix<4> = Matrix::<4>::new([
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [5.0, 4.0, 3.0, 2.0],
    ]);

    const B: Matrix<4> = Matrix::<4>::new([
        [-2.0, 1.0, 2.0, 3.0],
        [3.0, 2.0, 1.0, -1.0],
        [4.0, 3.0, 6.0, 5.0],
        [1.0, 2.0, 7.0, 8.0],
    ]);

    const C: Matrix<4> = Matrix::<4>::new([
        [20.0, 22.0, 50.0, 48.0],
        [44.0, 54.0, 114.0, 108.0],
        [40.0, 58.0, 110.0, 102.0],
        [16.0, 26.0, 46.0, 42.0],
    ]);

    assert_eq!(A * B, C);
}

#[test]
fn can_multiply_4x4_matrix_and_point() {
    const A: Matrix<4> = Matrix::<4>::new([
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 4.0, 4.0, 2.0],
        [8.0, 6.0, 4.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    let point = Point::new(1.0, 2.0, 3.0);
    let expected = Point::new(18.0, 24.0, 33.0);

    assert_eq!(A * point, expected);
}

// NOTE - The book mentions multiplication between a 4x4 matrix and a Tuple, but only gives a test for a mulitplication by a Point (w = 1.0).
#[test]
fn can_multiply_4x4_matrix_and_vector() {
    const A: Matrix<4> = Matrix::<4>::new([
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 4.0, 4.0, 2.0],
        [8.0, 6.0, 4.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    let vector = Vector::new(1.0, 2.0, 3.0);
    let expected = Vector::new(14.0, 22.0, 32.0);

    assert_eq!(A * vector, expected);
}

#[test]
fn can_multiply_matrix_by_the_identity_matrix() {
    const A: Matrix<4> = Matrix::<4>::new([
        [0.0, 1.0, 2.0, 3.0],
        [1.0, 2.0, 4.0, 8.0],
        [2.0, 4.0, 8.0, 16.0],
        [4.0, 8.0, 16.0, 32.0],
    ]);

    assert_eq!(A * Matrix::<4>::identity(), A);
}

// Point and Vector
#[test]
fn can_multiply_tuples_by_identity_matrix() {
    let point = Point::new(1.0, 2.0, 3.0);
    assert_eq!(&Matrix::<4>::identity() * &point, point);

    let vector = Vector::new(1.0, 2.0, 3.0);
    assert_eq!(&Matrix::<4>::identity() * &vector, vector);
}

#[test]
fn can_transpose_matrices() {
    const A: Matrix<4> = Matrix::<4>::new([
        [0.0, 9.0, 3.0, 0.0],
        [9.0, 8.0, 0.0, 8.0],
        [1.0, 8.0, 5.0, 3.0],
        [0.0, 0.0, 5.0, 8.0],
    ]);

    const B: Matrix<4> = Matrix::<4>::new([
        [0.0, 9.0, 1.0, 0.0],
        [9.0, 8.0, 8.0, 0.0],
        [3.0, 0.0, 5.0, 5.0],
        [0.0, 8.0, 3.0, 8.0],
    ]);

    let transposed_a = A.transpose();

    assert_eq!(transposed_a, B);
}

#[test]
fn can_transpose_identity_matrix() {
    const IDENTITY_MATRIX: Matrix<4> = Matrix::<4>::identity();
    let transposed_identity_matrix = IDENTITY_MATRIX.transpose();

    assert_eq!(IDENTITY_MATRIX, transposed_identity_matrix);
}

#[test]
fn can_compute_determinant() {
    const A: Matrix<2> = Matrix::<2>::new([[1.0, 5.0], [-3.0, 2.0]]);
    let determinant = A.determinant();
    let expected_determinant = 17.0;

    assert_eq!(determinant, expected_determinant);
}

#[test]
fn a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
    const A: Matrix<3> = Matrix::<3>::new([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);
    let submatrix = A.submatrix([0, 2]);
    const EXPECTED_SUBMATRIX: Matrix<2> = Matrix::<2>::new([[-3.0, 2.0], [0.0, 6.0]]);

    assert_eq!(submatrix, EXPECTED_SUBMATRIX);
}

#[test]
fn a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
    const A: Matrix<4> = Matrix::<4>::new([
        [-6.0, 1.0, 1.0, 6.0],
        [-8.0, 5.0, 8.0, 6.0],
        [-1.0, 0.0, 8.0, 2.0],
        [-7.0, 1.0, -1.0, 1.0],
    ]);
    let submatrix = A.submatrix([2, 1]);
    const EXPECTED_SUBMATRIX: Matrix<3> =
        Matrix::<3>::new([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]);

    assert_eq!(submatrix, EXPECTED_SUBMATRIX);
}

#[test]
fn calculating_a_minor_of_a_3x3_matrix() {
    const A: Matrix<3> = Matrix::<3>::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
    let minor = A.minor([1, 0]);
    let expected_minor = 25.0;

    assert_eq!(minor, expected_minor);
}

#[test]
fn calculating_a_cofactor_of_a_3x3_matrix() {
    const A: Matrix<3> = Matrix::<3>::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
    let cofactor_at_0_0 = A.cofactor([0, 0]);
    let expected_cofactor_at_0_0 = -12.0;

    assert_eq!(cofactor_at_0_0, expected_cofactor_at_0_0);

    let cofactor_at_1_0 = A.cofactor([1, 0]);
    let expected_cofactor_at_1_0 = -25.0;

    assert_eq!(cofactor_at_1_0, expected_cofactor_at_1_0);
}

#[test]
fn calculating_the_determinant_of_3x3_matrix() {
    const A: Matrix<3> = Matrix::<3>::new([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);

    assert_eq!(A.cofactor([0, 0]), 56.0);
    assert_eq!(A.cofactor([0, 1]), 12.0);
    assert_eq!(A.cofactor([0, 2]), -46.0);
    assert_eq!(A.determinant(), -196.0);
}

#[test]
fn calculating_the_determinant_of_4x4_matrix() {
    const A: Matrix<4> = Matrix::<4>::new([
        [-2.0, -8.0, 3.0, 5.0],
        [-3.0, 1.0, 7.0, 3.0],
        [1.0, 2.0, -9.0, 6.0],
        [-6.0, 7.0, 7.0, -9.0],
    ]);

    assert_eq!(A.cofactor([0, 0]), 690.0);
    assert_eq!(A.cofactor([0, 1]), 447.0);
    assert_eq!(A.cofactor([0, 2]), 210.0);
    assert_eq!(A.determinant(), -4071.0);
}

#[test]
fn testing_a_invertible_matrix_for_invertibility() {
    const A: Matrix<4> = Matrix::<4>::new([
        [6.0, 4.0, 4.0, 4.0],
        [5.0, 5.0, 7.0, 6.0],
        [4.0, -9.0, 3.0, 7.0],
        [9.0, 1.0, 7.0, -6.0],
    ]);

    assert_eq!(A.is_invertible(), true);
}

#[test]
fn testing_a_noninvertible_matrix_for_invertibility() {
    const A: Matrix<4> = Matrix::<4>::new([
        [-4.0, 2.0, -2.0, -3.0],
        [9.0, 6.0, 2.0, 6.0],
        [0.0, -5.0, 1.0, -5.0],
        [0.0, 0.0, 0.0, 0.0],
    ]);

    assert_eq!(A.is_invertible(), false);
}

#[test]
fn calculating_the_inverse_of_a_matrix() {
    const A: Matrix<4> = Matrix::<4>::new([
        [-5.0, 2.0, 6.0, -8.0],
        [1.0, -5.0, 1.0, 8.0],
        [7.0, 7.0, -6.0, -7.0],
        [1.0, -3.0, 7.0, 4.0],
    ]);

    let inverse = A.inverse().expect("Matrix is not invertible!");

    assert_eq!(A.determinant(), 532.0);
    assert_eq!(A.cofactor([2, 3]), -160.0);
    assert_eq!(inverse[[3, 2]], -160.0 / 532.0);
    assert_eq!(A.cofactor([3, 2]), 105.0);
    assert_eq!(inverse[[2, 3]], 105.0 / 532.0);

    const EXPECTED_INVERSE: Matrix<4> = Matrix::<4>::new([
        [0.21805, 0.45113, 0.24060, -0.04511],
        [-0.80827, -1.45677, -0.44361, 0.52068],
        [-0.07895, -0.22368, -0.05263, 0.19737],
        [-0.52256, -0.81391, -0.30075, 0.30639],
    ]);

    assert_eq!(inverse, EXPECTED_INVERSE);
}

#[test]
fn calculating_the_inverse_of_another_matrix() {
    const A: Matrix<4> = Matrix::<4>::new([
        [8.0, -5.0, 9.0, 2.0],
        [7.0, 5.0, 6.0, 1.0],
        [-6.0, 0.0, 9.0, 6.0],
        [-3.0, 0.0, -9.0, -4.0],
    ]);

    let inverse = A.inverse().expect("Matrix is not invertible!");

    const EXPECTED_INVERSE: Matrix<4> = Matrix::<4>::new([
        [-0.15385, -0.15385, -0.28205, -0.53846],
        [-0.07692, 0.12308, 0.02564, 0.03077],
        [0.35897, 0.35897, 0.43590, 0.92308],
        [-0.69231, -0.69231, -0.76923, -1.92308],
    ]);

    assert_eq!(inverse, EXPECTED_INVERSE);
}

#[test]
fn calculating_the_inverse_of_a_third_matrix() {
    const A: Matrix<4> = Matrix::<4>::new([
        [9.0, 3.0, 0.0, 9.0],
        [-5.0, -2.0, -6.0, -3.0],
        [-4.0, 9.0, 6.0, 4.0],
        [-7.0, 6.0, 6.0, 2.0],
    ]);

    let inverse = A.inverse().expect("Matrix is not invertible!");

    const EXPECTED_INVERSE: Matrix<4> = Matrix::<4>::new([
        [-0.04074, -0.07778, 0.14444, -0.22222],
        [-0.07778, 0.03333, 0.36667, -0.33333],
        [-0.02901, -0.14630, -0.10926, 0.12963],
        [0.17778, 0.06667, -0.26667, 0.33333],
    ]);

    assert_eq!(inverse, EXPECTED_INVERSE);
}

#[test]
fn multiplying_a_product_by_its_inverse() {
    const A: Matrix<4> = Matrix::<4>::new([
        [3.0, -9.0, 7.0, 3.0],
        [3.0, -8.0, 2.0, -9.0],
        [-4.0, 4.0, 4.0, 1.0],
        [-6.0, 5.0, -1.0, 1.0],
    ]);
    const B: Matrix<4> = Matrix::<4>::new([
        [8.0, 2.0, 2.0, 2.0],
        [3.0, -1.0, 7.0, 0.0],
        [7.0, 0.0, 5.0, 4.0],
        [6.0, -2.0, 0.0, 5.0],
    ]);

    let c = A * B;
    let b_inverse = B.inverse().expect("Matrix is not invertible!");

    assert_eq!(c * b_inverse, A);
}

#[test]
fn multiplying_by_a_translation_matrix() {
    let transform = Matrix::<4>::translation(5.0, -3.0, 2.0);
    let point = Point::new(-3.0, 4.0, 5.0);
    let expected_point = Point::new(2.0, 1.0, 7.0);

    assert_eq!(transform * point, expected_point);
}

#[test]
fn multiplying_by_the_inverse_of_translation_matrix() {
    let transform = Matrix::<4>::translation(5.0, -3.0, 2.0);
    let inverse = transform.inverse().expect("Matrix is not invertible!");
    let point = Point::new(-3.0, 4.0, 5.0);
    let expected_point = Point::new(-8.0, 7.0, 3.0);

    assert_eq!(inverse * point, expected_point);
}

#[test]
fn translation_does_not_affect_vectors() {
    let transform = Matrix::<4>::translation(5.0, -3.0, 2.0);
    let vector = Vector::new(-3.0, 4.0, 5.0);
    assert_eq!(&transform * &vector, vector);
}

#[test]
fn scaling_matrix_applied_to_a_point() {
    let transform = Matrix::<4>::scaling(2.0, 3.0, 4.0);
    let point = Point::new(-4.0, 6.0, 8.0);
    let expected_point = Point::new(-8.0, 18.0, 32.0);

    assert_eq!(transform * point, expected_point);
}

#[test]
fn scaling_matrix_applied_to_a_vector() {
    let transform = Matrix::<4>::scaling(2.0, 3.0, 4.0);
    let vector = Vector::new(-4.0, 6.0, 8.0);
    let expected_vector = Vector::new(-8.0, 18.0, 32.0);

    assert_eq!(transform * vector, expected_vector);
}

#[test]
fn reflection_by_a_negative_value() {
    let transform = Matrix::<4>::scaling(-1.0, 1.0, 1.0);
    let point = Point::new(2.0, 3.0, 4.0);
    let expected_point = Point::new(-2.0, 3.0, 4.0);

    assert_eq!(transform * point, expected_point);
}

#[test]
fn rotating_a_point_around_the_x_axis() {
    let point = Point::new(0.0, 1.0, 0.0);
    let half_quarter = Matrix::<4>::rotation_x(PI / 4.0);
    let full_quarter = Matrix::<4>::rotation_x(PI / 2.0);

    assert_eq!(
        half_quarter * &point,
        Point::new(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
    );
    assert_eq!(full_quarter * &point, Point::new(0.0, 0.0, 1.0));
}

#[test]
fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direciton() {
    let point = Point::new(0.0, 1.0, 0.0);
    let half_quarter = Matrix::<4>::rotation_x(PI / 4.0);
    let inverse = half_quarter.inverse().expect("Matrix is not invertible!");

    assert_eq!(
        inverse * point,
        Point::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0)
    );
}

#[test]
fn rotating_a_point_around_the_y_axis() {
    let point = Point::new(0.0, 0.0, 1.0);
    let half_quarter = Matrix::<4>::rotation_y(PI / 4.0);
    let full_quarter = Matrix::<4>::rotation_y(PI / 2.0);

    assert_eq!(
        half_quarter * &point,
        Point::new(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0)
    );
    assert_eq!(full_quarter * point, Point::new(1.0, 0.0, 0.0));
}

#[test]
fn rotating_a_point_around_the_z_axis() {
    let point = Point::new(0.0, 1.0, 0.0);
    let half_quarter = Matrix::<4>::rotation_z(PI / 4.0);
    let full_quarter = Matrix::<4>::rotation_z(PI / 2.0);

    assert_eq!(
        half_quarter * &point,
        Point::new(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0)
    );
    assert_eq!(full_quarter * &point, Point::new(-1.0, 0.0, 0.0));
}

#[test]
fn a_shearing_transformation_moves_x_in_proportion_to_y() {
    let transform = Matrix::<4>::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let point = Point::new(2.0, 3.0, 4.0);
    assert_eq!(transform * point, Point::new(5.0, 3.0, 4.0));
}

#[test]
fn a_shearing_transformation_moves_x_in_proportion_to_z() {
    let transform = Matrix::<4>::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
    let point = Point::new(2.0, 3.0, 4.0);
    assert_eq!(transform * point, Point::new(6.0, 3.0, 4.0));
}

#[test]
fn a_shearing_transformation_moves_y_in_proportion_to_x() {
    let transform = Matrix::<4>::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
    let point = Point::new(2.0, 3.0, 4.0);
    assert_eq!(transform * point, Point::new(2.0, 5.0, 4.0));
}

#[test]
fn a_shearing_transformation_moves_y_in_proportion_to_z() {
    let transform = Matrix::<4>::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    let point = Point::new(2.0, 3.0, 4.0);
    assert_eq!(transform * point, Point::new(2.0, 7.0, 4.0));
}

#[test]
fn a_shearing_transformation_moves_z_in_proportion_to_x() {
    let transform = Matrix::<4>::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    let point = Point::new(2.0, 3.0, 4.0);
    assert_eq!(transform * point, Point::new(2.0, 3.0, 6.0));
}

#[test]
fn a_shearing_transformation_moves_z_in_proportion_to_y() {
    let transform = Matrix::<4>::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    let point = Point::new(2.0, 3.0, 4.0);
    assert_eq!(transform * point, Point::new(2.0, 3.0, 7.0));
}

#[test]
fn individual_transformations_are_applied_in_sequance() {
    let p = Point::new(1.0, 0.0, 1.0);
    let a = Matrix::<4>::rotation_x(PI / 2.0);
    let b = Matrix::<4>::scaling(5.0, 5.0, 5.0);
    let c = Matrix::<4>::translation(10.0, 5.0, 7.0);

    // Apply rotation first
    let p2 = a * p;
    assert_eq!(p2, Point::new(1.0, -1.0, 0.0));

    // Then apply scaling
    let p3 = b * p2;
    assert_eq!(p3, Point::new(5.0, -5.0, 0.0));

    // Then apply translation
    let p4 = c * p3;
    assert_eq!(p4, Point::new(15.0, 0.0, 7.0));
}

#[test]
fn the_transformation_matrix_for_the_default_orientation() {
    let from = Point::new(0.0, 0.0, 0.0);
    let to = Point::new(0.0, 0.0, -1.0);
    let up = Vector::new(0.0, 1.0, 0.0);
    let t = Matrix::<4>::view_transform(&from, &to, &up);
    let expected_t = Matrix::<4>::identity();
    assert_eq!(t, expected_t);
}

#[test]
fn a_view_transformation_matrix_looking_in_positive_z_direction() {
    let from = Point::new(0.0, 0.0, 0.0);
    let to = Point::new(0.0, 0.0, 1.0);
    let up = Vector::new(0.0, 1.0, 0.0);
    let t = Matrix::<4>::view_transform(&from, &to, &up);
    let expected_t = Matrix::<4>::scaling(-1.0, 1.0, -1.0);
    assert_eq!(t, expected_t);
}

#[test]
fn the_view_transformation_moves_the_world() {
    let from = Point::new(0.0, 0.0, 8.0);
    let to = Point::new(0.0, 0.0, 0.0);
    let up = Vector::new(0.0, 1.0, 0.0);
    let t = Matrix::<4>::view_transform(&from, &to, &up);
    let expected_t = Matrix::<4>::translation(0.0, 0.0, -8.0);
    assert_eq!(t, expected_t);
}

#[test]
fn an_arbitrary_view_transformation() {
    let from = Point::new(1.0, 3.0, 2.0);
    let to = Point::new(4.0, -2.0, 8.0);
    let up = Vector::new(1.0, 1.0, 0.0);
    let t = Matrix::<4>::view_transform(&from, &to, &up);
    let expected_t = Matrix::<4>::new([
        [-0.50709, 0.50709, 0.67612, -2.36643],
        [0.76772, 0.60609, 0.12122, -2.82843],
        [-0.35857, 0.59761, -0.71714, 0.0000],
        [0.00000, 0.00000, 0.00000, 1.00000],
    ]);
    assert_eq!(t, expected_t);
}
