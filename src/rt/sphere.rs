use {
    super::{matrix::Matrix, shape::Shape},
    crate::primitive::{point::Point, tuple::Tuple},
};

#[derive(PartialEq, Debug)]
pub struct Sphere {
    id: u32, // TODO ?
    origin: Point,
    radii: f64, // TODO ?
    transform: Matrix<4_usize>,
}

impl Sphere {
    pub fn new(origin: Point, transform: Matrix<4_usize>) -> Self {
        Sphere {
            id: 1,
            origin,
            radii: 1.0,
            transform,
        }
    }

    pub fn origin(&self) -> &Point {
        &self.origin
    }

    pub fn transform(&self) -> &Matrix<4_usize> {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Matrix<4_usize>) {
        self.transform = transform;
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            id: 1,
            origin: Point::new(0.0, 0.0, 0.0),
            radii: 1.0,
            transform: Matrix::<4_usize>::identity(),
        }
    }
}

impl Shape for Sphere {}
