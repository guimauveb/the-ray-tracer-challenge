#[derive(Debug)]
pub struct Point2d {
    pub x: usize,
    pub y: usize,
}

impl Point2d {
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
