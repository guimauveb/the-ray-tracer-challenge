use {
    super::{tuple::Tuple, vector::Vector},
    crate::approx_eq::ApproxEq,
    std::ops,
};

#[derive(Debug)]
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

impl PartialEq for Point {
    fn eq(&self, rhs: &Self) -> bool {
        self.x.approx_eq(rhs.x) && self.y.approx_eq(rhs.y) && self.z.approx_eq(rhs.z)
    }
}

impl ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Point {
        Self {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
            z: self.z + rhs.z(),
        }
    }
}

// The resulting Vector is the Vector pointing from p2 to p1 (rhs to self).
impl ops::Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Vector {
        Vector::new(self.x - rhs.x(), self.y - rhs.y(), self.z - rhs.z())
    }
}

// Conceptually, it's 'moving backwards' by the given Vector.
impl ops::Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Point {
        Self {
            x: self.x - rhs.x(),
            y: self.y - rhs.y(),
            z: self.z - rhs.z(),
        }
    }
}

#[test]
fn can_create_a_point() {
    assert_eq!(
        Point::new(4.3, -4.2, 3.1),
        Point {
            x: 4.3,
            y: -4.2,
            z: 3.1,
        }
    );
}

#[test]
fn can_access_point_coordinates() {
    let point = Point::new(4.3, -4.2, 3.1);
    assert_eq!(point.x, 4.3);
    assert_eq!(point.y, -4.2);
    assert_eq!(point.z, 3.1);
}

#[test]
fn can_compare_points_for_equality() {
    let point_a = Point::new(4.0, -4.0, 3.0);
    let point_b = Point::new(4.0, -4.0, 3.0);
    assert_eq!(point_a, point_b);
}

#[test]
fn can_add_a_vector_to_a_point() {
    let vector = Vector::new(3.0, -2.0, 5.0);
    let point = Point::new(-2.0, 3.0, 1.0);
    let expected = Point::new(1.0, 1.0, 6.0);
    assert_eq!(vector + point, expected);
}

#[test]
fn can_add_a_point_to_a_vector() {
    let point = Point::new(-2.0, 3.0, 1.0);
    let vector = Vector::new(3.0, -2.0, 5.0);
    let expected = Point::new(1.0, 1.0, 6.0);
    assert_eq!(point + vector, expected);
}

#[test]
fn can_subtract_a_point_from_point() {
    let point_a = Point::new(3.0, 2.0, 1.0);
    let point_b = Point::new(5.0, 6.0, 7.0);
    let expected = Vector::new(-2.0, -4.0, -6.0);
    assert_eq!(point_a - point_b, expected);
}

#[test]
fn can_subtract_a_vector_from_point() {
    let point = Point::new(3.0, 2.0, 1.0);
    let vector = Vector::new(5.0, 6.0, 7.0);
    let expected = Point::new(-2.0, -4.0, -6.0);
    assert_eq!(point - vector, expected);
}
