#[cfg(test)]
use crate::tuple::{point::Point, vector::Vector};

#[test]
fn can_create_a_point() {
    let point_a = Point::new(4.3, -4.2, 3.1);
    let point_b = Point::new(4.3, -4.2, 3.1);

    assert_eq!(point_a, point_b);
}

#[test]
fn can_access_point_coordinates() {
    let point = Point::new(4.3, -4.2, 3.1);
    assert_eq!(point.x(), 4.3);
    assert_eq!(point.y(), -4.2);
    assert_eq!(point.z(), 3.1);
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

#[test]
fn can_multiply_point_by_scalar() {
    let point = Point::new(1.0, -2.0, 3.0);
    let scalar = 3.5_f32;
    let expected = Point::new(3.5, -7.0, 10.5);
    assert_eq!(point * scalar, expected);
}

#[test]
fn can_multiply_scalar_by_point() {
    let scalar = 3.5_f32;
    let point = Point::new(1.0, -2.0, 3.0);
    let expected = Point::new(3.5, -7.0, 10.5);
    assert_eq!(scalar * point, expected);
}

#[test]
fn can_divide_point_by_scalar() {
    let point = Point::new(1.0, -2.0, 3.0);
    let scalar = 2_f32;
    let expected = Point::new(0.5, -1.0, 1.5);
    assert_eq!(point / scalar, expected);
}
