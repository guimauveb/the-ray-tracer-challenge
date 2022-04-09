use {
    super::{color::Color, point_light::PointLight},
    crate::primitive::{point::Point, vector::Vector},
};

pub trait Lighting {
    fn lighting(&self, light: &PointLight, point: &Point, eye: &Vector, normal: &Vector) -> Color;
}
