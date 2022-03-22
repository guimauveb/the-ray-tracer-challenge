#[cfg(test)]
use crate::{
    primitive::{point::Point, tuple::Tuple, vector::Vector},
    rt::{
        intersect::Intersect,
        ray::{Position, Ray},
        sphere::Sphere,
    },
};

#[test]
fn creating_and_querying_a_ray() {
    let origin = Point::new(1.0, 2.0, 3.0);
    let direction = Vector::new(4.0, 5.0, 6.0);
    let ray = Ray::new(origin, direction);

    assert_eq!(ray.origin(), origin);
    assert_eq!(ray.direction(), direction);
}

#[test]
fn computing_a_point_from_a_distance() {
    let origin = Point::new(2.0, 3.0, 4.0);
    let direction = Vector::new(1.0, 0.0, 0.0);
    let ray = Ray::new(origin, direction);

    assert_eq!(ray.position(0.0), Point::new(2.0, 3.0, 4.0));
    assert_eq!(ray.position(1.0), Point::new(3.0, 3.0, 4.0));
    assert_eq!(ray.position(-1.0), Point::new(1.0, 3.0, 4.0));
    assert_eq!(ray.position(2.5), Point::new(4.5, 3.0, 4.0));
}

#[test]
fn a_ray_intersects_a_sphere_at_two_points() {
    let origin = Point::new(0.0, 0.0, -5.0);
    let direction = Vector::new(0.0, 0.0, 1.0);
    let ray = Ray::new(origin, direction);
    let sphere = Sphere::new();
    let intersection = sphere.intersect(&ray).expect("No intersection foud!");

    assert_eq!(intersection.len(), 2);
    assert_eq!(intersection[0].t(), 4.0);
    assert_eq!(intersection[1].t(), 6.0);
}

#[test]
fn a_ray_intersects_a_sphere_at_a_tangent() {
    let origin = Point::new(0.0, 1.0, -5.0);
    let direction = Vector::new(0.0, 0.0, 1.0);
    let ray = Ray::new(origin, direction);
    let sphere = Sphere::new();
    let intersection = sphere.intersect(&ray).expect("No intersection found!");

    assert_eq!(intersection.len(), 2);
    assert_eq!(intersection[0].t(), 5.0);
    assert_eq!(intersection[1].t(), 5.0);
}

#[test]
fn a_ray_originates_inside_a_sphere() {
    let origin = Point::new(0.0, 0.0, 0.0);
    let direction = Vector::new(0.0, 0.0, 1.0);
    let ray = Ray::new(origin, direction);
    let sphere = Sphere::new();
    let intersection = sphere.intersect(&ray).expect("No intersection found!");

    assert_eq!(intersection.len(), 2);
    assert_eq!(intersection[0].t(), -1.0);
    assert_eq!(intersection[1].t(), 1.0);
}

#[test]
fn a_sphere_is_behind_a_ray() {
    let origin = Point::new(0.0, 0.0, 5.0);
    let direction = Vector::new(0.0, 0.0, 1.0);
    let ray = Ray::new(origin, direction);
    let sphere = Sphere::new();
    let intersection = sphere.intersect(&ray).expect("No intersection found!");

    assert_eq!(intersection.len(), 2);
    assert_eq!(intersection[0].t(), -6.0);
    assert_eq!(intersection[1].t(), -4.0);
}
