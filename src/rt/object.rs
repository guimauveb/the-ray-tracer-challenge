use super::sphere::Sphere;

/// Lists all objects used in the ray tracer (Sphere, etc).
#[derive(PartialEq, Debug)]
#[non_exhaustive]
pub enum Object {
    Sphere(Sphere),
    //...
}
