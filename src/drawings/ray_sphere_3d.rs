use crate::{
    primitive::{point::Point, tuple::Tuple},
    rt::{
        canvas::Canvas,
        color::Color,
        intersect::Intersect,
        intersection::Object,
        lighting::Lighting,
        material::Material,
        normal::Normal,
        point_light::PointLight,
        /*matrix::*,*/ ray::{Position, Ray},
        sphere::Sphere,
        to_ppm::ToPPM,
    },
};

pub fn ray_sphere_hit() -> Result<(), std::io::Error> {
    let wall_z = -10.0;
    // Canvas size
    let canvas_pixels = 1024;
    let ray_origin = Point::new(0.0, 0.0, -3.00);
    // Wall is a square (7.0 * 7.0)
    let wall_size = 7.0;
    // Divide the wall size by the number of pixels to get the size of a single pixel (in world space units)
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    // Unit sphere
    let mut sphere = Sphere::default();
    let mut material = Material::default();
    material.set_color(Color::new(1.0, 0.2, 1.0));
    sphere.set_material(material);

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

    // Light source
    let light_position = Point::new(-10.0, 10.0, -10.0);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = PointLight::new(light_position, light_color);

    for y in 0..canvas.height() {
        // top = half, bottom = -half
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas.width() {
            // left = -half, right = half
            let world_x = -half + pixel_size * x as f64;
            // Point on the wall that the ray will target
            let position = Point::new(world_x, world_y, wall_z);
            // If we don't normalize the direction, we get a rather strange result
            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let intersections = r.intersect(&sphere);
            if let Some([hit, _]) = intersections {
                let point = r.position(hit.t());
                let normal = hit.object().normal_at(&point);
                let eye = -r.direction();
                let color = hit
                    .object()
                    .material()
                    .lighting(&light, &point, &eye, &normal);
                canvas.write_pixel(x, y, color);
            }
        }
    }

    let ppm = canvas.to_ppm();
    ppm.save_to_disk("src/drawings/ppms/ray_sphere_3d.ppm")?;

    Ok(())
}
