use {
    super::{computation::Computation, object::Object, ray::Ray, shape::Shape},
    crate::{float::epsilon::EPSILON, tuple::vector::Vector},
};

/// Used in replacement of EPSILON if needed.
const OFFSET: f64 = EPSILON;

#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
/// Represents the intersection between a ray and an object at point `t` along the ray.
pub struct Intersection<'object> {
    t: f64,
    object: &'object Object,
}

impl<'object> Intersection<'object> {
    pub const fn new(t: f64, object: &'object Object) -> Self {
        Self { t, object }
    }

    pub const fn t(&self) -> f64 {
        self.t
    }

    pub const fn object(&self) -> &Object {
        self.object
    }

    fn is_inside(eye_vector: &Vector, normal: &Vector) -> bool {
        eye_vector.dot(normal) < 0.0
    }

    // TODO
    pub fn prepare_computations(
        &'object self,
        ray: &Ray,
        intersections: Option<&[Intersection]>,
    ) -> Computation<'object> {
        let point = ray.position(self.t());
        let eye_vector = -ray.direction();
        let normal_vector = self.object().normal_at(&point);
        let inside = Self::is_inside(&eye_vector, &normal_vector);
        // If the hit occurs inside the shape, we inverse the normal to get the reflection on the "inside" material.
        let normal_vector = if inside {
            -normal_vector
        } else {
            normal_vector
        };
        let over_point = &point + (&normal_vector * OFFSET);
        let reflect_vector = ray.direction().reflect(&normal_vector);

        Computation::new(
            self,
            point,
            eye_vector,
            normal_vector,
            inside,
            over_point,
            reflect_vector,
            (1.0, 1.0), // TODO - Refractive indices
        )
    }
}
