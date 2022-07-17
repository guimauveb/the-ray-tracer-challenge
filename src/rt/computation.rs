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

    /// See [Reflections and refractions in Ray Tracing, Bram de Greve](https://graphics.stanford.edu/courses/cs148-10-summer/docs/2006--degreve--reflection_refraction.pdf)
    pub fn schlick(&self) -> f64 {
        // Find the cosine of the angle between the eye and normal vectors.
        let mut cos = self.eye_vector.dot(&self.normal_vector);
        // Total internal reflection can only occur if n1 > n2
        if self.n1() > self.n2() {
            let n = self.n1() / self.n2();
            let sin2_t = n.powi(2) * (1.0 - cos.powi(2));
            if sin2_t > 1.0 {
                return 1.0;
            }
            // Compute cos(theta_t) using trig identity
            let cos_t = (1.0 - sin2_t).sqrt();
            // When n1 > n2, use cos(theta_t) instead
            cos = cos_t;
        }
        let r0 = ((self.n1() - self.n2()) / (self.n1() + self.n2())).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}
