use {
    super::{material::Material, matrix::Matrix, shape::Shape},
    crate::tuple::{point::Point, vector::Vector},
};

/// Since a plane has no curvature, the normal is always a `Vector { 0.0, 1.0, 0.0 }`.
const PLANE_NORMAL: Vector = Vector::new(0.0, 1.0, 0.0);

#[derive(Debug)]
pub struct Plane {
    transform: Matrix<4>,
    material: Material,
}

impl PartialEq for Plane {
    fn eq(&self, rhs: &Self) -> bool {
        std::ptr::eq(self, rhs)
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            transform: Matrix::identity(),
            material: Material::default(),
        }
    }
}

impl Shape for Plane {
    fn transform(&self) -> &Matrix<4> {
        &self.transform
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn set_transform(&mut self, transform: Matrix<4>) {
        self.transform = transform;
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn normal_at(&self, _: &Point) -> Vector {
        let world_normal = self.transform.inverse().unwrap().transpose() * PLANE_NORMAL;

        world_normal.normalized()
    }
}
