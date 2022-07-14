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

        let (mut n1, mut n2) = (1.0, 1.0);
        if let Some(intersections) = intersections {
            let mut containers: Vec<&Object> = Vec::new();
            for intersection in intersections {
                if intersection == self {
                    n1 = if containers.is_empty() {
                        1.0
                    } else {
                        containers.last().unwrap().material().refractive_index()
                    };
                }

                if let Some(index) = containers
                    .iter()
                    .position(|&object| object == intersection.object())
                {
                    containers.remove(index);
                } else {
                    containers.push(intersection.object());
                }

                if intersection == self {
                    n2 = if containers.is_empty() {
                        1.0
                    } else {
                        containers.last().unwrap().material().refractive_index()
                    };
                    break;
                }
            }
        }

        Computation::new(
            self,
            point,
            eye_vector,
            normal_vector,
            inside,
            over_point,
            reflect_vector,
            (n1, n2),
        )
    }
}
