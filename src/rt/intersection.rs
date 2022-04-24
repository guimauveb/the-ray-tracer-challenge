use {
    super::{computation::Computation, normal::Normal, object::Object, ray::Ray},
    crate::tuple::vector::Vector,
};

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

    pub const fn object(&self) -> &'object Object {
        self.object
    }

    fn is_inside(eye_vector: &Vector, normal: &Vector) -> bool {
        eye_vector.dot(normal) < 0.0
    }

    pub fn prepare_computations(&'object self, ray: &Ray) -> Computation<'object> {
        let point = ray.position(self.t());
        let eye_vector = -ray.direction();
        let normal_vector = self.object().normal_at(&point);
        let inside = Self::is_inside(&eye_vector, &normal_vector);

        Computation::new(
            self,
            point,
            eye_vector,
            // If the hit occurs inside the shape, we inverse the normal to get the reflection on the "inside" material.
            if inside {
                -normal_vector
            } else {
                normal_vector
            },
            inside,
        )
    }
}
