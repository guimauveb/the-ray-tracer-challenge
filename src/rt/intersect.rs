use super::intersections::Intersections;

/// Describes how a ray intersects with one or multiple objects.
pub trait Intersect<'object, T> {
    fn intersect(&self, object: &'object T) -> Option<Intersections<'object>>;
}
