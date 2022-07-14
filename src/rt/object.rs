use {
    super::{material::Material, matrix::Matrix, plane::Plane, shape::Shape, sphere::Sphere},
    crate::tuple::{point::Point, vector::Vector},
};

/// Wrapper around an object used in the World (Sphere, Cube, etc).
/// We could have gone the dynamic dispatch way instead by using
/// the `dyn Shape` trait object type everywhere we need to use
/// any object in some collection, but for performance reasons,
/// I'll stick to a good old enum.
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
}

impl From<Sphere> for Object {
    fn from(sphere: Sphere) -> Self {
        Self::Sphere(sphere)
    }
}

impl From<Plane> for Object {
    fn from(plane: Plane) -> Self {
        Self::Plane(plane)
    }
}

impl Shape for Object {
    fn material(&self) -> &Material {
        match self {
            Self::Sphere(sphere) => sphere.material(),
            Self::Plane(plane) => plane.material(),
        }
    }

    fn material_mut(&mut self) -> &mut Material {
        match self {
            Self::Sphere(sphere) => sphere.material_mut(),
            Self::Plane(plane) => plane.material_mut(),
        }
    }

    fn transform(&self) -> &Matrix<4> {
        match self {
            Self::Sphere(sphere) => sphere.transform(),
            Self::Plane(plane) => plane.transform(),
        }
    }

    fn normal_at(&self, point: &Point) -> Vector {
        match self {
            Self::Sphere(sphere) => sphere.normal_at(point),
            Self::Plane(plane) => plane.normal_at(point),
        }
    }

    fn set_transform(&mut self, transform: Matrix<4>) {
        match self {
            Self::Sphere(sphere) => sphere.set_transform(transform),
            Self::Plane(plane) => plane.set_transform(transform),
        }
    }

    fn set_material(&mut self, material: Material) {
        match self {
            Self::Sphere(sphere) => sphere.set_material(material),
            Self::Plane(plane) => plane.set_material(material),
        }
    }
}
