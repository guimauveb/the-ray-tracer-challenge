use super::intersection::Intersection;

pub trait Intersect<T> {
    fn intersect(&self, object: &T) -> Option<[Intersection<Self>; 2]>
    where
        Self: Sized;
}
