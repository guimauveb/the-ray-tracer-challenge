pub struct Intersection<'a, T> {
    t: f64,
    object: &'a T,
}

impl<'a, T> Intersection<'a, T> {
    pub fn new(t: f64, object: &'a T) -> Self {
        Self { t, object }
    }
    pub fn t(&self) -> f64 {
        self.t
    }
    pub fn object(&self) -> &T {
        self.object
    }
}
