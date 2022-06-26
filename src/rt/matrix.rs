use {
    crate::{
        approx_eq::ApproxEq,
        tuple::{point::Point, vector::Vector},
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

impl<const N: usize> Display for MatrixError<'_, N> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            MatrixError::NotInvertible(matrix) => write!(f, "{} is not invertible", matrix),
        }
    }
}

pub trait Transform {
    fn transform(&self, m: &Matrix<4>) -> Self;
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

impl<const N: usize> Matrix<N> {
    pub const fn new(matrix: [[f64; N]; N]) -> Self {
        Self(matrix)
    }
}

type Idx = [usize; 2];
impl<const N: usize> Index<Idx> for Matrix<{ N }> {
    type Output = f64;
    /// Indexes Matrix like this: `matrix[[0, 1]]`
    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index[0]][index[1]]
    }
}

impl<const N: usize> IndexMut<Idx> for Matrix<{ N }> {
    /// Indexes Matrix like this: `matrix[[0, 1]]`
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
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:#?}", self)
    }
}

impl Mul for Matrix<4> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self([
            [
                self[[0, 0]].mul_add(
                    rhs[[0, 0]],
                    self[[0, 1]].mul_add(
                        rhs[[1, 0]],
                        self[[0, 2]].mul_add(rhs[[2, 0]], self[[0, 3]] * rhs[[3, 0]]),
                    ),
                ),
                self[[0, 0]].mul_add(
                    rhs[[0, 1]],
                    self[[0, 1]].mul_add(
                        rhs[[1, 1]],
                        self[[0, 2]].mul_add(rhs[[2, 1]], self[[0, 3]] * rhs[[3, 1]]),
                    ),
                ),
                self[[0, 0]].mul_add(
                    rhs[[0, 2]],
                    self[[0, 1]].mul_add(
                        rhs[[1, 2]],
                        self[[0, 2]].mul_add(rhs[[2, 2]], self[[0, 3]] * rhs[[3, 2]]),
                    ),
                ),
                self[[0, 0]].mul_add(
                    rhs[[0, 3]],
                    self[[0, 1]].mul_add(
                        rhs[[1, 3]],
                        self[[0, 2]].mul_add(rhs[[2, 3]], self[[0, 3]] * rhs[[3, 3]]),
                    ),
                ),
            ],
            [
                self[[1, 0]].mul_add(
                    rhs[[0, 0]],
                    self[[1, 1]].mul_add(
                        rhs[[1, 0]],
                        self[[1, 2]].mul_add(rhs[[2, 0]], self[[1, 3]] * rhs[[3, 0]]),
                    ),
                ),
                self[[1, 0]].mul_add(
                    rhs[[0, 1]],
                    self[[1, 1]].mul_add(
                        rhs[[1, 1]],
                        self[[1, 2]].mul_add(rhs[[2, 1]], self[[1, 3]] * rhs[[3, 1]]),
                    ),
                ),
                self[[1, 0]].mul_add(
                    rhs[[0, 2]],
                    self[[1, 1]].mul_add(
                        rhs[[1, 2]],
                        self[[1, 2]].mul_add(rhs[[2, 2]], self[[1, 3]] * rhs[[3, 2]]),
                    ),
                ),
                self[[1, 0]].mul_add(
                    rhs[[0, 3]],
                    self[[1, 1]].mul_add(
                        rhs[[1, 3]],
                        self[[1, 2]].mul_add(rhs[[2, 3]], self[[1, 3]] * rhs[[3, 3]]),
                    ),
                ),
            ],
            [
                self[[2, 0]].mul_add(
                    rhs[[0, 0]],
                    self[[2, 1]].mul_add(
                        rhs[[1, 0]],
                        self[[2, 2]].mul_add(rhs[[2, 0]], self[[2, 3]] * rhs[[3, 0]]),
                    ),
                ),
                self[[2, 0]].mul_add(
                    rhs[[0, 1]],
                    self[[2, 1]].mul_add(
                        rhs[[1, 1]],
                        self[[2, 2]].mul_add(rhs[[2, 1]], self[[2, 3]] * rhs[[3, 1]]),
                    ),
                ),
                self[[2, 0]].mul_add(
                    rhs[[0, 2]],
                    self[[2, 1]].mul_add(
                        rhs[[1, 2]],
                        self[[2, 2]].mul_add(rhs[[2, 2]], self[[2, 3]] * rhs[[3, 2]]),
                    ),
                ),
                self[[2, 0]].mul_add(
                    rhs[[0, 3]],
                    self[[2, 1]].mul_add(
                        rhs[[1, 3]],
                        self[[2, 2]].mul_add(rhs[[2, 3]], self[[2, 3]] * rhs[[3, 3]]),
                    ),
                ),
            ],
            [
                self[[3, 0]].mul_add(
                    rhs[[0, 0]],
                    self[[3, 1]].mul_add(
                        rhs[[1, 0]],
                        self[[3, 2]].mul_add(rhs[[2, 0]], self[[3, 3]] * rhs[[3, 0]]),
                    ),
                ),
                self[[3, 0]].mul_add(
                    rhs[[0, 1]],
                    self[[3, 1]].mul_add(
                        rhs[[1, 1]],
                        self[[3, 2]].mul_add(rhs[[2, 1]], self[[3, 3]] * rhs[[3, 1]]),
                    ),
                ),
                self[[3, 0]].mul_add(
                    rhs[[0, 2]],
                    self[[3, 1]].mul_add(
                        rhs[[1, 2]],
                        self[[3, 2]].mul_add(rhs[[2, 2]], self[[3, 3]] * rhs[[3, 2]]),
                    ),
                ),
                self[[3, 0]].mul_add(
                    rhs[[0, 3]],
                    self[[3, 1]].mul_add(
                        rhs[[1, 3]],
                        self[[3, 2]].mul_add(rhs[[2, 3]], self[[3, 3]] * rhs[[3, 3]]),
                    ),
                ),
            ],
        ])
    }
}

impl<const N: usize> Submatrix<Matrix<{ N - 1 }>> for Matrix<N> {
    fn submatrix(&self, index: Idx) -> Matrix<{ N - 1 }> {
        let mut submatrix = Matrix::<{ N - 1 }>([[0.0; N - 1]; N - 1]);
        let (mut i, mut j) = (0, 0);

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

impl Minor for Matrix<3> {
    fn minor(&self, index: Idx) -> f64 {
        self.submatrix(index).determinant()
    }
}

impl Minor for Matrix<4> {
    fn minor(&self, index: Idx) -> f64 {
        self.submatrix(index).determinant()
    }
}

impl Cofactor for Matrix<3> {
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

impl Determinant for Matrix<2> {
    fn determinant(&self) -> f64 {
        self[[0, 0]] * self[[1, 1]] - self[[0, 1]] * self[[1, 0]]
    }
}

impl Determinant for Matrix<3> {
    fn determinant(&self) -> f64 {
        (0..3).map(|x| self[[0, x]] * self.cofactor([0, x])).sum()
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
        !(self.determinant().approx_eq(0.0))
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
            Ok(Self([
                [
                    self.cofactor([0, 0]) / self.determinant(),
                    self.cofactor([1, 0]) / self.determinant(),
                    self.cofactor([2, 0]) / self.determinant(),
                    self.cofactor([3, 0]) / self.determinant(),
                ],
                [
                    self.cofactor([0, 1]) / self.determinant(),
                    self.cofactor([1, 1]) / self.determinant(),
                    self.cofactor([2, 1]) / self.determinant(),
                    self.cofactor([3, 1]) / self.determinant(),
                ],
                [
                    self.cofactor([0, 2]) / self.determinant(),
                    self.cofactor([1, 2]) / self.determinant(),
                    self.cofactor([2, 2]) / self.determinant(),
                    self.cofactor([3, 2]) / self.determinant(),
                ],
                [
                    self.cofactor([0, 3]) / self.determinant(),
                    self.cofactor([1, 3]) / self.determinant(),
                    self.cofactor([2, 3]) / self.determinant(),
                    self.cofactor([3, 3]) / self.determinant(),
                ],
            ]))
        }
    }

    pub const fn translation(x: f64, y: f64, z: f64) -> Self {
        Self([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub const fn scaling(x: f64, y: f64, z: f64) -> Self {
        Self([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_x(radians: f64) -> Self {
        Self([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, radians.cos(), -radians.sin(), 0.0],
            [0.0, radians.sin(), radians.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_y(radians: f64) -> Self {
        Self([
            [radians.cos(), 0.0, radians.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-radians.sin(), 0.0, radians.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_z(radians: f64) -> Self {
        Self([
            [radians.cos(), -radians.sin(), 0.0, 0.0],
            [radians.sin(), radians.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub const fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Self([
            [1.0, xy, xz, 0.0],
            [yx, 1.0, yz, 0.0],
            [zx, zy, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn transpose(&self) -> Self {
        Self([
            [self[[0, 0]], self[[1, 0]], self[[2, 0]], self[[3, 0]]],
            [self[[0, 1]], self[[1, 1]], self[[2, 1]], self[[3, 1]]],
            [self[[0, 2]], self[[1, 2]], self[[2, 2]], self[[3, 2]]],
            [self[[0, 3]], self[[1, 3]], self[[2, 3]], self[[3, 3]]],
        ])
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
    pub fn view_transform(from: &Point, to: &Point, up: &Vector) -> Self {
        let forward = (to - from).normalized();
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

impl Mul<Point> for Matrix<4> {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        Point::new(
            self[[0, 0]].mul_add(
                rhs[0],
                self[[0, 1]].mul_add(rhs[1], self[[0, 2]].mul_add(rhs[2], self[[0, 3]] * rhs[3])),
            ),
            self[[1, 0]].mul_add(
                rhs[0],
                self[[1, 1]].mul_add(rhs[1], self[[1, 2]].mul_add(rhs[2], self[[1, 3]] * rhs[3])),
            ),
            self[[2, 0]].mul_add(
                rhs[0],
                self[[2, 1]].mul_add(rhs[1], self[[2, 2]].mul_add(rhs[2], self[[2, 3]] * rhs[3])),
            ),
        )
    }
}

impl Mul<Point> for &Matrix<4> {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        Point::new(
            self[[0, 0]].mul_add(
                rhs[0],
                self[[0, 1]].mul_add(rhs[1], self[[0, 2]].mul_add(rhs[2], self[[0, 3]] * rhs[3])),
            ),
            self[[1, 0]].mul_add(
                rhs[0],
                self[[1, 1]].mul_add(rhs[1], self[[1, 2]].mul_add(rhs[2], self[[1, 3]] * rhs[3])),
            ),
            self[[2, 0]].mul_add(
                rhs[0],
                self[[2, 1]].mul_add(rhs[1], self[[2, 2]].mul_add(rhs[2], self[[2, 3]] * rhs[3])),
            ),
        )
    }
}

impl Mul<&Point> for Matrix<4> {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Point {
        Point::new(
            self[[0, 0]].mul_add(
                rhs[0],
                self[[0, 1]].mul_add(rhs[1], self[[0, 2]].mul_add(rhs[2], self[[0, 3]] * rhs[3])),
            ),
            self[[1, 0]].mul_add(
                rhs[0],
                self[[1, 1]].mul_add(rhs[1], self[[1, 2]].mul_add(rhs[2], self[[1, 3]] * rhs[3])),
            ),
            self[[2, 0]].mul_add(
                rhs[0],
                self[[2, 1]].mul_add(rhs[1], self[[2, 2]].mul_add(rhs[2], self[[2, 3]] * rhs[3])),
            ),
        )
    }
}

impl Mul<&Point> for &Matrix<4> {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Point {
        Point::new(
            self[[0, 0]].mul_add(
                rhs[0],
                self[[0, 1]].mul_add(rhs[1], self[[0, 2]].mul_add(rhs[2], self[[0, 3]] * rhs[3])),
            ),
            self[[1, 0]].mul_add(
                rhs[0],
                self[[1, 1]].mul_add(rhs[1], self[[1, 2]].mul_add(rhs[2], self[[1, 3]] * rhs[3])),
            ),
            self[[2, 0]].mul_add(
                rhs[0],
                self[[2, 1]].mul_add(rhs[1], self[[2, 2]].mul_add(rhs[2], self[[2, 3]] * rhs[3])),
            ),
        )
    }
}

impl Mul<Vector> for Matrix<4> {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector::new(
            self[[0, 0]].mul_add(
                rhs[0],
                self[[0, 1]].mul_add(rhs[1], self[[0, 2]].mul_add(rhs[2], self[[0, 3]] * rhs[3])),
            ),
            self[[1, 0]].mul_add(
                rhs[0],
                self[[1, 1]].mul_add(rhs[1], self[[1, 2]].mul_add(rhs[2], self[[1, 3]] * rhs[3])),
            ),
            self[[2, 0]].mul_add(
                rhs[0],
                self[[2, 1]].mul_add(rhs[1], self[[2, 2]].mul_add(rhs[2], self[[2, 3]] * rhs[3])),
            ),
        )
    }
}

impl Mul<Vector> for &Matrix<4> {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector::new(
            self[[0, 0]].mul_add(
                rhs[0],
                self[[0, 1]].mul_add(rhs[1], self[[0, 2]].mul_add(rhs[2], self[[0, 3]] * rhs[3])),
            ),
            self[[1, 0]].mul_add(
                rhs[0],
                self[[1, 1]].mul_add(rhs[1], self[[1, 2]].mul_add(rhs[2], self[[1, 3]] * rhs[3])),
            ),
            self[[2, 0]].mul_add(
                rhs[0],
                self[[2, 1]].mul_add(rhs[1], self[[2, 2]].mul_add(rhs[2], self[[2, 3]] * rhs[3])),
            ),
        )
    }
}

impl Mul<&Vector> for &Matrix<4> {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Vector {
        Vector::new(
            self[[0, 0]].mul_add(
                rhs[0],
                self[[0, 1]].mul_add(rhs[1], self[[0, 2]].mul_add(rhs[2], self[[0, 3]] * rhs[3])),
            ),
            self[[1, 0]].mul_add(
                rhs[0],
                self[[1, 1]].mul_add(rhs[1], self[[1, 2]].mul_add(rhs[2], self[[1, 3]] * rhs[3])),
            ),
            self[[2, 0]].mul_add(
                rhs[0],
                self[[2, 1]].mul_add(rhs[1], self[[2, 2]].mul_add(rhs[2], self[[2, 3]] * rhs[3])),
            ),
        )
    }
}

impl Mul<&Vector> for Matrix<4> {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Vector {
        Vector::new(
            self[[0, 0]].mul_add(
                rhs[0],
                self[[0, 1]].mul_add(rhs[1], self[[0, 2]].mul_add(rhs[2], self[[0, 3]] * rhs[3])),
            ),
            self[[1, 0]].mul_add(
                rhs[0],
                self[[1, 1]].mul_add(rhs[1], self[[1, 2]].mul_add(rhs[2], self[[1, 3]] * rhs[3])),
            ),
            self[[2, 0]].mul_add(
                rhs[0],
                self[[2, 1]].mul_add(rhs[1], self[[2, 2]].mul_add(rhs[2], self[[2, 3]] * rhs[3])),
            ),
        )
    }
}
