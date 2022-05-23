use {
    super::intersection::Intersection,
    crate::tuple::{point::Point, vector::Vector},
};

#[derive(Debug, PartialEq)]
pub struct Computation<'object> {
    /// In the book, Computation is implemented with the fields `t` and `object` which refer to the Intersection fields.
    /// I find it cleaner to reference the intersection itself and access those fields from it.
    intersection: &'object Intersection<'object>,
    point: Point,
    eye_vector: Vector,
    /// If the intersection hit occurs inside the shape, we inverse the normal to get the reflection on the "inside" material.
    normal_vector: Vector,
    inside: bool,
    over_point: Point,
}

impl<'object> Computation<'object> {
    pub const fn new(
        intersection: &'object Intersection<'object>,
        point: Point,
        eye_vector: Vector,
        normal_vector: Vector,
        inside: bool,
        over_point: Point,
    ) -> Self {
        Self {
            intersection,
            point,
            eye_vector,
            normal_vector,
            inside,
            over_point,
        }
    }

    pub const fn intersection(&self) -> &Intersection {
        self.intersection
    }

    pub const fn inside(&self) -> bool {
        self.inside
    }

    pub const fn eye_vector(&self) -> &Vector {
        &self.eye_vector
    }

    pub const fn point(&self) -> &Point {
        &self.point
    }

    pub const fn normal_vector(&self) -> &Vector {
        &self.normal_vector
    }

    pub const fn over_point(&self) -> &Point {
        &self.over_point
    }
}
