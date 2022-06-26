use {
    crate::approx_eq::ApproxEq,
    std::ops::{Add, Mul, Sub},
};

#[derive(Debug, Clone)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub const fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }

    /// Returns the black color.
    /// ```
    ///    Self {
    ///        red: 0.0,
    ///        green: 0.0,
    ///        blue: 0.0,
    ///    }
    /// ```
    pub const fn black() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }

    /// Returns the white color.
    /// ```
    ///    Self {
    ///        red: 1.0,
    ///        green: 1.0,
    ///        blue: 1.0,
    ///    }
    /// ```
    pub const fn white() -> Self {
        Self {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        }
    }

    /// Returns the red component.
    pub const fn red(&self) -> f64 {
        self.red
    }

    /// Returns the green component.
    pub const fn green(&self) -> f64 {
        self.green
    }

    /// Returns the blue component.
    pub const fn blue(&self) -> f64 {
        self.blue
    }
}

impl PartialEq for Color {
    fn eq(&self, rhs: &Self) -> bool {
        self.red.approx_eq(rhs.red)
            && self.green.approx_eq(rhs.green)
            && self.blue.approx_eq(rhs.blue)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Add for &Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

impl Sub for &Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl Mul<f64> for &Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl Mul for Color {
    type Output = Self;

    /// Computes the Hadamard product.
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl Mul for &Color {
    type Output = Color;

    /// Computes the Hadamard product.
    fn mul(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}
