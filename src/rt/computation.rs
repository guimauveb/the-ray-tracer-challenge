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
    under_point: Point,
    reflect_vector: Vector,
    /// `(n1, n2)` are the names given to the refractive indices of the material on either side of the ray,
    /// with `n1` belonging to the material being "exited", and `n2` belonging to the material being "entered".
    refractive_indices: (f64, f64),
}

impl<'object> Computation<'object> {
    pub const fn new(
        intersection: &'object Intersection<'object>,
        point: Point,
        eye_vector: Vector,
        normal_vector: Vector,
        inside: bool,
        over_point: Point,
        under_point: Point,
        reflect_vector: Vector,
        /* (n1, n2) */ refractive_indices: (f64, f64),
    ) -> Self {
        Self {
            intersection,
            point,
            eye_vector,
            normal_vector,
            inside,
            over_point,
            under_point,
            reflect_vector,
            refractive_indices,
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

    pub const fn under_point(&self) -> &Point {
        &self.under_point
    }

    pub const fn reflect_vector(&self) -> &Vector {
        &self.reflect_vector
    }

    pub const fn n1(&self) -> f64 {
        self.refractive_indices.0
    }

    pub const fn n2(&self) -> f64 {
        self.refractive_indices.1
    }
}
