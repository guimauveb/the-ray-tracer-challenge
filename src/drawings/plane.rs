use {
    crate::{
        rt::{
            camera::Camera, color::Color, material::Material, matrix::Matrix, plane::Plane,
            point_light::PointLight, shape::Shape, sphere::Sphere, to_ppm::ToPPM, world::World,
        },
        tuple::{point::Point, vector::Vector},
    },
    std::f64::consts::PI,
};

pub fn spheres() -> Result<(), std::io::Error> {
    let mut material = Material::default();
    material.set_color(Color::new(1.0, 0.9, 0.9));
    material.set_specular(0.0);

    let mut floor = Plane::default();
    floor.set_material(material.clone());

    let mut wall = Plane::default();
    wall.set_transform(Matrix::<4>::rotation_x(PI / 2.0) * Matrix::<4>::translation(0.0, 0.0, 5.0));
    wall.set_material(material.clone());

    // The large sphere in the middle is a unit sphere, translated upward slightly and colored green.
    let mut middle = Sphere::default();
    let mut middle_material = material.clone();
    middle.set_transform(Matrix::<4>::translation(-0.5, 1.0, 0.5));
    middle_material.set_color(Color::new(0.1, 1.0, 0.5));
    middle_material.set_diffuse(0.7);
    middle_material.set_specular(0.3);
    middle.set_material(middle_material);

    // The smaller green sphere on the right is scaled in half.
    let mut right = Sphere::default();
    right.set_transform(
        Matrix::<4>::translation(1.5, 0.5, -0.5) * Matrix::<4>::scaling(0.5, 0.5, 0.5),
    );
    let mut right_material = material;
    right_material.set_color(Color::new(0.5, 1.0, 0.1));
    right_material.set_diffuse(0.7);
    right_material.set_specular(0.3);
    right.set_material(right_material);

    // The smallest sphere is scaled by a third, before being translated.
    let mut left = Sphere::default();
    left.set_transform(
        Matrix::<4>::translation(-1.5, 0.33, -0.75) * Matrix::<4>::scaling(0.33, 0.33, 0.33),
    );
    let mut left_material = Material::default();
    left_material.set_color(Color::new(1.0, 0.8, 0.1));
    left_material.set_diffuse(0.7);
    left_material.set_specular(0.3);

    // The light source is white, shining from above and to the left.
    let world = World::new(
        Some(vec![
            floor.into(),
            wall.into(),
            middle.into(),
            right.into(),
            left.into(),
        ]),
        Some(PointLight::new(
            Point::new(-10.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        )),
    );

    let camera = Camera::new(
        384.0,
        216.0,
        PI / 3.0,
        Some(Matrix::<4>::view_transform(
            &Point::new(0.0, 1.5, -5.0),
            &Point::new(0.0, 1.0, 0.0),
            &Vector::new(0.0, 1.0, 0.0),
        )),
    );

    let image = camera.render(&world);

    let ppm = image.to_ppm();
    ppm.save_to_disk("src/drawings/ppms/plane.ppm")?;

    Ok(())
}
