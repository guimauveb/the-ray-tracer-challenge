#[cfg(test)]
use crate::{
    approx_eq::ApproxEq,
    rt::{
        color::Color, material::Material, object::Object, patterns::Stripe,
        point_light::PointLight, sphere::Sphere,
    },
    tuple::{point::Point, vector::Vector},
};

#[test]
fn the_default_material() {
    let m = Material::default();
    assert_eq!(m.color(), &Color::white());
    assert!(m.ambient().approx_eq(0.1));
    assert!(m.diffuse().approx_eq(0.9));
    assert!(m.shininess().approx_eq(200.0));
}

#[test]
fn lighting_with_the_eye_between_the_light_and_the_surface() {
    let object = Object::Sphere(Sphere::default());
    let m = Material::default();
    let position = Point::new(0.0, 0.0, 0.0);
    let eye = Vector::new(0.0, 0.0, -1.0);
    let normal = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::white());
    let result = m.lighting(&object, &light, &position, &eye, &normal, false);
    assert_eq!(result, Color::new(1.9, 1.9, 1.9));
}

#[test]
fn lighting_with_the_eye_between_the_light_and_surface_eye_offset_45deg() {
    let object = Object::Sphere(Sphere::default());
    let m = Material::default();
    let position = Point::new(0.0, 0.0, 0.0);
    let eye = Vector::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
    let normal = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::white());
    let result = m.lighting(&object, &light, &position, &eye, &normal, false);
    assert_eq!(result, Color::white());
}

#[test]
fn lighting_with_eye_opposite_surface_light_offset_45deg() {
    let object = Object::Sphere(Sphere::default());
    let m = Material::default();
    let position = Point::new(0.0, 0.0, 0.0);
    let eye = Vector::new(0.0, 0.0, -1.0);
    let normal = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::white());
    let result = m.lighting(&object, &light, &position, &eye, &normal, false);
    assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
}

#[test]
fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
    let object = Object::Sphere(Sphere::default());
    let m = Material::default();
    let position = Point::new(0.0, 0.0, 0.0);
    let eye = Vector::new(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
    let normal = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::white());
    let result = m.lighting(&object, &light, &position, &eye, &normal, false);
    assert_eq!(
        result,
        Color::new(1.6363961030678928, 1.6363961030678928, 1.6363961030678928)
    );
}

#[test]
fn lighting_with_the_eye_behind_the_suface() {
    let object = Object::Sphere(Sphere::default());
    let m = Material::default();
    let position = Point::new(0.0, 0.0, 0.0);
    let eye = Vector::new(0.0, 0.0, -1.0);
    let normal = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::white());
    let result = m.lighting(&object, &light, &position, &eye, &normal, false);
    assert_eq!(result, Color::new(0.1, 0.1, 0.1));
}

#[test]
fn lighting_with_the_surface_in_shadow() {
    let object = Object::Sphere(Sphere::default());
    let m = Material::default();
    let position = Point::new(0.0, 0.0, 0.0);
    let eye = Vector::new(0.0, 0.0, -1.0);
    let normal = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::white());
    let in_shadow = true;
    let result = m.lighting(&object, &light, &position, &eye, &normal, in_shadow);
    assert_eq!(result, Color::new(0.1, 0.1, 0.1));
}

#[test]
fn lighting_with_a_pattern_applied() {
    let object = Object::Sphere(Sphere::default());
    let p = Stripe::new(Color::white(), Color::black(), None);
    let m = Material::new(
        Color::new(0.8, 1.0, 0.6),
        Some(p.into()),
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
    );
    let eye = Vector::new(0.0, 0.0, -1.0);
    let normal = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let c1 = m.lighting(
        &object,
        &light,
        &Point::new(0.9, 0.0, 0.0),
        &eye,
        &normal,
        false,
    );
    let c2 = m.lighting(
        &object,
        &light,
        &Point::new(1.1, 0.0, 0.0),
        &eye,
        &normal,
        false,
    );
    assert_eq!(c1, Color::white());
    assert_eq!(c2, Color::black());
}
