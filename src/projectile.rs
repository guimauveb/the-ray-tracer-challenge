use crate::primitive::{point::Point, tuple::Tuple, vector::Vector};

#[derive(Debug)]
pub struct Projectile {
    pub position: Point,
    pub velocity: Vector,
}

#[derive(Debug)]
pub struct Environment {
    pub gravity: Vector,
    pub wind: Vector,
}

impl Projectile {
    pub const fn new(position: Point, velocity: Vector) -> Self {
        Self { position, velocity }
    }
    pub fn tick(&self, environment: &Environment) -> Self {
        Self {
            position: self.position + self.velocity,
            velocity: self.velocity + environment.gravity + environment.wind,
        }
    }
}

impl Environment {
    pub const fn new(gravity: Vector, wind: Vector) -> Self {
        Self { gravity, wind }
    }
}

pub fn launch_projecticle() {
    let environment = Environment::new(Vector::new(0.0, -0.1, 0.0), Vector::new(-0.01, 0.0, 0.0));
    let mut projectile = Projectile::new(
        Point::new(0.0, 1.0, 0.0),
        Vector::new(6.0, 0.0, 0.0).normalize(),
    );

    while projectile.position.y() > 0.0 {
        println!("Projectile position: {:#?}", &projectile.position);
        projectile = projectile.tick(&environment);
    }
}
