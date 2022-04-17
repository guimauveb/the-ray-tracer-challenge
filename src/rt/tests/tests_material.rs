#[cfg(test)]
use crate::{
    primitive::{point::Point, tuple::Tuple, vector::Vector},
    rt::{color::Color, lighting::Lighting, material::Material, point_light::PointLight},
};

#[test]
fn the_default_material() {
    let m = Material::default();
    assert_eq!(m.color(), &Color::new(1.0, 1.0, 1.0));
    assert_eq!(m.ambient(), 0.1);
    assert_eq!(m.diffuse(), 0.9);
    assert_eq!(m.shininess(), 200.0);
}

#[test]
fn lighting_with_the_eye_between_the_light_and_the_surface() {
    let m = Material::default();
    let position = Point::new(0.0, 0.0, 0.0);
    let eye = Vector::new(0.0, 0.0, -1.0);
    let normal = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let result = m.lighting(&light, &position, &eye, &normal);
    assert_eq!(result, Color::new(1.9, 1.9, 1.9));
}

#[test]
fn lighting_with_the_eye_between_the_light_and_surface_eye_offset_45deg() {
    let m = Material::default();
    let position = Point::new(0.0, 0.0, 0.0);
    let eye = Vector::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
    let normal = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let result = m.lighting(&light, &position, &eye, &normal);
    assert_eq!(result, Color::new(1.0, 1.0, 1.0));
}

#[test]
fn lighting_with_eye_opposite_surface_light_offset_45deg() {
    let m = Material::default();
    let position = Point::new(0.0, 0.0, 0.0);
    let eye = Vector::new(0.0, 0.0, -1.0);
    let normal = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let result = m.lighting(&light, &position, &eye, &normal);
    assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
}

#[test]
fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
    let m = Material::default();
    let position = Point::new(0.0, 0.0, 0.0);
    let eye = Vector::new(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
    let normal = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let result = m.lighting(&light, &position, &eye, &normal);
    assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
}

#[test]
fn lighting_with_the_eye_behind_the_suface() {
    let m = Material::default();
    let position = Point::new(0.0, 0.0, 0.0);
    let eye = Vector::new(0.0, 0.0, -1.0);
    let normal = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
    let result = m.lighting(&light, &position, &eye, &normal);
    assert_eq!(result, Color::new(0.1, 0.1, 0.1));
}