#[cfg(test)]
use crate::primitive::vector::Vector;

#[test]
fn can_create_a_vector() {
    let vector_a = Vector::new(4.0, -4.0, 3.0);
    let vector_b = Vector::new(4.0, -4.0, 3.0);

    assert_eq!(vector_a, vector_b);
}

#[test]
fn can_access_vector_coordinates() {
    let vector = Vector::new(4.0, -4.0, 3.0);
    assert_eq!(vector.x(), 4.0);
    assert_eq!(vector.y(), -4.0);
    assert_eq!(vector.z(), 3.0);
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
        Vector::new(4.0, 0.0, 0.0).normalized(),
        Vector::new(1.0, 0.0, 0.0)
    );
}
#[test]
fn can_normalize_vector_2() {
    assert_eq!(
        Vector::new(1.0, 2.0, 3.0).normalized(),
        // Vector { 1.0/sqrt(14.0), 2.0/sqrt(14.0), 3.0/sqrt(14.0) }
        Vector::new(0.26726, 0.53452, 0.80178)
    );
}
#[test]
fn can_normalize_unit_vector() {
    let vector = Vector::new(1.0, 2.0, 3.0);
    let normalized = vector.normalized();
    assert_eq!(normalized.magnitude(), 1.0)
}

#[test]
fn can_compute_vector_dot_product() {
    let vector_a = Vector::new(1.0, 2.0, 3.0);
    let vector_b = Vector::new(2.0, 3.0, 4.0);
    assert_eq!(vector_a.dot(&vector_b), 20.0)
}

#[test]
fn can_compute_vector_cross_product() {
    let vector_a = Vector::new(1.0, 2.0, 3.0);
    let vector_b = Vector::new(2.0, 3.0, 4.0);
    assert_eq!(vector_a.cross(&vector_b), Vector::new(-1.0, 2.0, -1.0));
    assert_eq!(vector_b.cross(&vector_a), Vector::new(1.0, -2.0, 1.0));
}

#[test]
fn reflecting_a_vector_approaching_at_45_deg() {
    let v = Vector::new(1.0, -1.0, 0.0);
    let normal = Vector::new(0.0, 1.0, 0.0);
    let r = v.reflect(&normal);
    assert_eq!(r, Vector::new(1.0, 1.0, 0.0));
}

#[test]
fn reflecting_a_vector_off_a_slanted_surface() {
    let v = Vector::new(0.0, -1.0, 0.0);
    let normal = Vector::new(2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);
    let r = v.reflect(&normal);
    assert_eq!(r, Vector::new(1.0, 0.0, 0.0));
}
