use {
    super::{
        normal::Normal,
        ray::{Position, Ray},
        sphere::Sphere,
    },
    crate::primitive::{point::Point, vector::Vector},
};

#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
/// Represents the intersection between a ray and an object at point `t` along the ray.
pub enum Intersection<'object> {
    Sphere(f64, &'object Sphere),
}

// To implement for each enum variant
// NOTE - Might need to return an Object enum containing the actual object and deref it.
pub trait IntersectionObject<O> {
    fn object(&self) -> &O;
}

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

impl<'object> Intersection<'object> {
    pub fn t(&self) -> f64 {
        match self {
            Intersection::Sphere(t, _) => *t,
        }
    }

    fn is_inside(eye_vector: &Vector, &normal: &Vector) -> bool {
        eye_vector.dot(&normal) < 0.0
    }

    pub fn prepare_computations(&'object self, ray: &Ray) -> Computation<'object> {
        let point = ray.position(self.t());
        let eye_vector = -ray.direction();
        let normal_vector = self.object().normal_at(&point);
        let inside = Self::is_inside(&eye_vector, &normal_vector);

        Computation {
            intersection: &self,
            point,
            eye_vector,
            // If the hit occurs inside the shape, we inverse the normal to get the reflection on the "inside" material.
            normal_vector: if inside {
                -normal_vector
            } else {
                normal_vector
            },
            inside,
        }
    }
}

impl<'object> Computation<'object> {
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

/* NOTE - Could use a dyn method here...
 *      fn object(&self) -> &dyn Shape
 *  ...but to make the code as fast as possible I'd rather avoid the level of inderection brought by it.
 */
impl<'object> IntersectionObject<Sphere> for Intersection<'object> {
    fn object(&self) -> &Sphere {
        // Intersection can only be a Sphere variant for now, but we'll add more variants later.
        #[allow(irrefutable_let_patterns)]
        if let Self::Sphere(_, object) = self {
            object
        } else {
            unreachable!()
        }
    }
}
