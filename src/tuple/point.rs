use {
    super::vector::Vector,
    crate::approx_eq::ApproxEq,
    std::ops::{Add, Div, Index, IndexMut, Mul, Sub},
};

#[derive(Debug, Clone)]
pub struct Point {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Index<usize> for Point {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index out of bound!"),
        }
    }
}

impl IndexMut<usize> for Point {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Index out of bound!"),
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, rhs: &Self) -> bool {
        self.x.approx_eq(rhs.x) && self.y.approx_eq(rhs.y) && self.z.approx_eq(rhs.z)
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
            z: self.z + rhs.z(),
            w: 1.0,
        }
    }
}

impl Add<Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
            z: self.z + rhs.z(),
            w: 1.0,
        }
    }
}

impl Add<&Vector> for Point {
    type Output = Self;

    fn add(self, rhs: &Vector) -> Self::Output {
        Self {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
            z: self.z + rhs.z(),
            w: 1.0,
        }
    }
}

impl Add<&Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: &Vector) -> Self::Output {
        Point {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
            z: self.z + rhs.z(),
            w: 1.0,
        }
    }
}

impl Sub for Point {
    type Output = Vector;

    /// The resulting Vector is the Vector pointing from p2 to p1 (`rhs` to `self`).
    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x(), self.y - rhs.y(), self.z - rhs.z())
    }
}

impl Sub for &Point {
    type Output = Vector;

    /// The resulting Vector is the Vector pointing from p2 to p1 (`rhs` to `self`).
    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x(), self.y - rhs.y(), self.z - rhs.z())
    }
}

impl Sub<Vector> for Point {
    type Output = Self;

    /// Conceptually, it's "moving backwards" by the given Vector.
    fn sub(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x - rhs.x(),
            y: self.y - rhs.y(),
            z: self.z - rhs.z(),
            w: 1.0,
        }
    }
}

impl Sub<Vector> for &Point {
    type Output = Point;

    /// Conceptually, it's "moving backwards" by the given Vector.
    fn sub(self, rhs: Vector) -> Self::Output {
        Point {
            x: self.x - rhs.x(),
            y: self.y - rhs.y(),
            z: self.z - rhs.z(),
            w: 1.0,
        }
    }
}

impl Mul<f32> for Point {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: 1.0,
        }
    }
}

impl Mul<f32> for &Point {
    type Output = Point;

    fn mul(self, rhs: f32) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: 1.0,
        }
    }
}

impl Mul<Point> for f32 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            w: 1.0,
        }
    }
}

impl Mul<&Point> for f32 {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        Point {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            w: 1.0,
        }
    }
}

impl Div<f32> for Point {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: 1.0,
        }
    }
}

impl Div<f32> for &Point {
    type Output = Point;

    fn div(self, rhs: f32) -> Self::Output {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: 1.0,
        }
    }
}

impl Default for Point {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }
}

impl Point {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    pub const fn x(&self) -> f32 {
        self.x
    }
    pub const fn y(&self) -> f32 {
        self.y
    }
    pub const fn z(&self) -> f32 {
        self.z
    }
    pub const fn w(&self) -> f32 {
        self.w
    }
}
