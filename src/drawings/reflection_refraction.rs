use {
    crate::{
        rt::{
            camera::Camera,
            color::{Color, BLACK, WHITE},
            material::Material,
            matrix::Matrix,
            pattern::Pattern,
            plane::Plane,
            point_light::PointLight,
            shape::Shape,
            sphere::Sphere,
            to_ppm::ToPPM,
            world::World,
        },
        tuple::{point::Point, vector::Vector},
    },
    std::f64::consts::PI,
};

pub fn spheres() -> Result<(), std::io::Error> {
    // The floor is an extremely flattened sphere with a matte texture.
    let mut material = Material::default();
    material.set_color(Color::new(1.0, 0.9, 0.9));
    material.set_specular(1.0);
    material.set_pattern(Pattern::checkers(BLACK, WHITE, None));
    material.set_reflective(0.9);
    material.set_transparency(0.9);
    material.set_shininess(300.0);
    let mut floor = Plane::default();
    floor.set_material(material);

    // The wall on the left has the same scale and color as the floor, but is also rotated and translated into place.
    let mut material = Material::default();
    material.set_color(Color::new(1.0, 0.9, 0.9));
    material.set_specular(1.0);
    material.set_pattern(Pattern::checkers(BLACK, WHITE, None));
    material.set_reflective(0.9);
    material.set_shininess(300.0);
    let left_wall = Sphere::new(
        Point::default(),
        Matrix::<4>::translation(0.0, 0.0, 5.0)
            * Matrix::<4>::rotation_y(-PI / 4.0)
            * Matrix::<4>::rotation_x(PI / 2.0)
            * Matrix::<4>::scaling(10.0, 0.01, 10.0),
        material,
    );

    // The wall on the right side is identical to the left wall, but is rotated the opposite direction in y.
    let mut material = Material::default();
    material.set_color(Color::new(1.0, 0.9, 0.9));
    material.set_specular(1.0);
    material.set_pattern(Pattern::checkers(BLACK, WHITE, None));
    material.set_reflective(0.9);
    material.set_shininess(300.0);
    let right_wall = Sphere::new(
        Point::default(),
        Matrix::<4>::translation(0.0, 0.0, 5.0)
            * Matrix::<4>::rotation_y(PI / 4.0)
            * Matrix::<4>::rotation_x(PI / 2.0)
            * Matrix::<4>::scaling(10.0, 0.01, 10.0),
        material,
    );

    // The large sphere in the middle is a unit sphere, translated upward slightly and colored green.
    let mut middle = Sphere::default();
    let mut middle_material = Material::default();
    middle.set_transform(Matrix::<4>::translation(-0.5, 1.0, 0.5));
    middle_material.set_color(Color::new(0.1, 1.0, 0.5));
    middle_material.set_diffuse(0.7);
    middle_material.set_specular(1.0);
    middle_material.set_reflective(0.9);
    middle_material.set_shininess(300.0);
    middle.set_material(middle_material);

    // The smaller green sphere on the right is scaled in half.
    let mut right = Sphere::default();
    right.set_transform(
        Matrix::<4>::translation(1.5, 0.5, -0.5) * Matrix::<4>::scaling(0.5, 0.5, 0.5),
    );
    let mut right_material = Material::default();
    right_material.set_color(Color::new(0.5, 1.0, 0.1));
    right_material.set_diffuse(0.7);
    right_material.set_specular(1.0);
    right_material.set_reflective(0.9);
    right_material.set_shininess(300.0);
    right.set_material(right_material);

    // The smallest sphere is scaled by a third, before being translated.
    let mut left = Sphere::default();
    left.set_transform(
        Matrix::<4>::translation(-1.5, 0.33, -0.75) * Matrix::<4>::scaling(0.33, 0.33, 0.33),
    );
    let mut left_material = Material::default();
    left_material.set_color(Color::new(1.0, 0.8, 0.1));
    left_material.set_diffuse(0.7);
    left_material.set_specular(1.0);
    left_material.set_reflective(0.9);
    left_material.set_shininess(300.0);
    left.set_material(left_material);

    // The light source is white, shining from above and to the left.
    let world = World::new(
        Some(vec![
            floor.into(),
            left_wall.into(),
            right_wall.into(),
            middle.into(),
            right.into(),
            left.into(),
        ]),
        Some(PointLight::new(
            Point::new(-10.0, 15.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        )),
    );

    let camera = Camera::new(
        1280.0,
        720.0,
        //3840.0,
        //2160.0,
        PI / 3.0,
        Some(Matrix::<4>::view_transform(
            &Point::new(0.0, 1.5, -5.0),
            &Point::new(0.0, 1.0, 0.0),
            &Vector::new(0.0, 1.0, 0.0),
        )),
    );

    let image = camera.render(&world);

    let ppm = image.to_ppm();
    ppm.save_to_disk("src/drawings/ppms/spheres.ppm")?;

    Ok(())
}
