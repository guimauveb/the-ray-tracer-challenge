use {
    super::{color::Color, matrix::Matrix, object::Object, shape::Shape},
    crate::tuple::point::Point,
    std::fmt::{Debug, Formatter, Result as FmtResult},
};

pub struct Pattern {
    pattern: Box<dyn Fn(&Point) -> Color + 'static>,
    transform: Matrix<4>,
    /// Used to satsify some tests  that requires access
    /// to `a` and `b`, but can be too restrictive when
    /// creating patterns.
    colors: Option<(Color, Color)>,
}

impl Debug for Pattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Pattern")
            .field("pattern", &"Box<dyn Fn(&Point) -> Color + 'static>")
            .field("transform", &self.transform)
            .field("colors", &self.colors)
            .finish()
    }
}

impl Pattern {
    pub fn new(
        pattern: impl Fn(&Point) -> Color + 'static,
        transform: Option<Matrix<4>>,
        colors: Option<(Color, Color)>,
    ) -> Self {
        Self {
            pattern: Box::new(pattern),
            transform: transform.map_or_else(Matrix::identity, |t| t),
            colors,
        }
    }

    /// Constructs a stripe pattern.
    pub fn stripe(a: Color, b: Color, transform: Option<Matrix<4>>) -> Self {
        Self::new(
            move |point: &Point| {
                if point.x().floor() % 2.0 == 0.0 {
                    a
                } else {
                    b
                }
            },
            transform,
            Some((a, b)),
        )
    }

    /// Constructs a gradient pattern.
    pub fn gradient(a: Color, b: Color, transform: Option<Matrix<4>>) -> Self {
        Self::new(
            move |point: &Point| {
                let distance = b - a;
                let fraction = point.x() - point.x().floor();

                a + (distance * fraction)
            },
            transform,
            Some((a, b)),
        )
    }

    /// Constructs a ring pattern.
    pub fn ring(a: Color, b: Color, transform: Option<Matrix<4>>) -> Self {
        Self::new(
            move |point: &Point| {
                if point.x().hypot(point.z()) == 0.0 {
                    a
                } else {
                    b
                }
            },
            transform,
            Some((a, b)),
        )
    }

    /// Constructs a checkers pattern.
    pub fn checkers(a: Color, b: Color, transform: Option<Matrix<4>>) -> Self {
        Self::new(
            move |point: &Point| {
                if (point.x().floor() + point.y().floor() + point.z().floor()) % 2.0 == 0.0 {
                    a
                } else {
                    b
                }
            },
            transform,
            Some((a, b)),
        )
    }

    pub const fn transform(&self) -> &Matrix<4> {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Matrix<4>) {
        self.transform = transform;
    }

    /// Returns the `Color`at the specified `Point`
    pub fn at(&self, point: &Point) -> Color {
        (self.pattern)(point)
    }

    pub fn at_object(&self, object: &Object, point: &Point) -> Color {
        let object_point = object.transform().inverse().unwrap() * point;
        let pattern_point = self.transform().inverse().unwrap() * object_point;

        self.at(&pattern_point)
    }

    pub fn a(&self) -> &Color {
        &self
            .colors
            .as_ref()
            .expect("Colors are not set on the Pattern")
            .0
    }

    pub fn b(&self) -> &Color {
        &self
            .colors
            .as_ref()
            .expect("Colors are not set on the Pattern")
            .1
    }
}

/// Function used in tests only.
#[cfg(test)]
pub fn test_pattern() -> Pattern {
    Pattern::new(
        move |point: &Point| Color::new(point.x(), point.y(), point.z()),
        Some(Matrix::identity()),
        None,
    )
}
