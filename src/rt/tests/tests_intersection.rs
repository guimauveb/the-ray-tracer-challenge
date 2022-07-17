#[cfg(test)]
use crate::{
    approx_eq::ApproxEq,
    float::epsilon::EPSILON,
    rt::{
        computation::Computation,
        intersection::Intersection,
        intersections::Intersections,
        matrix::Matrix,
        object::Object,
        plane::Plane,
        ray::{Intersect, Ray},
        shape::Shape,
        sphere::Sphere,
    },
    tuple::{point::Point, vector::Vector},
};

#[test]
fn an_intersection_encapsulates_t_and_object() {
    let sphere = Object::Sphere(Sphere::default());
    let i = Intersection::new(3.5, &sphere);
    assert_eq!(i.t(), 3.5);
    assert_eq!(i.object(), &sphere);
}

#[test]
fn aggregating_intersections() {
    let sphere = Object::Sphere(Sphere::default());
    let i1 = Intersection::new(1.0, &sphere);
    let i2 = Intersection::new(2.0, &sphere);
    let xs = Intersections::new(vec![i1, i2]);

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t(), 1.0);
    assert_eq!(xs[1].t(), 2.0);
}

#[test]
fn intersect_sets_the_object_on_the_intersection() {
    let origin = Point::new(0.0, 0.0, -5.0);
    let direction = Vector::new(0.0, 0.0, 1.0);
    let ray = Ray::new(origin, direction);
    let sphere = Object::Sphere(Sphere::default());
    let xs = ray.intersect(&sphere).expect("No intersection found!");

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].object(), &sphere);
    assert_eq!(xs[1].object(), &sphere);
}

#[test]
fn the_hit_when_all_intersections_have_positive_t() {
    let sphere = Object::Sphere(Sphere::default());
    let i1 = Intersection::new(1.0, &sphere);
    let i2 = Intersection::new(2.0, &sphere);
    let xs = Intersections::new(vec![i1.clone(), i2.clone()]);

    let i = xs.hit();
    assert_eq!(i, Some(&i1));
}

#[test]
fn the_hit_when_some_intersections_have_negative_t() {
    let sphere = Object::Sphere(Sphere::default());
    let i1 = Intersection::new(-1.0, &sphere);
    let i2 = Intersection::new(1.0, &sphere);
    let xs = Intersections::new(vec![i1.clone(), i2.clone()]);

    let i = xs.hit();
    assert_eq!(i, Some(&i2));
}

#[test]
fn the_hit_when_all_intersections_have_negative_t() {
    let sphere = Object::Sphere(Sphere::default());
    let i1 = Intersection::new(-2.0, &sphere);
    let i2 = Intersection::new(-1.0, &sphere);
    let xs = Intersections::new(vec![i1, i2]);

    let i = xs.hit();
    assert_eq!(i, None);
}

#[test]
fn the_hit_is_always_the_lowest_nonnegative_intersection() {
    let sphere = Object::Sphere(Sphere::default());
    let i1 = Intersection::new(5.0, &sphere);
    let i2 = Intersection::new(7.0, &sphere);
    let i3 = Intersection::new(-3.0, &sphere);
    let i4 = Intersection::new(2.0, &sphere);

    let xs = Intersections::new(vec![i1.clone(), i2.clone(), i3.clone(), i4.clone()]);
    let i = xs.hit();
    assert_eq!(i, Some(&i4));
}

#[test]
fn precomputing_the_state_of_an_intersection() {
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let shape = Object::Sphere(Sphere::default());
    let i = Intersection::new(4.0, &shape);
    let comps = i.prepare_computations(&r, None);
    let expected_comps = Computation::new(
        &i,
        Point::new(0.0, 0.0, -1.0),
        Vector::new(0.0, 0.0, -1.0),
        Vector::new(0.0, 0.0, -1.0),
        false,
        Point::new(0.0, 0.0, -1.00001),
        Point::new(0.0, 0.0, -0.99999),
        Vector::new(0.0, 0.0, -1.0),
        (1.0, 1.0),
    );

    assert_eq!(comps, expected_comps);
}

#[test]
fn the_hit_when_the_instersection_occurs_on_the_outside() {
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let shape = Object::Sphere(Sphere::default());
    let i = Intersection::new(4.0, &shape);
    let comps = i.prepare_computations(&r, None);
    assert_eq!(comps.inside(), false);
}

#[test]
fn the_hit_when_the_instersection_occurs_on_the_inside() {
    let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    let shape = Object::Sphere(Sphere::default());
    let i = Intersection::new(1.0, &shape);
    let comps = i.prepare_computations(&r, None);
    assert_eq!(comps.point(), &Point::new(0.0, 0.0, 1.0));
    assert_eq!(comps.eye_vector(), &Vector::new(0.0, 0.0, -1.0));
    assert_eq!(comps.inside(), true);
    // The normal is inverted to reflect the light properly.
    assert_eq!(comps.normal_vector(), &Vector::new(0.0, 0.0, -1.0));
}

#[test]
fn the_hit_should_offset_the_point() {
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let shape = Object::Sphere(Sphere::with_transform(Matrix::<4>::translation(
        0.0, 0.0, 1.0,
    )));
    let i = Intersection::new(5.0, &shape);
    let comps = i.prepare_computations(&r, None);
    assert!(comps.over_point().z() < -EPSILON / 2.0);
    assert!(comps.point().z() > comps.over_point().z());
}

#[test]
fn precomputing_the_reflection_vector() {
    let shape = Object::Plane(Plane::default());
    let r = Ray::new(
        Point::new(0.0, 1.0, -1.0),
        Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
    );
    let i = Intersection::new(2.0_f64.sqrt(), &shape);
    let comps = i.prepare_computations(&r, None);
    assert_eq!(
        comps.reflect_vector(),
        &Vector::new(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
    );
}

#[test]
fn the_under_point_is_offset_below_the_surface() {
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let mut shape = Object::Sphere(Sphere::glassy());
    shape.set_transform(Matrix::translation(0.0, 0.0, 1.0));
    let i = Intersection::new(5.0, &shape);
    let xs = Intersections::new(vec![i]);
    let comps = xs[0].prepare_computations(&r, Some(&xs));
    assert!(comps.under_point().z() > EPSILON / 2.0);
    assert!(comps.point().z() < comps.under_point().z());
}

#[test]
fn the_schlick_approximation_under_total_internal_reflection() {
    let shape = Object::Sphere(Sphere::glassy());
    let r = Ray::new(
        Point::new(0.0, 0.0, -2.0_f64 / 2.0),
        Vector::new(0.0, 1.0, 0.0),
    );
    let xs = Intersections::new(vec![
        Intersection::new(-2.0_f64.sqrt() / 2.0, &shape),
        Intersection::new(2.0_f64.sqrt() / 2.0, &shape),
    ]);
    let comps = xs[1].prepare_computations(&r, Some(&xs));
    let reflectance = comps.schlick();
    assert!(reflectance.approx_eq(1.0));
}

#[test]
fn the_schlick_approximation_with_a_perpendicular_viewing_angle() {
    let shape = Object::Sphere(Sphere::glassy());
    let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
    let xs = Intersections::new(vec![
        Intersection::new(-1.0, &shape),
        Intersection::new(1.0, &shape),
    ]);
    let comps = xs[1].prepare_computations(&r, Some(&xs));
    let reflectance = comps.schlick();
    assert!(reflectance.approx_eq(0.04));
}

#[test]
fn the_schlick_approximation_with_small_angle_and_n2_gt_n1() {
    let shape = Object::Sphere(Sphere::glassy());
    let r = Ray::new(Point::new(0.0, 0.99, -2.0), Vector::new(0.0, 0.0, 1.0));
    let xs = Intersections::new(vec![Intersection::new(1.8589, &shape)]);
    let comps = xs[0].prepare_computations(&r, Some(&xs));
    let reflectance = comps.schlick();
    assert!(reflectance.approx_eq(0.48873));
}
