use {super::epsilon::EPSILON, crate::approx_eq::ApproxEq};

impl ApproxEq for f64 {
    fn approx_eq(self, rhs: Self) -> bool {
        (self - rhs).abs() < EPSILON
    }
}
