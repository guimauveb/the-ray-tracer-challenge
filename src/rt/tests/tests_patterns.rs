#[cfg(test)]
use crate::{
    rt::{
        color::{Color, BLACK, WHITE},
        matrix::Matrix,
        object::Object,
        patterns::{Checkers, Gradient, Pattern, Ring, Stripe},
        shape::Shape,
        sphere::Sphere,
    },
    tuple::point::Point,
};

#[test]
fn creating_a_stripe_pattern() {
    let pattern = Stripe::new(WHITE, BLACK, None);
    assert_eq!(pattern.a(), &WHITE);
    assert_eq!(pattern.b(), &BLACK);
}

#[test]
fn a_stripe_pattern_is_constant_in_y() {
    let pattern = Stripe::new(WHITE, BLACK, None);
    assert_eq!(pattern.stripe_at(&Point::new(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(pattern.stripe_at(&Point::new(0.0, 1.0, 0.0)), WHITE);
    assert_eq!(pattern.stripe_at(&Point::new(0.0, 2.0, 0.0)), WHITE);
}

#[test]
fn a_stripe_pattern_is_constant_in_z() {
    let pattern = Stripe::new(WHITE, BLACK, None);
    assert_eq!(pattern.stripe_at(&Point::new(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(pattern.stripe_at(&Point::new(0.0, 0.0, 1.0)), WHITE);
    assert_eq!(pattern.stripe_at(&Point::new(0.0, 0.0, 2.0)), WHITE);
}

#[test]
fn a_stripe_pattern_alternates_in_x() {
    let pattern = Stripe::new(WHITE, BLACK, None);
    assert_eq!(pattern.stripe_at(&Point::new(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(pattern.stripe_at(&Point::new(1.0, 0.0, 0.0)), BLACK);
    assert_eq!(pattern.stripe_at(&Point::new(2.0, 0.0, 0.0)), WHITE);
}

#[test]
fn stripes_with_an_object_transformation() {
    let mut object = Object::Sphere(Sphere::default());
    object.set_transform(Matrix::<4>::scaling(2.0, 2.0, 2.0));
    let pattern = Stripe::new(WHITE, BLACK, None);
    let c = pattern.stripe_at_object(&object, &Point::new(1.5, 0.0, 0.0));
    assert_eq!(c, WHITE);
}

#[test]
fn stripes_with_a_pattern_transformation() {
    let object = Object::Sphere(Sphere::default());
    let mut pattern = Stripe::new(WHITE, BLACK, None);
    pattern.set_transform(Matrix::<4>::scaling(2.0, 2.0, 2.0));
    let c = pattern.stripe_at_object(&object, &Point::new(1.5, 0.0, 0.0));
    assert_eq!(c, WHITE);
}

#[test]
fn stripes_with_both_an_object_and_a_pattern_transformation() {
    let mut object = Object::Sphere(Sphere::default());
    object.set_transform(Matrix::<4>::scaling(2.0, 2.0, 2.0));
    let mut pattern = Stripe::new(WHITE, BLACK, None);
    pattern.set_transform(Matrix::<4>::translation(0.5, 0.0, 0.0));
    let c = pattern.stripe_at_object(&object, &Point::new(2.5, 0.0, 0.0));
    assert_eq!(c, WHITE);
}

#[test]
fn a_gradient_linearly_interpolates_between_colors() {
    let pattern = Pattern::Gradient(Gradient::new(WHITE, BLACK, None));
    assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(
        pattern.pattern_at(&Point::new(0.25, 0.0, 0.0)),
        Color::new(0.75, 0.75, 0.75)
    );
    assert_eq!(
        pattern.pattern_at(&Point::new(0.5, 0.0, 0.0)),
        Color::new(0.5, 0.5, 0.5)
    );
    assert_eq!(
        pattern.pattern_at(&Point::new(0.75, 0.0, 0.0)),
        Color::new(0.25, 0.25, 0.25)
    );
}

#[test]
fn a_ring_should_extend_in_both_x_and_z() {
    let pattern = Pattern::Ring(Ring::new(WHITE, BLACK, None));
    assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(pattern.pattern_at(&Point::new(1.0, 0.0, 0.0)), BLACK);
    assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 1.0)), BLACK);
    assert_eq!(
        // 0.708 = just slightly more than 2.0.sqrt() / 2
        pattern.pattern_at(&Point::new(0.708, 0.0, 0.708)),
        BLACK
    );
}

#[test]
fn checkers_should_repeat_in_x() {
    let pattern = Pattern::Checkers(Checkers::new(WHITE, BLACK, None));
    assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(pattern.pattern_at(&Point::new(0.99, 0.0, 0.0)), WHITE);
    assert_eq!(pattern.pattern_at(&Point::new(1.01, 0.0, 0.0)), BLACK);
}

#[test]
fn checkers_should_repeat_in_y() {
    let pattern = Pattern::Checkers(Checkers::new(WHITE, BLACK, None));
    assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.99, 0.0)), WHITE);
    assert_eq!(pattern.pattern_at(&Point::new(0.0, 1.01, 0.0)), BLACK);
}

#[test]
fn checkers_should_repeat_in_z() {
    let pattern = Pattern::Checkers(Checkers::new(WHITE, BLACK, None));
    assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 0.99)), WHITE);
    assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 1.01)), BLACK);
}
