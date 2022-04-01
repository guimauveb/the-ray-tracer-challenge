use crate::{
    primitive::{point::Point, tuple::Tuple, vector::Vector},
    rt::{
        canvas::Canvas, color::Color, intersect::Intersect, ray::Ray, sphere::Sphere, to_ppm::ToPPM,
    },
};

pub fn ray_sphere_hit() -> Result<(), std::io::Error> {
    let mut canvas = Canvas::new(256, 256);
    let red = Color::new(1.0, 0.0, 0.0);
    // Unit sphere
    let sphere = Sphere::default();
    let mut wall: Vec<Point> = vec![];
    let mut rays: Vec<Ray> = vec![];

    // Wall
    for y in -128..127 {
        for x in -128..127 {
            wall.push(Point::new(f64::from(x), f64::from(y), 10.0));
        }
    }
    // Rays
    for y in -128..127 {
        for x in -128..127 {
            let origin = Point::new(0.0, 0.0, -1.1);
            let direction = Vector::new(f64::from(x), f64::from(y), 10.0);
            rays.push(Ray::new(origin, direction));
        }
    }

    for r in rays {
        let xs = r.intersect(&sphere);
        if let Some(intersections) = xs {
            //println!(
            //    "Ray intersects sphere at {} and {}",
            //    intersections[0].t(),
            //    intersections[1].t()
            //);
            canvas.write_pixel(
                canvas.width() / 2 + r.direction().x() as usize,
                canvas.height() / 2 + r.direction().y() as usize,
                red,
            );
        }
    }

    let ppm = canvas.to_ppm();
    ppm.save_to_disk("src/drawings/ppms/ray_sphere.ppm")?;

    Ok(())
}
