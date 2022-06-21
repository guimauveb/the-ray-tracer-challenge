use {
    super::{color::Color, matrix::Matrix, object::Object, shape::Shape},
    crate::tuple::point::Point,
};

#[derive(PartialEq, Debug, Clone)]
pub enum Pattern {
    Stripe(Stripe),
    Gradient(Gradient),
    Ring(Ring),
    Checkers(Checkers),
}

impl Pattern {
    pub const fn get_transform(&self) -> &Matrix<4> {
        match self {
            Self::Stripe(stripe) => stripe.get_transform(),
            Self::Gradient(gradient) => gradient.get_transform(),
            Self::Ring(ring) => ring.get_transform(),
            Self::Checkers(checkers) => checkers.get_transform(),
        }
    }

    pub fn set_transform(&mut self, transform: Matrix<4>) {
        match self {
            Self::Stripe(stripe) => stripe.set_transform(transform),
            Self::Gradient(gradient) => gradient.set_transform(transform),
            Self::Ring(ring) => ring.set_transform(transform),
            Self::Checkers(checkers) => checkers.set_transform(transform),
        }
    }

    pub fn pattern_at(&self, point: &Point) -> Color {
        match self {
            Self::Stripe(stripe) => stripe.stripe_at(point),
            Self::Gradient(gradient) => gradient.gradient_at(point),
            Self::Ring(ring) => ring.ring_at(point),
            Self::Checkers(checkers) => checkers.checkers_at(point),
        }
    }

    pub fn pattern_at_object(&self, object: &Object, point: &Point) -> Color {
        match self {
            Self::Stripe(stripe) => stripe.stripe_at_object(object, point),
            Self::Gradient(gradient) => gradient.gradient_at_object(object, point),
            Self::Ring(ring) => ring.ring_at_object(object, point),
            Self::Checkers(checkers) => checkers.checkers_at_object(object, point),
        }
    }
}

impl From<Stripe> for Pattern {
    fn from(stripe: Stripe) -> Self {
        Self::Stripe(stripe)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Stripe {
    a: Color,
    b: Color,
    transform: Matrix<4>,
}

impl Stripe {
    pub fn new(a: Color, b: Color, transform: Option<Matrix<4>>) -> Self {
        Self {
            a,
            b,
            transform: transform.map_or_else(Matrix::<4>::identity, |t| t),
        }
    }

    pub const fn a(&self) -> &Color {
        &self.a
    }

    pub const fn b(&self) -> &Color {
        &self.b
    }

    pub const fn get_transform(&self) -> &Matrix<4> {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Matrix<4>) {
        self.transform = transform;
    }

    pub fn stripe_at(&self, point: &Point) -> Color {
        if point.x().floor() % 2.0 == 0.0 {
            self.a.clone()
        } else {
            self.b.clone()
        }
    }

    pub fn stripe_at_object(&self, object: &Object, point: &Point) -> Color {
        let object_point = object.get_transform().inverse().unwrap() * point;
        let pattern_point = self.get_transform().inverse().unwrap() * object_point;

        self.stripe_at(&pattern_point)
    }
}

impl From<Gradient> for Pattern {
    fn from(gradient: Gradient) -> Self {
        Self::Gradient(gradient)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Gradient {
    a: Color,
    b: Color,
    transform: Matrix<4>,
}

impl Gradient {
    pub fn new(a: Color, b: Color, transform: Option<Matrix<4>>) -> Self {
        Self {
            a,
            b,
            transform: transform.map_or_else(Matrix::<4>::identity, |t| t),
        }
    }

    pub const fn get_transform(&self) -> &Matrix<4> {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Matrix<4>) {
        self.transform = transform;
    }

    pub fn gradient_at(&self, point: &Point) -> Color {
        let distance = &self.b - &self.a;
        let fraction = point.x() - point.x().floor();

        &self.a + &(distance * fraction)
    }

    pub fn gradient_at_object(&self, object: &Object, point: &Point) -> Color {
        let object_point = object.get_transform().inverse().unwrap() * point;
        let pattern_point = self.get_transform().inverse().unwrap() * object_point;

        self.gradient_at(&pattern_point)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Ring {
    a: Color,
    b: Color,
    transform: Matrix<4>,
}

impl From<Ring> for Pattern {
    fn from(ring: Ring) -> Self {
        Self::Ring(ring)
    }
}

impl Ring {
    pub fn new(a: Color, b: Color, transform: Option<Matrix<4>>) -> Self {
        Self {
            a,
            b,
            transform: transform.map_or_else(Matrix::<4>::identity, |t| t),
        }
    }

    pub const fn get_transform(&self) -> &Matrix<4> {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Matrix<4>) {
        self.transform = transform;
    }

    pub fn ring_at(&self, point: &Point) -> Color {
        if ((point.x().powi(2) + point.z().powi(2)).sqrt().floor()) % 2.0 == 0.0 {
            self.a.clone()
        } else {
            self.b.clone()
        }
    }

    pub fn ring_at_object(&self, object: &Object, point: &Point) -> Color {
        let object_point = object.get_transform().inverse().unwrap() * point;
        let pattern_point = self.get_transform().inverse().unwrap() * object_point;

        self.ring_at(&pattern_point)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Checkers {
    a: Color,
    b: Color,
    transform: Matrix<4>,
}

impl From<Checkers> for Pattern {
    fn from(checkers: Checkers) -> Self {
        Self::Checkers(checkers)
    }
}

impl Checkers {
    pub fn new(a: Color, b: Color, transform: Option<Matrix<4>>) -> Self {
        Self {
            a,
            b,
            transform: transform.map_or_else(Matrix::<4>::identity, |t| t),
        }
    }

    pub const fn get_transform(&self) -> &Matrix<4> {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Matrix<4>) {
        self.transform = transform;
    }

    pub fn checkers_at(&self, point: &Point) -> Color {
        if (point.x().floor() + point.y().floor() + point.z().floor()) % 2.0 == 0.0 {
            self.a.clone()
        } else {
            self.b.clone()
        }
    }

    pub fn checkers_at_object(&self, object: &Object, point: &Point) -> Color {
        let object_point = object.get_transform().inverse().unwrap() * point;
        let pattern_point = self.get_transform().inverse().unwrap() * object_point;

        self.checkers_at(&pattern_point)
    }
}
