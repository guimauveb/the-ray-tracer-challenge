#[cfg(test)]
use crate::{
    primitive::point::Point,
    rt::{color::Color, point_light::PointLight},
};

#[test]
fn a_point_light_has_a_position_and_intensity() {
    let intensity = Color::white();
    let position = Point::new(0.0, 0.0, 0.0);
    let light = PointLight::new(position, intensity);
    assert_eq!(light.position(), &position);
    assert_eq!(light.intensity(), &intensity);
}
