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
