use crate::primitive::{point::Point, vector::Vector};

pub trait Normal {
    fn normal_at(&self, point: &Point) -> Vector;
}
