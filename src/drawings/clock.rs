use {
    crate::{
        primitive::point::Point,
        rt::{
            canvas::Canvas,
            color::Color,
            matrix::{Matrix, Rotation},
            to_ppm::ToPPM,
        },
    },
    std::f64::consts::PI,
};

pub fn draw_clock() -> Result<(), std::io::Error> {
    let mut canvas = Canvas::new(512, 512);
    // The origin will be at the middle of the canvas
    let origin = Point::new(512.0 / 2.0, 512.0 / 2.0, 0.0);
    // The first point will be at (x: 196.0, y: 0.0) and will be rotated by (loop index * 30°) at each loop
    let point = Point::new(196.0, 0.0, 0.0);
    let white = Color::white();
    // 12 hours
    for i in 0..12 {
        // 30° == (PI/6.0)
        let rotation = Matrix::<4>::rotation_z(f64::from(i) * PI / 6.0);
        let rotated_point = rotation * point;
        canvas.write_pixel(
            (origin.x() + rotated_point.x()) as usize,
            canvas.height() - (origin.y() + rotated_point.y()) as usize,
            white,
        );
    }

    let ppm = canvas.to_ppm();
    ppm.save_to_disk("src/drawings/ppms/clock.ppm")?;
    Ok(())
}
