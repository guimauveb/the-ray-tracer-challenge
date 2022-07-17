#[cfg(test)]
use crate::{
    rt::{
        object::Object,
        plane::Plane,
        ray::{Intersect, Ray},
        shape::Shape,
    },
    tuple::{point::Point, vector::Vector},
};

#[test]
fn the_normal_of_a_plane_is_constant_everywhere() {
    let p = Object::Plane(Plane::default());
    let (n1, n2, n3) = (
        p.normal_at(&Point::new(0.0, 0.0, 0.0)),
        p.normal_at(&Point::new(10.0, 0.0, -10.0)),
        p.normal_at(&Point::new(-5.0, 0.0, 150.0)),
    );
    let expected = Vector::new(0.0, 1.0, 0.0);
    assert_eq!(n1, expected);
    assert_eq!(n2, expected);
    assert_eq!(n3, expected);
}

#[test]
fn intersect_with_a_ray_parallel_to_the_plane() {
    let p = Object::Plane(Plane::default());
    let r = Ray::new(Point::new(0.0, 10.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    let xs = r.intersect(&p);
    assert!(xs.is_none());
}

#[test]
fn intersect_with_a_coplanar_ray() {
    let p = Object::Plane(Plane::default());
    let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    let xs = r.intersect(&p);
    assert!(xs.is_none());
}

#[test]
fn a_ray_intersecting_a_plane_from_above() {
    let p = Object::Plane(Plane::default());
    let r = Ray::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0));
    let xs = r.intersect(&p);
    if let Some(xs) = xs {
        assert_eq!(xs[0].t(), 1.0);
        assert!(std::ptr::eq(xs[0].object(), &p));
    } else {
        panic!("No intersections!");
    }
}

#[test]
fn a_ray_intersecting_a_plane_from_below() {
    let p = Object::Plane(Plane::default());
    let r = Ray::new(Point::new(0.0, -1.0, 0.0), Vector::new(0.0, 1.0, 0.0));
    let xs = r.intersect(&p);
    if let Some(xs) = xs {
        assert_eq!(xs[0].t(), 1.0);
        assert_eq!(xs[0].object(), &p);
    } else {
        panic!("No intersections!");
    }
}
