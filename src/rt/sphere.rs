use {
    super::shape::Shape,
    crate::primitive::{point::Point, tuple::Tuple},
};

#[derive(PartialEq, Debug)]
pub struct Sphere {
    id: u32, //?
    origin: Point,
    radii: f64, //?
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            id: 1,
            origin: Point::new(0.0, 0.0, 0.0), // TODO
            radii: 1.0,
        }
    }

    pub fn origin(&self) -> Point {
        self.origin
    }
}

impl Shape for Sphere {}
