use {
    crate::{
        approx_eq::ApproxEq,
        primitive::{point::Point, tuple::Tuple, vector::Vector},
    },
    std::{
        fmt::{Display, Formatter, Result},
        ops::{Index, IndexMut, Mul},
    },
};

#[derive(Debug)]
pub struct Matrix<const N: usize>([[f64; N]; N]);

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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:#?}", self.0)
    }
}

impl<const N: usize> Mul for Matrix<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result = Self([[0.0; N]; N]);
        for r in 0..N {
            for c in 0..N {
                result[[r, c]] = (0..N)
                    .map(|n| self[[r, n]] * rhs[[n, c]])
                    .collect::<Vec<f64>>()
                    .iter()
                    .sum();
            }
        }
        result
    }
}

impl Mul<Point> for Matrix<4_usize> {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        let mut point = Point::zero();
        // Could map as well but the index (usize) is moved instead of being copied?
        for r in 0..(4_usize - 1) {
            point[r] = (0..4_usize)
                .map(|c| self[[r, c]] * if c < 3 { rhs[c] } else { 1.0 }) // rhs[3] is (self.w) is equal to 1.0 but not accessible from the Point type.
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
        for r in 0..(4_usize - 1) {
            vec[r] = (0..4_usize)
                .map(|c| self[[r, c]] * if c < 3 { rhs[c] } else { 0.0 }) // rhs[3] (self.w) is equal to 0.0 but not accessible from the Vector type.
                .collect::<Vec<f64>>()
                .iter()
                .sum();
        }
        vec
    }
}

#[allow(dead_code)]
impl Matrix<4_usize> {
    const fn identity() -> Self {
        Matrix::<4_usize>([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

// TODO - Make it const?
#[allow(dead_code)]
impl<const N: usize> Matrix<N> {
    fn transpose(&self) -> Self {
        let mut result = Self([[0.0; N]; N]);
        for r in 0..N {
            for c in 0..N {
                result[[r, c]] = self[[c, r]];
            }
        }
        result
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

// The book mentions multiplication between a 4x4 matrix and a Tuple, but only gives a test for a mulitplication by a Point (w = 1.0).
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
