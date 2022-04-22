use {
    super::intersection::Intersection,
    crate::primitive::{point::Point, vector::Vector},
};

#[derive(Debug, PartialEq)]
pub struct Computation<'object> {
    /// In the book, Computation is implemented with the fields `t` and `object` which refer to the Intersection fields.
    /// I find it cleaner to reference the intersection itself and access those fields from it.
    pub intersection: &'object Intersection<'object>,
    pub point: Point,
    pub eye_vector: Vector,
    pub normal_vector: Vector,
    pub inside: bool,
}

impl<'object> Computation<'object> {
    pub fn new(
        intersection: &'object Intersection<'object>,
        point: Point,
        eye_vector: Vector,
        normal_vector: Vector,
        inside: bool,
    ) -> Self {
        Self {
            intersection,
            point,
            eye_vector,
            normal_vector,
            inside,
        }
    }

    pub fn inside(&self) -> bool {
        self.inside
    }

    pub fn eye_vector(&self) -> &Vector {
        &self.eye_vector
    }

    pub fn point(&self) -> &Point {
        &self.point
    }

    pub fn normal_vector(&self) -> &Vector {
        &self.normal_vector
    }
}
