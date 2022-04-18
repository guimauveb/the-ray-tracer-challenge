#[cfg(test)]
use crate::{
    primitive::{point::Point, tuple::Tuple, vector::Vector},
    rt::{
        color::Color,
        intersect::Intersect,
        material::Material,
        matrix::{Matrix, Scaling},
        object::Object,
        point_light::PointLight,
        ray::Ray,
        sphere::Sphere,
        world::World,
    },
};

#[test]
fn creating_a_world() {
    let world = World::empty();
    assert_eq!(world.objects(), None);
    assert_eq!(world.light(), None);
}

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

#[test]
fn intersect_a_world_with_a_ray() {
    let w = World::default();
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let xs = r
        .intersect(&w)
        .expect("No intersection between the ray and the world!");
    assert_eq!(xs.len(), 4);
    assert_eq!(xs[0].t(), 4.0);
    assert_eq!(xs[1].t(), 4.5);
    assert_eq!(xs[2].t(), 5.5);
    assert_eq!(xs[3].t(), 6.0);
}
