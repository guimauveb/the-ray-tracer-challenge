use {
    crate::{
        primitive::{point::Point, tuple::Tuple, vector::Vector},
        rt::{
            canvas::Canvas, color::Color, intersect::Intersect, matrix::*, ray::Ray,
            sphere::Sphere, to_ppm::ToPPM,
        },
    },
    std::f64::consts::PI,
};

pub fn ray_sphere_hit() -> Result<(), std::io::Error> {
    let mut canvas = Canvas::new(256, 256);
    let red = Color::new(1.0, 0.0, 0.0);

    // Unit sphere
    let sphere = Sphere::default();

    //// Shrink it along the y axis
    //sphere.set_transform(Matrix::<4_usize>::scaling(1.0, 0.5, 1.0));
    //// Shrink it along the x axis
    //sphere.set_transform(Matrix::<4_usize>::scaling(0.5, 1.0, 1.0));
    //// Shrink it and rotate it
    //sphere.set_transform(
    //    Matrix::<4_usize>::rotation_z(PI / 4.0) * Matrix::<4_usize>::scaling(0.5, 1.0, 1.0),
    //);
    //// Shrink it and skew it
    //sphere.set_transform(
    //    Matrix::<4_usize>::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0)
    //        * Matrix::<4_usize>::scaling(0.5, 1.0, 1.0),
    //);

    let mut wall: Vec<Point> = Vec::with_capacity(256_usize.pow(2));
    let mut rays: Vec<Ray> = Vec::with_capacity(256_usize.pow(2));

    // Wall
    for y in -128..127 {
        for x in -128..127 {
            wall.push(Point::new(f64::from(x), f64::from(y), 10.0));
        }
    }
    assert_eq!(wall.capacity(), 256_usize.pow(2));

    // Rays
    for y in -128..127 {
        for x in -128..127 {
            let origin = Point::new(0.0, 0.0, -1.005);
            let direction = Vector::new(f64::from(x), f64::from(y), 10.0);
            rays.push(Ray::new(origin, direction));
        }
    }
    assert_eq!(rays.capacity(), 256_usize.pow(2));

    for r in rays {
        let xs = r.intersect(&sphere);
        if let Some(intersections) = xs {
            // TODO - Create a method to map a plane with origin at x = 0 and y = 0 to the canvas coordinates.
            canvas.write_pixel(
                (canvas.width() as i64 / 2 as i64 + r.direction().x() as i64) as usize,
                (canvas.height() as i64 / 2 as i64 + r.direction().y() as i64) as usize,
                red,
            );
        }
    }

    let ppm = canvas.to_ppm();
    ppm.save_to_disk("src/drawings/ppms/ray_sphere.ppm")?;

    Ok(())
}