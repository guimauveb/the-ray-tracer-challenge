#[cfg(test)]
use crate::{
    primitive::{point::Point, tuple::Tuple},
    rt::{
        color::Color,
        material::Material,
        matrix::{Matrix, Scaling},
        object::Object,
        point_light::PointLight,
        sphere::Sphere,
        world::World,
    },
};

//#[test]
//fn creating_a_world() {
//    let world = World::default();
//    assert_eq!(world.objects(), None);
//    assert_eq!(world.light(), None);
//}

#[test]
fn the_default_world() {
    let material = Material::new(Color::new(0.8, 1.0, 0.6), 0.1, 0.7, 0.2, 200.0);
    let s1 = Sphere::with_material(material);

    let transform = Matrix::<4_usize>::scaling(0.5, 0.5, 0.5);
    let s2 = Sphere::with_transform(transform);

    let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let world = World::default();

    assert_eq!(world.light(), Some(&light));
    assert!(world.objects().unwrap().contains(&Object::Sphere(s1)));
    assert!(world.objects().unwrap().contains(&Object::Sphere(s2)));
}
