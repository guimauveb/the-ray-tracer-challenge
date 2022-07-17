#[cfg(test)]
use crate::{
    rt::{
        color::{Color, BLACK, WHITE},
        matrix::Matrix,
        object::Object,
        pattern::{test_pattern, Pattern},
        shape::Shape,
        sphere::Sphere,
    },
    tuple::point::Point,
};

#[test]
fn creating_a_stripe_pattern() {
    let pattern = Pattern::stripe(WHITE, BLACK, None);
    assert_eq!(pattern.a(), &WHITE);
    assert_eq!(pattern.b(), &BLACK);
}

#[test]
fn a_stripe_pattern_is_constant_in_y() {
    let pattern = Pattern::stripe(WHITE, BLACK, None);
    assert_eq!(pattern.at(&Point::new(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(pattern.at(&Point::new(0.0, 1.0, 0.0)), WHITE);
    assert_eq!(pattern.at(&Point::new(0.0, 2.0, 0.0)), WHITE);
}

#[test]
fn a_stripe_pattern_is_constant_in_z() {
    let pattern = Pattern::stripe(WHITE, BLACK, None);
    assert_eq!(pattern.at(&Point::new(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(pattern.at(&Point::new(0.0, 0.0, 1.0)), WHITE);
    assert_eq!(pattern.at(&Point::new(0.0, 0.0, 2.0)), WHITE);
}

#[test]
fn a_stripe_pattern_alternates_in_x() {
    let pattern = Pattern::stripe(WHITE, BLACK, None);
    assert_eq!(pattern.at(&Point::new(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(pattern.at(&Point::new(1.0, 0.0, 0.0)), BLACK);
    assert_eq!(pattern.at(&Point::new(2.0, 0.0, 0.0)), WHITE);
}

#[test]
fn stripes_with_an_object_transformation() {
    let mut object = Object::Sphere(Sphere::default());
    object.set_transform(Matrix::<4>::scaling(2.0, 2.0, 2.0));
    let pattern = Pattern::stripe(WHITE, BLACK, None);
    let c = pattern.at_object(&object, &Point::new(1.5, 0.0, 0.0));
    assert_eq!(c, WHITE);
}

#[test]
fn stripes_with_a_pattern_transformation() {
    let object = Object::Sphere(Sphere::default());
    let mut pattern = Pattern::stripe(WHITE, BLACK, None);
    pattern.set_transform(Matrix::<4>::scaling(2.0, 2.0, 2.0));
    let c = pattern.at_object(&object, &Point::new(1.5, 0.0, 0.0));
    assert_eq!(c, WHITE);
}

#[test]
fn stripes_with_both_an_object_and_a_pattern_transformation() {
    let mut object = Object::Sphere(Sphere::default());
    object.set_transform(Matrix::<4>::scaling(2.0, 2.0, 2.0));
    let mut pattern = Pattern::stripe(WHITE, BLACK, None);
    pattern.set_transform(Matrix::<4>::translation(0.5, 0.0, 0.0));
    let c = pattern.at_object(&object, &Point::new(2.5, 0.0, 0.0));
    assert_eq!(c, WHITE);
}

#[test]
fn the_default_pattern_transformation() {
    let pattern = test_pattern();
    assert_eq!(pattern.transform(), &Matrix::identity());
}

#[test]
fn assigning_a_transformation() {
    let mut pattern = test_pattern();
    pattern.set_transform(Matrix::translation(1.0, 2.0, 3.0));
    assert_eq!(pattern.transform(), &Matrix::translation(1.0, 2.0, 3.0));
}

#[test]
fn a_pattern_with_an_object_transformation() {
    let mut object = Object::Sphere(Sphere::default());
    object.set_transform(Matrix::<4>::scaling(2.0, 2.0, 2.0));
    let pattern = test_pattern();
    let c = pattern.at_object(&object, &Point::new(2.0, 3.0, 4.0));
    assert_eq!(c, Color::new(1.0, 1.5, 2.0));
}

#[test]
fn a_pattern_with_a_pattern_transformation() {
    let object = Object::Sphere(Sphere::default());
    let mut pattern = test_pattern();
    pattern.set_transform(Matrix::<4>::scaling(2.0, 2.0, 2.0));
    let c = pattern.at_object(&object, &Point::new(2.0, 3.0, 4.0));
    assert_eq!(c, Color::new(1.0, 1.5, 2.0));
}

#[test]
fn a_pattern_with_both_an_object_and_a_pattern_transformation() {
    let mut object = Object::Sphere(Sphere::default());
    object.set_transform(Matrix::<4>::scaling(2.0, 2.0, 2.0));
    let mut pattern = test_pattern();
    pattern.set_transform(Matrix::<4>::translation(0.5, 1.0, 1.5));
    let c = pattern.at_object(&object, &Point::new(2.5, 3.0, 3.5));
    assert_eq!(c, Color::new(0.75, 0.5, 0.25));
}

#[test]
fn a_gradient_linearly_interpolates_between_colors() {
    let pattern = Pattern::gradient(WHITE, BLACK, None);
    assert_eq!(pattern.at(&Point::new(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(
        pattern.at(&Point::new(0.25, 0.0, 0.0)),
        Color::new(0.75, 0.75, 0.75)
    );
    assert_eq!(
        pattern.at(&Point::new(0.5, 0.0, 0.0)),
        Color::new(0.5, 0.5, 0.5)
    );
    assert_eq!(
        pattern.at(&Point::new(0.75, 0.0, 0.0)),
        Color::new(0.25, 0.25, 0.25)
    );
}

#[test]
fn a_ring_should_extend_in_both_x_and_z() {
    let pattern = Pattern::ring(WHITE, BLACK, None);
    assert_eq!(pattern.at(&Point::new(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(pattern.at(&Point::new(1.0, 0.0, 0.0)), BLACK);
    assert_eq!(pattern.at(&Point::new(0.0, 0.0, 1.0)), BLACK);
    assert_eq!(
        // 0.708 = just slightly more than 2.0.sqrt() / 2
        pattern.at(&Point::new(0.708, 0.0, 0.708)),
        BLACK
    );
}

#[test]
fn checkers_should_repeat_in_x() {
    let pattern = Pattern::checkers(WHITE, BLACK, None);
    assert_eq!(pattern.at(&Point::new(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(pattern.at(&Point::new(0.99, 0.0, 0.0)), WHITE);
    assert_eq!(pattern.at(&Point::new(1.01, 0.0, 0.0)), BLACK);
}

#[test]
fn checkers_should_repeat_in_y() {
    let pattern = Pattern::checkers(WHITE, BLACK, None);
    assert_eq!(pattern.at(&Point::new(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(pattern.at(&Point::new(0.0, 0.99, 0.0)), WHITE);
    assert_eq!(pattern.at(&Point::new(0.0, 1.01, 0.0)), BLACK);
}

#[test]
fn checkers_should_repeat_in_z() {
    let pattern = Pattern::checkers(WHITE, BLACK, None);
    assert_eq!(pattern.at(&Point::new(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(pattern.at(&Point::new(0.0, 0.0, 0.99)), WHITE);
    assert_eq!(pattern.at(&Point::new(0.0, 0.0, 1.01)), BLACK);
}
