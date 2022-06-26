#[cfg(test)]
use {
    crate::{
        rt::{
            intersection::Intersection,
            intersections::Intersections,
            material::Material,
            matrix::Matrix,
            object::Object,
            ray::{Intersect, Ray},
            shape::Shape,
            sphere::Sphere,
        },
        tuple::{point::Point, vector::Vector},
    },
    std::{f64::consts::PI, ops::Deref},
};

#[test]
fn a_ray_intersects_a_sphere_at_two_points() {
    let origin = Point::new(0.0, 0.0, -5.0);
    let direction = Vector::new(0.0, 0.0, 1.0);
    let ray = Ray::new(origin, direction);
    let sphere = Object::Sphere(Sphere::default());
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
    let sphere = Object::Sphere(Sphere::default());
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
    let sphere = Object::Sphere(Sphere::default());
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
    let sphere = Object::Sphere(Sphere::default());
    let intersection = ray.intersect(&sphere).expect("No intersection found!");

    assert_eq!(intersection.len(), 2);
    assert_eq!(intersection[0].t(), -6.0);
    assert_eq!(intersection[1].t(), -4.0);
}

#[test]
fn a_sphere_default_transform() {
    let s = Object::Sphere(Sphere::default());
    assert_eq!(s.transform(), &Matrix::<4>::identity());
}

#[test]
fn changing_a_sphere_transform() {
    let t = Matrix::<4>::translation(2.0, 3.0, 4.0);
    let s = Object::Sphere(Sphere::with_transform(t.clone()));
    assert_eq!(s.transform(), &t);
}

#[test]
fn intersecting_a_scaled_sphere_with_a_ray() {
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let t = Matrix::<4>::scaling(2.0, 2.0, 2.0);
    let s = Object::Sphere(Sphere::with_transform(t));
    let xs = r.intersect(&s).expect("No intersection found!");
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t(), 3.0);
    assert_eq!(xs[1].t(), 7.0);
}

#[test]
fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
    let s = Object::Sphere(Sphere::default());
    let n = s.normal_at(&Point::new(1.0, 0.0, 0.0));
    assert_eq!(n, Vector::new(1.0, 0.0, 0.0));
}

#[test]
fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
    let s = Object::Sphere(Sphere::default());
    let n = s.normal_at(&Point::new(0.0, 1.0, 0.0));
    assert_eq!(n, Vector::new(0.0, 1.0, 0.0));
}

#[test]
fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
    let s = Object::Sphere(Sphere::default());
    let n = s.normal_at(&Point::new(0.0, 0.0, 1.0));
    assert_eq!(n, Vector::new(0.0, 0.0, 1.0));
}

#[test]
fn the_normal_on_a_sphere_at_a_nonaxial_point() {
    let s = Object::Sphere(Sphere::default());
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
    let t = Matrix::<4>::translation(0.0, 1.0, 0.0);
    let s = Object::Sphere(Sphere::with_transform(t));
    let n = s.normal_at(&Point::new(0.0, 1.70711, -0.70711));
    assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
}

#[test]
fn computing_the_normal_on_a_transformed_sphere() {
    let m = Matrix::<4>::scaling(1.0, 0.5, 1.0) * Matrix::<4>::rotation_z(PI / 5.0);
    let s = Object::Sphere(Sphere::with_transform(m));
    let n = s.normal_at(&Point::new(
        0.0,
        2.0_f64.sqrt() / 2.0,
        -2.0_f64.sqrt() / 2.0,
    ));
    assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
}

#[test]
fn a_sphere_has_a_default_material() {
    let s = Object::Sphere(Sphere::default());
    let m = s.material();
    assert_eq!(m, &Material::default());
}

#[test]
fn a_sphere_may_be_assigned_a_material() {
    let mut m = Material::default();
    m.set_ambient(1.0);
    let s = Object::Sphere(Sphere::with_material(m.clone()));
    assert_eq!(s.material(), &m);
}

#[test]
fn a_helper_for_producing_a_sphere_with_a_glassy_material() {
    let s = Sphere::glassy();
    assert_eq!(s.transform(), &Matrix::identity());
    assert_eq!(s.material().transparency(), 1.0);
    assert_eq!(s.material().refractive_index(), 1.5);
}

// TODO
#[test]
fn finding_n1_and_n2_at_various_intersections() {
    let mut a = Object::Sphere(Sphere::glassy());
    a.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
    a.material_mut().set_refractive_index(1.5);

    let mut b = Object::Sphere(Sphere::glassy());
    b.set_transform(Matrix::translation(0.0, 0.0, -0.25));
    b.material_mut().set_refractive_index(2.0);

    let mut c = Object::Sphere(Sphere::glassy());
    c.set_transform(Matrix::translation(0.0, 0.0, 0.25));
    c.material_mut().set_refractive_index(2.5);

    let r = Ray::new(Point::new(0.0, 0.0, -4.0), Vector::new(0.0, 0.0, 1.0));
    let xs = Intersections::new(vec![
        Intersection::new(2.0, &a),
        Intersection::new(2.75, &b),
        Intersection::new(3.25, &c),
        Intersection::new(4.75, &b),
        Intersection::new(5.25, &c),
        Intersection::new(6.0, &a),
    ]);
    let examples = vec![
        (1.0, 1.5),
        (1.5, 2.0),
        (2.0, 2.5),
        (2.0, 2.5),
        (2.5, 2.5),
        (2.5, 1.5),
        (1.5, 1.0),
    ];
    for i in 0..examples.len() {
        let comps = xs[i].prepare_computations(&r, Some(&xs));
        assert_eq!(comps.n1(), examples[i].0);
        assert_eq!(comps.n2(), examples[i].1);
    }
}
