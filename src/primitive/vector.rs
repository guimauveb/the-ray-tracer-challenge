use {
    super::{point::Point, tuple::Tuple},
    crate::approx_eq::ApproxEq,
    std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub},
};

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Tuple for Vector {
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
        0.0
    }
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

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point::new(self.x + rhs.x(), self.y + rhs.y(), self.z + rhs.z())
    }
}

// The resulting Vector represents the change in direction between the two.
impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

/* Used to get the opposite of a Vector.
 * Given a vector pointing from a surface toward a light source, we get the vector that points from the light source to the surface.
 */
impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Vector {
    pub fn magnitude(&self) -> f64 {
        f64::sqrt(
            self.z
                .mul_add(self.z, self.y.mul_add(self.y, self.x.powi(2))),
        )
    }

    pub fn normalize(&self) -> Self {
        *self / self.magnitude()
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        //(self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
        self.z.mul_add(rhs.z, self.x.mul_add(rhs.x, self.y * rhs.y))
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x),
        }
    }
}

#[test]
fn can_create_a_vector() {
    assert_eq!(
        Vector::new(4.0, -4.0, 3.0),
        Vector {
            x: 4.0,
            y: -4.0,
            z: 3.0,
        }
    );
}

#[test]
fn can_access_vector_coordinates() {
    let vector = Vector::new(4.0, -4.0, 3.0);
    assert_eq!(vector.x, 4.0);
    assert_eq!(vector.y, -4.0);
    assert_eq!(vector.z, 3.0);
}

#[test]
fn can_compare_vectors_for_equality() {
    let vector_a = Vector::new(4.0, -4.0, 3.0);
    let vector_b = Vector::new(4.0, -4.0, 3.0);
    assert_eq!(vector_a, vector_b);
}

// We test addition between Vector and Point in point.rs
#[test]
fn can_add_two_vectors() {
    let vector_a = Vector::new(3.0, -2.0, 5.0);
    let vector_b = Vector::new(-2.0, 3.0, 1.0);
    let expected = Vector::new(1.0, 1.0, 6.0);
    assert_eq!(vector_a + vector_b, expected);
}

#[test]
fn can_subtract_a_vector_from_a_vector() {
    let vector_a = Vector::new(3.0, 2.0, 1.0);
    let vector_b = Vector::new(5.0, 6.0, 7.0);
    let expected = Vector::new(-2.0, -4.0, -6.0);
    assert_eq!(vector_a - vector_b, expected);
}

#[test]
fn can_negate_a_vector() {
    let vector = Vector::new(3.0, 2.0, 1.0);
    let expected = Vector::new(-3.0, -2.0, -1.0);
    assert_eq!(-vector, expected);
}

#[test]
fn can_multiply_vector_by_scalar() {
    let vector = Vector::new(1.0, -2.0, 3.0);
    let scalar = 3.5_f64;
    let expected = Vector::new(3.5, -7.0, 10.5);
    assert_eq!(vector * scalar, expected);
}

#[test]
fn can_multiply_scalar_by_vector() {
    let scalar = 3.5_f64;
    let vector = Vector::new(1.0, -2.0, 3.0);
    let expected = Vector::new(3.5, -7.0, 10.5);
    assert_eq!(scalar * vector, expected);
}

#[test]
fn can_divide_vector_by_scalar() {
    let vector = Vector::new(1.0, -2.0, 3.0);
    let scalar = 2_f64;
    let expected = Vector::new(0.5, -1.0, 1.5);
    assert_eq!(vector / scalar, expected);
}

#[test]
fn can_compute_vector_magnitude_1() {
    assert_eq!(Vector::new(0.0, 1.0, 0.0).magnitude(), 1.0)
}
#[test]
fn can_compute_vector_magnitude_2() {
    assert_eq!(Vector::new(0.0, 0.0, 1.0).magnitude(), 1.0)
}
#[test]
fn can_compute_vector_magnitude_3() {
    assert_eq!(Vector::new(1.0, 2.0, 3.0).magnitude(), f64::sqrt(14.0))
}
#[test]
fn can_compute_vector_magnitude_4() {
    assert_eq!(Vector::new(-1.0, -2.0, -3.0).magnitude(), f64::sqrt(14.0))
}

#[test]
fn can_normalize_vector_1() {
    assert_eq!(
        Vector::new(4.0, 0.0, 0.0).normalize(),
        Vector::new(1.0, 0.0, 0.0)
    );
}
#[test]
fn can_normalize_vector_2() {
    assert_eq!(
        Vector::new(1.0, 2.0, 3.0).normalize(),
        // Vector { 1.0/sqrt(14.0), 2.0/sqrt(14.0), 3.0/sqrt(14.0) }
        Vector::new(0.26726, 0.53452, 0.80178)
    );
}
#[test]
fn can_normalize_unit_vector() {
    let vector = Vector::new(1.0, 2.0, 3.0);
    let normalized = vector.normalize();
    assert_eq!(normalized.magnitude(), 1.0)
}

#[test]
fn can_compute_vector_dot_product() {
    let vector_a = Vector::new(1.0, 2.0, 3.0);
    let vector_b = Vector::new(2.0, 3.0, 4.0);
    assert_eq!(vector_a.dot(vector_b), 20.0)
}

#[test]
fn can_compute_vector_cross_product() {
    let vector_a = Vector::new(1.0, 2.0, 3.0);
    let vector_b = Vector::new(2.0, 3.0, 4.0);
    assert_eq!(vector_a.cross(vector_b), Vector::new(-1.0, 2.0, -1.0));
    assert_eq!(vector_b.cross(vector_a), Vector::new(1.0, -2.0, 1.0));
}
