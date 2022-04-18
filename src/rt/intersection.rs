use super::sphere::Sphere;

#[derive(Debug, PartialEq, Clone)]
pub enum Intersection<'a> {
    Sphere(f64, &'a Sphere),
    //...
}

// To implement for each enum variant
pub trait IntersectionObject<T> {
    fn object(&self) -> &T;
}

impl<'a> Intersection<'a> {
    pub fn t(&self) -> f64 {
        match self {
            Intersection::Sphere(t, _) => *t,
        }
    }
}

/* NOTE - Could use a dyn method here...
 *   fn object(&self) -> &dyn Shape
 * ...but to make the code as fast as possible I'd rather avoid the level of inderection brought by it. */
impl<'a> IntersectionObject<Sphere> for Intersection<'a> {
    fn object(&self) -> &Sphere {
        let Self::Sphere(_, object) = self;
        object
    }
}
