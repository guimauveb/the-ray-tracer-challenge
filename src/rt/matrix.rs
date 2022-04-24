use {
    crate::{
        approx_eq::ApproxEq,
        primitive::{point::Point, vector::Vector},
    },
    std::{
        fmt::{Display, Formatter, Result as FmtResult},
        ops::{Index, IndexMut, Mul},
    },
};

#[derive(Debug, Clone)]
pub struct Matrix<const N: usize>([[f64; N]; N]);

#[derive(Debug)]
pub enum MatrixError<'a, const N: usize> {
    NotInvertible(&'a Matrix<N>),
}

impl<'a, const N: usize> Display for MatrixError<'a, N> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            MatrixError::NotInvertible(matrix) => write!(f, "{} is not invertible", matrix),
        }
    }
}

pub trait Transpose {
    fn transpose(&self) -> Self;
}

// Submatrix can only be a Matrix<N> where N >= 2
pub trait Submatrix<T> {
    fn submatrix(&self, index: Idx) -> T;
}

// Minor can only be computed for Matrix<N> where N >= 3
pub trait Minor {
    fn minor(&self, index: Idx) -> f64;
}

pub trait Cofactor {
    fn cofactor(&self, index: Idx) -> f64;
}

pub trait Determinant {
    fn determinant(&self) -> f64;
}

pub trait Translation {
    fn translation(x: f64, y: f64, z: f64) -> Self;
}

pub trait Scaling {
    fn scaling(x: f64, y: f64, z: f64) -> Self;
}

pub trait Rotation {
    fn rotation_x(radians: f64) -> Self;
    fn rotation_y(radians: f64) -> Self;
    fn rotation_z(radians: f64) -> Self;
}

pub trait Shearing {
    fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self;
}

impl<const N: usize> Matrix<N> {
    pub const fn new(matrix: [[f64; N]; N]) -> Self {
        Self(matrix)
    }
}

// Index Matrix like this: M[[0, 1]]
type Idx = [usize; 2];
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
        self.0.iter().enumerate().all(|(r, row)| {
            row.iter()
                .enumerate()
                .all(|(c, column)| column.approx_eq(rhs[[r, c]]))
        })
    }
}

impl<const N: usize> Display for Matrix<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl<const N: usize> Mul for Matrix<N> {
    type Output = Self;

    // NOTE - Use a mul_add?
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

impl<const N: usize> Transpose for Matrix<N> {
    fn transpose(&self) -> Self {
        let mut result = Self([[0.0; N]; N]);
        for row in 0..N {
            for column in 0..N {
                result[[row, column]] = self[[column, row]];
            }
        }
        result
    }
}

impl<const N: usize> Submatrix<Matrix<{ N - 1 }>> for Matrix<N> {
    fn submatrix(&self, index: Idx) -> Matrix<{ N - 1 }> {
        let mut submatrix = Matrix::<{ N - 1 }>([[0.0; N - 1]; N - 1]);
        let (mut i, mut j) = (0_usize, 0_usize);

        for row in 0..N {
            // Skip excluded row
            if row == index[0] {
                continue;
            }
            for column in 0..N {
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
}

impl Minor for Matrix<3_usize> {
    fn minor(&self, index: Idx) -> f64 {
        self.submatrix(index).determinant()
    }
}

impl Minor for Matrix<4> {
    fn minor(&self, index: Idx) -> f64 {
        self.submatrix(index).determinant()
    }
}

impl Cofactor for Matrix<3_usize> {
    fn cofactor(&self, index: Idx) -> f64 {
        let minor = self.minor(index);

        if (index[0] + index[1]) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }
}

impl Cofactor for Matrix<4> {
    fn cofactor(&self, index: Idx) -> f64 {
        let minor = self.minor(index);

        if (index[0] + index[1]) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }
}

impl Determinant for Matrix<2_usize> {
    fn determinant(&self) -> f64 {
        self[[0, 0]] * self[[1, 1]] - self[[0, 1]] * self[[1, 0]]
    }
}

impl Determinant for Matrix<3_usize> {
    fn determinant(&self) -> f64 {
        (0..3_usize)
            .map(|x| self[[0, x]] * self.cofactor([0, x]))
            .sum()
    }
}

impl Determinant for Matrix<4> {
    fn determinant(&self) -> f64 {
        (0..4).map(|x| self[[0, x]] * self.cofactor([0, x])).sum()
    }
}

impl Matrix<4> {
    pub const fn identity() -> Self {
        Self([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn is_invertible(&self) -> bool {
        !(self.determinant() == 0.0)
    }

    /// If the matrix is invertible, we compute the inverse matrix like the following.
    /// For each element of the matrix:
    ///     1. Compute the cofactor
    ///     2. Divide each cofactor by the determinant of the matrix
    ///     3. Return the resulting matrix
    pub fn inverse(&self) -> Result<Self, MatrixError<4>> {
        if !(self.is_invertible()) {
            Err(MatrixError::NotInvertible(self))
        } else {
            let mut inverse_matrix = Self([[0.0; 4]; 4]);
            for row in 0..4 {
                for column in 0..4 {
                    inverse_matrix[[column, row]] =
                        self.cofactor([row, column]) / self.determinant();
                }
            }
            Ok(inverse_matrix)
        }
    }

    /// Given 3 inputs, `from`, `to` and `up`:
    ///     1. Compute the `forward` vector by subtracting `from` from `to`.
    ///     2. Compute the `left` vector by taking the cross product of `forward` and the normalized `up` vector.
    ///     3. Compute the `true_up` vector by taking the cross product of `left` and `forward`.
    ///     4. With `left`, `true_up` and `forward`, we can now construct a matrix that represents the orientation transformation:
    /// ```
    /// let orientation = Self([
    ///     [left.x, left.y, left.z, 0.0],
    ///     [true_up.x, true_up.y, true_up.z, 0.0],
    ///     [-forward.x, -forward.y, -forward.z, 0.0],
    ///     [0.0, 0.0, 0.0, 1.0],
    /// ]);
    /// ```
    pub fn view_transform(from: Point, to: Point, up: Vector) -> Self {
        let forward = (&to - &from).normalized();
        let normalized_up = up.normalized();
        let left = forward.cross(&normalized_up);
        let true_up = left.cross(&forward);

        let orientation = Self([
            [left.x(), left.y(), left.z(), 0.0],
            [true_up.x(), true_up.y(), true_up.z(), 0.0],
            [-forward.x(), -forward.y(), -forward.z(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        orientation * Self::translation(-from.x(), -from.y(), -from.z())
    }
}

impl Translation for Matrix<4> {
    fn translation(x: f64, y: f64, z: f64) -> Self {
        Self([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

impl Scaling for Matrix<4> {
    fn scaling(x: f64, y: f64, z: f64) -> Self {
        Self([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

impl Mul<Point> for Matrix<4> {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        let mut point = Point::zero();
        // Could map as well but the index (usize) is moved instead of being copied?
        for row in 0..3_usize {
            point[row] = (0..4)
                .map(|column| self[[row, column]] * if column < 3 { rhs[column] } else { 1.0 }) // rhs[3] is (self.w) is equal to 1.0 but not accessible from the Point type.
                .collect::<Vec<f64>>()
                .iter()
                .sum();
        }
        point
    }
}

impl Mul<Point> for &Matrix<4> {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        let mut point = Point::zero();
        // Could map as well but the index (usize) is moved instead of being copied?
        for row in 0..3_usize {
            point[row] = (0..4)
                .map(|column| self[[row, column]] * if column < 3 { rhs[column] } else { 1.0 }) // rhs[3] is (self.w) is equal to 1.0 but not accessible from the Point type.
                .collect::<Vec<f64>>()
                .iter()
                .sum();
        }
        point
    }
}

impl Mul<&Point> for Matrix<4> {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Point {
        let mut point = Point::zero();
        // Could map as well but the index (usize) is moved instead of being copied?
        for row in 0..3_usize {
            point[row] = (0..4)
                .map(|column| self[[row, column]] * if column < 3 { rhs[column] } else { 1.0 }) // rhs[3] is (self.w) is equal to 1.0 but not accessible from the Point type.
                .collect::<Vec<f64>>()
                .iter()
                .sum();
        }
        point
    }
}

impl Mul<&Point> for &Matrix<4> {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Point {
        let mut point = Point::zero();
        // Could map as well but the index (usize) is moved instead of being copied?
        for row in 0..3_usize {
            point[row] = (0..4)
                .map(|column| self[[row, column]] * if column < 3 { rhs[column] } else { 1.0 }) // rhs[3] is (self.w) is equal to 1.0 but not accessible from the Point type.
                .collect::<Vec<f64>>()
                .iter()
                .sum();
        }
        point
    }
}

impl Mul<Vector> for Matrix<4> {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        let mut vec = Vector::zero();
        // Could map as well but the index (usize) is moved instead of being copied?
        for row in 0..3_usize {
            vec[row] = (0..4)
                .map(|column| self[[row, column]] * if column < 3 { rhs[column] } else { 0.0 }) // rhs[3] (self.w) is equal to 0.0 but not accessible from the Vector type.
                .collect::<Vec<f64>>()
                .iter()
                .sum();
        }
        vec
    }
}

impl Mul<Vector> for &Matrix<4> {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        let mut vec = Vector::zero();
        // Could map as well but the index (usize) is moved instead of being copied?
        for row in 0..3_usize {
            vec[row] = (0..4)
                .map(|column| self[[row, column]] * if column < 3 { rhs[column] } else { 0.0 }) // rhs[3] (self.w) is equal to 0.0 but not accessible from the Vector type.
                .collect::<Vec<f64>>()
                .iter()
                .sum();
        }
        vec
    }
}

impl Mul<&Vector> for &Matrix<4> {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Vector {
        let mut vec = Vector::zero();
        // Could map as well but the index (usize) is moved instead of being copied?
        for row in 0..3_usize {
            vec[row] = (0..4)
                .map(|column| self[[row, column]] * if column < 3 { rhs[column] } else { 0.0 }) // rhs[3] (self.w) is equal to 0.0 but not accessible from the Vector type.
                .collect::<Vec<f64>>()
                .iter()
                .sum();
        }
        vec
    }
}

impl Mul<&Vector> for Matrix<4> {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Vector {
        let mut vec = Vector::zero();
        // Could map as well but the index (usize) is moved instead of being copied?
        for row in 0..3_usize {
            vec[row] = (0..4)
                .map(|column| self[[row, column]] * if column < 3 { rhs[column] } else { 0.0 }) // rhs[3] (self.w) is equal to 0.0 but not accessible from the Vector type.
                .collect::<Vec<f64>>()
                .iter()
                .sum();
        }
        vec
    }
}

impl Rotation for Matrix<4> {
    fn rotation_x(radians: f64) -> Self {
        Self([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, radians.cos(), -radians.sin(), 0.0],
            [0.0, radians.sin(), radians.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    fn rotation_y(radians: f64) -> Self {
        Self([
            [radians.cos(), 0.0, radians.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-radians.sin(), 0.0, radians.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    fn rotation_z(radians: f64) -> Self {
        Self([
            [radians.cos(), -radians.sin(), 0.0, 0.0],
            [radians.sin(), radians.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

impl Shearing for Matrix<4> {
    fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Self([
            [1.0, xy, xz, 0.0],
            [yx, 1.0, yz, 0.0],
            [zx, zy, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}
