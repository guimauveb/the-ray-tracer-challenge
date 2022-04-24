use {super::color::Color, crate::tuple::point::Point};

#[derive(PartialEq, Debug)]
pub struct PointLight {
    position: Point,
    intensity: Color,
}

impl PointLight {
    pub const fn new(position: Point, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }

    pub const fn position(&self) -> &Point {
        &self.position
    }

    pub const fn intensity(&self) -> &Color {
        &self.intensity
    }
}
