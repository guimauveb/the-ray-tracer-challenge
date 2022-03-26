use super::intersection::Intersection;

pub trait Intersect<T> {
    fn intersect<'a>(&self, object: &'a T) -> Option<[Intersection<'a>; 2]>;
}
