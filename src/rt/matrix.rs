use {
    crate::{
        approx_eq::ApproxEq,
        primitive::{point::Point, tuple::Tuple, vector::Vector},
    },
    std::{
        fmt::{Display, Formatter},
        ops::{Index, IndexMut, Mul},
    },
};

#[derive(Debug)]
pub struct Matrix<const N: usize>(pub [[f64; N]; N]);

type Idx = [usize; 2];

// Index Matrix like this: M[[0, 1]]
impl<const N: usize> Index<Idx> for Matrix<{ N }> {
    type Output = f64;
    fn index(&self, index: Idx) -> &f64 {
        &self.0[index[0]][index[1]]
    }
}

impl<const N: usize> IndexMut<Idx> for Matrix<{ N }> {
    fn index_mut(&mut self, index: Idx) -> &mut f64 {
        &mut self.0[index[0]][index[1]]
    }
}

impl<const N: usize> PartialEq for Matrix<N> {
    fn eq(&self, rhs: &Self) -> bool {
        self.0
            .iter()
            .enumerate()
            .all(|(i, x)| x.iter().enumerate().all(|(j, y)| y.approx_eq(rhs[[i, j]])))
    }
}

impl<const N: usize> Display for Matrix<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl<const N: usize> Mul for Matrix<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result = Self([[0.0; N]; N]);
        for row in 0..N {
            for column in 0..N {
                result[[row, column]] = (0..N)
                    .map(|n| self[[row, n]] * rhs[[n, column]])
                    .collect::<Vec<f64>>()
                    .iter()
                    .sum();
            }
        }
        result
    }
}

#[derive(Debug)]
pub enum MatrixError<'a, const N: usize> {
    NotInvertible(&'a Matrix<N>),
}

impl<'a, const N: usize> Display for MatrixError<'a, N> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            MatrixError::NotInvertible(matrix) => write!(f, "{} is not invertible", matrix),
        }
    }
}

/* NOTE - Defining determinant, minor and cofactor for Matrix<N> unfortunatly currently
 * doesn't work with the generic type Matrix<{ N-1 }> returned by submatrix(). Therefore I have to
 * implement these methods for Matrix<2_usize>, Matrix<3_usize> and Matrix<4_usize>. BTW all these
 * methods could also certainly be defined as const methods.*/

// Matrix<N>
#[allow(dead_code)]
impl<const N: usize> Matrix<N> {
    pub fn transpose(&self) -> Self {
        let mut result = Self([[0.0; N]; N]);
        for row in 0..N {
            for column in 0..N {
                result[[row, column]] = self[[column, row]];
            }
        }
        result
    }
}

// Matrix<2_usize>
#[allow(dead_code)]
impl Matrix<2_usize> {
    pub fn determinant(&self) -> f64 {
        self[[0, 0]] * self[[1, 1]] - self[[0, 1]] * self[[1, 0]]
    }
}

// Matrix<3_usize>
#[allow(dead_code)]
impl Matrix<3_usize> {
    pub fn submatrix(&self, index: Idx) -> Matrix<2_usize> {
        let mut submatrix = Matrix::<2_usize>([[0.0; 2_usize]; 2_usize]);
        let (mut i, mut j) = (0_usize, 0_usize);

        for row in 0..3_usize {
            // Skip excluded row
            if row == index[0] {
                continue;
            }
            for column in 0..3_usize {
                // Skip excluded column
                if column == index[1] {
                    continue;
                }
                submatrix[[i, j]] = self[[row, column]];
                j += 1;
            }
            // Reset submatrix column index
            j = 0;
            // Increment submatrix row index
            i += 1;
        }
        submatrix
    }

    pub fn minor(&self, index: Idx) -> f64 {
        let submatrix = self.submatrix(index);
        submatrix.determinant()
    }

    pub fn cofactor(&self, index: Idx) -> f64 {
        let minor = self.minor(index);

        // If column + row is odd, the cofactor is equal to the minor negated. Else it's equal to the minor itself.
        if (index[0] + index[1]) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    pub fn determinant(&self) -> f64 {
        (0..3_usize)
            .map(|x| self[[0, x]] * self.cofactor([0, x]))
            .sum()
    }
}

// Matrix<4_usize>
#[allow(dead_code)]
impl Matrix<4_usize> {
    pub const fn identity() -> Self {
        Matrix::<4_usize>([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn submatrix(&self, index: Idx) -> Matrix<3_usize> {
        let mut submatrix = Matrix::<3_usize>([[0.0; 3_usize]; 3_usize]);
        let (mut i, mut j) = (0_usize, 0_usize);

        for row in 0..4_usize {
            // Skip excluded row
            if row == index[0] {
                continue;
            }
            // Iterate over columns
            for column in 0..4_usize {
                // Skip excluded column
                if column == index[1] {
                    continue;
                }
                submatrix[[i, j]] = self[[row, column]];
                j += 1;
            }
            // Reset submatrix column index
            j = 0;
            // Increment submatrix row index
            i += 1;
        }
        submatrix
    }

    pub fn minor(&self, index: Idx) -> f64 {
        let submatrix = self.submatrix(index);
        submatrix.determinant()
    }

    pub fn cofactor(&self, index: Idx) -> f64 {
        let minor = self.minor(index);

        // If column + row is odd, the cofactor is equal to the minor negated. Else it's equal to the minor itself.
        if (index[0] + index[1]) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    pub fn determinant(&self) -> f64 {
        (0..4_usize)
            .map(|x| self[[0, x]] * self.cofactor([0, x]))
            .sum()
    }

    pub fn is_invertible(&self) -> bool {
        !(self.determinant() == 0.0)
    }

    /* If the matrix is invertible, we compute the inverse matrix like the following:
     *      - Compute the cofactor of every element of the matrix
     *      - Divide each cofactor by the determinant of the matrix
     *      - Return the resulting matrix
     */
    pub fn inverse(&self) -> Result<Self, MatrixError<4_usize>> {
        if !(self.is_invertible()) {
            Err(MatrixError::NotInvertible(self))
        } else {
            let mut inverse_matrix = Matrix::<4_usize>([[0.0; 4_usize]; 4_usize]);
            for row in 0..4_usize {
                for column in 0..4_usize {
                    inverse_matrix[[column, row]] =
                        self.cofactor([row, column]) / self.determinant();
                }
            }
            Ok(inverse_matrix)
        }
    }
}

impl Mul<Point> for Matrix<4_usize> {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        let mut point = Point::zero();
        // Could map as well but the index (usize) is moved instead of being copied?
        for row in 0..(4_usize - 1) {
            point[row] = (0..4_usize)
                .map(|column| self[[row, column]] * if column < 3 { rhs[column] } else { 1.0 }) // rhs[3] is (self.w) is equal to 1.0 but not accessible from the Point type.
                .collect::<Vec<f64>>()
                .iter()
                .sum();
        }
        point
    }
}

// Might not be needed
impl Mul<Vector> for Matrix<4_usize> {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        let mut vec = Vector::zero();
        // Could map as well but the index (usize) is moved instead of being copied?
        for row in 0..(4_usize - 1) {
            vec[row] = (0..4_usize)
                .map(|column| self[[row, column]] * if column < 3 { rhs[column] } else { 0.0 }) // rhs[3] (self.w) is equal to 0.0 but not accessible from the Vector type.
                .collect::<Vec<f64>>()
                .iter()
                .sum();
        }
        vec
    }
}

#[test]
fn can_create_4x4_matrix() {
    const M: Matrix<4_usize> = Matrix::<4_usize>([
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
    const M: Matrix<3_usize> =
        Matrix::<3_usize>([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);

    assert_eq!(M[[0, 0]], -3.0);
    assert_eq!(M[[1, 1]], -2.0);
    assert_eq!(M[[2, 2]], 1.0);
}

#[test]
fn can_create_2x2_matrix() {
    const M: Matrix<2_usize> = Matrix::<2_usize>([[-3.0, 5.0], [1.0, -2.0]]);

    assert_eq!(M[[0, 0]], -3.0);
    assert_eq!(M[[0, 1]], 5.0);
    assert_eq!(M[[1, 0]], 1.0);
    assert_eq!(M[[1, 1]], -2.0);
}

#[test]
fn matrix_equality_with_identical_matrices() {
    const A: Matrix<4_usize> = Matrix::<4_usize>([
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [5.0, 4.0, 3.0, 2.0],
    ]);

    const B: Matrix<4_usize> = Matrix::<4_usize>([
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [5.0, 4.0, 3.0, 2.0],
    ]);

    assert_eq!(A, B);
}

#[test]
fn matrix_equality_with_different_matrices() {
    const A: Matrix<4_usize> = Matrix::<4_usize>([
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [5.0, 4.0, 3.0, 2.0],
    ]);

    const B: Matrix<4_usize> = Matrix::<4_usize>([
        [2.0, 3.0, 4.0, 5.0],
        [6.0, 7.0, 8.0, 9.0],
        [8.0, 7.0, 6.0, 5.0],
        [4.0, 3.0, 2.0, 1.0],
    ]);

    assert_ne!(A, B);
}

#[test]
fn can_multiply_matrices() {
    const A: Matrix<4_usize> = Matrix::<4_usize>([
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [5.0, 4.0, 3.0, 2.0],
    ]);

    const B: Matrix<4_usize> = Matrix::<4_usize>([
        [-2.0, 1.0, 2.0, 3.0],
        [3.0, 2.0, 1.0, -1.0],
        [4.0, 3.0, 6.0, 5.0],
        [1.0, 2.0, 7.0, 8.0],
    ]);

    const C: Matrix<4_usize> = Matrix::<4_usize>([
        [20.0, 22.0, 50.0, 48.0],
        [44.0, 54.0, 114.0, 108.0],
        [40.0, 58.0, 110.0, 102.0],
        [16.0, 26.0, 46.0, 42.0],
    ]);

    assert_eq!(A * B, C);
}

#[test]
fn can_multiply_4x4_matrix_and_point() {
    const A: Matrix<4_usize> = Matrix::<4_usize>([
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
    const A: Matrix<4_usize> = Matrix::<4_usize>([
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
    const A: Matrix<4_usize> = Matrix::<4_usize>([
        [0.0, 1.0, 2.0, 3.0],
        [1.0, 2.0, 4.0, 8.0],
        [2.0, 4.0, 8.0, 16.0],
        [4.0, 8.0, 16.0, 32.0],
    ]);

    assert_eq!(A * Matrix::<4_usize>::identity(), A);
}

// Point and Vector
#[test]
fn can_multiply_tuples_by_identity_matrix() {
    let point = Point::new(1.0, 2.0, 3.0);
    assert_eq!(Matrix::<4_usize>::identity() * point, point);

    let vector = Vector::new(1.0, 2.0, 3.0);
    assert_eq!(Matrix::<4_usize>::identity() * vector, vector);
}

#[test]
fn can_transpose_matrices() {
    const A: Matrix<4_usize> = Matrix::<4_usize>([
        [0.0, 9.0, 3.0, 0.0],
        [9.0, 8.0, 0.0, 8.0],
        [1.0, 8.0, 5.0, 3.0],
        [0.0, 0.0, 5.0, 8.0],
    ]);

    const B: Matrix<4_usize> = Matrix::<4_usize>([
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
    const IDENTITY_MATRIX: Matrix<4_usize> = Matrix::<4_usize>::identity();
    let transposed_identity_matrix = IDENTITY_MATRIX.transpose();

    assert_eq!(IDENTITY_MATRIX, transposed_identity_matrix);
}

#[test]
fn can_compute_determinant() {
    const A: Matrix<2_usize> = Matrix::<2_usize>([[1.0, 5.0], [-3.0, 2.0]]);
    let determinant = A.determinant();
    let expected_determinant = 17.0;

    assert_eq!(determinant, expected_determinant);
}

#[test]
fn a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
    const A: Matrix<3_usize> =
        Matrix::<3_usize>([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);
    let submatrix = A.submatrix([0, 2]);
    const EXPECTED_SUBMATRIX: Matrix<2_usize> = Matrix::<2_usize>([[-3.0, 2.0], [0.0, 6.0]]);

    assert_eq!(submatrix, EXPECTED_SUBMATRIX);
}

#[test]
fn a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
    const A: Matrix<4_usize> = Matrix::<4_usize>([
        [-6.0, 1.0, 1.0, 6.0],
        [-8.0, 5.0, 8.0, 6.0],
        [-1.0, 0.0, 8.0, 2.0],
        [-7.0, 1.0, -1.0, 1.0],
    ]);
    let submatrix = A.submatrix([2, 1]);
    const EXPECTED_SUBMATRIX: Matrix<3_usize> =
        Matrix::<3_usize>([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]);

    assert_eq!(submatrix, EXPECTED_SUBMATRIX);
}

#[test]
fn calculating_a_minor_of_a_3x3_matrix() {
    const A: Matrix<3_usize> =
        Matrix::<3_usize>([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
    let minor = A.minor([1, 0]);
    let expected_minor = 25.0;

    assert_eq!(minor, expected_minor);
}

#[test]
fn calculating_a_cofactor_of_a_3x3_matrix() {
    const A: Matrix<3_usize> =
        Matrix::<3_usize>([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
    let cofactor_at_0_0 = A.cofactor([0, 0]);
    let expected_cofactor_at_0_0 = -12.0;

    assert_eq!(cofactor_at_0_0, expected_cofactor_at_0_0);

    let cofactor_at_1_0 = A.cofactor([1, 0]);
    let expected_cofactor_at_1_0 = -25.0;

    assert_eq!(cofactor_at_1_0, expected_cofactor_at_1_0);
}

#[test]
fn calculating_the_determinant_of_3x3_matrix() {
    const A: Matrix<3_usize> =
        Matrix::<3_usize>([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);

    assert_eq!(A.cofactor([0, 0]), 56.0);
    assert_eq!(A.cofactor([0, 1]), 12.0);
    assert_eq!(A.cofactor([0, 2]), -46.0);
    assert_eq!(A.determinant(), -196.0);
}

#[test]
fn calculating_the_determinant_of_4x4_matrix() {
    const A: Matrix<4_usize> = Matrix::<4_usize>([
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
    const A: Matrix<4_usize> = Matrix::<4_usize>([
        [6.0, 4.0, 4.0, 4.0],
        [5.0, 5.0, 7.0, 6.0],
        [4.0, -9.0, 3.0, 7.0],
        [9.0, 1.0, 7.0, -6.0],
    ]);

    assert_eq!(A.is_invertible(), true);
}

#[test]
fn testing_a_noninvertible_matrix_for_invertibility() {
    const A: Matrix<4_usize> = Matrix::<4_usize>([
        [-4.0, 2.0, -2.0, -3.0],
        [9.0, 6.0, 2.0, 6.0],
        [0.0, -5.0, 1.0, -5.0],
        [0.0, 0.0, 0.0, 0.0],
    ]);

    assert_eq!(A.is_invertible(), false);
}

#[test]
fn calculating_the_inverse_of_a_matrix() {
    const A: Matrix<4_usize> = Matrix::<4_usize>([
        [-5.0, 2.0, 6.0, -8.0],
        [1.0, -5.0, 1.0, 8.0],
        [7.0, 7.0, -6.0, -7.0],
        [1.0, -3.0, 7.0, 4.0],
    ]);

    let inverse = A.inverse().unwrap_or_else(|err| panic!("{}", err));

    assert_eq!(A.determinant(), 532.0);
    assert_eq!(A.cofactor([2, 3]), -160.0);
    assert_eq!(inverse[[3, 2]], -160.0 / 532.0);
    assert_eq!(A.cofactor([3, 2]), 105.0);
    assert_eq!(inverse[[2, 3]], 105.0 / 532.0);

    const EXPECTED_INVERSE: Matrix<4_usize> = Matrix::<4_usize>([
        [0.21805, 0.45113, 0.24060, -0.04511],
        [-0.80827, -1.45677, -0.44361, 0.52068],
        [-0.07895, -0.22368, -0.05263, 0.19737],
        [-0.52256, -0.81391, -0.30075, 0.30639],
    ]);

    assert_eq!(inverse, EXPECTED_INVERSE);
}

#[test]
fn calculating_the_inverse_of_another_matrix() {
    const A: Matrix<4_usize> = Matrix::<4_usize>([
        [8.0, -5.0, 9.0, 2.0],
        [7.0, 5.0, 6.0, 1.0],
        [-6.0, 0.0, 9.0, 6.0],
        [-3.0, 0.0, -9.0, -4.0],
    ]);

    let inverse = A.inverse().unwrap_or_else(|err| panic!("{}", err));

    const EXPECTED_INVERSE: Matrix<4_usize> = Matrix::<4_usize>([
        [-0.15385, -0.15385, -0.28205, -0.53846],
        [-0.07692, 0.12308, 0.02564, 0.03077],
        [0.35897, 0.35897, 0.43590, 0.92308],
        [-0.69231, -0.69231, -0.76923, -1.92308],
    ]);

    assert_eq!(inverse, EXPECTED_INVERSE);
}

#[test]
fn calculating_the_inverse_of_a_third_matrix() {
    const A: Matrix<4_usize> = Matrix::<4_usize>([
        [9.0, 3.0, 0.0, 9.0],
        [-5.0, -2.0, -6.0, -3.0],
        [-4.0, 9.0, 6.0, 4.0],
        [-7.0, 6.0, 6.0, 2.0],
    ]);

    let inverse = A.inverse().unwrap_or_else(|err| panic!("{}", err));

    const EXPECTED_INVERSE: Matrix<4_usize> = Matrix::<4_usize>([
        [-0.04074, -0.07778, 0.14444, -0.22222],
        [-0.07778, 0.03333, 0.36667, -0.33333],
        [-0.02901, -0.14630, -0.10926, 0.12963],
        [0.17778, 0.06667, -0.26667, 0.33333],
    ]);

    assert_eq!(inverse, EXPECTED_INVERSE);
}

#[test]
fn multiplying_a_product_by_its_inverse() {
    const A: Matrix<4_usize> = Matrix::<4_usize>([
        [3.0, -9.0, 7.0, 3.0],
        [3.0, -8.0, 2.0, -9.0],
        [-4.0, 4.0, 4.0, 1.0],
        [-6.0, 5.0, -1.0, 1.0],
    ]);
    const B: Matrix<4_usize> = Matrix::<4_usize>([
        [8.0, 2.0, 2.0, 2.0],
        [3.0, -1.0, 7.0, 0.0],
        [7.0, 0.0, 5.0, 4.0],
        [6.0, -2.0, 0.0, 5.0],
    ]);

    let c = A * B;
    let b_inverse = B.inverse().unwrap_or_else(|err| panic!("{}", err));

    assert_eq!(c * b_inverse, A);
}
