use super::intersections::Intersections;

/// Describes how two objects (e.g a ray and a sphere) intersects. NOTE - Self = Ray only?
pub trait Intersect<'a, T> {
    fn intersect(&self, object: &'a T) -> Option<Intersections<'a>>;
}
