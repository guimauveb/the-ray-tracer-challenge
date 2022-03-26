use super::intersection::Intersection;

pub struct Intersections<'a>(&'a [Intersection<'a>]);

impl<'a> Intersections<'a> {
    pub fn new(intersections: &'a mut [Intersection<'a>]) -> Self {
        intersections.sort_by(|a, b| a.t().partial_cmp(&b.t()).unwrap());
        Self(intersections)
    }
}
