#[cfg(test)]
use {
    crate::{
        rt::{matrix::Matrix, ray::Ray, transform::Transform},
        tuple::{point::Point, vector::Vector},
    },
    std::f64::consts::PI,
};

// TODO
#[test]
fn constructing_a_camera() {
    let (hsize, vsize, field_of_view) = (160.0, 120.0, PI / 2.0);
    //let c = Camera::new(hsize, vsize, field_of_view);
    //assert_eq!(c.hsize(), hsize);
    //assert_eq!(c.vsize(), vsize);
    //assert_eq!(c.field_of_view(), field_of_view);
    //assert_eq!(c.transform(), &Matrix::<4>::identity());
}

// TODO
#[test]
fn the_pixel_size_for_a_horizontal_canvas() {
    //let c = Camera::new(200.0, 125.0, PI / 2.0);
    //assert_eq!(c.pixel_size(), 0.01);
}

// TODO
#[test]
fn the_pixel_size_for_a_vertical_canvas() {
    //let c = Camera::new(125.0, 200.0, PI / 2.0);
    //assert_eq!(c.pixel_size(), 0.01);
}
