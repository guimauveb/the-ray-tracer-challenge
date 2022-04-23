use {
    super::{
        matrix::Matrix,
        sphere::Sphere,
        {material::Material, normal::Normal},
    },
    crate::primitive::{point::Point, vector::Vector},
};

/// Wrapper around an object used in the ray tracer (Sphere, Cube, etc).
#[derive(PartialEq, Debug)]
#[non_exhaustive]
pub enum Object {
    Sphere(Sphere),
    //...
}

impl Normal for Object {
    fn normal_at(&self, point: &Point) -> Vector {
        match self {
            Self::Sphere(sphere) => sphere.normal_at(point),
        }
    }
}

impl Object {
    pub const fn material(&self) -> &Material {
        match self {
            Self::Sphere(sphere) => sphere.material(),
        }
    }

    pub const fn transform(&self) -> &Matrix<4> {
        match self {
            Self::Sphere(sphere) => sphere.transform(),
        }
    }
}
