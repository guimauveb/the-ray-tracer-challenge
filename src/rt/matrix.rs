use {
    crate::approx_eq::ApproxEq,
    std::{
        fmt::{Display, Formatter, Result},
        ops::Index,
    },
};

#[derive(Debug)]
pub struct Matrix<const N: usize>([[f64; N]; N]);

// Index Matrix like this: M[[0, 1]]
impl<const N: usize> Index<[usize; 2]> for Matrix<{ N }> {
    type Output = f64;
    fn index(&self, index: [usize; 2]) -> &f64 {
        &self.0[index[0]][index[1]]
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
