#[cfg(test)]
use {
    crate::{
        primitive::{point::Point, tuple::Tuple, vector::Vector},
        rt::{
            intersect::Intersect,
            material::Material,
            matrix::{Matrix, Rotation, Scaling, Translation},
            normal::Normal,
            ray::Ray,
            sphere::Sphere,
        },
    },
    std::f64::consts::PI,
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
    let t = Matrix::<4_usize>::translation(2.0, 3.0, 4.0);
    let s = Sphere::with_transform(t.clone());
    assert_eq!(s.transform(), &t);
}

#[test]
fn intersecting_a_scaled_sphere_with_a_ray() {
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let t = Matrix::<4_usize>::scaling(2.0, 2.0, 2.0);
    let s = Sphere::with_transform(t);
    let xs = r.intersect(&s).expect("No intersection found!");
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t(), 3.0);
    assert_eq!(xs[1].t(), 7.0);
}

#[test]
fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
    let s = Sphere::default();
    let n = s.normal_at(&Point::new(1.0, 0.0, 0.0));
    assert_eq!(n, Vector::new(1.0, 0.0, 0.0));
}

#[test]
fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
    let s = Sphere::default();
    let n = s.normal_at(&Point::new(0.0, 1.0, 0.0));
    assert_eq!(n, Vector::new(0.0, 1.0, 0.0));
}

#[test]
fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
    let s = Sphere::default();
    let n = s.normal_at(&Point::new(0.0, 0.0, 1.0));
    assert_eq!(n, Vector::new(0.0, 0.0, 1.0));
}

#[test]
fn the_normal_on_a_sphere_at_a_nonaxial_point() {
    let s = Sphere::default();
    let n = s.normal_at(&Point::new(
        3.0_f64.sqrt() / 3.0,
        3.0_f64.sqrt() / 3.0,
        3.0_f64.sqrt() / 3.0,
    ));
    assert_eq!(
        n,
        Vector::new(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        )
    );
}

#[test]
fn computing_the_normal_on_a_translated_sphere() {
    let t = Matrix::<4_usize>::translation(0.0, 1.0, 0.0);
    let s = Sphere::with_transform(t);
    let n = s.normal_at(&Point::new(0.0, 1.70711, -0.70711));
    assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
}

#[test]
fn computing_the_normal_on_a_transformed_sphere() {
    let m = Matrix::<4_usize>::scaling(1.0, 0.5, 1.0) * Matrix::<4_usize>::rotation_z(PI / 5.0);
    let s = Sphere::with_transform(m);
    let n = s.normal_at(&Point::new(
        0.0,
        2.0_f64.sqrt() / 2.0,
        -2.0_f64.sqrt() / 2.0,
    ));
    assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
}

#[test]
fn a_sphere_has_a_default_material() {
    let s = Sphere::default();
    let m = s.material();
    assert_eq!(m, &Material::default());
}

#[test]
fn a_sphere_may_be_assigned_a_material() {
    let mut m = Material::default();
    m.set_ambient(1.0);
    let s = Sphere::with_material(m.clone());
    assert_eq!(s.material(), &m);
}
