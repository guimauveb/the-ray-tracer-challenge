/// Describes how a ray intersects with one or multiple objects.
/// `O` is the object (most likely an Object or a World (composed of many objects)) being intersected.
/// `I` is the type of the intersection returned (could be of type `[f64; 2]` or `Intersections`)
pub trait Intersect<'object, O, I> {
    fn intersect(&self, object: &'object O) -> Option<I>;
}
