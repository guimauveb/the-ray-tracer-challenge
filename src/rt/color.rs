use {crate::approx_eq::ApproxEq, std::ops};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

#[allow(dead_code)]
impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }

    pub fn black() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }

    pub fn white() -> Self {
        Self {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        }
    }

    pub fn red(&self) -> f64 {
        self.red
    }
    pub fn green(&self) -> f64 {
        self.green
    }
    pub fn blue(&self) -> f64 {
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

#[test]
fn can_create_color() {
    let color = Color::new(-0.5, 0.4, 1.7);
    assert_eq!(color.red, -0.5);
    assert_eq!(color.green, 0.4);
    assert_eq!(color.blue, 1.7);
}

#[test]
fn can_create_black_color() {
    let black = Color::black();
    let expected = Color::new(0.0, 0.0, 0.0);
    assert_eq!(black, expected);
}

#[test]
fn can_create_white_color() {
    let white = Color::white();
    let expected = Color::new(1.0, 1.0, 1.0);
    assert_eq!(white, expected);
}

#[test]
fn can_add_colors() {
    let color_a = Color::new(0.9, 0.6, 0.75);
    let color_b = Color::new(0.7, 0.1, 0.25);
    let expected = Color::new(1.6, 0.7, 1.0);
    assert_eq!(color_a + color_b, expected);
}

#[test]
fn can_sub_clors() {
    let color_a = Color::new(0.9, 0.6, 0.75);
    let color_b = Color::new(0.7, 0.1, 0.25);
    let expected = Color::new(0.2, 0.5, 0.5);
    assert_eq!(color_a - color_b, expected);
}

#[test]
fn can_multiply_color_by_sclar() {
    let color = Color::new(0.2, 0.3, 0.4);
    let scalar = 2.0;
    let expected = Color::new(0.4, 0.6, 0.8);
    assert_eq!(color * scalar, expected);
}

#[test]
// Mul
fn can_compute_hadamard_product() {
    let color_a = Color::new(1.0, 0.2, 0.4);
    let color_b = Color::new(0.9, 1.0, 0.1);
    let expected = Color::new(0.9, 0.2, 0.04);
    assert_eq!(color_a * color_b, expected);
}
