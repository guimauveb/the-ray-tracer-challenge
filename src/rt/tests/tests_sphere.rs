#[cfg(test)]
use crate::{
    primitive::{point::Point, tuple::Tuple, vector::Vector},
    rt::{
        intersect::Intersect,
        matrix::{Matrix, Scaling, Translation},
        ray::Ray,
        sphere::Sphere,
    },
};

#[test]
fn a_ray_intersects_a_sphere_at_two_points() {
    let origin = Point::new(0.0, 0.0, -5.0);
    let direction = Vector::new(0.0, 0.0, 1.0);
    let ray = Ray::new(origin, direction);
    let sphere = Sphere::default();
    let intersection = ray.intersect(&sphere).expect("No intersection foud!");

    assert_eq!(intersection.len(), 2);
    assert_eq!(intersection[0].t(), 4.0);
    assert_eq!(intersection[1].t(), 6.0);
}

#[test]
fn a_ray_intersects_a_sphere_at_a_tangent() {
    let origin = Point::new(0.0, 1.0, -5.0);
    let direction = Vector::new(0.0, 0.0, 1.0);
    let ray = Ray::new(origin, direction);
    let sphere = Sphere::default();
    let intersection = ray.intersect(&sphere).expect("No intersection found!");

    assert_eq!(intersection.len(), 2);
    assert_eq!(intersection[0].t(), 5.0);
    assert_eq!(intersection[1].t(), 5.0);
}

#[test]
fn a_ray_originates_inside_a_sphere() {
    let origin = Point::new(0.0, 0.0, 0.0);
    let direction = Vector::new(0.0, 0.0, 1.0);
    let ray = Ray::new(origin, direction);
    let sphere = Sphere::default();
    let intersection = ray.intersect(&sphere).expect("No intersection found!");

    assert_eq!(intersection.len(), 2);
    assert_eq!(intersection[0].t(), -1.0);
    assert_eq!(intersection[1].t(), 1.0);
}

#[test]
fn a_sphere_is_behind_a_ray() {
    let origin = Point::new(0.0, 0.0, 5.0);
    let direction = Vector::new(0.0, 0.0, 1.0);
    let ray = Ray::new(origin, direction);
    let sphere = Sphere::default();
    let intersection = ray.intersect(&sphere).expect("No intersection found!");

    assert_eq!(intersection.len(), 2);
    assert_eq!(intersection[0].t(), -6.0);
    assert_eq!(intersection[1].t(), -4.0);
}

#[test]
fn a_sphere_default_transform() {
    let s = Sphere::default();
    assert_eq!(s.transform(), &Matrix::<4_usize>::identity());
}

#[test]
fn changing_a_sphere_transform() {
    let mut s = Sphere::default();
    let t = Matrix::<4_usize>::translation(2.0, 3.0, 4.0);
    s.set_transform(t.clone());

    assert_eq!(s.transform(), &t);
}

#[test]
fn intersecting_a_scaled_sphere_with_a_ray() {
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let mut s = Sphere::default();
    s.set_transform(Matrix::<4_usize>::scaling(2.0, 2.0, 2.0));
    let xs = r.intersect(&s).expect("No intersection found!");
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t(), 3.0);
    assert_eq!(xs[1].t(), 7.0);
}
