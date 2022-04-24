use {
    super::point::Point,
    crate::approx_eq::ApproxEq,
    std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub},
};

#[derive(Debug, Clone)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

type Idx = usize;
impl Index<Idx> for Vector {
    type Output = f64;

    fn index(&self, index: Idx) -> &f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bound!"),
        }
    }
}

impl IndexMut<Idx> for Vector {
    fn index_mut(&mut self, index: Idx) -> &mut f64 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bound!"),
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, rhs: &Self) -> bool {
        self.x.approx_eq(rhs.x) && self.y.approx_eq(rhs.y) && self.z.approx_eq(rhs.z)
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add for &Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<Vector> for &Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<&Self> for Vector {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.x + rhs.x(), self.y + rhs.y(), self.z + rhs.z())
    }
}

impl Add<Point> for &Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.x + rhs.x(), self.y + rhs.y(), self.z + rhs.z())
    }
}

impl Add<&Point> for &Vector {
    type Output = Point;

    fn add(self, rhs: &Self::Output) -> Self::Output {
        Point::new(self.x + rhs.x(), self.y + rhs.y(), self.z + rhs.z())
    }
}

// The resulting Vector represents the change in direction between the two.
impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub for &Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Vector {
    type Output = Self;

    /// Used to get the opposite of a Vector.
    /// Example: Given a vector pointing from a surface toward a light source, we get the vector that points from the light source to the surface.
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for &Vector {
    type Output = Vector;

    /// Used to get the opposite of a Vector.
    /// Example: Given a vector pointing from a surface toward a light source, we get the vector that points from the light source to the surface.
    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<f64> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<&Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Self::Output {
        Vector {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<f64> for &Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Vector {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub const fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub const fn x(&self) -> f64 {
        self.x
    }

    pub const fn y(&self) -> f64 {
        self.y
    }

    pub const fn z(&self) -> f64 {
        self.z
    }

    pub const fn w(&self) -> f64 {
        0.0
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(
            self.z
                .mul_add(self.z, self.y.mul_add(self.y, self.x.powi(2))),
        )
    }

    /// Normalization is the process of taking an arbitrary vector and converting it into a unit vector.
    /// It will keep our calculations anchored relative to a common scale (the unit vector).
    /// If we were to skip normalizing ray vectors or surface normals, the calculations would be scaled differently
    /// for every casted ray and scenes would look terrible (if they rendered at all).
    pub fn normalized(&self) -> Self {
        self / self.magnitude()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        //(self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
        self.z.mul_add(rhs.z, self.x.mul_add(rhs.x, self.y * rhs.y))
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x),
        }
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        // Good explaination: https://www.youtube.com/watch?v=naaeH1qbjdQ
        self - &(normal * 2.0 * self.dot(normal))
    }
}
