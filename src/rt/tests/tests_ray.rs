#[cfg(test)]
use crate::{
    rt::{
        matrix::{Matrix, Transform},
        ray::Ray,
    },
    tuple::{point::Point, vector::Vector},
};

#[test]
fn creating_and_querying_a_ray() {
    let origin = Point::new(1.0, 2.0, 3.0);
    let direction = Vector::new(4.0, 5.0, 6.0);
    let ray = Ray::new(origin.clone(), direction.clone());

    assert_eq!(ray.origin(), &origin);
    assert_eq!(ray.direction(), &direction);
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
fn translating_a_ray() {
    let origin = Point::new(1.0, 2.0, 3.0);
    let direction = Vector::new(0.0, 1.0, 0.0);
    let ray = Ray::new(origin, direction);
    let m = Matrix::<4>::translation(3.0, 4.0, 5.0);

    let r2 = ray.transform(&m);
    assert_eq!(r2.origin(), &Point::new(4.0, 6.0, 8.0));
    assert_eq!(r2.direction(), &Vector::new(0.0, 1.0, 0.0));
}

#[test]
fn scaling_a_ray() {
    let origin = Point::new(1.0, 2.0, 3.0);
    let direction = Vector::new(0.0, 1.0, 0.0);
    let ray = Ray::new(origin, direction);
    let m = Matrix::<4>::scaling(2.0, 3.0, 4.0);

    let r2 = ray.transform(&m);
    assert_eq!(r2.origin(), &Point::new(2.0, 6.0, 12.0));
    assert_eq!(r2.direction(), &Vector::new(0.0, 3.0, 0.0));
}
