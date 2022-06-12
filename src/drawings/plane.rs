use {
    crate::{
        rt::{
            camera::Camera, color::Color, material::Material, matrix::Matrix, plane::Plane,
            point_light::PointLight, shape::Shape, sphere::Sphere, to_ppm::ToPPM, world::World,
        },
        tuple::{point::Point, vector::Vector},
    },
    std::f32::consts::PI,
};

pub fn spheres() -> Result<(), std::io::Error> {
    let mut material = Material::default();
    material.set_color(Color::new(0.2, 0.2, 0.2));
    material.set_specular(0.0);

    let mut floor = Plane::default();
    floor.set_material(material.clone());

    let mut first_pane = Plane::default();
    first_pane.set_transform(
        Matrix::<4>::scaling(0.1, 0.1, 0.1)
            * Matrix::<4>::translation(0.0, 0.0, 2.5)
            * Matrix::<4>::rotation_x(PI / 2.0),
    );
    let mut first_pane_material = material.clone();
    first_pane_material.set_color(Color::new(0.3, 0.3, 0.4));
    first_pane_material.set_diffuse(0.7);
    first_pane_material.set_specular(0.3);
    first_pane.set_material(first_pane_material);

    let mut second_pane = Plane::default();
    second_pane.set_transform(
        Matrix::<4>::translation(1.0, 0.0, 2.5)
            * Matrix::<4>::rotation_y(PI / 3.0)
            * Matrix::<4>::rotation_x(PI / 2.0),
    );
    let mut second_pane_material = material.clone();
    second_pane_material.set_color(Color::new(0.3, 0.3, 0.4));
    second_pane_material.set_diffuse(0.7);
    second_pane_material.set_specular(0.3);
    second_pane.set_material(second_pane_material);

    let mut third_pane = Plane::default();
    third_pane.set_transform(
        Matrix::<4>::translation(4.0, 0.0, -2.5)
            * Matrix::<4>::rotation_y(2.0 * PI / 3.0)
            * Matrix::<4>::rotation_x(PI / 2.0),
    );
    let mut third_pane_material = material.clone();
    third_pane_material.set_color(Color::new(0.3, 0.3, 0.4));
    third_pane_material.set_diffuse(0.7);
    third_pane_material.set_specular(0.3);
    third_pane.set_material(third_pane_material);

    let mut fourth_pane = Plane::default();
    fourth_pane.set_transform(
        Matrix::<4>::translation(0.0, 0.0, -6.0) * Matrix::<4>::rotation_x(PI / 2.0),
    );
    let mut fourth_pane_material = material.clone();
    fourth_pane_material.set_color(Color::new(0.1, 0.1, 0.1));
    fourth_pane_material.set_diffuse(0.7);
    fourth_pane.set_material(fourth_pane_material);

    let mut fifth_pane = Plane::default();
    fifth_pane.set_transform(
        Matrix::<4>::translation(-4.0, 0.0, -2.5)
            * Matrix::<4>::rotation_y(PI / 3.0)
            * Matrix::<4>::rotation_x(PI / 2.0),
    );
    let mut fifth_pane_material = material.clone();
    fifth_pane_material.set_color(Color::new(0.3, 0.3, 0.4));
    fifth_pane_material.set_diffuse(0.7);
    fifth_pane_material.set_specular(0.3);
    fifth_pane.set_material(fifth_pane_material);

    let mut sixth_pane = Plane::default();
    sixth_pane.set_transform(
        Matrix::<4>::translation(-1.0, 0.0, 2.5)
            * Matrix::<4>::rotation_y(-PI / 3.0)
            * Matrix::<4>::rotation_x(PI / 2.0),
    );
    let mut sixth_pane_material = material.clone();
    sixth_pane_material.set_color(Color::new(0.3, 0.3, 0.4));
    sixth_pane_material.set_diffuse(0.7);
    sixth_pane_material.set_specular(0.3);
    sixth_pane.set_material(sixth_pane_material);

    // The large sphere in the middle is a unit sphere, translated upward slightly and colored green.
    let mut middle = Sphere::default();
    let mut middle_material = material.clone();
    middle.set_transform(Matrix::<4>::translation(-0.5, 10.0, -5.0));
    middle_material.set_color(Color::new(0.1, 1.0, 0.5));
    middle_material.set_diffuse(0.7);
    middle_material.set_specular(0.3);
    middle.set_material(middle_material);

    // The smaller green sphere on the right is scaled in half.
    let mut right = Sphere::default();
    right.set_transform(
        Matrix::<4>::translation(1.0, 11.3, -5.3) * Matrix::<4>::scaling(0.5, 0.5, 0.5),
    );
    let mut right_material = material.clone();
    right_material.set_color(Color::new(0.5, 1.0, 0.1));
    right_material.set_diffuse(0.7);
    right_material.set_specular(0.3);
    right.set_material(right_material);

    // The smallest sphere is scaled by a third, before being translated.
    let mut left = Sphere::default();
    left.set_transform(
        Matrix::<4>::translation(1.3, 12.5, -5.3) * Matrix::<4>::scaling(0.33, 0.33, 0.33),
    );
    let mut left_material = material;
    left_material.set_color(Color::new(1.0, 0.8, 0.1));
    left_material.set_diffuse(0.7);
    left_material.set_specular(0.3);
    left.set_material(left_material);

    // The light source is white, shining from above and to the left.
    let world = World::new(
        Some(vec![
            floor.into(),
            middle.into(),
            right.into(),
            left.into(),
            first_pane.into(),
            second_pane.into(),
            third_pane.into(),
            fourth_pane.into(),
            fifth_pane.into(),
            sixth_pane.into(),
        ]),
        Some(PointLight::new(
            Point::new(0.0, 1.0, -2.5),
            Color::new(1.0, 1.0, 1.0),
        )),
    );

    let camera = Camera::new(
        1280.0,
        720.0,
        PI / 3.0,
        Some(Matrix::<4>::view_transform(
            &Point::new(0.0, 15.0, -2.5),
            &Point::new(0.0, 0.0, 0.0),
            &Vector::new(0.0, 1.0, 0.0),
        )),
    );

    let image = camera.render(&world);

    let ppm = image.to_ppm();
    ppm.save_to_disk("src/drawings/ppms/hex.ppm")?;

    Ok(())
}
