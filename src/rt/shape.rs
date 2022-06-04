use {
    super::{material::Material, matrix::Matrix},
    crate::tuple::{point::Point, vector::Vector},
};

/// Methods common to all objects.
pub trait Shape {
    fn get_transform(&self) -> &Matrix<4>;
    fn set_transform(&mut self, transform: Matrix<4>);
    fn get_material(&self) -> &Material;
    fn set_material(&mut self, material: Material);
    fn normal_at(&self, point: &Point) -> Vector;
}
