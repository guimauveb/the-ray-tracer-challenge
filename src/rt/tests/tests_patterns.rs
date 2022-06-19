#[cfg(test)]
use crate::{
    rt::{
        color::Color, matrix::Matrix, object::Object, patterns::Stripe, shape::Shape,
        sphere::Sphere,
    },
    tuple::point::Point,
};

#[test]
fn creating_a_stripe_pattern() {
    let pattern = Stripe::new(Color::white(), Color::black(), None);
    assert_eq!(pattern.a(), &Color::white());
    assert_eq!(pattern.b(), &Color::black());
}

#[test]
fn a_stripe_pattern_is_constant_in_y() {
    let pattern = Stripe::new(Color::white(), Color::black(), None);
    assert_eq!(
        pattern.stripe_at(&Point::new(0.0, 0.0, 0.0)),
        &Color::white()
    );
    assert_eq!(
        pattern.stripe_at(&Point::new(0.0, 1.0, 0.0)),
        &Color::white()
    );
    assert_eq!(
        pattern.stripe_at(&Point::new(0.0, 2.0, 0.0)),
        &Color::white()
    );
}

#[test]
fn a_stripe_pattern_is_constant_in_z() {
    let pattern = Stripe::new(Color::white(), Color::black(), None);
    assert_eq!(
        pattern.stripe_at(&Point::new(0.0, 0.0, 0.0)),
        &Color::white()
    );
    assert_eq!(
        pattern.stripe_at(&Point::new(0.0, 0.0, 1.0)),
        &Color::white()
    );
    assert_eq!(
        pattern.stripe_at(&Point::new(0.0, 0.0, 2.0)),
        &Color::white()
    );
}

#[test]
fn a_stripe_pattern_alternates_in_x() {
    let pattern = Stripe::new(Color::white(), Color::black(), None);
    assert_eq!(
        pattern.stripe_at(&Point::new(0.0, 0.0, 0.0)),
        &Color::white()
    );
    assert_eq!(
        pattern.stripe_at(&Point::new(1.0, 0.0, 0.0)),
        &Color::black()
    );
    assert_eq!(
        pattern.stripe_at(&Point::new(2.0, 0.0, 0.0)),
        &Color::white()
    );
}

#[test]
fn stripes_with_an_object_transformation() {
    let mut object = Object::Sphere(Sphere::default());
    object.set_transform(Matrix::<4>::scaling(2.0, 2.0, 2.0));
    let pattern = Stripe::new(Color::white(), Color::black(), None);
    let c = pattern.stripe_at_object(&object, &Point::new(1.5, 0.0, 0.0));
    assert_eq!(c, &Color::white());
}

#[test]
fn stripes_with_a_pattern_transformation() {
    let object = Object::Sphere(Sphere::default());
    let mut pattern = Stripe::new(Color::white(), Color::black(), None);
    pattern.set_transform(Matrix::<4>::scaling(2.0, 2.0, 2.0));
    let c = pattern.stripe_at_object(&object, &Point::new(1.5, 0.0, 0.0));
    assert_eq!(c, &Color::white());
}

#[test]
fn stripes_with_both_an_object_and_a_pattern_transformation() {
    let mut object = Object::Sphere(Sphere::default());
    object.set_transform(Matrix::<4>::scaling(2.0, 2.0, 2.0));
    let mut pattern = Stripe::new(Color::white(), Color::black(), None);
    pattern.set_transform(Matrix::<4>::translation(0.5, 0.0, 0.0));
    let c = pattern.stripe_at_object(&object, &Point::new(2.5, 0.0, 0.0));
    assert_eq!(c, &Color::white());
}
