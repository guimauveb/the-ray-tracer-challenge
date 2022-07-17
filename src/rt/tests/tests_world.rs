#[cfg(test)]
use crate::{
    rt::{
        color::{Color, BLACK, WHITE},
        intersection::Intersection,
        intersections::Intersections,
        material::Material,
        matrix::Matrix,
        pattern::test_pattern,
        plane::Plane,
        point_light::PointLight,
        ray::{Intersect, Ray},
        shape::Shape,
        sphere::Sphere,
        world::{World, MAX_REFLECTION_DEPTH},
    },
    tuple::{point::Point, vector::Vector},
};

#[test]
fn creating_a_world() {
    let world = World::empty();
    assert_eq!(world.objects(), None);
    assert_eq!(world.light(), None);
}

#[test]
fn the_default_world() {
    let material = Material::new(
        Color::new(0.8, 1.0, 0.6),
        None,
        0.1,
        0.7,
        0.2,
        200.0,
        0.0,
        0.0,
        1.0,
    );
    let s1 = Sphere::with_material(material);

    let transform = Matrix::<4>::scaling(0.5, 0.5, 0.5);
    let s2 = Sphere::with_transform(transform);

    let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), WHITE);
    let world = World::default();

    assert_eq!(world.light(), Some(&light));
    assert!(world.objects().unwrap().contains(&s1.into()));
    assert!(world.objects().unwrap().contains(&s2.into()));
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

#[test]
fn shading_an_intersection() {
    let w = World::default();
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let shape = &w.objects().unwrap()[0]; // The first object in the world
    let i = Intersection::new(4.0, &shape);

    let comps = i.prepare_computations(&r, None);
    let c = w.shade_hit(&comps, MAX_REFLECTION_DEPTH);
    let expected_c = Color::new(0.3806609553101071, 0.47582619413763383, 0.2854957164825803);
    assert_eq!(c, expected_c);
}

#[test]
fn shading_an_intersection_from_the_inside() {
    let w = World::with_light(Some(PointLight::new(Point::new(0.0, 0.25, 0.0), WHITE)));
    let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    let shape = &w.objects().unwrap()[1];
    let i = Intersection::new(0.5, &shape);
    let comps = i.prepare_computations(&r, None);
    let c = w.shade_hit(&comps, MAX_REFLECTION_DEPTH);
    let expected_c = Color::new(0.9049812520679432, 0.9049812520679432, 0.9049812520679432);
    assert_eq!(c, expected_c);
}

#[test]
fn the_color_when_a_ray_misses() {
    let w = World::default();
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
    let c = w.color_at(&r, MAX_REFLECTION_DEPTH);
    let expected_c = BLACK;
    assert_eq!(c, expected_c);
}

#[test]
fn the_color_when_a_ray_hits() {
    let w = World::default();
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let c = w.color_at(&r, MAX_REFLECTION_DEPTH);
    let expected_c = Color::new(0.3806609553101071, 0.47582619413763383, 0.2854957164825803);
    assert_eq!(c, expected_c);
}

#[test]
fn the_color_with_an_intersection_behind_the_ray() {
    // Create the default objects of the world with the values specfied in the test instead of having to make the material() method return a mutable reference
    // when it shouldnt be necessary for the rest of the program.
    let material = Material::new(
        Color::new(0.8, 1.0, 0.6),
        None,
        1.0,
        0.7,
        0.2,
        200.0,
        0.0,
        0.0,
        0.0,
    );
    let s1 = Sphere::with_material(material.clone());

    let transform = Matrix::<4>::scaling(0.5, 0.5, 0.5);
    let s2 = Sphere::new(Point::new(0.0, 0.0, 0.0), transform, material);

    let w = World::with_objects(Some(vec![s1.into(), s2.into()]));

    let _outer = &w.objects().unwrap()[0];
    let inner = &w.objects().unwrap()[1];

    let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
    let c = w.color_at(&r, MAX_REFLECTION_DEPTH);
    assert_eq!(&c, inner.material().color());
}

#[test]
fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
    let w = World::default();
    let p = Point::new(0.0, 10.0, 0.0);
    assert_eq!(w.is_shadowed(&p), false);
}

#[test]
fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
    let w = World::default();
    let p = Point::new(10.0, -10.0, 10.0);
    assert_eq!(w.is_shadowed(&p), true);
}

#[test]
fn there_is_no_shadow_when_an_object_is_behind_the_light() {
    let w = World::default();
    let p = Point::new(-20.0, 20.0, -20.0);
    assert_eq!(w.is_shadowed(&p), false);
}

#[test]
fn there_is_no_shadow_when_an_object_is_behind_the_point() {
    let w = World::default();
    let p = Point::new(-2.0, 2.0, -2.0);
    assert_eq!(w.is_shadowed(&p), false);
}

#[test]
fn shade_hit_is_given_an_intersection_in_shadow() {
    let (s1, s2) = (
        Sphere::default(),
        Sphere::with_transform(Matrix::<4>::translation(0.0, 0.0, 10.0)),
    );
    let w = World::new(
        Some(vec![s1.into(), s2.into()]),
        Some(PointLight::new(
            Point::new(0.0, 0.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        )),
    );
    let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
    // Intersection with s2 in the world
    let i = Intersection::new(4.0, &w.objects().as_ref().unwrap()[1]);
    let comps = i.prepare_computations(&r, None);
    let color = w.shade_hit(&comps, MAX_REFLECTION_DEPTH);
    assert_eq!(color, Color::new(0.1, 0.1, 0.1));
}

#[test]
fn the_reflected_color_for_a_non_reflective_material() {
    let mut w = World::default();
    let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    w.objects_mut().unwrap()[1].material_mut().set_ambient(1.0);
    let shape = &w.objects().unwrap()[1];
    let i = Intersection::new(1.0, &shape);
    let comps = i.prepare_computations(&r, None);
    let color = w.reflected_color(&comps, MAX_REFLECTION_DEPTH);
    assert_eq!(color, Color::new(0.0, 0.0, 0.0));
}

#[test]
fn the_reflected_color_for_a_reflective_material() {
    let mut w = World::default();
    let mut shape = Plane::default();
    shape.material_mut().set_reflective(0.5);
    shape.set_transform(Matrix::translation(0.0, -1.0, 0.0));
    w.add_object(shape.into());
    let r = Ray::new(
        Point::new(0.0, 0.0, -3.0),
        Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
    );
    let i = Intersection::new(2.0_f64.sqrt(), &w.objects().unwrap()[2]);
    let comps = i.prepare_computations(&r, None);
    let color = w.reflected_color(&comps, MAX_REFLECTION_DEPTH);
    assert_eq!(
        color,
        Color::new(0.190332201495133, 0.23791525186891627, 0.14274915112134975)
    );
}

#[test]
fn shade_hit_with_a_reflective_material() {
    let mut w = World::default();
    let mut shape = Plane::default();
    shape.material_mut().set_reflective(0.5);
    shape.set_transform(Matrix::translation(0.0, -1.0, 0.0));
    w.add_object(shape.into());
    let r = Ray::new(
        Point::new(0.0, 0.0, -3.0),
        Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
    );
    let i = Intersection::new(2.0_f64.sqrt(), &w.objects().unwrap()[2]);
    let comps = i.prepare_computations(&r, None);
    let color = w.shade_hit(&comps, MAX_REFLECTION_DEPTH);
    assert_eq!(
        color,
        Color::new(0.8767572837020907, 0.924340334075874, 0.8291742333283075)
    );
}

#[test]
fn color_at_with_mutually_reflective_surfaces() {
    let mut w = World::default();
    w.set_light(PointLight::new(
        Point::new(0.0, 0.0, 0.0),
        Color::new(1.0, 1.0, 1.0),
    ));
    let mut lower = Plane::default();
    lower.material_mut().set_reflective(1.0);
    lower.set_transform(Matrix::translation(0.0, -1.0, 0.0));
    w.add_object(lower.into());
    let mut upper = Plane::default();
    upper.material_mut().set_reflective(1.0);
    upper.set_transform(Matrix::translation(0.0, 1.0, 0.0));
    w.add_object(upper.into());
    let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
    // NOTE - Here, we're actually testing that the function
    // "should terminate successfully" (not cause a stack overflow).
    assert_eq!(
        w.color_at(&r, MAX_REFLECTION_DEPTH),
        Color::new(1.9, 1.9, 1.9)
    );
}

#[test]
fn the_reflected_color_at_the_maximum_recursive_depth() {
    let mut w = World::default();
    let mut shape = Plane::default();
    shape.material_mut().set_reflective(0.5);
    shape.set_transform(Matrix::translation(0.0, -1.0, 0.0));
    w.add_object(shape.into());
    let r = Ray::new(
        Point::new(0.0, 0.0, -3.0),
        Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
    );
    let i = Intersection::new(2.0_f64.sqrt(), &w.objects().unwrap()[2]);
    let comps = i.prepare_computations(&r, None);
    let color = w.reflected_color(&comps, 0);
    assert_eq!(color, BLACK);
}

#[test]
fn the_refracted_color_with_an_opaque_surface() {
    let w = World::default();
    let shape = &w.objects().unwrap()[0];
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let xs = Intersections::new(vec![
        Intersection::new(4.0, &shape),
        Intersection::new(6.0, &shape),
    ]);
    let comps = xs[0].prepare_computations(&r, Some(&xs));
    let c = w.refracted_color(&comps, 5);
    assert_eq!(c, BLACK);
}

#[test]
fn the_refracted_color_at_the_maximum_recursive_depth() {
    let mut w = World::default();
    w.objects_mut().unwrap()[0]
        .material_mut()
        .set_transparency(1.0);
    w.objects_mut().unwrap()[0]
        .material_mut()
        .set_refractive_index(1.5);
    let shape = &w.objects().unwrap()[0];
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let xs = Intersections::new(vec![
        Intersection::new(4.0, &shape),
        Intersection::new(6.0, &shape),
    ]);
    let comps = xs[0].prepare_computations(&r, Some(&xs));
    let c = w.refracted_color(&comps, 0);
    assert_eq!(c, BLACK);
}

#[test]
fn the_refracted_color_under_total_internal_reflection() {
    let mut w = World::default();
    w.objects_mut().unwrap()[0]
        .material_mut()
        .set_transparency(1.0);
    w.objects_mut().unwrap()[0]
        .material_mut()
        .set_refractive_index(1.5);
    let shape = &w.objects().unwrap()[0];
    let r = Ray::new(
        Point::new(0.0, 0.0, -2.0_f64.sqrt() / 2.0),
        Vector::new(0.0, 1.0, 0.0),
    );
    let xs = Intersections::new(vec![
        Intersection::new(-2.0_f64.sqrt() / 2.0, &shape),
        Intersection::new(2.0_f64.sqrt() / 2.0, &shape),
    ]);
    // NOTE - This time we're inside the sphere,
    // so we need to look at the second intersection (x[1])
    let comps = xs[1].prepare_computations(&r, Some(&xs));
    let c = w.refracted_color(&comps, 5);
    assert_eq!(c, BLACK);
}

#[test]
fn the_refracted_color_with_a_refracted_ray() {
    let mut w = World::default();
    // a
    w.objects_mut().unwrap()[0].material_mut().set_ambient(1.0);
    w.objects_mut().unwrap()[0]
        .material_mut()
        .set_pattern(test_pattern());
    // b
    w.objects_mut().unwrap()[1]
        .material_mut()
        .set_transparency(1.0);
    w.objects_mut().unwrap()[1]
        .material_mut()
        .set_refractive_index(1.5);

    let a = &w.objects().unwrap()[0];
    let b = &w.objects().unwrap()[1];

    let r = Ray::new(Point::new(0.0, 0.0, 0.1), Vector::new(0.0, 1.0, 0.0));
    let xs = Intersections::new(vec![
        Intersection::new(-0.9899, &a),
        Intersection::new(-0.4899, &b),
        Intersection::new(0.4899, &b),
        Intersection::new(0.9899, &a),
    ]);
    let comps = xs[2].prepare_computations(&r, Some(&xs));
    let c = w.refracted_color(&comps, 5);
    // NOTE - Values from the book
    // assert_eq!(c, Color::new(0.0, 0.99888, 0.04725));
    // Values computed by this implementation
    assert_eq!(c, Color::new(0.0, 0.9988745506795582, 0.04721898034382347));
}
