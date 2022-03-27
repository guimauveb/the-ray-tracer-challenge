use {
    super::{tuple::Tuple, vector::Vector},
    crate::approx_eq::ApproxEq,
    std::ops::{Add, Div, Index, IndexMut, Mul, Sub},
};

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Tuple for Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
    fn z(&self) -> f64 {
        self.z
    }
    fn w(&self) -> f64 {
        1.0
    }
}

impl Index<usize> for Point {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bound!"),
        }
    }
}

impl IndexMut<usize> for Point {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
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

    fn add(self, rhs: Vector) -> Self {
        Self {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
            z: self.z + rhs.z(),
        }
    }
}

impl Add<Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Point {
        Point {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
            z: self.z + rhs.z(),
        }
    }
}

// The resulting Vector is the Vector pointing from p2 to p1 (rhs to self).
impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Vector {
        Vector::new(self.x - rhs.x(), self.y - rhs.y(), self.z - rhs.z())
    }
}

impl Sub for &Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Vector {
        Vector::new(self.x - rhs.x(), self.y - rhs.y(), self.z - rhs.z())
    }
}

// Conceptually, it's 'moving backwards' by the given Vector.
impl Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, rhs: Vector) -> Self {
        Self {
            x: self.x - rhs.x(),
            y: self.y - rhs.y(),
            z: self.z - rhs.z(),
        }
    }
}

impl Sub<Vector> for &Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Point {
        Point {
            x: self.x - rhs.x(),
            y: self.y - rhs.y(),
            z: self.z - rhs.z(),
        }
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<f64> for &Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Point {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        Point {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<&Point> for f64 {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Point {
        Point {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div<f64> for Point {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<f64> for &Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Point {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
