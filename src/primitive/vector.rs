use {super::tuple::Tuple, crate::approx_eq::ApproxEq, std::ops};

#[derive(Debug)]
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

impl PartialEq for Vector {
    fn eq(&self, rhs: &Self) -> bool {
        self.x.approx_eq(rhs.x) && self.y.approx_eq(rhs.y) && self.z.approx_eq(rhs.z)
    }
}

impl ops::Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
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

#[test]
fn can_add_two_vectors() {
    let vector_a = Vector::new(3.0, -2.0, 5.0);
    let vector_b = Vector::new(-2.0, 3.0, 1.0);
    let expected = Vector::new(1.0, 1.0, 6.0);
    assert_eq!(vector_a + vector_b, expected);
}

// We test addition between Vector and Point in point.rs
