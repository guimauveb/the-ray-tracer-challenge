use crate::{
    rt::{canvas::Canvas, color::Color, to_ppm::ToPPM},
    tuple::{point::Point, vector::Vector},
};

#[derive(Debug)]
pub struct Projectile {
    position: Point,
    velocity: Vector,
}

#[derive(Debug)]
pub struct Environment {
    gravity: Vector,
    wind: Vector,
}

impl Projectile {
    pub const fn new(position: Point, velocity: Vector) -> Self {
        Self { position, velocity }
    }

    pub fn tick(&mut self, environment: &Environment) {
        self.position = &self.position + &self.velocity;
        self.velocity = &self.velocity + &environment.gravity + &environment.wind;
    }
}

impl Environment {
    pub const fn new(gravity: Vector, wind: Vector) -> Self {
        Self { gravity, wind }
    }
}

pub fn launch_projecticle() -> Result<(), std::io::Error> {
    let start = Point::new(0.0, 1.0, 0.0);
    let velocity = Vector::new(1.0, 1.8, 0.0).normalized() * 11.25;
    let mut projectile = Projectile::new(start, velocity);

    let gravity = Vector::new(0.0, -0.1, 0.0);
    let wind = Vector::new(-0.01, 0.0, 0.0);
    let environment = Environment::new(gravity, wind);

    let mut canvas = Canvas::new(900, 550);
    let projectile_color = Color::new(1.0, 0.0, 0.0);

    while projectile.position.y() > 0.0 {
        canvas.write_pixel(
            projectile.position.x() as usize,
            canvas.height() - (projectile.position.y() as usize),
            projectile_color,
        );
        projectile.tick(&environment);
    }

    let ppm = canvas.to_ppm();
    ppm.save_to_disk("src/drawings/ppms/projectile.ppm")?;
    Ok(())
}
