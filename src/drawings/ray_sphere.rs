use crate::{
    rt::{
        canvas::Canvas,
        color::Color,
        object::Object,
        /*matrix::*,*/ ray::{Intersect, Ray},
        sphere::Sphere,
        to_ppm::ToPPM,
    },
    tuple::{point::Point, vector::Vector},
};

// "Works" but the conversion from world space to canvas space is quite hacky.
pub fn ray_sphere_hit() -> Result<(), std::io::Error> {
    let mut canvas = Canvas::new(256, 256);
    let red = Color::new(1.0, 0.0, 0.0);

    // Unit sphere
    let sphere = Object::Sphere(Sphere::default());

    //// Shrink it along the y axis
    //sphere.set_transform(Matrix::<4>::scaling(1.0, 0.5, 1.0));
    //// Shrink it along the x axis
    //sphere.set_transform(Matrix::<4>::scaling(0.5, 1.0, 1.0));
    //// Shrink it and rotate it
    //sphere.set_transform(
    //    Matrix::<4>::rotation_z(PI / 4.0) * Matrix::<4>::scaling(0.5, 1.0, 1.0),
    //);
    //// Shrink it and skew it
    //sphere.set_transform(
    //    Matrix::<4>::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0)
    //        * Matrix::<4>::scaling(0.5, 1.0, 1.0),
    //);

    let mut wall: Vec<Point> = Vec::with_capacity(256_usize.pow(2));
    let mut rays: Vec<Ray> = Vec::with_capacity(256_usize.pow(2));

    // Wall
    for y in -128..127 {
        for x in -128..127 {
            wall.push(Point::new(x as f32, y as f32, 10.0));
        }
    }
    assert_eq!(wall.capacity(), 256_usize.pow(2));

    // Rays
    for y in -128..127 {
        for x in -128..127 {
            let origin = Point::new(0.0, 0.0, -1.005);
            let direction = Vector::new(x as f32, y as f32, 10.0);
            rays.push(Ray::new(origin, direction));
        }
    }
    assert_eq!(rays.capacity(), 256_usize.pow(2));

    for r in rays {
        let xs = r.intersect(&sphere);
        if let Some(_intersections) = xs {
            // TODO - Create a method to map a plane with origin at x = 0 and y = 0 to the canvas coordinates.
            canvas.write_pixel(
                (canvas.width() as i64 / 2_i64 + r.direction().x() as i64) as usize,
                (canvas.height() as i64 / 2_i64 + r.direction().y() as i64) as usize,
                red.clone(),
            );
        }
    }

    let ppm = canvas.to_ppm();
    ppm.save_to_disk("src/drawings/ppms/ray_sphere.ppm")?;

    Ok(())
}
