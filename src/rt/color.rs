use {crate::approx_eq::ApproxEq, std::ops};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub const fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }

    pub const fn black() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }

    pub const fn white() -> Self {
        Self {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        }
    }

    pub const fn red(&self) -> f64 {
        self.red
    }
    pub const fn green(&self) -> f64 {
        self.green
    }
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

impl ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl ops::Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

impl ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}
/*
// Not implemented to avoid unnecessary call
 * trait HadamardProduct<Rhs: Mul = Self> {
 *     fn hadamard_product(self, rhs: Rhs) -> Self;
 * }
 *
 * impl HadamardProduct for Color {
 *     fn hadamard_product(self, rhs: Self) -> Self {
 *         self.mul(rhs)
 *     }
 * }
*/
impl ops::Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}
