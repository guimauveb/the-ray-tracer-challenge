#[cfg(test)]
use crate::{
    primitive::{point::Point, tuple::Tuple, vector::Vector},
    rt::color::Color,
};

#[test]
fn a_point_light_has_a_position_and_intensity() {
    let intensity = Color::white();
    let position = Point::new(0.0, 0.0, 0.0);
    //let light = point_light(position, intensity);
    //assert_eq!(light.position(), position);
    //assert_eq!(light.intensity(), intensity);
}
