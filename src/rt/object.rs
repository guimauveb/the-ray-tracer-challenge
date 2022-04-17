use super::sphere::Sphere;

/// Lists all objects used in the ray tracer (Sphere, etc).
#[derive(PartialEq, Debug)]
pub enum Object {
    Sphere(Sphere),
    //...
}
