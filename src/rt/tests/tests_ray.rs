#[cfg(test)]
use crate::{
    primitive::{point::Point, tuple::Tuple, vector::Vector},
    rt::ray::{Position, Ray},
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
