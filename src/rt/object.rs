use super::sphere::Sphere;

/// Any object used in the ray tracer (Sphere, Cube, etc).
#[derive(PartialEq, Debug)]
#[non_exhaustive]
pub enum Object {
    Sphere(Sphere),
    //...
}
