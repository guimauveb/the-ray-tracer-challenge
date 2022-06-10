#[cfg(test)]
use {
    crate::{
        approx_eq::ApproxEq,
        rt::{camera::Camera, color::Color, matrix::Matrix, world::World},
        tuple::{point::Point, vector::Vector},
    },
    std::f32::consts::PI,
};

#[test]
fn constructing_a_camera() {
    let (hsize, vsize, field_of_view) = (160.0, 120.0, PI / 2.0);
    let c = Camera::new(hsize, vsize, field_of_view, None);
    assert_eq!(c.hsize(), hsize);
    assert_eq!(c.vsize(), vsize);
    assert_eq!(c.field_of_view(), field_of_view);
    assert_eq!(c.transform(), &Matrix::<4>::identity());
}

#[test]
fn the_pixel_size_for_a_horizontal_canvas() {
    let c = Camera::new(200.0, 125.0, PI / 2.0, None);
    println!("c: {}", c.pixel_size());
    assert!(c.pixel_size().approx_eq(0.01));
}

#[test]
fn the_pixel_size_for_a_vertical_canvas() {
    let c = Camera::new(125.0, 200.0, PI / 2.0, None);
    assert!(c.pixel_size().approx_eq(0.01));
}

#[test]
fn constructing_a_ray_through_the_center_of_the_canvas() {
    let c = Camera::new(201.0, 101.0, PI / 2.0, None);
    let r = c.ray_for_pixel(100.0, 50.0);
    assert_eq!(r.origin(), &Point::new(0.0, 0.0, 0.0));
    assert_eq!(r.direction(), &Vector::new(0.0, 0.0, -1.0));
}

#[test]
fn constructing_a_ray_through_the_corner_of_the_canvas() {
    let c = Camera::new(201.0, 101.0, PI / 2.0, None);
    let r = c.ray_for_pixel(0.0, 0.0);
    assert_eq!(r.origin(), &Point::new(0.0, 0.0, 0.0));
    assert_eq!(r.direction(), &Vector::new(0.66519, 0.33259, -0.66851));
}

#[test]
fn constructing_a_ray_when_the_camera_is_transformed() {
    let c = Camera::new(
        201.0,
        101.0,
        PI / 2.0,
        Some(Matrix::<4>::rotation_y(PI / 4.0) * Matrix::<4>::translation(0.0, -2.0, 5.0)),
    );
    let r = c.ray_for_pixel(100.0, 50.0);
    assert_eq!(r.origin(), &Point::new(0.0, 2.0, -5.0));
    assert_eq!(
        r.direction(),
        &Vector::new(2.0_f32.sqrt() / 2.0, 0.0, -2.0_f32.sqrt() / 2.0)
    );
}

#[test]
fn rendering_a_world_with_a_camera() {
    let w = World::default();
    let from = Point::new(0.0, 0.0, -5.0);
    let up = Vector::new(0.0, 1.0, 0.0);
    let to = Point::new(0.0, 0.0, 0.0);
    let c = Camera::new(
        11.0,
        11.0,
        PI / 2.0,
        Some(Matrix::<4>::view_transform(&from, &to, &up)),
    );
    let image = c.render(&w);
    assert_eq!(image.pixel_at(5, 5), &Color::new(0.38066, 0.47583, 0.2855));
}
