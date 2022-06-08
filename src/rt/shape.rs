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
    /// Before computing the normal at some point,
    /// all shapes must first convert the point to
    /// object space by multiplying it by the inverse
    /// of the shape transformation matrix.
    /// Then, the normal obtained must be converted
    /// to world space by multiplying it by the inverse
    /// of the transpose of the shape transformation matrix,
    /// and finally be normalized.
    fn normal_at(&self, point: &Point) -> Vector;
}
