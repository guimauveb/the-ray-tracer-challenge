pub trait ApproxEq<Rhs = Self> {
    fn approx_eq(self, rhs: Rhs) -> bool;
}
