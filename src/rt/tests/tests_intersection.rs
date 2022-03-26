#[cfg(test)]
use crate::{
    primitive::{point::Point, tuple::Tuple, vector::Vector},
    rt::{
        intersect::Intersect,
        intersection::{Intersection, Object},
        intersections::Intersections,
        ray::Ray,
        sphere::Sphere,
    },
};

#[test]
fn an_intersection_encapsulates_t_and_object() {
    let sphere = Sphere::new();
    let i = Intersection::Sphere(3.5, &sphere);
    assert_eq!(i.t(), 3.5);
    assert_eq!(i.object(), &sphere);
}

// NOTE - Not really testing something useful
#[test]
fn aggregating_intersections() {
    let sphere = Sphere::new();
    let i1 = Intersection::Sphere(1.0, &sphere);
    let i2 = Intersection::Sphere(2.0, &sphere);
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
    let sphere = Sphere::new();
    let xs = ray.intersect(&sphere).expect("No intersection found!");

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].object(), &sphere);
    assert_eq!(xs[1].object(), &sphere);
}

#[test]
fn the_hit_when_all_intersections_have_positive_t() {
    let sphere = Sphere::new();
    let i1 = Intersection::Sphere(1.0, &sphere);
    let i2 = Intersection::Sphere(2.0, &sphere);
    let xs = Intersections::new(vec![i1, i2]);

    let i = xs.hit();
    assert_eq!(i, Some(&i1));
}

#[test]
fn the_hit_when_some_intersections_have_negative_t() {
    let sphere = Sphere::new();
    let i1 = Intersection::Sphere(-1.0, &sphere);
    let i2 = Intersection::Sphere(1.0, &sphere);
    let xs = Intersections::new(vec![i1, i2]);

    let i = xs.hit();
    assert_eq!(i, Some(&i2));
}

#[test]
fn the_hit_when_all_intersections_have_negative_t() {
    let sphere = Sphere::new();
    let i1 = Intersection::Sphere(-2.0, &sphere);
    let i2 = Intersection::Sphere(-1.0, &sphere);
    let xs = Intersections::new(vec![i1, i2]);

    let i = xs.hit();
    assert_eq!(i, None);
}

#[test]
fn the_hit_is_always_the_lowest_nonnegative_intersection() {
    let sphere = Sphere::new();
    let i1 = Intersection::Sphere(5.0, &sphere);
    let i2 = Intersection::Sphere(7.0, &sphere);
    let i3 = Intersection::Sphere(-3.0, &sphere);
    let i4 = Intersection::Sphere(2.0, &sphere);

    let xs = Intersections::new(vec![i1, i2, i3, i4]);
    let i = xs.hit();
    assert_eq!(i, Some(&i4));
}
