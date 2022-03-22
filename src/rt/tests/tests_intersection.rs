#[cfg(test)]
use crate::{
    primitive::{point::Point, tuple::Tuple, vector::Vector},
    rt::{intersect::Intersect, intersection::Intersection, ray::Ray, sphere::Sphere},
};

#[test]
fn an_intersection_encapsulates_t_and_object() {
    let sphere = Sphere::new();
    let i = Intersection::new(3.5, &sphere);
    assert_eq!(i.t(), 3.5);
    assert_eq!(i.object(), &sphere);
}

// NOTE - Not really testing something useful
#[test]
fn aggregating_intersections() {
    let sphere = Sphere::new();
    let i1 = Intersection::new(1.0, &sphere);
    let i2 = Intersection::new(2.0, &sphere);

    let xs = [i1, i2];
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
    let xs = sphere.intersect(&ray).expect("No intersection found!");

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].object(), &sphere);
    assert_eq!(xs[1].object(), &sphere);
}
