use super::matrix::Matrix;

pub trait Transform {
    fn transform(&self, m: &Matrix<4_usize>) -> Self;
}
