use {
    super::{intersection::Intersection, object::Object},
    std::ops::Index,
};

/// Wrapper around a `Vec<Intersection<'objects>>` that keeps intersections sorted.
pub struct Intersections<'objects>(Vec<Intersection<'objects>>);

impl<'object> From<([f64; 2], &'object Object)> for Intersections<'object> {
    /// Used to transform intersections computed from an Object variant to intersections refering the Object enum.
    fn from((intersections, object): ([f64; 2], &'object Object)) -> Intersections<'object> {
        Self::new(vec![
            Intersection::new(intersections[0], object),
            Intersection::new(intersections[1], object),
        ])
    }
}

type Idx = usize;

impl<'objects> Index<Idx> for Intersections<'objects> {
    type Output = Intersection<'objects>;
    fn index(&self, idx: Idx) -> &Self::Output {
        &self.0[idx]
    }
}

impl<'object> IntoIterator for Intersections<'object> {
    type Item = Intersection<'object>;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

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

    // NOTE - Use a generic (Self, IntoIterator<Item = Intersection>)?
    pub fn append(&mut self, intersections: &mut Self) {
        self.0.append(&mut intersections.0);
        Self::sort(&mut self.0);
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /* If all intersections are positive, the iterator will stop at the first intersection in the list and return it,
     * so we don't have to check if all intersections are positive and then return the first element of the list.
     * Return None if there is no positive intersection. */
    pub fn hit(&self) -> Option<&Intersection<'objects>> {
        self.0.iter().find(|&i| i.t() > 0.0)
    }
}
