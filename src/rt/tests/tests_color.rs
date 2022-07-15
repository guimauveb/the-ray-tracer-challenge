#[cfg(test)]
use crate::rt::color::{Color, BLACK, WHITE};

#[test]
fn can_create_color() {
    let color = Color::new(-0.5, 0.4, 1.7);
    assert_eq!(color.red(), -0.5);
    assert_eq!(color.green(), 0.4);
    assert_eq!(color.blue(), 1.7);
}

#[test]
fn can_create_black_color() {
    let black = BLACK;
    let expected = BLACK;
    assert_eq!(black, expected);
}

#[test]
fn can_create_white_color() {
    let white = WHITE;
    let expected = WHITE;
    assert_eq!(white, expected);
}

#[test]
fn can_add_colors() {
    let color_a = Color::new(0.9, 0.6, 0.75);
    let color_b = Color::new(0.7, 0.1, 0.25);
    let expected = Color::new(1.6, 0.7, 1.0);
    assert_eq!(color_a + color_b, expected);
}

#[test]
fn can_sub_clors() {
    let color_a = Color::new(0.9, 0.6, 0.75);
    let color_b = Color::new(0.7, 0.1, 0.25);
    let expected = Color::new(0.2, 0.5, 0.5);
    assert_eq!(color_a - color_b, expected);
}

#[test]
fn can_multiply_color_by_sclar() {
    let color = Color::new(0.2, 0.3, 0.4);
    let scalar = 2.0;
    let expected = Color::new(0.4, 0.6, 0.8);
    assert_eq!(color * scalar, expected);
}

#[test]
// Mul
fn can_compute_hadamard_product() {
    let color_a = Color::new(1.0, 0.2, 0.4);
    let color_b = Color::new(0.9, 1.0, 0.1);
    let expected = Color::new(0.9, 0.2, 0.04);
    assert_eq!(color_a * color_b, expected);
}
