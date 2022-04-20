use super::intersections::Intersections;

/// Describes how a ray intersects with one or multiple objects.
pub trait Intersect<'object, O> {
    fn intersect(&self, object: &'object O) -> Option<Intersections<'object>>;
}
