use {super::intersection::Intersection, std::ops::Index};

pub struct Intersections<'a>(Vec<Intersection<'a>>);

impl<'a> Intersections<'a> {
    pub fn new(mut intersections: Vec<Intersection<'a>>) -> Self {
        intersections.sort_by(|a, b| a.t().partial_cmp(&b.t()).unwrap());
        Self(intersections)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /* If all intersections are positive, the iterator will stop at the first intersection in the list and return it,
     * so we don't have to check if all intersections are positive and then return the first element of the list.
     * Return None if there is no positive intersection. */
    pub fn hit(&self) -> Option<&Intersection<'a>> {
        self.0.iter().find(|&i| i.t() > 0.0)
    }
}

type Idx = usize;

impl<'a> Index<Idx> for Intersections<'a> {
    type Output = Intersection<'a>;
    fn index(&self, idx: Idx) -> &Self::Output {
        &self.0[idx]
    }
}
