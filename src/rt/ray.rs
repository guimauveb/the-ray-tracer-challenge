use crate::primitive::{point::Point, vector::Vector};

pub struct Ray {
    origin: Point,
    direction: Vector,
}

pub trait Position {
    fn position(&self, distance: f64) -> Point;
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn direction(&self) -> Vector {
        self.direction
    }
}

impl Position for Ray {
    fn position(&self, distance: f64) -> Point {
        self.origin + self.direction * distance
    }
}
