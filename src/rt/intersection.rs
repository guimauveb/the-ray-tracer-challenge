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
pub enum Intersection<'object> {
    Sphere(f64, &'object Sphere),
    //...
}

// To implement for each enum variant
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
}

impl<'object> Intersection<'object> {
    pub fn t(&self) -> f64 {
        match self {
            Intersection::Sphere(t, _) => *t,
        }
    }

    pub fn prepare_computations(&'object self, ray: &Ray) -> Computation<'object> {
        let point = ray.position(self.t());
        Computation {
            intersection: &self,
            point,
            eye_vector: -ray.direction(),
            normal_vector: self.object().normal_at(&point),
        }
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
