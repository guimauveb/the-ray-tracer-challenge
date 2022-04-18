use {super::intersection::Intersection, std::ops::Index};

/// Wrapper around a `Vec<Intersection<'objects>>` that keeps intersections always sorted.
pub struct Intersections<'objects>(Vec<Intersection<'objects>>);

impl<'objects> Intersections<'objects> {
    fn sort(intersections: &mut [Intersection<'objects>]) {
        intersections.sort_by(|a, b| a.t().partial_cmp(&b.t()).unwrap());
    }

    pub fn new(mut intersections: Vec<Intersection<'objects>>) -> Self {
        Self::sort(&mut intersections);
        Self(intersections)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    // TODO - Use a generic (Self, IntoIterator<Item = Intersection>)?
    pub fn extend(&mut self, intersections: Self) {
        self.0.extend(intersections.0);
        Self::sort(&mut self.0);
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    /* If all intersections are positive, the iterator will stop at the first intersection in the list and return it,
     * so we don't have to check if all intersections are positive and then return the first element of the list.
     * Return None if there is no positive intersection. */
    pub fn hit(&self) -> Option<&Intersection<'objects>> {
        self.0.iter().find(|&i| i.t() > 0.0)
    }
}

type Idx = usize;

impl<'objects> Index<Idx> for Intersections<'objects> {
    type Output = Intersection<'objects>;
    fn index(&self, idx: Idx) -> &Self::Output {
        &self.0[idx]
    }
}
