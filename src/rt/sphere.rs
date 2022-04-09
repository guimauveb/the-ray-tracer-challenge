use {
    super::{
        material::Material,
        matrix::{Matrix, Transpose},
        normal::Normal,
    },
    crate::primitive::{point::Point, tuple::Tuple, vector::Vector},
};

#[derive(PartialEq, Debug)]
pub struct Sphere {
    id: u32, // TODO ?
    origin: Point,
    radii: f64, // TODO ?
    transform: Matrix<4_usize>,
    material: Material,
}

impl Sphere {
    pub fn new(origin: Point, transform: Matrix<4_usize>, material: Material) -> Self {
        Sphere {
            id: 1,
            origin,
            radii: 1.0,
            transform,
            material,
        }
    }

    pub fn origin(&self) -> &Point {
        &self.origin
    }

    pub fn transform(&self) -> &Matrix<4_usize> {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Matrix<4_usize>) {
        self.transform = transform;
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            id: 1,
            origin: Point::new(0.0, 0.0, 0.0),
            radii: 1.0,
            transform: Matrix::<4_usize>::identity(),
            material: Material::default(),
        }
    }
}

impl Normal for Sphere {
    fn normal_at(&self, point: &Point) -> Vector {
        // Convert the point from world space to object space
        let object_point = &self.transform.inverse().expect("Matrix is not invertible!") * point;
        let object_normal = object_point - self.origin;
        // To keep the normal perpendicular to their surface, we multiply the object normal by the inverted then transposed object transform (matrix).
        let world_normal = self
            .transform
            .inverse()
            .expect("Matrix is not invertible!")
            .transpose()
            * object_normal;

        world_normal.normalize()
    }
}
