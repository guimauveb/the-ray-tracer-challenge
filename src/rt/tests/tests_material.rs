#[cfg(test)]
use crate::{
    approx_eq::ApproxEq,
    rt::{color::Color, material::Material, point_light::PointLight},
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
    let m = Material::default();
    let position = Point::new(0.0, 0.0, 0.0);
    let eye = Vector::new(0.0, 0.0, -1.0);
    let normal = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::white());
    let result = m.lighting(&light, &position, &eye, &normal, false);
    assert_eq!(result, Color::new(1.9, 1.9, 1.9));
}

#[test]
fn lighting_with_the_eye_between_the_light_and_surface_eye_offset_45deg() {
    let m = Material::default();
    let position = Point::new(0.0, 0.0, 0.0);
    let eye = Vector::new(0.0, 2.0_f32.sqrt() / 2.0, -2.0_f32.sqrt() / 2.0);
    let normal = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::white());
    let result = m.lighting(&light, &position, &eye, &normal, false);
    assert_eq!(result, Color::white());
}

#[test]
fn lighting_with_eye_opposite_surface_light_offset_45deg() {
    let m = Material::default();
    let position = Point::new(0.0, 0.0, 0.0);
    let eye = Vector::new(0.0, 0.0, -1.0);
    let normal = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::white());
    let result = m.lighting(&light, &position, &eye, &normal, false);
    assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
}

#[test]
fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
    let m = Material::default();
    let position = Point::new(0.0, 0.0, 0.0);
    let eye = Vector::new(0.0, -2.0_f32.sqrt() / 2.0, -2.0_f32.sqrt() / 2.0);
    let normal = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::white());
    let result = m.lighting(&light, &position, &eye, &normal, false);
    assert_eq!(result, Color::new(1.6363853, 1.6363853, 1.6363853));
}

#[test]
fn lighting_with_the_eye_behind_the_suface() {
    let m = Material::default();
    let position = Point::new(0.0, 0.0, 0.0);
    let eye = Vector::new(0.0, 0.0, -1.0);
    let normal = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::white());
    let result = m.lighting(&light, &position, &eye, &normal, false);
    assert_eq!(result, Color::new(0.1, 0.1, 0.1));
}

#[test]
fn lighting_with_the_surface_in_shadow() {
    let m = Material::default();
    let position = Point::new(0.0, 0.0, 0.0);
    let eye = Vector::new(0.0, 0.0, -1.0);
    let normal = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::white());
    let in_shadow = true;
    let result = m.lighting(&light, &position, &eye, &normal, in_shadow);
    assert_eq!(result, Color::new(0.1, 0.1, 0.1));
}
